use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Bar, BarChart, BarGroup, Block, Borders, ListState, Paragraph, Row, Table, TableState, Wrap,
};
use ratatui::Terminal;
use std::collections::BTreeMap;
use std::io;
use subsy_core::calc;
use subsy_core::model::*;
use subsy_core::Store;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use uuid::Uuid;

struct App {
    store: Store,
    subs: Vec<Subscription>,
    screen: Screen,
    list: ListState,
    table: TableState,
    status: String,
    add_form: Form,
    edit_form: Option<(Uuid, Form)>,
    search: String,
    search_active: bool,
    calendar_month: NaiveDate,
    selected_day: u32,
    help_scroll: u16,
}

#[derive(Clone, Copy, PartialEq)]
enum Screen {
    Dashboard,
    List,
    Calendar,
    Analytics,
    Detail,
    Add,
    Edit,
    Help,
}

impl Screen {
    fn label(&self) -> &'static str {
        match self {
            Screen::Dashboard => "dashboard",
            Screen::List => "list",
            Screen::Calendar => "calendar",
            Screen::Analytics => "analytics",
            Screen::Detail => "detail",
            Screen::Add => "add",
            Screen::Edit => "edit",
            Screen::Help => "help",
        }
    }
}

struct Form {
    name: String,
    provider: String,
    account: String,
    plan: String,
    price: String,
    currency: String,
    cycle: String,
    status: String,
    category: String,
    payment_method: String,
    reminder_days: String,
    start: String,
    end: String,
    next_renewal: String,
    credits: String,
    url: String,
    notes: String,
    tags: String,
    field: usize,
}

impl Form {
    fn new() -> Self {
        Self {
            name: String::new(),
            provider: String::new(),
            account: String::new(),
            plan: String::new(),
            price: String::new(),
            currency: String::new(),
            cycle: "monthly".into(),
            status: "active".into(),
            category: String::new(),
            payment_method: String::new(),
            reminder_days: "7".into(),
            start: String::new(),
            end: String::new(),
            next_renewal: String::new(),
            credits: String::new(),
            url: String::new(),
            notes: String::new(),
            tags: String::new(),
            field: 0,
        }
    }

    fn from_sub(s: &Subscription) -> Self {
        Self {
            name: s.name.clone(),
            provider: s.provider.clone().unwrap_or_default(),
            account: s.account.clone().unwrap_or_default(),
            plan: s.plan.clone().unwrap_or_default(),
            price: s.price.map(|p| p.to_string()).unwrap_or_default(),
            currency: s.currency.clone().unwrap_or_default(),
            cycle: s.billing_cycle.as_str().into(),
            status: s.status.as_str().into(),
            category: s.category.clone().unwrap_or_default(),
            payment_method: s.payment_method.clone().unwrap_or_default(),
            reminder_days: s.reminder_days.map(|d| d.to_string()).unwrap_or_default(),
            start: s.start_date.map(|d| d.to_string()).unwrap_or_default(),
            end: s.end_date.map(|d| d.to_string()).unwrap_or_default(),
            next_renewal: s.next_renewal.map(|d| d.to_string()).unwrap_or_default(),
            credits: s.credits_remaining.map(|c| c.to_string()).unwrap_or_default(),
            url: s.url.clone().unwrap_or_default(),
            notes: s.notes.clone().unwrap_or_default(),
            tags: s.tags.join(", "),
            field: 0,
        }
    }

    fn fields(&self) -> Vec<(&str, &str)> {
        vec![
            ("name", &self.name),
            ("provider", &self.provider),
            ("account", &self.account),
            ("plan", &self.plan),
            ("price", &self.price),
            ("currency", &self.currency),
            ("billing_cycle", &self.cycle),
            ("status", &self.status),
            ("category", &self.category),
            ("payment_method", &self.payment_method),
            ("reminder_days", &self.reminder_days),
            ("start_date", &self.start),
            ("end_date", &self.end),
            ("next_renewal", &self.next_renewal),
            ("credits_remaining", &self.credits),
            ("url", &self.url),
            ("notes", &self.notes),
            ("tags", &self.tags),
        ]
    }

    fn apply(&self, s: &mut Subscription) {
        s.name = self.name.trim().to_string();
        s.provider = opt(&self.provider);
        s.account = opt(&self.account);
        s.plan = opt(&self.plan);
        s.price = self.price.trim().parse().ok();
        s.currency = opt(&self.currency);
        s.billing_cycle = BillingCycle::from_str(&self.cycle);
        s.status = Status::from_str(&self.status);
        s.category = opt(&self.category);
        s.payment_method = opt(&self.payment_method);
        s.reminder_days = self.reminder_days.trim().parse().ok();
        s.start_date = parse_date(&self.start);
        s.end_date = parse_date(&self.end);
        s.next_renewal = parse_date(&self.next_renewal);
        s.credits_remaining = self.credits.trim().parse().ok();
        s.url = opt(&self.url);
        s.notes = opt(&self.notes);
        s.tags = self
            .tags
            .split(',')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
    }
}

fn opt(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t.to_string()) }
}

fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok()
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let store = Store::open_default()?;
    let subs = store.list()?;
    let today = Local::now().date_naive();
    let mut app = App {
        store,
        subs,
        screen: Screen::Dashboard,
        list: ListState::default(),
        table: TableState::default(),
        status: "1-4: screens · a: add · /: search · ?: help · q: quit".into(),
        add_form: Form::new(),
        edit_form: None,
        search: String::new(),
        search_active: false,
        calendar_month: NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap(),
        selected_day: today.day(),
        help_scroll: 0,
    };
    if !app.subs.is_empty() {
        app.list.select(Some(0));
        app.table.select(Some(0));
    }

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode().ok();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .ok();
    terminal.show_cursor().ok();
    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.search_active {
                match key.code {
                    KeyCode::Esc => app.search_active = false,
                    KeyCode::Enter => app.search_active = false,
                    KeyCode::Backspace => {
                        app.search.pop();
                    }
                    KeyCode::Char(c) => app.search.push(c),
                    _ => {}
                }
                continue;
            }

            match app.screen {
                Screen::Dashboard
                | Screen::List
                | Screen::Calendar
                | Screen::Analytics => handle_nav(app, key.code)?,
                Screen::Detail => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => app.screen = Screen::List,
                    KeyCode::Char('e') => start_edit(app)?,
                    KeyCode::Char('d') => delete_selected(app)?,
                    _ => handle_nav(app, key.code)?,
                },
                Screen::Add => handle_form(app, key.code, false)?,
                Screen::Edit => handle_form(app, key.code, true)?,
                Screen::Help => match key.code {
                    KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
                        app.screen = Screen::Dashboard
                    }
                    KeyCode::Char('j') | KeyCode::Down => app.help_scroll += 1,
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.help_scroll = app.help_scroll.saturating_sub(1)
                    }
                    _ => {}
                },
            }
        }
    }
}

fn handle_nav(app: &mut App, code: KeyCode) -> Result<()> {
    match code {
        KeyCode::Char('q') => {
            std::process::exit(0);
        }
        KeyCode::Char('?') => app.screen = Screen::Help,
        KeyCode::Char('1') => app.screen = Screen::Dashboard,
        KeyCode::Char('2') => app.screen = Screen::List,
        KeyCode::Char('3') => app.screen = Screen::Calendar,
        KeyCode::Char('4') => app.screen = Screen::Analytics,
        KeyCode::Char('a') => {
            app.add_form = Form::new();
            app.screen = Screen::Add;
        }
        KeyCode::Char('/') => app.search_active = true,
        KeyCode::Char('s') => {
            let m = calc::total_monthly(&app.subs);
            let y = calc::total_yearly(&app.subs);
            app.status = format!("monthly: {m} · yearly: {y}");
        }
        _ => match app.screen {
            Screen::List => handle_list_nav(app, code)?,
            Screen::Calendar => handle_calendar_nav(app, code),
            Screen::Analytics => handle_table_nav(app, code),
            _ => {}
        },
    }
    Ok(())
}

fn handle_list_nav(app: &mut App, code: KeyCode) -> Result<()> {
    let filtered = filtered_subs(app);
    match code {
        KeyCode::Char('j') | KeyCode::Down => move_list(&mut app.list, &filtered, 1),
        KeyCode::Char('k') | KeyCode::Up => move_list(&mut app.list, &filtered, -1),
        KeyCode::Char('g') => app.list.select(Some(0)),
        KeyCode::Char('G') => {
            if !filtered.is_empty() {
                app.list.select(Some(filtered.len() - 1));
            }
        }
        KeyCode::Enter => {
            if !filtered.is_empty() {
                app.screen = Screen::Detail;
            }
        }
        KeyCode::Char('d') => {
            if let Some(s) = selected_sub(app) {
                let _ = app.store.delete(&s.id);
                refresh(app)?;
                app.status = format!("removed {}", s.name);
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_calendar_nav(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('h') | KeyCode::Left => {
            app.calendar_month = app
                .calendar_month
                .checked_sub_months(chrono::Months::new(1))
                .unwrap_or(app.calendar_month);
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app.calendar_month = app
                .calendar_month
                .checked_add_months(chrono::Months::new(1))
                .unwrap_or(app.calendar_month);
        }
        KeyCode::Char('j') | KeyCode::Down => app.selected_day = (app.selected_day + 7).min(days_in_month(app.calendar_month)),
        KeyCode::Char('k') | KeyCode::Up => app.selected_day = app.selected_day.saturating_sub(7).max(1),
        _ => {}
    }
}

fn handle_table_nav(app: &mut App, code: KeyCode) {
    let rows = analytics_rows(app);
    match code {
        KeyCode::Char('j') | KeyCode::Down => {
            let i = app.table.selected().unwrap_or(0);
            if i + 1 < rows.len() {
                app.table.select(Some(i + 1));
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let i = app.table.selected().unwrap_or(0);
            if i > 0 {
                app.table.select(Some(i - 1));
            }
        }
        _ => {}
    }
}

fn handle_form(app: &mut App, code: KeyCode, editing: bool) -> Result<()> {
    let form = if editing {
        &mut app.edit_form.as_mut().unwrap().1
    } else {
        &mut app.add_form
    };
    let n = form.fields().len();
    match code {
        KeyCode::Esc => app.screen = Screen::List,
        KeyCode::Tab => form.field = (form.field + 1) % n,
        KeyCode::BackTab => form.field = if form.field == 0 { n - 1 } else { form.field - 1 },
        KeyCode::Char(c) => push_field(form, c),
        KeyCode::Backspace => pop_field(form),
        KeyCode::Enter => {
            if editing {
                let (id, f) = app.edit_form.take().unwrap();
                if let Some(mut s) = app.store.get(&id)? {
                    f.apply(&mut s);
                    s.updated_at = chrono::Utc::now();
                    app.store.update(&s)?;
                    refresh(app)?;
                    app.status = format!("updated {}", s.name);
                }
                app.screen = Screen::List;
            } else {
                let mut s = Subscription::new("");
                app.add_form.apply(&mut s);
                if s.name.trim().is_empty() {
                    app.status = "name is required".into();
                } else {
                    s.source = Source::Manual;
                    app.store.add(&s)?;
                    refresh(app)?;
                    app.screen = Screen::List;
                    app.status = format!("added {}", s.name);
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn push_field(f: &mut Form, c: char) {
    let target = match f.field {
        0 => &mut f.name,
        1 => &mut f.provider,
        2 => &mut f.account,
        3 => &mut f.plan,
        4 => &mut f.price,
        5 => &mut f.currency,
        6 => &mut f.cycle,
        7 => &mut f.status,
        8 => &mut f.category,
        9 => &mut f.payment_method,
        10 => &mut f.reminder_days,
        11 => &mut f.start,
        12 => &mut f.end,
        13 => &mut f.next_renewal,
        14 => &mut f.credits,
        15 => &mut f.url,
        16 => &mut f.notes,
        17 => &mut f.tags,
        _ => return,
    };
    target.push(c);
}

fn pop_field(f: &mut Form) {
    let target = match f.field {
        0 => &mut f.name,
        1 => &mut f.provider,
        2 => &mut f.account,
        3 => &mut f.plan,
        4 => &mut f.price,
        5 => &mut f.currency,
        6 => &mut f.cycle,
        7 => &mut f.status,
        8 => &mut f.category,
        9 => &mut f.payment_method,
        10 => &mut f.reminder_days,
        11 => &mut f.start,
        12 => &mut f.end,
        13 => &mut f.next_renewal,
        14 => &mut f.credits,
        15 => &mut f.url,
        16 => &mut f.notes,
        17 => &mut f.tags,
        _ => return,
    };
    target.pop();
}

fn start_edit(app: &mut App) -> Result<()> {
    if let Some(s) = selected_sub(app) {
        app.edit_form = Some((s.id, Form::from_sub(&s)));
        app.screen = Screen::Edit;
    }
    Ok(())
}

fn delete_selected(app: &mut App) -> Result<()> {
    if let Some(s) = selected_sub(app) {
        app.store.delete(&s.id)?;
        refresh(app)?;
        app.status = format!("removed {}", s.name);
    }
    Ok(())
}

fn refresh(app: &mut App) -> Result<()> {
    app.subs = app.store.list()?;
    if app.subs.is_empty() {
        app.list.select(None);
    } else if app.list.selected().unwrap_or(0) >= app.subs.len() {
        app.list.select(Some(app.subs.len() - 1));
    } else if app.list.selected().is_none() {
        app.list.select(Some(0));
    }
    Ok(())
}

fn selected_sub(app: &App) -> Option<Subscription> {
    let filtered = filtered_subs(app);
    app.list
        .selected()
        .and_then(|i| filtered.get(i).cloned())
        .or_else(|| app.subs.first().cloned())
}

fn filtered_subs(app: &App) -> Vec<Subscription> {
    let q = app.search.to_lowercase();
    app.subs
        .iter()
        .filter(|s| {
            if q.is_empty() {
                true
            } else {
                s.name.to_lowercase().contains(&q)
                    || s.provider.as_deref().unwrap_or("").to_lowercase().contains(&q)
                    || s.category.as_deref().unwrap_or("").to_lowercase().contains(&q)
                    || s.tags.iter().any(|t| t.to_lowercase().contains(&q))
            }
        })
        .cloned()
        .collect()
}

fn move_list(state: &mut ListState, items: &[Subscription], delta: i32) {
    if items.is_empty() {
        return;
    }
    let cur = state.selected().unwrap_or(0) as i32;
    let n = items.len() as i32;
    let next = (cur + delta).rem_euclid(n) as usize;
    state.select(Some(next));
}

fn days_in_month(d: NaiveDate) -> u32 {
    let next = d.checked_add_months(chrono::Months::new(1)).unwrap();
    (NaiveDate::from_ymd_opt(next.year(), next.month(), 1).unwrap() - chrono::Days::new(1))
        .day()
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(5), Constraint::Length(1)])
        .split(area);

    render_tabs(f, app, chunks[0]);

    if app.search_active {
        let search = Paragraph::new(format!("/{}", app.search))
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::BOTTOM));
        f.render_widget(search, chunks[0]);
    }

    match app.screen {
        Screen::Dashboard => render_dashboard(f, app, chunks[1]),
        Screen::List => render_list(f, app, chunks[1]),
        Screen::Calendar => render_calendar(f, app, chunks[1]),
        Screen::Analytics => render_analytics(f, app, chunks[1]),
        Screen::Detail => render_detail(f, app, chunks[1]),
        Screen::Add => render_form(f, app, chunks[1], "add subscription"),
        Screen::Edit => render_form(f, app, chunks[1], "edit subscription"),
        Screen::Help => render_help(f, app, chunks[1]),
    }

    let status = Paragraph::new(app.status.as_str()).style(Style::default().fg(Color::DarkGray));
    f.render_widget(status, chunks[2]);
}

fn render_tabs(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let screens = [Screen::Dashboard, Screen::List, Screen::Calendar, Screen::Analytics];
    let spans: Vec<Span> = screens
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let active = *s == app.screen;
            let style = if active {
                Style::default().bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Span::styled(format!(" {} {} ", i + 1, s.label()), style)
        })
        .collect();
    let line = Line::from(spans);
    let p = Paragraph::new(line).block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(p, area);
}

fn render_dashboard(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let today = Local::now().date_naive();
    let monthly = calc::total_monthly(&app.subs);
    let yearly = calc::total_yearly(&app.subs);
    let active = app
        .subs
        .iter()
        .filter(|s| matches!(s.status, Status::Active | Status::Trial))
        .count();
    let ending = app
        .subs
        .iter()
        .filter(|s| calc::is_ending_soon(s, today, 7))
        .count();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(5)])
        .split(area);

    // Cards
    let cards = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(chunks[0]);

    card(f, cards[0], "active", &active.to_string(), Color::Green);
    card(f, cards[1], "ending soon", &ending.to_string(), Color::Yellow);
    card(f, cards[2], "monthly", &format!("{:.2}", monthly), Color::Cyan);
    card(f, cards[3], "yearly", &format!("{:.2}", yearly), Color::Magenta);

    // Bottom: upcoming renewals + recent
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(chunks[1]);

    let mut upcoming: Vec<_> = app
        .subs
        .iter()
        .filter(|s| s.next_renewal.is_some())
        .cloned()
        .collect();
    upcoming.sort_by_key(|s| s.next_renewal.unwrap());
    let up_lines: Vec<Line> = upcoming
        .iter()
        .take(12)
        .map(|s| {
            let d = s.next_renewal.unwrap();
            let days = (d - today).num_days();
            let color = if days < 0 {
                Color::DarkGray
            } else if days <= 7 {
                Color::Yellow
            } else {
                Color::White
            };
            Line::from(vec![
                Span::styled(format!("{:<12}", d.to_string()), Style::default().fg(color)),
                Span::raw(format!("{:<22}", truncate(&s.name, 22))),
                Span::raw(s.price.map(|p| format!("{p}")).unwrap_or_default()),
            ])
        })
        .collect();
    let up = Paragraph::new(Text::from(up_lines))
        .block(Block::default().borders(Borders::ALL).title("upcoming renewals"));
    f.render_widget(up, bottom[0]);

    let mut recent = app.subs.clone();
    recent.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    let recent_lines: Vec<Line> = recent
        .iter()
        .take(12)
        .map(|s| {
            Line::from(vec![
                Span::raw(format!("{:<22}", truncate(&s.name, 22))),
                Span::styled(
                    s.category.as_deref().unwrap_or("-"),
                    Style::default().fg(Color::Cyan),
                ),
            ])
        })
        .collect();
    let recent_w = Paragraph::new(Text::from(recent_lines))
        .block(Block::default().borders(Borders::ALL).title("recently added"));
    f.render_widget(recent_w, bottom[1]);
}

fn card(f: &mut ratatui::Frame, area: Rect, title: &str, value: &str, color: Color) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(title);
    let inner = block.inner(area);
    f.render_widget(block, area);
    let p = Paragraph::new(value)
        .alignment(Alignment::Center)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD));
    f.render_widget(p, inner);
}

fn render_list(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let today = Local::now().date_naive();
    let filtered = filtered_subs(app);

    let header = Row::new(vec!["", "name", "provider", "category", "cycle", "renewal", "price"])
        .style(Style::default().fg(Color::DarkGray))
        .bottom_margin(0);
    let rows: Vec<Row> = filtered
        .iter()
        .map(|s| {
            let badge = match s.status {
                Status::Active => "●",
                Status::Trial => "◐",
                Status::PreCanceled => "◍",
                Status::Ended => "○",
                Status::Unknown => "?",
            };
            let renewal = match s.next_renewal {
                Some(d) => {
                    let days = (d - today).num_days();
                    let txt = if days < 0 {
                        "past".into()
                    } else if days == 0 {
                        "today".into()
                    } else {
                        format!("{d} ({days}d)")
                    };
                    txt
                }
                None => "-".into(),
            };
            let price = s
                .price
                .map(|p| match &s.currency {
                    Some(c) => format!("{p} {c}"),
                    None => p.to_string(),
                })
                .unwrap_or_else(|| "-".into());
            Row::new(vec![
                badge.to_string(),
                truncate(&s.name, 24),
                truncate(s.provider.as_deref().unwrap_or("-"), 14),
                truncate(s.category.as_deref().unwrap_or("-"), 12),
                s.billing_cycle.as_str().into(),
                renewal,
                price,
            ])
        })
        .collect();

    let search_title = if app.search.is_empty() {
        String::new()
    } else {
        format!(" · search: {}", app.search)
    };
    let table = Table::new(rows, [
        Constraint::Length(2),
        Constraint::Min(18),
        Constraint::Length(14),
        Constraint::Length(12),
        Constraint::Length(10),
        Constraint::Length(16),
        Constraint::Length(12),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(
        "subscriptions ({}){}",
        filtered.len(),
        search_title
    )))
    .row_highlight_style(Style::default().bg(Color::Rgb(30, 30, 30)));

    // need TableState; use app.table but reset when screen changes is ok
    f.render_stateful_widget(table, area, &mut app.table);
}

fn render_calendar(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let month = app.calendar_month;
    let first = NaiveDate::from_ymd_opt(month.year(), month.month(), 1).unwrap();
    let mut start_weekday = first.weekday().num_days_from_sunday() as usize;
    if start_weekday == 7 {
        start_weekday = 0;
    }
    let days = days_in_month(month);

    let title = format!("{} {}", month.format("%B"), month.year());
    let block = Block::default().borders(Borders::ALL).title(title);
    let inner = block.inner(area);
    f.render_widget(block, area);

    // renewals map
    let mut renewals: BTreeMap<u32, Vec<&Subscription>> = BTreeMap::new();
    for s in &app.subs {
        if let Some(d) = s.next_renewal {
            if d.year() == month.year() && d.month() == month.month() {
                renewals.entry(d.day()).or_default().push(s);
            }
        }
    }

    let grid = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2); 7])
        .split(inner);

    let days_header = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let header_cells: Vec<Span> = days_header
        .iter()
        .map(|d| Span::styled(format!(" {:^4}", d), Style::default().fg(Color::DarkGray)))
        .collect();
    f.render_widget(
        Paragraph::new(Line::from(header_cells)),
        grid[0],
    );

    let mut day = 1usize;
    'outer: for week in 1..7 {
        let mut cells: Vec<Span> = vec![];
        for wd in 0..7 {
            if week == 1 && wd < start_weekday {
                cells.push(Span::raw("     "));
                continue;
            }
            if day > days as usize {
                break 'outer;
            }
            let has = renewals.contains_key(&(day as u32));
            let selected = day as u32 == app.selected_day;
            let style = if selected {
                Style::default().bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD)
            } else if has {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let marker = if has { "●" } else { " " };
            cells.push(Span::styled(format!("{}{:<3}", marker, day), style));
            day += 1;
        }
        f.render_widget(Paragraph::new(Line::from(cells)), grid[week]);
    }

    // selected day renewals
    let info_area = inner.inner(Margin::new(0, inner.height.saturating_sub(8)));
    if let Some(subs) = renewals.get(&app.selected_day) {
        let lines: Vec<Line> = subs
            .iter()
            .map(|s| {
                Line::from(vec![
                    Span::raw(format!("{:<24}", truncate(&s.name, 24))),
                    Span::styled(
                        s.price.map(|p| format!("{p}")).unwrap_or_default(),
                        Style::default().fg(Color::Cyan),
                    ),
                ])
            })
            .collect();
        f.render_widget(
            Paragraph::new(Text::from(lines))
                .block(Block::default().borders(Borders::TOP).title("renewals on selected day")),
            info_area,
        );
    }
}

fn render_analytics(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);

    // Monthly bar chart (last 6 months)
    let today = Local::now().date_naive();
    let mut bars = vec![];
    for i in (0..6).rev() {
        let m = today.checked_sub_months(chrono::Months::new(i)).unwrap();
        let label = format!("{}-{}", m.year(), m.month());
        let value = calc::total_monthly(&app.subs).to_i64().unwrap_or(0).max(0) as u64;
        bars.push(Bar::default().label(label.into()).value(value));
    }
    let bar_chart = BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::default().borders(Borders::ALL).title("projected monthly spend"))
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Black));
    f.render_widget(bar_chart, chunks[0]);

    // Category table
    let rows = analytics_rows(app);
    let table = Table::new(
        rows,
        [Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)],
    )
    .header(
        Row::new(vec!["category", "monthly"])
            .style(Style::default().fg(Color::DarkGray)),
    )
    .block(Block::default().borders(Borders::ALL).title("spend by category"))
    .row_highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(table, chunks[1], &mut app.table);
}

fn analytics_rows(app: &App) -> Vec<Row<'static>> {
    let mut by_cat: BTreeMap<String, Decimal> = BTreeMap::new();
    for s in &app.subs {
        let cat = s.category.clone().unwrap_or_else(|| "uncategorized".into());
        if let Some(m) = calc::monthly_cost(s) {
            *by_cat.entry(cat).or_default() += m;
        }
    }
    let mut items: Vec<_> = by_cat.into_iter().collect();
    items.sort_by(|a, b| a.0.cmp(&b.0));
    items
        .into_iter()
        .map(|(cat, total)| Row::new(vec![cat, format!("{:.2}", total)]))
        .collect()
}

fn render_detail(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let s = match selected_sub(app) {
        Some(s) => s,
        None => {
            f.render_widget(Paragraph::new("(no subscriptions)"), area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);

    let mut lines = vec![Line::from(Span::styled(
        format!(" {} ", s.name),
        Style::default().bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD),
    ))];
    let mut row = |k: &str, v: String| {
        lines.push(Line::from(vec![
            Span::styled(format!("{:<16}", k), Style::default().fg(Color::DarkGray)),
            Span::raw(v),
        ]));
    };
    row("provider", s.provider.clone().unwrap_or_else(|| "-".into()));
    row("account", s.account.clone().unwrap_or_else(|| "-".into()));
    row("plan", s.plan.clone().unwrap_or_else(|| "-".into()));
    row("status", s.status.as_str().into());
    row("billing", s.billing_cycle.as_str().into());
    row("category", s.category.clone().unwrap_or_else(|| "-".into()));
    row("payment method", s.payment_method.clone().unwrap_or_else(|| "-".into()));
    row(
        "reminder",
        s.reminder_days
            .map(|d| format!("{d} days before"))
            .unwrap_or_else(|| "-".into()),
    );
    let price = s
        .price
        .map(|p| match &s.currency {
            Some(c) => format!("{p} {c}"),
            None => p.to_string(),
        })
        .unwrap_or_else(|| "-".into());
    row("price", price);
    if let Some(m) = calc::monthly_cost(&s) {
        row("≈ monthly", format!("{:.2}", m));
    }
    row("start", s.start_date.map(|d| d.to_string()).unwrap_or_else(|| "-".into()));
    row("end", s.end_date.map(|d| d.to_string()).unwrap_or_else(|| "-".into()));
    row(
        "next renewal",
        s.next_renewal.map(|d| d.to_string()).unwrap_or_else(|| "-".into()),
    );
    if let Some(c) = s.credits_remaining {
        row("credits", c.to_string());
    }
    row("url", s.url.clone().unwrap_or_else(|| "-".into()));
    row("notes", s.notes.clone().unwrap_or_else(|| "-".into()));
    row("tags", if s.tags.is_empty() { "-".into() } else { s.tags.join(", ") });
    let p = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("detail · e: edit · d: delete · esc: back"))
        .wrap(Wrap { trim: true });
    f.render_widget(p, chunks[0]);

    // Payments panel
    let mut pay_lines = vec![Line::from(Span::styled(
        "payments",
        Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
    ))];
    if let Ok(paid) = app.store.paid_so_far(&s.id) {
        pay_lines.push(Line::from(vec![
            Span::styled("paid so far: ", Style::default().fg(Color::DarkGray)),
            Span::raw(format!("{:.2}", paid)),
        ]));
    }
    if let Ok(payments) = app.store.list_payments(&s.id) {
        if payments.is_empty() {
            pay_lines.push(Line::from("no payments recorded"));
        } else {
            for p in payments.iter().take(20) {
                let c = p.currency.as_deref().unwrap_or("");
                pay_lines.push(Line::from(format!("{}  {}  {}", p.date, p.amount, c)));
            }
        }
    }
    let pay = Paragraph::new(Text::from(pay_lines))
        .block(Block::default().borders(Borders::ALL).title("history"))
        .wrap(Wrap { trim: true });
    f.render_widget(pay, chunks[1]);
}

fn render_form(f: &mut ratatui::Frame, app: &mut App, area: Rect, title: &str) {
    let form = if app.screen == Screen::Edit {
        &app.edit_form.as_ref().unwrap().1
    } else {
        &app.add_form
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("{title} · tab: next · enter: save · esc: cancel"));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints((0..form.fields().len()).map(|_| Constraint::Length(1)).collect::<Vec<_>>())
        .split(inner);
    for (i, (label, value)) in form.fields().iter().enumerate() {
        let focused = i == form.field;
        let style = if focused {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let prefix = if focused { "> " } else { "  " };
        let text = format!("{}{:<24} {}", prefix, label, value);
        let p = Paragraph::new(text).style(style);
        f.render_widget(p, rows[i]);
    }
}

fn render_help(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let text = format!(
        "\
subsy TUI keys
───────────────
1              dashboard
2              subscriptions list
3              calendar
4              analytics

List
j / k          move up/down
g / G          top / bottom
enter          detail
a              add subscription
e              edit subscription (in detail)
d              delete subscription
/              search

Calendar
h / l          prev/next month
j / k          move week up/down

Detail
e              edit
d              delete
esc            back

Global
?              this help
q              quit

{} matches for search: '{}'",
        if app.search.is_empty() { 0 } else { filtered_subs(app).len() },
        app.search
    );
    let p = Paragraph::new(text)
        .scroll((app.help_scroll, 0))
        .block(Block::default().borders(Borders::ALL).title("help · esc: close"));
    f.render_widget(p, area);
}


fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n - 1).collect();
        out.push('…');
        out
    }
}
