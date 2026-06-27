import { useState, useRef, useCallback, useEffect } from "react";
import {
  Search,
  Plus,
  CalendarDays,
  Sparkles,
  Clock,
  ChevronRight,
} from "lucide-react";
import { TopBar } from "@/components/TopBar";
import { WaveDivider } from "@/components/WaveDivider";
import type { NavId } from "@/components/BottomNav";

const SECTIONS = [
  {
    id: "groceries",
    label: "groceries",
    emoji: "🛒",
    color: "rgba(51,255,136,0.47)",
  },
  {
    id: "recipes",
    label: "recipes",
    emoji: "🍽️",
    color: "rgba(255,160,80,0.45)",
  },
  {
    id: "pantry",
    label: "pantry",
    emoji: "🥦",
    color: "rgba(100,200,255,0.45)",
  },
  {
    id: "mealplan",
    label: "meal plan",
    emoji: "📅",
    color: "rgba(200,140,255,0.45)",
  },
];

const QUICK_ACTIONS = [
  { label: "Add Recipe", Icon: Plus, color: "#ffa050" },
  { label: "Plan Week", Icon: CalendarDays, color: "#c88cff" },
  { label: "Discover", Icon: Sparkles, color: "#33ff88" },
  { label: "Recent", Icon: Clock, color: "#64c8ff" },
];

const RECENT_ITEMS = [
  {
    emoji: "🍝",
    name: "Pasta Carbonara",
    tag: "recipe",
    time: "2d ago",
    tagColor: "#ffa050",
  },
  {
    emoji: "🥗",
    name: "Greek Salad",
    tag: "recipe",
    time: "4d ago",
    tagColor: "#ffa050",
  },
  {
    emoji: "🛒",
    name: "Weekly shop",
    tag: "grocery",
    time: "5d ago",
    tagColor: "#33ff88",
  },
  {
    emoji: "🛒",
    name: "Weekly shop",
    tag: "grocery",
    time: "5d ago",
    tagColor: "#33ff88",
  },
  {
    emoji: "🛒",
    name: "Weekly shop",
    tag: "grocery",
    time: "5d ago",
    tagColor: "#33ff88",
  },
  {
    emoji: "🛒",
    name: "Weekly shop",
    tag: "grocery",
    time: "5d ago",
    tagColor: "#33ff88",
  },
];

const CARD_GAP = 12;

interface HomeScreenProps {
  setNav: (id: NavId) => void;
}

function getCardSpan(elWidth: number) {
  return elWidth * 0.82 + CARD_GAP;
}

export function HomeScreen({ setNav }: HomeScreenProps) {
  const [activeCard, setActiveCard] = useState(0);
  const scrollRef = useRef<HTMLDivElement>(null);
  const scrolling = useRef(false);

  // Determine which card is most visible after scrolling
  const updateActive = useCallback(() => {
    const el = scrollRef.current;
    if (!el) return;
    const scrollLeft = el.scrollLeft;
    const cardSpan = getCardSpan(el.clientWidth);
    const idx = Math.round(scrollLeft / cardSpan);
    const clamped = Math.max(0, Math.min(idx, SECTIONS.length - 1));
    setActiveCard(clamped);
  }, []);

  // Listen for scroll end to update active dot
  useEffect(() => {
    const el = scrollRef.current;
    if (!el) return;
    let timer: ReturnType<typeof setTimeout>;
    const onScroll = () => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        updateActive();
        scrolling.current = false;
      }, 80);
    };
    el.addEventListener("scroll", onScroll, { passive: true });
    return () => {
      el.removeEventListener("scroll", onScroll);
      clearTimeout(timer);
    };
  }, [updateActive]);

  const scrollToCard = (index: number) => {
    const el = scrollRef.current;
    if (!el) return;
    scrolling.current = true;
    const cardSpan = getCardSpan(el.clientWidth);
    el.scrollTo({ left: index * cardSpan, behavior: "smooth" });
    setActiveCard(index);
  };

  return (
    <>
      <TopBar
        title="foody 🍴"
        right={
          <button
            className="flex items-center justify-center rounded-full"
            style={{
              width: 38,
              height: 38,
              background: "rgba(0,0,0,0.1)",
            }}
          >
            <Search size={17} strokeWidth={2.2} />
          </button>
        }
      />

      <div
        className="flex-1 flex flex-col"
        style={{ minHeight: 0, background: "var(--yellow)" }}
      >
        {/* Greeting */}
        <div className="px-5 pt-5 pb-2 shrink-0">
          <p style={{ fontSize: 15, fontWeight: 500, opacity: 0.55 }}>
            Good morning 👋
          </p>
          <p style={{ fontSize: 26, fontWeight: 700, lineHeight: 1.2 }}>
            What are we
            <br />
            cooking today?
          </p>
        </div>

        {/* CSS Scroll-snap Carousel */}
        <div className="shrink-0" style={{ height: 240 }}>
          <div
            ref={scrollRef}
            className="flex h-full"
            style={{
              scrollSnapType: "x mandatory",
              scrollbarWidth: "none",
              paddingLeft: "9%",
              paddingRight: "9%",
              gap: CARD_GAP,
            }}
          >
            {SECTIONS.map((s, i) => {
              const isActive = i === activeCard;
              return (
                <button
                  key={s.id}
                  onClick={() => {
                    if (!scrolling.current) {
                      if (isActive) {
                        setNav(s.id as NavId);
                      } else {
                        scrollToCard(i);
                      }
                    }
                  }}
                  className="flex flex-col items-center justify-center shrink-0"
                  style={{
                    width: "82%",
                    height: 210,
                    background: "#fafafa",
                    borderRadius: 44,
                    boxShadow: "0 8px 32px rgba(0,0,0,0.12)",
                    border: "none",
                    cursor: "pointer",
                    scrollSnapAlign: "center",
                    opacity: isActive ? 1 : 0.65,
                    transform: isActive ? "scale(1)" : "scale(0.94)",
                    transition: "opacity 0.3s, transform 0.3s",
                  }}
                >
                  <span
                    style={{
                      fontSize: 72,
                      lineHeight: 1,
                      marginBottom: 12,
                      display: "block",
                    }}
                  >
                    {s.emoji}
                  </span>
                  <div
                    className="flex items-center justify-center"
                    style={{
                      background: s.color,
                      borderRadius: 16,
                      paddingLeft: 20,
                      paddingRight: 20,
                      height: 34,
                      minWidth: 120,
                    }}
                  >
                    <span style={{ fontSize: 20, fontWeight: 400 }}>
                      {s.label}
                    </span>
                  </div>
                </button>
              );
            })}
          </div>
        </div>

        {/* Dots */}
        <div className="flex items-center justify-center gap-2 shrink-0 mt-2 pb-1">
          {SECTIONS.map((_, i) => (
            <button
              key={i}
              onClick={() => scrollToCard(i)}
              style={{
                width: i === activeCard ? 20 : 8,
                height: 8,
                borderRadius: 4,
                background: i === activeCard ? "#000" : "rgba(0,0,0,0.18)",
                transition: "width 0.2s ease, background 0.2s",
                border: "none",
                padding: 0,
                cursor: "pointer",
              }}
            />
          ))}
        </div>

        {/* Wave + white content */}
        <div className="flex-1 flex flex-col" style={{ minHeight: 0 }}>
          <WaveDivider variant="home" />

          <div
            className="flex-1"
            style={{ background: "#fff", minHeight: 0 }}
          >
            {/* Quick Actions */}
            <div className="px-5 pt-4 pb-2 flex items-center justify-between">
              <span style={{ fontSize: 18, fontWeight: 600 }}>
                Quick Actions
              </span>
            </div>
            <div className="grid grid-cols-4 gap-2 px-5 pb-4">
              {QUICK_ACTIONS.map(({ label, Icon, color }) => (
                <button
                  key={label}
                  className="flex flex-col items-center gap-2 py-3 rounded-2xl active:scale-95 transition-transform"
                  style={{ background: "#f5f5f5" }}
                >
                  <div
                    className="flex items-center justify-center rounded-xl"
                    style={{
                      width: 40,
                      height: 40,
                      background: color + "33",
                    }}
                  >
                    <Icon
                      size={20}
                      style={{ color }}
                      strokeWidth={2}
                    />
                  </div>
                  <span
                    style={{
                      fontSize: 11,
                      fontWeight: 500,
                      textAlign: "center",
                      lineHeight: 1.2,
                    }}
                  >
                    {label}
                  </span>
                </button>
              ))}
            </div>

            {/* Recent */}
            <div className="px-5 pt-1 pb-2 flex items-center justify-between">
              <span style={{ fontSize: 18, fontWeight: 600 }}>
                Recent
              </span>
              <button className="flex items-center gap-0.5 opacity-40">
                <span style={{ fontSize: 13 }}>see all</span>
                <ChevronRight size={13} />
              </button>
            </div>
            <div className="px-5 flex flex-col gap-3 pb-6">
              {RECENT_ITEMS.map((item) => (
                <button
                  key={item.name}
                  className="flex items-center gap-3 rounded-2xl px-4 py-3 active:scale-[0.98] transition-transform text-left"
                  style={{ background: "#f5f5f5" }}
                >
                  <span style={{ fontSize: 28 }}>{item.emoji}</span>
                  <div className="flex-1 min-w-0">
                    <p style={{ fontSize: 15, fontWeight: 500 }}>
                      {item.name}
                    </p>
                    <p style={{ fontSize: 12, opacity: 0.45 }}>
                      {item.time}
                    </p>
                  </div>
                  <span
                    className="shrink-0 px-2 py-0.5 rounded-full"
                    style={{
                      fontSize: 11,
                      fontWeight: 500,
                      background: item.tagColor + "40",
                    }}
                  >
                    {item.tag}
                  </span>
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
