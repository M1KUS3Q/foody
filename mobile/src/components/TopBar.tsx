import type { ReactNode } from "react";

interface TopBarProps {
  title: ReactNode;
  right?: ReactNode;
}

export function TopBar({ title, right }: TopBarProps) {
  return (
    <div
      className="shrink-0 flex items-center justify-between px-5"
      style={{
        paddingTop: 50, paddingLeft: 20, paddingRight: 20, paddingBottom: 10, background: "var(--gold)",
      }}
    >
      <span
        style={{
          fontSize: 26,
          fontWeight: 600,
          letterSpacing: "-0.02em",
        }}
      >
        {title}
      </span>
      {right}
    </div >
  );
}
