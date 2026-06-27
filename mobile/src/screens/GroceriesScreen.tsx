import { useState, useEffect } from "react";
import { Plus, Check, Trash2 } from "lucide-react";
import { foody } from "@/bindings";
import { TopBar } from "@/components/TopBar";
import { FAB } from "@/components/FAB";
import { WaveDivider } from "@/components/WaveDivider";

interface GroceryItem {
  id: number;
  emoji: string;
  name: string;
  qty: string;
  category: string;
  checked: boolean;
}

const MOCK_GROCERY: GroceryItem[] = [
  {
    id: 1,
    emoji: "🥛",
    name: "Milk",
    qty: "2L",
    category: "dairy",
    checked: false,
  },
  {
    id: 2,
    emoji: "🧀",
    name: "Parmesan",
    qty: "100g",
    category: "dairy",
    checked: true,
  },
  {
    id: 3,
    emoji: "🥚",
    name: "Eggs",
    qty: "12",
    category: "dairy",
    checked: false,
  },
  {
    id: 4,
    emoji: "🍅",
    name: "Cherry tomatoes",
    qty: "250g",
    category: "produce",
    checked: true,
  },
  {
    id: 5,
    emoji: "🌿",
    name: "Fresh basil",
    qty: "1 bunch",
    category: "produce",
    checked: false,
  },
  {
    id: 6,
    emoji: "🧄",
    name: "Garlic",
    qty: "1 bulb",
    category: "produce",
    checked: false,
  },
  {
    id: 7,
    emoji: "🍗",
    name: "Chicken breast",
    qty: "500g",
    category: "meat",
    checked: false,
  },
  {
    id: 8,
    emoji: "🫙",
    name: "Pasta",
    qty: "500g",
    category: "dry",
    checked: true,
  },
  {
    id: 9,
    emoji: "🥫",
    name: "Tomato sauce",
    qty: "2 cans",
    category: "dry",
    checked: false,
  },
  {
    id: 10,
    emoji: "🍞",
    name: "Sourdough",
    qty: "1 loaf",
    category: "bakery",
    checked: false,
  },
];

export function GroceriesScreen() {
  const [items, setItems] = useState<GroceryItem[]>(MOCK_GROCERY);
  const [planGrocery, setPlanGrocery] = useState<string | null>(null);
  const [plans, setPlans] = useState<string[]>([]);
  const [selectedPlan, setSelectedPlan] = useState("");

  useEffect(() => {
    async function load() {
      try {
        const names = await foody.plan.list();
        setPlans(names);
      } catch {
        // No plans available
      }
    }
    load();
  }, []);

  const toggle = (id: number) =>
    setItems((prev) =>
      prev.map((i) =>
        i.id === id ? { ...i, checked: !i.checked } : i,
      ),
    );

  const remove = (id: number) =>
    setItems((prev) => prev.filter((i) => i.id !== id));

  const unchecked = items.filter((i) => !i.checked);
  const checked = items.filter((i) => i.checked);

  const grouped = unchecked.reduce<Record<string, GroceryItem[]>>(
    (acc, item) => {
      acc[item.category] = [...(acc[item.category] || []), item];
      return acc;
    },
    {},
  );

  const handleGenerate = async () => {
    if (!selectedPlan) return;
    try {
      const result = await foody.grocery.plan(selectedPlan);
      setPlanGrocery(result);
    } catch (e) {
      console.error("Failed to generate grocery list:", e);
    }
  };

  return (
    <>
      <TopBar
        title="groceries 🛒"
        right={
          <button
            className="flex items-center justify-center rounded-full"
            style={{
              width: 38,
              height: 38,
              background: "rgba(0,0,0,0.1)",
            }}
          >
            <Plus size={18} strokeWidth={2.5} />
          </button>
        }
      />
      <div
        className="flex-1 flex flex-col overflow-hidden"
        style={{ background: "var(--yellow)", minHeight: 0 }}
      >
        {/* Plan-based generation */}
        {plans.length > 0 && (
          <div className="px-6 pt-4 pb-2 shrink-0 flex items-center gap-3">
            <select
              value={selectedPlan}
              onChange={(e) => setSelectedPlan(e.target.value)}
              className="flex-1 rounded-xl px-3 py-2 text-sm bg-black/10 border-none outline-none"
              style={{ fontFamily: "inherit" }}
            >
              <option value="">Select a plan…</option>
              {plans.map((p) => (
                <option key={p} value={p}>
                  {p}
                </option>
              ))}
            </select>
            <button
              onClick={handleGenerate}
              disabled={!selectedPlan}
              className="shrink-0 px-4 py-2 rounded-xl text-sm font-medium"
              style={{
                background: selectedPlan ? "#000" : "rgba(0,0,0,0.15)",
                color: selectedPlan ? "#fff" : "rgba(0,0,0,0.3)",
              }}
            >
              Generate
            </button>
          </div>
        )}

        {planGrocery && (
          <div
            className="mx-5 mb-3 shrink-0 p-4 rounded-2xl"
            style={{ background: "rgba(0,0,0,0.06)", fontSize: 14, whiteSpace: "pre-wrap", maxHeight: 160, overflowY: "auto" }}
          >
            {planGrocery}
          </div>
        )}

        {/* Progress bar + list — hidden when a generated list is shown */}
        {!planGrocery ? (
          <>
            <div className="px-6 pt-4 pb-3 shrink-0">
              <div className="flex items-end justify-between mb-2">
                <span style={{ fontSize: 15, fontWeight: 500 }}>
                  {checked.length} of {items.length} items
                </span>
                <span style={{ fontSize: 13, opacity: 0.5 }}>
                  {Math.round((checked.length / items.length) * 100)}%
                </span>
              </div>
              <div
                className="w-full rounded-full overflow-hidden"
                style={{
                  height: 6,
                  background: "rgba(0,0,0,0.12)",
                }}
              >
                <div
                  className="h-full rounded-full transition-all"
                  style={{
                    width: `${(checked.length / items.length) * 100}%`,
                    background: "#000",
                  }}
                />
              </div>
            </div>

            <WaveDivider />

            <div
              className="flex-1 overflow-y-auto px-6 pt-2 pb-4 flex flex-col gap-4"
              style={{ background: "#fff" }}
            >
            {Object.entries(grouped).map(([group, groupItems]) => (
              <div key={group}>
                <p
                  style={{
                    fontSize: 12,
                    fontWeight: 600,
                    opacity: 0.35,
                    textTransform: "uppercase",
                    letterSpacing: "0.06em",
                    marginBottom: 8,
                  }}
                >
                  {group}
                </p>
                <div className="flex flex-col gap-2">
                  {groupItems.map((item) => (
                    <div
                      key={item.id}
                      className="flex items-center gap-3 px-4 py-3 rounded-2xl"
                      style={{ background: "#f5f5f5" }}
                    >
                      <button
                        onClick={() => toggle(item.id)}
                        className="shrink-0 flex items-center justify-center rounded-full border-2 transition-colors"
                        style={{
                          width: 28,
                          height: 28,
                          borderColor: item.checked
                            ? "#000"
                            : "rgba(0,0,0,0.2)",
                          background: item.checked
                            ? "#000"
                            : "transparent",
                        }}
                      >
                        {item.checked && (
                          <Check
                            size={14}
                            strokeWidth={3}
                            color="#fff"
                          />
                        )}
                      </button>
                      <span style={{ fontSize: 22 }}>
                        {item.emoji}
                      </span>
                      <div className="flex-1 min-w-0">
                        <p
                          style={{
                            fontSize: 15,
                            fontWeight: 500,
                          }}
                        >
                          {item.name}
                        </p>
                        <p style={{ fontSize: 12, opacity: 0.4 }}>
                          {item.qty}
                        </p>
                      </div>
                      <button
                        onClick={() => remove(item.id)}
                        style={{ opacity: 0.2 }}
                      >
                        <Trash2 size={15} />
                      </button>
                    </div>
                  ))}
                </div>
              </div>
            ))}

            {/* Checked items */}
            {checked.length > 0 && (
              <div>
                <p
                  style={{
                    fontSize: 12,
                    fontWeight: 600,
                    opacity: 0.35,
                    textTransform: "uppercase",
                    letterSpacing: "0.06em",
                    marginBottom: 8,
                  }}
                >
                  in cart ({checked.length})
                </p>
                <div className="flex flex-col gap-2">
                  {checked.map((item) => (
                    <div
                      key={item.id}
                      className="flex items-center gap-3 px-4 py-3 rounded-2xl"
                      style={{
                        background: "#f5f5f5",
                        opacity: 0.5,
                      }}
                    >
                      <button
                        onClick={() => toggle(item.id)}
                        className="shrink-0 flex items-center justify-center rounded-full border-2"
                        style={{
                          width: 28,
                          height: 28,
                          borderColor: "#000",
                          background: "#000",
                        }}
                      >
                        <Check
                          size={14}
                          strokeWidth={3}
                          color="#fff"
                        />
                      </button>
                      <span style={{ fontSize: 22 }}>
                        {item.emoji}
                      </span>
                      <p
                        className="flex-1"
                        style={{
                          fontSize: 15,
                          fontWeight: 500,
                          textDecoration: "line-through",
                        }}
                      >
                        {item.name}
                      </p>
                      <button
                        onClick={() => remove(item.id)}
                        style={{ opacity: 0.3 }}
                      >
                        <Trash2 size={15} />
                      </button>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        </>
        ) : null}
      </div>

      {!planGrocery && <FAB />}
    </>
  );
}
