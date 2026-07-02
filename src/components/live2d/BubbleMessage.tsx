import { useEffect, useState } from "react";

interface BubbleMessageProps {
  text: string;
  visible?: boolean;
  autoHide?: number; // 自动隐藏毫秒数
  onHide?: () => void;
}

export function BubbleMessage({
  text,
  visible = true,
  autoHide = 8000,
  onHide,
}: BubbleMessageProps) {
  const [show, setShow] = useState(visible);
  const [animating, setAnimating] = useState(false);

  useEffect(() => {
    if (visible && text) {
      setShow(true);
      setAnimating(true);
      setTimeout(() => setAnimating(false), 300);

      if (autoHide > 0) {
        const timer = setTimeout(() => {
          setShow(false);
          onHide?.();
        }, autoHide);
        return () => clearTimeout(timer);
      }
    } else {
      setShow(false);
    }
  }, [text, visible, autoHide, onHide]);

  if (!show || !text) return null;

  return (
    <div
      className={`absolute left-1/2 -translate-x-1/2 transition-all duration-300 ${
        animating
          ? "opacity-0 translate-y-2 scale-95"
          : "opacity-100 translate-y-0 scale-100"
      }`}
      style={{ top: "30%" }}
    >
      <div className="relative bg-white/95 backdrop-blur-sm rounded-2xl px-4 py-2.5 shadow-lg border border-gray-100 max-w-[220px]">
        <p className="text-sm text-gray-700 leading-relaxed">{text}</p>
        {/* 小三角 */}
        <div className="absolute -bottom-2 left-1/2 -translate-x-1/2 w-0 h-0 border-l-[6px] border-l-transparent border-r-[6px] border-r-transparent border-t-[8px] border-t-white/95" />
      </div>
    </div>
  );
}
