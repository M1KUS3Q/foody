export type WaveVariant = "default" | "home";

interface WaveDividerProps {
  variant?: WaveVariant;
  height?: number;
}

const WAVE: Record<
  WaveVariant,
  { path: string; viewBox: string; defaultHeight: number }
> = {
  default: {
    path: "M0 0 Q100 32 200 16 Q300 0 402 24 L402 32 L0 32 Z",
    viewBox: "0 0 402 32",
    defaultHeight: 32,
  },
  home: {
    path:
      "M0 48 L0 24.143 L13.4 20.119 C26.8 16.095 53.6 8.048 80.4 8.048 C107.2 8.048 134 16.095 160.8 24.143 C187.6 32.19 214.4 40.238 241.2 42.92 C268 45.603 294.8 42.92 321.6 38.896 C348.4 34.873 375.2 29.508 388.6 26.825 L402 24.143 L402 48 Z",
    viewBox: "0 0 402 48",
    defaultHeight: 48,
  },
};

export function WaveDivider({
  variant = "default",
  height,
}: WaveDividerProps) {
  const { path, viewBox, defaultHeight } = WAVE[variant];

  return (
    <svg
      viewBox={viewBox}
      fill="none"
      className="shrink-0 w-full"
      preserveAspectRatio="none"
      height={height ?? defaultHeight}
      style={{ display: "block" }}
    >
      <path d={path} fill="#fff" />
    </svg>
  );
}
