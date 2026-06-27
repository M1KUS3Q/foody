import { useState, useEffect } from "react";
import {
  Search,
  Plus,
  Timer,
  Users,
} from "lucide-react";
import { foody } from "@/bindings";
import { TopBar } from "@/components/TopBar";
import { FAB } from "@/components/FAB";
import { WaveDivider } from "@/components/WaveDivider";
import type { MealView } from "@/bindings/types";

// Mock data used when backend returns no data yet
const MOCK_RECIPES: MealView[] = [
  {
    id: 1,
    name: "Pasta Carbonara",
    dayparts: ["lunch", "dinner"],
    ingredients: ["pasta", "eggs", "parmesan", "bacon"],
    recipe: "Cook pasta, mix eggs and cheese, combine with bacon.",
  },
  {
    id: 2,
    name: "Greek Salad",
    dayparts: ["lunch"],
    ingredients: ["tomato", "cucumber", "feta", "olives"],
    recipe: "Chop vegetables, add feta and olives, dress with olive oil.",
  },
  {
    id: 3,
    name: "Chicken Tikka",
    dayparts: ["dinner"],
    ingredients: ["chicken", "yogurt", "spices"],
    recipe: "Marinate chicken, grill, serve with rice.",
  },
  {
    id: 4,
    name: "Blueberry Pancakes",
    dayparts: ["breakfast"],
    ingredients: ["flour", "blueberries", "eggs", "milk"],
    recipe: "Mix batter, add blueberries, cook on griddle.",
  },
  {
    id: 5,
    name: "Lentil Soup",
    dayparts: ["lunch", "dinner"],
    ingredients: ["lentils", "carrots", "onion", "broth"],
    recipe: "Simmer lentils with vegetables until tender.",
  },
  {
    id: 6,
    name: "Fish Tacos",
    dayparts: ["lunch", "dinner"],
    ingredients: ["fish", "tortillas", "lime", "cabbage"],
    recipe: "Grill fish, assemble tacos with slaw.",
  },
];

export function RecipesScreen() {
  const [search, setSearch] = useState("");
  const [recipes, setRecipes] = useState<MealView[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        const names = await foody.meal.list();
        if (names.length > 0) {
          const views = await Promise.all(
            names.map((n) => foody.meal.view(n)),
          );
          setRecipes(views);
        } else {
          setRecipes(MOCK_RECIPES);
        }
      } catch {
        setRecipes(MOCK_RECIPES);
      } finally {
        setLoading(false);
      }
    }
    load();
  }, []);

  const filtered = recipes.filter((r) =>
    r.name.toLowerCase().includes(search.toLowerCase()),
  );

  return (
    <>
      <TopBar title="recipes 🍽️" />
      <div
        className="flex-1 flex flex-col overflow-hidden"
        style={{ background: "var(--yellow)", minHeight: 0 }}
      >
        {/* Search */}
        <div className="px-6 pt-4 pb-3 shrink-0">
          <div
            className="flex items-center gap-2 px-4 rounded-2xl"
            style={{
              background: "rgba(0,0,0,0.08)",
              height: 44,
            }}
          >
            <Search
              size={16}
              strokeWidth={2}
              style={{ opacity: 0.5 }}
            />
            <input
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="search recipes…"
              className="flex-1 bg-transparent outline-none"
              style={{ fontSize: 15, fontFamily: "inherit" }}
            />
          </div>
        </div>

        <WaveDivider />

        {/* List */}
        <div
          className="flex-1 overflow-y-auto px-6 pt-2 pb-4 flex flex-col gap-3"
          style={{ background: "#fff" }}
        >
          {loading ? (
            <p style={{ textAlign: "center", opacity: 0.5, padding: 40 }}>
              Loading recipes…
            </p>
          ) : (
            <>
              <div className="flex items-center justify-between py-1">
                <span style={{ fontSize: 13, opacity: 0.45 }}>
                  {filtered.length} recipes
                </span>
                <button
                  className="flex items-center gap-1"
                  style={{ fontSize: 13, opacity: 0.45 }}
                >
                  <Plus size={14} /> add new
                </button>
              </div>
              {filtered.map((r) => (
                <button
                  key={r.id}
                  className="flex items-center gap-4 rounded-2xl px-4 py-3 active:scale-[0.98] transition-transform text-left w-full"
                  style={{ background: "#f5f5f5" }}
                >
                  <span style={{ fontSize: 36 }}>🍽️</span>
                  <div className="flex-1 min-w-0">
                    <p style={{ fontSize: 15, fontWeight: 600 }}>
                      {r.name}
                    </p>
                    <div className="flex items-center gap-3 mt-0.5">
                      {r.ingredients.length > 0 && (
                        <span
                          className="flex items-center gap-1"
                          style={{ fontSize: 12, opacity: 0.5 }}
                        >
                          <Timer size={11} /> {r.ingredients.length} ingredients
                        </span>
                      )}
                      {r.dayparts.length > 0 && (
                        <span
                          className="flex items-center gap-1"
                          style={{ fontSize: 12, opacity: 0.5 }}
                        >
                          <Users size={11} /> {r.dayparts.length} dayparts
                        </span>
                      )}
                    </div>
                  </div>
                  <div className="flex flex-col items-end gap-1 shrink-0">
                    <span style={{ fontSize: 16 }}>
                      {r.recipe ? "📝" : "📄"}
                    </span>
                    {r.dayparts.slice(0, 1).map((t) => (
                      <span
                        key={t}
                        className="px-2 py-0.5 rounded-full"
                        style={{
                          fontSize: 10,
                          background: "rgba(255,160,80,0.25)",
                          color: "#c06000",
                        }}
                      >
                        {t}
                      </span>
                    ))}
                  </div>
                </button>
              ))}
            </>
          )}
        </div>
      </div>

      <FAB />
    </>
  );
}
