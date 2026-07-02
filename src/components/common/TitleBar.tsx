interface TitleBarProps {
  title?: string;
}

export function TitleBar({ title = "Companion" }: TitleBarProps) {
  return (
    <div className="titlebar flex items-center justify-between px-3 py-2 select-none">
      <div className="flex items-center gap-2">
        <span className="text-sm">🐾</span>
        <span className="text-xs font-medium text-gray-600">{title}</span>
      </div>
    </div>
  );
}
