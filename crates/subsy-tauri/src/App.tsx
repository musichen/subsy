import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Subscription {
  id: string;
  name: string;
  provider?: string;
  price?: string;
  currency?: string;
  billing_cycle: string;
  status: string;
  category?: string;
  payment_method?: string;
  next_renewal?: string;
  tags: string[];
  paid_so_far: string;
}

interface Summary {
  count: number;
  active: number;
  ending: number;
  monthly: string;
  yearly: string;
}

type View = "dashboard" | "subscriptions" | "calendar" | "analytics";

function App() {
  const [view, setView] = useState<View>("dashboard");
  const [subs, setSubs] = useState<Subscription[]>([]);
  const [summary, setSummary] = useState<Summary>({ count: 0, active: 0, ending: 0, monthly: "0", yearly: "0" });
  const [upcoming, setUpcoming] = useState<any[]>([]);
  const [categories, setCategories] = useState<any[]>([]);

  const [name, setName] = useState("");
  const [provider, setProvider] = useState("");
  const [price, setPrice] = useState("");
  const [category, setCategory] = useState("");
  const [nextRenewal, setNextRenewal] = useState("");

  const load = async () => {
    const [s, sm, up, cat] = await Promise.all([
      invoke<Subscription[]>("list_subscriptions"),
      invoke<Summary>("get_summary"),
      invoke<any[]>("get_upcoming"),
      invoke<any[]>("get_categories"),
    ]);
    setSubs(s);
    setSummary(sm);
    setUpcoming(up);
    setCategories(cat);
  };

  useEffect(() => {
    load();
  }, []);

  const add = async () => {
    if (!name.trim()) return;
    await invoke("add_subscription", {
      name,
      provider: provider || null,
      price: price || null,
      currency: "USD",
      billing_cycle: "monthly",
      category: category || null,
      payment_method: null,
      next_renewal: nextRenewal || null,
      tags: null,
    });
    setName("");
    setProvider("");
    setPrice("");
    setCategory("");
    setNextRenewal("");
    await load();
  };

  const remove = async (id: string) => {
    await invoke("delete_subscription", { id });
    await load();
  };

  const navBtn = (key: View, label: string) => (
    <button
      key={key}
      onClick={() => setView(key)}
      className={`px-4 py-2 rounded-lg font-medium transition ${
        view === key
          ? "bg-green-500 text-slate-900"
          : "bg-slate-800 text-slate-300 hover:bg-slate-700"
      }`}
    >
      {label}
    </button>
  );

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 p-4 md:p-6">
      <header className="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-6">
        <div>
          <h1 className="text-3xl font-bold text-green-400">subsy</h1>
          <p className="text-slate-400 text-sm">
            {summary.active} active · {summary.ending} ending soon · ${summary.monthly}/mo · ${summary.yearly}/yr
          </p>
        </div>
        <nav className="flex gap-2 flex-wrap">
          {navBtn("dashboard", "Dashboard")}
          {navBtn("subscriptions", "Subscriptions")}
          {navBtn("calendar", "Calendar")}
          {navBtn("analytics", "Analytics")}
        </nav>
      </header>

      {view === "dashboard" && (
        <Dashboard summary={summary} upcoming={upcoming} subs={subs} />
      )}
      {view === "subscriptions" && (
        <Subscriptions
          subs={subs}
          name={name}
          setName={setName}
          provider={provider}
          setProvider={setProvider}
          price={price}
          setPrice={setPrice}
          category={category}
          setCategory={setCategory}
          nextRenewal={nextRenewal}
          setNextRenewal={setNextRenewal}
          add={add}
          remove={remove}
        />
      )}
      {view === "calendar" && <Calendar subs={subs} />}
      {view === "analytics" && <Analytics categories={categories} monthly={summary.monthly} />}
    </div>
  );
}

function Dashboard({ summary, upcoming, subs }: { summary: Summary; upcoming: any[]; subs: Subscription[] }) {
  const recent = useMemo(() => [...subs].sort((a, b) => b.id.localeCompare(a.id)).slice(0, 8), [subs]);
  return (
    <div className="space-y-6">
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <Card title="Active" value={summary.active.toString()} color="green" />
        <Card title="Ending soon" value={summary.ending.toString()} color="yellow" />
        <Card title="Monthly" value={`$${summary.monthly}`} color="cyan" />
        <Card title="Yearly" value={`$${summary.yearly}`} color="purple" />
      </div>

      <div className="grid md:grid-cols-2 gap-6">
        <section className="bg-slate-900 rounded-xl p-4">
          <h2 className="text-lg font-semibold mb-3 text-slate-200">Upcoming renewals</h2>
          <div className="space-y-2">
            {upcoming.slice(0, 10).map((u) => (
              <div key={u.id} className="flex justify-between items-center bg-slate-800/50 rounded-lg px-3 py-2">
                <div>
                  <div className="font-medium">{u.name}</div>
                  <div className="text-xs text-slate-400">{u.date}</div>
                </div>
                <div className={`text-sm font-medium ${u.days <= 7 ? "text-yellow-400" : "text-slate-300"}`}>
                  {u.days === 0 ? "today" : `${u.days}d`}
                </div>
              </div>
            ))}
            {upcoming.length === 0 && <p className="text-slate-500 text-sm">No upcoming renewals</p>}
          </div>
        </section>

        <section className="bg-slate-900 rounded-xl p-4">
          <h2 className="text-lg font-semibold mb-3 text-slate-200">Recently added</h2>
          <div className="space-y-2">
            {recent.map((s) => (
              <div key={s.id} className="flex justify-between items-center bg-slate-800/50 rounded-lg px-3 py-2">
                <div>
                  <div className="font-medium">{s.name}</div>
                  <div className="text-xs text-slate-400">{s.provider || "-"} · {s.category || "-"}</div>
                </div>
                <div className="text-sm text-cyan-400">
                  {s.price ? `$${s.price}` : "-"}
                </div>
              </div>
            ))}
          </div>
        </section>
      </div>
    </div>
  );
}

function Card({ title, value, color }: { title: string; value: string; color: "green" | "yellow" | "cyan" | "purple" }) {
  const colors = {
    green: "border-green-500/50 text-green-400",
    yellow: "border-yellow-500/50 text-yellow-400",
    cyan: "border-cyan-500/50 text-cyan-400",
    purple: "border-purple-500/50 text-purple-400",
  };
  return (
    <div className={`bg-slate-900 rounded-xl p-4 border ${colors[color]}`}>
      <div className="text-xs uppercase tracking-wider text-slate-400 mb-1">{title}</div>
      <div className="text-2xl font-bold">{value}</div>
    </div>
  );
}

function Subscriptions({
  subs,
  name, setName,
  provider, setProvider,
  price, setPrice,
  category, setCategory,
  nextRenewal, setNextRenewal,
  add, remove,
}: any) {
  return (
    <div className="space-y-6">
      <section className="bg-slate-900 rounded-xl p-4">
        <h2 className="text-lg font-semibold mb-3">Add subscription</h2>
        <div className="grid grid-cols-2 md:grid-cols-6 gap-3">
          <input className="bg-slate-800 rounded-lg px-3 py-2 text-sm" placeholder="Name" value={name} onChange={(e) => setName(e.target.value)} />
          <input className="bg-slate-800 rounded-lg px-3 py-2 text-sm" placeholder="Provider" value={provider} onChange={(e) => setProvider(e.target.value)} />
          <input className="bg-slate-800 rounded-lg px-3 py-2 text-sm" placeholder="Price" value={price} onChange={(e) => setPrice(e.target.value)} />
          <input className="bg-slate-800 rounded-lg px-3 py-2 text-sm" placeholder="Category" value={category} onChange={(e) => setCategory(e.target.value)} />
          <input className="bg-slate-800 rounded-lg px-3 py-2 text-sm" type="date" placeholder="Next renewal" value={nextRenewal} onChange={(e) => setNextRenewal(e.target.value)} />
          <button onClick={add} className="bg-green-500 hover:bg-green-400 text-slate-900 font-bold rounded-lg px-4 py-2 text-sm">Add</button>
        </div>
      </section>

      <section className="bg-slate-900 rounded-xl p-4">
        <h2 className="text-lg font-semibold mb-3">Subscriptions ({subs.length})</h2>
        <div className="space-y-2">
          {subs.map((s: Subscription) => (
            <div key={s.id} className="flex items-center justify-between bg-slate-800/50 rounded-lg px-4 py-3">
              <div className="flex items-center gap-3">
                <StatusDot status={s.status} />
                <div>
                  <div className="font-medium">{s.name}</div>
                  <div className="text-xs text-slate-400">
                    {s.provider || "-"} · {s.category || "-"} · {s.billing_cycle}
                    {s.next_renewal ? ` · renews ${s.next_renewal}` : ""}
                  </div>
                </div>
              </div>
              <div className="flex items-center gap-4">
                <div className="text-right">
                  <div className="font-medium text-cyan-400">
                    {s.price ? `$${s.price} ${s.currency || ""}` : "-"}
                  </div>
                  <div className="text-xs text-slate-500">paid: ${s.paid_so_far}</div>
                </div>
                <button onClick={() => remove(s.id)} className="text-red-400 hover:text-red-300 text-sm">delete</button>
              </div>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}

function StatusDot({ status }: { status: string }) {
  const colors: Record<string, string> = {
    active: "bg-green-500",
    trial: "bg-cyan-500",
    precanceled: "bg-yellow-500",
    ended: "bg-slate-500",
    unknown: "bg-slate-500",
  };
  return <div className={`w-2.5 h-2.5 rounded-full ${colors[status] || colors.unknown}`} />;
}

function Calendar({ subs }: { subs: Subscription[] }) {
  const [month, setMonth] = useState(() => {
    const d = new Date();
    return new Date(d.getFullYear(), d.getMonth(), 1);
  });

  const daysInMonth = new Date(month.getFullYear(), month.getMonth() + 1, 0).getDate();
  const startDay = month.getDay();

  const renewals = useMemo(() => {
    const map: Record<number, Subscription[]> = {};
    for (const s of subs) {
      if (!s.next_renewal) continue;
      const d = new Date(s.next_renewal + "T00:00:00");
      if (d.getFullYear() === month.getFullYear() && d.getMonth() === month.getMonth()) {
        const day = d.getDate();
        if (!map[day]) map[day] = [];
        map[day].push(s);
      }
    }
    return map;
  }, [subs, month]);

  const selected = useMemo(() => {
    const d = new Date();
    return Math.min(d.getDate(), daysInMonth);
  }, [daysInMonth]);

  const cells = [];
  for (let i = 0; i < startDay; i++) cells.push(null);
  for (let d = 1; d <= daysInMonth; d++) cells.push(d);

  return (
    <div className="bg-slate-900 rounded-xl p-4">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold">
          {month.toLocaleDateString("en-US", { month: "long", year: "numeric" })}
        </h2>
        <div className="flex gap-2">
          <button onClick={() => setMonth(new Date(month.getFullYear(), month.getMonth() - 1, 1))} className="bg-slate-800 px-3 py-1 rounded hover:bg-slate-700">&larr;</button>
          <button onClick={() => setMonth(new Date(month.getFullYear(), month.getMonth() + 1, 1))} className="bg-slate-800 px-3 py-1 rounded hover:bg-slate-700">&rarr;</button>
        </div>
      </div>

      <div className="grid grid-cols-7 gap-2 mb-2">
        {["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"].map((d) => (
          <div key={d} className="text-center text-xs text-slate-500 font-medium">{d}</div>
        ))}
      </div>
      <div className="grid grid-cols-7 gap-2">
        {cells.map((day, idx) => (
          <div
            key={idx}
            className={`aspect-square rounded-lg flex flex-col items-center justify-center text-sm relative ${
              day === null ? "bg-transparent" : day === selected ? "bg-green-500 text-slate-900 font-bold" : "bg-slate-800 text-slate-200 hover:bg-slate-700"
            }`}
          >
            {day}
            {day && renewals[day] && (
              <div className="absolute bottom-1 w-1.5 h-1.5 rounded-full bg-yellow-400" />
            )}
          </div>
        ))}
      </div>

      <div className="mt-4">
        <h3 className="text-sm font-medium text-slate-400 mb-2">Renewals on selected day</h3>
        <div className="space-y-1">
          {(renewals[selected] || []).map((s) => (
            <div key={s.id} className="flex justify-between bg-slate-800/50 rounded px-3 py-2">
              <span>{s.name}</span>
              <span className="text-cyan-400">{s.price ? `$${s.price}` : "-"}</span>
            </div>
          ))}
          {!renewals[selected] && <p className="text-slate-500 text-sm">No renewals</p>}
        </div>
      </div>
    </div>
  );
}

function Analytics({ categories, monthly }: { categories: any[]; monthly: string }) {
  const max = useMemo(() => {
    return Math.max(...categories.map((c) => parseFloat(c.monthly) || 0), 1);
  }, [categories]);

  return (
    <div className="space-y-6">
      <section className="bg-slate-900 rounded-xl p-4">
        <h2 className="text-lg font-semibold mb-4">Spend by category</h2>
        <div className="space-y-3">
          {categories.map((c) => {
            const pct = max ? ((parseFloat(c.monthly) || 0) / max) * 100 : 0;
            return (
              <div key={c.category}>
                <div className="flex justify-between text-sm mb-1">
                  <span className="text-slate-300">{c.category}</span>
                  <span className="text-cyan-400 font-medium">${c.monthly}</span>
                </div>
                <div className="h-2 bg-slate-800 rounded-full overflow-hidden">
                  <div className="h-full bg-green-500 rounded-full" style={{ width: `${pct}%` }} />
                </div>
              </div>
            );
          })}
        </div>
      </section>

      <section className="bg-slate-900 rounded-xl p-4">
        <h2 className="text-lg font-semibold mb-2">Projected monthly spend</h2>
        <div className="text-3xl font-bold text-green-400">${monthly}</div>
        <p className="text-slate-500 text-sm">Based on normalized monthly costs</p>
      </section>
    </div>
  );
}

export default App;
