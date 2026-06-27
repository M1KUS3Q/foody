import { useState, useEffect } from "react";
import { Plus, ChevronRight } from "lucide-react";
import { foody } from "@/bindings";
import { TopBar } from "@/components/TopBar";
import { Pill } from "@/components/Pill";
import { FAB } from "@/components/FAB";
import { WaveDivider } from "@/components/WaveDivider";
import type { IngredientView } from "@/bindings/types";

const PANTRY_CATS = ["all", "dairy", "produce", "dry", "meat"];

// Mock pantry items when no real data yet
interface PantryItem {
  emoji: string;
  name: string;
  qty: string;
  category: string;
  expiry?: string;
  low?: boolean;
}

const MOCK_PANTRY: PantryItem[] = [
  {
    emoji: "🥚",
    name: "Eggs",
    qty: "6",
    category: "dairy",
    expiry: "in 4d",
  },
  {
    emoji: "🥛",
    name: "Milk",
    qty: "1L",
    category: "dairy",
    expiry: "in 2d",
    low: true,
  },
  {
    emoji: "🧀",
    name: "Cheddar",
    qty: "200g",
    category: "dairy",
  },
  {
    emoji: "🍅",
    name: "Tomatoes",
    qty: "4",
    category: "produce",
    expiry: "in 3d",
  },
  {
    emoji: "🥦",
    name: "Broccoli",
    qty: "1 head",
    category: "produce",
  },
  {
    emoji: "🧅",
    name: "Onions",
    qty: "3",
    category: "produce",
  },
  { emoji: "🫙", name: "Pasta", qty: "500g", category: "dry" },
  { emoji: "🫙", name: "Rice", qty: "1kg", category: "dry" },
  {
    emoji: "🫒",
    name: "Olive Oil",
    qty: "400ml",
    category: "dry",
    low: true,
  },
  {
    emoji: "🍗",
    name: "Chicken",
    qty: "600g",
    category: "meat",
    expiry: "today",
    low: true,
  },
  {
    emoji: "🥩",
    name: "Beef Mince",
    qty: "250g",
    category: "meat",
  },
];

export function PantryScreen() {
  const [cat, setCat] = useState("all");
  const [ingredients, setIngredients] = useState<IngredientView[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        const names = await foody.ingredient.list();
        if (names.length > 0) {
          const views = await Promise.all(
            names.map((n) => foody.ingredient.view(n)),
          );
          setIngredients(views);
        }
      } catch {
        // Fall back to mock
      } finally {
        setLoading(false);
      }
    }
    load();
  }, []);

  const hasRealData = ingredients.length > 0;

  const groupedMock = MOCK_PANTRY.filter(
    (item) => cat === "all" || item.category === cat,
  ).reduce<Record<string, PantryItem[]>>((acc, item) => {
    const g = cat === "all" ? item.category : "items";
    (acc[g] ??= []).push(item);
    return acc;
  }, {});

  const lowCount = MOCK_PANTRY.filter((i) => i.low).length;

  return (
    <>
      <TopBar
        title="pantry 🥦"
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
        {/* Filter pills */}
        <div
          className="flex gap-2 px-6 pt-4 pb-3 overflow-x-auto shrink-0"
          style={{ scrollbarWidth: "none" }}
        >
          {PANTRY_CATS.map((c) => (
            <Pill
              key={c}
              label={c}
              active={cat === c}
              onClick={() => setCat(c)}
            />
          ))}
        </div>

        {/* Low stock banner */}
        {lowCount > 0 && (
          <div
            className="mx-5 mb-3 shrink-0 flex items-center gap-2 px-4 py-2.5 rounded-2xl"
            style={{ background: "rgba(0,0,0,0.08)" }}
          >
            <span style={{ fontSize: 16 }}>⚠️</span>
            <span style={{ fontSize: 13, fontWeight: 500 }}>
              {lowCount} items running low
            </span>
            <button
              className="ml-auto flex items-center gap-0.5"
              style={{ fontSize: 12, opacity: 0.6 }}
            >
              add to grocery <ChevronRight size={12} />
            </button>
          </div>
        )}

        <WaveDivider />

        {/* Items */}
        <div
          className="flex-1 overflow-y-auto px-6 pt-2 pb-4 flex flex-col gap-4"
          style={{ background: "#fff" }}
        >
          {loading ? (
            <p style={{ textAlign: "center", opacity: 0.5, padding: 40 }}>
              Loading pantry…
            </p>
          ) : hasRealData ? (
            Object.entries(
              ingredients.reduce<Record<string, typeof ingredients>>(
                (acc, ing) => {
                  const g =
                    cat === "all"
                      ? (ing.categories[0] || "other")
                      : "items";
                  (acc[g] ??= []).push(ing);
                  return acc;
                },
                {},
              ),
            ).map(([group, groupItems]) => (
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
                  {groupItems.map((ing) => (
                    <div
                      key={ing.name}
                      className="flex items-center gap-3 px-4 py-3 rounded-2xl"
                      style={{ background: "#f5f5f5" }}
                    >
                      <span style={{ fontSize: 26 }}>📦</span>
                      <div className="flex-1 min-w-0">
                        <p
                          style={{
                            fontSize: 15,
                            fontWeight: 500,
                          }}
                        >
                          {ing.name}
                        </p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            ))
          ) : (
            Object.entries(groupedMock).map(([group, items]) => (
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
                  {items.map((item) => {
                    const isLow = item.low;
                    const expiry = item.expiry;
                    return (
                      <div
                        key={item.name}
                        className="flex items-center gap-3 px-4 py-3 rounded-2xl"
                        style={{
                          background: isLow
                            ? "rgba(255,80,80,0.07)"
                            : "#f5f5f5",
                        }}
                      >
                        <span style={{ fontSize: 26 }}>
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
                          {expiry && (
                            <p
                              style={{
                                fontSize: 12,
                                color:
                                  expiry === "today"
                                    ? "#e03030"
                                    : "rgba(0,0,0,0.4)",
                              }}
                            >
                              {expiry === "today"
                                ? "⚠️ expires today"
                                : `expires ${expiry}`}
                            </p>
                          )}
                        </div>
                        <div className="flex flex-col items-end gap-1">
                          <span
                            style={{
                              fontSize: 14,
                              fontWeight: 600,
                            }}
                          >
                            {item.qty}
                          </span>
                          {isLow && (
                            <span
                              className="px-2 py-0.5 rounded-full"
                              style={{
                                fontSize: 10,
                                background:
                                  "rgba(255,80,80,0.15)",
                                color: "#c00",
                              }}
                            >
                              low
                            </span>
                          )}
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      <FAB />
    </>
  );
}
