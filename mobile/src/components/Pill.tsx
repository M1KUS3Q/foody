interface PillProps {
  label: string;
  active?: boolean;
  onClick?: () => void;
}

export function Pill({ label, active, onClick }: PillProps) {
  return (
    <button
      onClick={onClick}
      className="shrink-0 px-4 py-1.5 rounded-full transition-colors"
      style={{
        background: active ? "#000" : "rgba(0,0,0,0.08)",
        color: active ? "#fff" : "rgba(0,0,0,0.6)",
        fontSize: 13,
        fontWeight: active ? 600 : 400,
      }}
    >
      {label}
    </button>
  );
}
