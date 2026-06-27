import {
  ShoppingCart,
  BookOpen,
  CalendarDays,
  Package,
  Home,
} from "lucide-react";

export type NavId = "home" | "recipes" | "mealplan" | "pantry" | "groceries";

const NAV_ITEMS: {
  id: NavId;
  label: string;
  Icon: React.ElementType;
}[] = [
  { id: "home", label: "home", Icon: Home },
  { id: "recipes", label: "recipes", Icon: BookOpen },
  { id: "mealplan", label: "plan", Icon: CalendarDays },
  { id: "pantry", label: "pantry", Icon: Package },
  { id: "groceries", label: "grocery", Icon: ShoppingCart },
];

interface BottomNavProps {
  active: NavId;
  onChange: (id: NavId) => void;
}

export function BottomNav({ active, onChange }: BottomNavProps) {
  return (
    <div
      className="shrink-0 flex items-center justify-around px-2"
      style={{
        height: 72,
        background: "#fff",
        borderTop: "1px solid rgba(0,0,0,0.06)",
      }}
    >
      {NAV_ITEMS.map(({ id, label, Icon }) => {
        const isActive = active === id;
        return (
          <button
            key={id}
            onClick={() => onChange(id)}
            className="flex flex-col items-center gap-1 flex-1 py-2 active:scale-90 transition-transform"
          >
            <div
              className="flex items-center justify-center rounded-xl"
              style={{
                width: 40,
                height: 28,
                background: isActive ? "var(--yellow-33)" : "transparent",
                transition: "background 0.15s",
              }}
            >
              <Icon
                size={20}
                strokeWidth={isActive ? 2.5 : 1.8}
                style={{
                  color: isActive
                    ? "var(--gold-dark)"
                    : "rgba(0,0,0,0.4)",
                }}
              />
            </div>
            <span
              style={{
                fontSize: 10,
                fontWeight: isActive ? 700 : 400,
                color: isActive
                  ? "var(--gold-dark)"
                  : "rgba(0,0,0,0.4)",
                lineHeight: 1,
              }}
            >
              {label}
            </span>
          </button>
        );
      })}
    </div>
  );
}
