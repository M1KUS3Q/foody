import { useState, useEffect } from "react";
import { Plus, MoreHorizontal } from "lucide-react";
import { foody } from "@/bindings";
import { TopBar } from "@/components/TopBar";
import { WaveDivider } from "@/components/WaveDivider";
import type { PlanView } from "@/bindings/types";

const DAYS = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const TODAY_IDX = 1; // Tuesday demo

const MEAL_COLORS: Record<string, string> = {
  breakfast: "rgba(255,200,50,0.3)",
  lunch: "rgba(100,200,100,0.3)",
  dinner: "rgba(100,140,255,0.3)",
};

interface DayMeals {
  breakfast?: string;
  lunch?: string;
  dinner?: string;
}

// Mock data for demo when no plans exist
const MOCK_PLAN: Record<string, DayMeals> = {
  Mon: {
    breakfast: "🥞 Pancakes",
    lunch: "🥗 Greek Salad",
    dinner: "🍝 Carbonara",
  },
  Tue: {
    breakfast: "🍳 Eggs & Toast",
    lunch: "🌮 Fish Tacos",
    dinner: "🍛 Chicken Tikka",
  },
  Wed: { lunch: "🥗 Caesar Salad", dinner: "🍲 Lentil Soup" },
  Thu: { breakfast: "🥣 Oatmeal", dinner: "🍕 Pizza" },
  Fri: { lunch: "🌯 Wrap", dinner: "🍣 Sushi" },
  Sat: {
    breakfast: "🥞 Pancakes",
    lunch: "🍔 Burger",
    dinner: "🥘 Paella",
  },
  Sun: {
    breakfast: "🍳 Full English",
    lunch: "🍲 Lentil Soup",
    dinner: "🍝 Carbonara",
  },
};

const SLOTS = ["breakfast", "lunch", "dinner"] as const;

export function MealPlanScreen() {
  const [selectedDay, setSelectedDay] = useState(TODAY_IDX);
  const [plans, setPlans] = useState<PlanView[]>([]);
  const [currentPlan, setCurrentPlan] = useState<PlanView | null>(null);

  useEffect(() => {
    async function load() {
      try {
        const names = await foody.plan.list();
        if (names.length > 0) {
          const views = await Promise.all(
            names.map((n) => foody.plan.view(n)),
          );
          setPlans(views);
          setCurrentPlan(views[0] ?? null);
        }
      } catch {
        // Use mock data below
      }
    }
    load();
  }, []);

  // Build day meals from the current plan or mock
  const dayName = DAYS[selectedDay];
  let meals: DayMeals = {};

  if (currentPlan) {
    const dayItems = currentPlan.items.filter(
      (item) => item.day_index === selectedDay,
    );
    for (const item of dayItems) {
      const slot = item.daypart_name.toLowerCase();
      if (SLOTS.includes(slot as (typeof SLOTS)[number])) {
        (meals as Record<string, string>)[slot] = item.meal_name;
      }
    }
  } else {
    meals = MOCK_PLAN[dayName] || {};
  }

  return (
    <>
      <TopBar title="meal plan 📅" />
      <div
        className="flex-1 flex flex-col overflow-hidden"
        style={{ background: "var(--yellow)", minHeight: 0 }}
      >
        {/* Plan selector if multiple plans */}
        {plans.length > 1 && (
          <div className="px-6 pt-2 pb-1 shrink-0">
            <select
              className="w-full rounded-xl px-3 py-2 text-sm bg-black/10 border-none outline-none"
              style={{ fontFamily: "inherit" }}
              value={currentPlan?.id ?? ""}
              onChange={(e) => {
                const plan = plans.find(
                  (p) => p.id === Number(e.target.value),
                );
                setCurrentPlan(plan ?? null);
              }}
            >
              {plans.map((p) => (
                <option key={p.id} value={p.id}>
                  {p.name}
                </option>
              ))}
            </select>
          </div>
        )}

        {/* Week strip */}
        <div className="flex px-4 pt-4 pb-3 gap-2 shrink-0">
          {DAYS.map((d, i) => {
            const hasMeals = currentPlan
              ? currentPlan.items.some(
                (item) => item.day_index === i,
              )
              : Object.keys(MOCK_PLAN[d] || {}).length > 0;
            const isToday = i === TODAY_IDX;
            const isSelected = i === selectedDay;
            return (
              <button
                key={d}
                onClick={() => setSelectedDay(i)}
                className="flex-1 flex flex-col items-center py-2 rounded-2xl transition-colors"
                style={{
                  background: isSelected
                    ? "#000"
                    : "rgba(0,0,0,0.08)",
                  color: isSelected
                    ? "#fff"
                    : "rgba(0,0,0,0.7)",
                }}
              >
                <span
                  style={{
                    fontSize: 11,
                    fontWeight: 500,
                    opacity: isSelected ? 0.7 : 0.6,
                  }}
                >
                  {d}
                </span>
                <span
                  style={{
                    fontSize: 17,
                    fontWeight: 700,
                    lineHeight: 1.4,
                  }}
                >
                  {i + 9}
                </span>
                {hasMeals && (
                  <div
                    className="w-1 h-1 rounded-full mt-0.5"
                    style={{
                      background: isSelected ? "var(--yellow)" : "#000",
                      opacity: 0.7,
                    }}
                  />
                )}
                {isToday && !isSelected && (
                  <span
                    style={{
                      fontSize: 8,
                      opacity: 0.5,
                      marginTop: 1,
                    }}
                  >
                    today
                  </span>
                )}
              </button>
            );
          })}
        </div>

        <WaveDivider />

        {/* Day detail */}
        <div
          className="flex-1 overflow-y-auto px-6 pt-3 pb-4 flex flex-col gap-4"
          style={{ background: "#fff" }}
        >
          <div className="flex items-center justify-between">
            <p style={{ fontSize: 20, fontWeight: 700 }}>
              {dayName}
              {TODAY_IDX === selectedDay ? " · today" : ""}
            </p>
            <button
              className="flex items-center gap-1 px-3 py-1.5 rounded-full"
              style={{ background: "#f0f0f0", fontSize: 13 }}
            >
              <Plus size={13} /> add meal
            </button>
          </div>

          {SLOTS.map((slot) => {
            const meal = (meals as Record<string, string>)[slot];
            return (
              <div key={slot}>
                <p
                  style={{
                    fontSize: 12,
                    fontWeight: 600,
                    opacity: 0.4,
                    textTransform: "uppercase",
                    letterSpacing: "0.06em",
                    marginBottom: 8,
                  }}
                >
                  {slot}
                </p>
                {meal ? (
                  <div
                    className="flex items-center gap-3 px-4 py-3 rounded-2xl"
                    style={{ background: MEAL_COLORS[slot] }}
                  >
                    <span style={{ fontSize: 28 }}>
                      {meal.split(" ")[0]}
                    </span>
                    <span
                      style={{ fontSize: 15, fontWeight: 500 }}
                    >
                      {meal.split(" ").slice(1).join(" ")}
                    </span>
                    <button className="ml-auto opacity-30">
                      <MoreHorizontal size={18} />
                    </button>
                  </div>
                ) : (
                  <button
                    className="w-full flex items-center justify-center gap-2 py-4 rounded-2xl border-2 border-dashed"
                    style={{
                      borderColor: "rgba(0,0,0,0.12)",
                      color: "rgba(0,0,0,0.3)",
                      fontSize: 14,
                    }}
                  >
                    <Plus size={16} strokeWidth={1.5} /> plan {slot}
                  </button>
                )}
              </div>
            );
          })}

          {/* Nutrition summary placeholder */}
          <div
            className="rounded-2xl p-4 mt-1"
            style={{ background: "#f5f5f5" }}
          >
            <p
              style={{
                fontSize: 14,
                fontWeight: 600,
                marginBottom: 12,
              }}
            >
              Nutrition estimate
            </p>
            <div className="flex gap-3">
              {[
                ["~1840", "kcal"],
                ["72g", "protein"],
                ["210g", "carbs"],
                ["61g", "fat"],
              ].map(([val, lbl]) => (
                <div key={lbl} className="flex-1 text-center">
                  <p style={{ fontSize: 16, fontWeight: 700 }}>
                    {val}
                  </p>
                  <p style={{ fontSize: 10, opacity: 0.45 }}>
                    {lbl}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
