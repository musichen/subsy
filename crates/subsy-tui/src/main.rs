use anyhow::Result;
use chrono::Local;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Terminal;
use std::io;
use subsy_core::calc;
use subsy_core::model::*;
use subsy_core::Store;

struct App {
    store: Store,
    subs: Vec<Subscription>,
    list_state: ListState,
    mode: Mode,
    status_msg: String,
    add_form: AddForm,
    last_total_m: String,
    last_total_y: String,
}

#[derive(PartialEq)]
enum Mode {
    List,
    Detail,
    Add,
    Help,
}

struct AddForm {
    name: String,
    provider: String,
    price: String,
    cycle: String,
    next_renewal: String,
    tags: String,
    field: usize,
}

impl AddForm {
    fn new() -> Self {
        Self {
            name: String::new(),
            provider: String::new(),
            price: String::new(),
            cycle: "monthly".into(),
            next_renewal: String::new(),
            tags: String::new(),
            field: 0,
        }
    }
    fn fields(&self) -> Vec<(&str, &str)> {
        vec![
            ("name", &self.name),
            ("provider", &self.provider),
            ("price", &self.price),
            ("billing_cycle", &self.cycle),
            ("next_renewal (YYYY-MM-DD)", &self.next_renewal),
            ("tags (comma)", &self.tags),
        ]
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let store = Store::open_default()?;
    let subs = store.list()?;
    let mut app = App {
        store,
        subs,
        list_state: ListState::default(),
        mode: Mode::List,
        status_msg: "j/k: move · enter: detail · a: add · s: summary · ?: help · q: quit".into(),
        add_form: AddForm::new(),
        last_total_m: String::new(),
        last_total_y: String::new(),
    };
    if !app.subs.is_empty() {
        app.list_state.select(Some(0));
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
            match app.mode {
                Mode::List => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('?') => app.mode = Mode::Help,
                    KeyCode::Char('j') | KeyCode::Down => move_sel(app, 1),
                    KeyCode::Char('k') | KeyCode::Up => move_sel(app, -1),
                    KeyCode::Char('g') => {
                        if !app.subs.is_empty() {
                            app.list_state.select(Some(0));
                        }
                    }
                    KeyCode::Char('G') => {
                        if !app.subs.is_empty() {
                            app.list_state.select(Some(app.subs.len() - 1));
                        }
                    }
                    KeyCode::Enter => {
                        if !app.subs.is_empty() {
                            app.mode = Mode::Detail;
                        }
                    }
                    KeyCode::Char('a') => {
                        app.mode = Mode::Add;
                        app.add_form = AddForm::new();
                    }
                    KeyCode::Char('d') => {
                        if let Some(i) = app.list_state.selected() {
                            if let Some(s) = app.subs.get(i).cloned() {
                                if app.store.delete(&s.id)? {
                                    app.subs = app.store.list()?;
                                    if app.subs.is_empty() {
                                        app.list_state.select(None);
                                    } else if i >= app.subs.len() {
                                        app.list_state.select(Some(app.subs.len() - 1));
                                    }
                                    app.status_msg = format!("removed {}", s.name);
                                }
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        let m = calc::total_monthly(&app.subs);
                        let y = calc::total_yearly(&app.subs);
                        app.last_total_m = m.to_string();
                        app.last_total_y = y.to_string();
                        app.status_msg = format!("monthly: {m}  ·  yearly: {y}");
                    }
                    _ => {}
                },
                Mode::Detail => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => app.mode = Mode::List,
                    KeyCode::Char('d') => {
                        if let Some(i) = app.list_state.selected() {
                            if let Some(s) = app.subs.get(i).cloned() {
                                if app.store.delete(&s.id)? {
                                    app.subs = app.store.list()?;
                                    app.list_state.select(None);
                                    app.mode = Mode::List;
                                    app.status_msg = format!("removed {}", s.name);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                Mode::Add => match key.code {
                    KeyCode::Esc => app.mode = Mode::List,
                    KeyCode::Tab => {
                        let n = app.add_form.fields().len();
                        app.add_form.field = (app.add_form.field + 1) % n;
                    }
                    KeyCode::BackTab => {
                        let n = app.add_form.fields().len();
                        app.add_form.field = if app.add_form.field == 0 { n - 1 } else { app.add_form.field - 1 };
                    }
                    KeyCode::Char(c) => {
                        push_to_active(&mut app.add_form, c);
                    }
                    KeyCode::Backspace => {
                        pop_from_active(&mut app.add_form);
                    }
                    KeyCode::Enter => {
                        let s = build_sub_from_form(&app.add_form);
                        if s.name.trim().is_empty() {
                            app.status_msg = "name is required".into();
                        } else {
                            app.store.add(&s)?;
                            app.subs = app.store.list()?;
                            app.list_state.select(Some(app.subs.len() - 1));
                            app.mode = Mode::List;
                            app.status_msg = format!("added {}", s.name);
                        }
                    }
                    _ => {}
                },
                Mode::Help => match key.code {
                    KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => app.mode = Mode::List,
                    _ => {}
                },
            }
        }
    }
}

fn push_to_active(f: &mut AddForm, c: char) {
    let target = match f.field {
        0 => &mut f.name,
        1 => &mut f.provider,
        2 => &mut f.price,
        3 => &mut f.cycle,
        4 => &mut f.next_renewal,
        5 => &mut f.tags,
        _ => return,
    };
    target.push(c);
}

fn pop_from_active(f: &mut AddForm) {
    let target = match f.field {
        0 => &mut f.name,
        1 => &mut f.provider,
        2 => &mut f.price,
        3 => &mut f.cycle,
        4 => &mut f.next_renewal,
        5 => &mut f.tags,
        _ => return,
    };
    target.pop();
}

fn build_sub_from_form(f: &AddForm) -> Subscription {
    let mut s = Subscription::new(f.name.clone());
    s.provider = nonempty(&f.provider);
    s.billing_cycle = BillingCycle::from_str(&f.cycle);
    if let Some(p) = f.price.trim().split_whitespace().next() {
        s.price = p.parse().ok();
    }
    s.next_renewal = parse_date(&f.next_renewal);
    s.tags = f
        .tags
        .split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
    s
}

fn nonempty(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t.to_string()) }
}

fn parse_date(s: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok()
}

fn move_sel(app: &mut App, delta: i32) {
    if app.subs.is_empty() {
        return;
    }
    let cur = app.list_state.selected().unwrap_or(0) as i32;
    let n = app.subs.len() as i32;
    let next = (cur + delta).rem_euclid(n) as usize;
    app.list_state.select(Some(next));
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(area);

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
    let header = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            " subsy ",
            Style::default().bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            "   {active} active · {ending} ending soon · monthly: {monthly} · yearly: {yearly}"
        )),
    ])])
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header, chunks[0]);

    match app.mode {
        Mode::List => render_list(f, app, chunks[1]),
        Mode::Detail => render_detail(f, app, chunks[1]),
        Mode::Add => render_add(f, app, chunks[1]),
        Mode::Help => render_help(f, chunks[1]),
    }

    let status = Paragraph::new(app.status_msg.as_str())
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(status, chunks[2]);
}

fn render_list(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let today = Local::now().date_naive();
    let items: Vec<ListItem> = app
        .subs
        .iter()
        .map(|s| {
            let badge = match s.status {
                Status::Active => Span::styled("● ", Style::default().fg(Color::Green)),
                Status::Trial => Span::styled("◐ ", Style::default().fg(Color::Cyan)),
                Status::PreCanceled => Span::styled("◍ ", Style::default().fg(Color::Yellow)),
                Status::Ended => Span::styled("○ ", Style::default().fg(Color::DarkGray)),
                Status::Unknown => Span::styled("? ", Style::default().fg(Color::DarkGray)),
            };
            let name = Span::styled(
                format!("{:<22}", truncate(&s.name, 22)),
                Style::default().add_modifier(Modifier::BOLD),
            );
            let provider = Span::raw(format!(
                "{:<14}",
                truncate(s.provider.as_deref().unwrap_or("-"), 14)
            ));
            let cycle = Span::raw(format!("{:<10}", s.billing_cycle.as_str()));
            let renewal = match s.next_renewal {
                Some(d) => {
                    let days = (d - today).num_days();
                    let txt = if days < 0 {
                        "past".to_string()
                    } else if days == 0 {
                        "today".to_string()
                    } else {
                        format!("{d} ({days}d)")
                    };
                    let color = if days <= 7 && days >= 0 {
                        Color::Yellow
                    } else if days < 0 {
                        Color::DarkGray
                    } else {
                        Color::White
                    };
                    Span::styled(format!("{txt:<14}"), Style::default().fg(color))
                }
                None => Span::raw(format!("{:<14}", "-")),
            };
            let price = s
                .price
                .map(|p| {
                    let c = s.currency.as_deref().unwrap_or("");
                    if c.is_empty() { p.to_string() } else { format!("{p} {c}") }
                })
                .unwrap_or_else(|| "-".into());
            ListItem::new(Line::from(vec![
                badge,
                name,
                provider,
                cycle,
                renewal,
                Span::raw(price),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("subscriptions"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_detail(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let i = match app.list_state.selected() {
        Some(i) => i,
        None => {
            f.render_widget(Paragraph::new("(none)"), area);
            return;
        }
    };
    let s = match app.subs.get(i) {
        Some(s) => s,
        None => return,
    };
    let mut lines: Vec<Line> = vec![];
    lines.push(Line::from(Span::styled(
        format!(" {} ", s.name),
        Style::default().bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD),
    )));
    let mut row = |k: &str, v: String| {
        lines.push(Line::from(vec![
            Span::styled(format!("{k:<14}"), Style::default().fg(Color::DarkGray)),
            Span::raw(v),
        ]));
    };
    row("id", s.id.to_string());
    row("provider", s.provider.clone().unwrap_or_else(|| "-".into()));
    row("account", s.account.clone().unwrap_or_else(|| "-".into()));
    row("plan", s.plan.clone().unwrap_or_else(|| "-".into()));
    row("status", s.status.as_str().into());
    row("billing", s.billing_cycle.as_str().into());
    let price = s
        .price
        .map(|p| match &s.currency {
            Some(c) => format!("{p} {c}"),
            None => p.to_string(),
        })
        .unwrap_or_else(|| "-".into());
    row("price", price);
    if let Some(m) = calc::monthly_cost(s) {
        row("≈ monthly", format!("{m}"));
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
    row("source", s.source.as_str().into());
    row("created", s.created_at.to_rfc3339());
    row("updated", s.updated_at.to_rfc3339());
    let p = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("detail · esc to back · d to delete"))
        .wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn render_add(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("add subscription · enter to save · esc to cancel");
    let inner = centered_rect(60, 70, area);
    f.render_widget(Clear, inner);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints((0..app.add_form.fields().len()).map(|_| Constraint::Length(2)).collect::<Vec<_>>())
        .split(inner);
    for (i, (label, value)) in app.add_form.fields().iter().enumerate() {
        let focused = i == app.add_form.field;
        let style = if focused {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let text = if focused {
            format!("> {label:<24} {value}▏")
        } else {
            format!("  {label:<24} {value}")
        };
        let p = Paragraph::new(text).style(style);
        f.render_widget(p, layout[i]);
    }
    f.render_widget(block, area);
}

fn render_help(f: &mut ratatui::Frame, area: Rect) {
    let text = "\
subsy keys
───────────
j / k         move
g / G         top / bottom
enter         open detail
a             add
d             delete
s             summary (monthly / yearly)
?             toggle this help
q / esc       quit / back";
    let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("help"));
    f.render_widget(p, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_y = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_y[1])[1]
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
