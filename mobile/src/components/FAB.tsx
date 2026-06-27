import { Plus } from "lucide-react";

interface FABProps {
  onClick?: () => void;
}

export function FAB({ onClick }: FABProps) {
  return (
    <button
      onClick={onClick}
      className="absolute flex items-center justify-center rounded-full shadow-lg active:scale-95 transition-transform"
      style={{
        right: 20,
        bottom: 88,
        width: 52,
        height: 52,
        background: "#000",
        color: "#fff",
        zIndex: 10,
      }}
    >
      <Plus size={22} strokeWidth={2.5} />
    </button>
  );
}
