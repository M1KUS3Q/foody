import { useState } from "react";
import { BottomNav, type NavId } from "@/components/BottomNav";
import { HomeScreen } from "@/screens/HomeScreen";
import { RecipesScreen } from "@/screens/RecipesScreen";
import { MealPlanScreen } from "@/screens/MealPlanScreen";
import { PantryScreen } from "@/screens/PantryScreen";
import { GroceriesScreen } from "@/screens/GroceriesScreen";

export default function App() {
  const [activeNav, setActiveNav] = useState<NavId>("home");

  return (
    <div
      className="flex flex-col w-full overflow-hidden"
      style={{
        height: "100dvh",
        background: "#e5e5e5",
      }}
    >
      {/* Screen area — fills all space above bottom nav */}
      <div className="flex-1 flex flex-col overflow-hidden" style={{ minHeight: 0 }}>
        {activeNav === "home" && (
          <HomeScreen setNav={setActiveNav} />
        )}
        {activeNav === "recipes" && <RecipesScreen />}
        {activeNav === "mealplan" && <MealPlanScreen />}
        {activeNav === "pantry" && <PantryScreen />}
        {activeNav === "groceries" && <GroceriesScreen />}
      </div>

      <BottomNav active={activeNav} onChange={setActiveNav} />
    </div>
  );
}
