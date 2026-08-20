import { useEffect, useState } from "react";
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

function App() {
  const [subs, setSubs] = useState<Subscription[]>([]);
  const [summary, setSummary] = useState({ count: 0, active: 0, monthly: "0", yearly: "0" });
  const [name, setName] = useState("");
  const [provider, setProvider] = useState("");
  const [price, setPrice] = useState("");
  const [category, setCategory] = useState("");

  const load = async () => {
    const data = await invoke<Subscription[]>("list_subscriptions");
    setSubs(data);
    const s = await invoke<typeof summary>("get_summary");
    setSummary(s);
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
      next_renewal: null,
      tags: null,
    });
    setName("");
    setProvider("");
    setPrice("");
    setCategory("");
    await load();
  };

  const remove = async (id: string) => {
    await invoke("delete_subscription", { id });
    await load();
  };

  return (
    <div className="min-h-screen bg-subsy-dark text-white p-6">
      <header className="mb-8">
        <h1 className="text-3xl font-bold text-subsy-green">subsy</h1>
        <p className="text-slate-400">{summary.active} active · {summary.count} total · {summary.monthly}/mo · {summary.yearly}/yr</p>
      </header>

      <section className="bg-slate-900 rounded-xl p-4 mb-6">
        <h2 className="text-lg font-semibold mb-3">Add subscription</h2>
        <div className="grid grid-cols-2 md:grid-cols-5 gap-3">
          <input className="bg-slate-800 rounded px-3 py-2" placeholder="Name" value={name} onChange={(e) => setName(e.target.value)} />
          <input className="bg-slate-800 rounded px-3 py-2" placeholder="Provider" value={provider} onChange={(e) => setProvider(e.target.value)} />
          <input className="bg-slate-800 rounded px-3 py-2" placeholder="Price" value={price} onChange={(e) => setPrice(e.target.value)} />
          <input className="bg-slate-800 rounded px-3 py-2" placeholder="Category" value={category} onChange={(e) => setCategory(e.target.value)} />
          <button className="bg-subsy-green text-subsy-dark font-bold rounded px-4 py-2" onClick={add}>Add</button>
        </div>
      </section>

      <section className="space-y-3">
        {subs.map((s) => (
          <div key={s.id} className="bg-slate-900 rounded-xl p-4 flex items-center justify-between">
            <div>
              <div className="font-semibold text-lg">{s.name}</div>
              <div className="text-slate-400 text-sm">
                {s.provider || "-"} · {s.category || "-"} · {s.billing_cycle}
                {s.price ? ` · ${s.price} ${s.currency || ""}` : ""}
                {s.next_renewal ? ` · renews ${s.next_renewal}` : ""}
              </div>
              <div className="text-xs text-slate-500 mt-1">paid so far: {s.paid_so_far}</div>
            </div>
            <div className="flex items-center gap-3">
              <span className={`text-xs px-2 py-1 rounded ${s.status === "active" ? "bg-green-900 text-green-300" : "bg-slate-700"}`}>
                {s.status}
              </span>
              <button className="text-red-400 text-sm" onClick={() => remove(s.id)}>delete</button>
            </div>
          </div>
        ))}
      </section>
    </div>
  );
}

export default App;
