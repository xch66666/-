import { useEffect, useRef, useState } from "react";
import * as PIXI from "pixi.js";
import { Live2DModel } from "pixi-live2d-display/cubism4";

// 全局 PIXI
(window as any).PIXI = PIXI;

export type CompanionState =
  | "idle"
  | "speaking"
  | "thinking"
  | "sleeping"
  | "excited"
  | "concerned";

interface Live2DViewProps {
  modelPath?: string;
  state?: CompanionState;
  width?: number;
  height?: number;
}

const STATE_MAPPINGS: Record<CompanionState, { motion?: string }> = {
  idle: { motion: "Idle" },
  speaking: { motion: "Tap" },
  thinking: { motion: "Idle" },
  sleeping: {},
  excited: { motion: "Tap" },
  concerned: {},
};

export function Live2DView({
  modelPath = "/models/haru/Haru/haru.model3.json",
  state = "idle",
  width = 300,
  height = 400,
}: Live2DViewProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const modelRef = useRef<any>(null);
  const [status, setStatus] = useState("等待渲染...");
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;

    setStatus("创建 PIXI...");

    // --- 创建 PIXI ---
    const app = new PIXI.Application({
      width,
      height,
      backgroundAlpha: 0,
      antialias: true,
      resolution: window.devicePixelRatio || 1,
      autoDensity: true,
    });
    el.appendChild(app.view as unknown as Node);
    setStatus("PIXI ✅ 加载模型...");

    let cancelled = false;

    // --- 加载模型 ---
    Live2DModel.from(modelPath)
      .then((model) => {
        if (cancelled) return;
        setStatus(`模型 ✅ ${model.width}x${model.height}`);

        const s = Math.min(width / model.width, height / model.height) * 0.8;
        model.scale.set(s);
        model.anchor.set(0.5, 0.5);
        model.x = width / 2;
        model.y = height / 2;
        app.stage.addChild(model as unknown as PIXI.Container);

        // 鼠标追踪
        const onMove = (e: PointerEvent) => {
          const r = (app.view as HTMLCanvasElement).getBoundingClientRect();
          model.focus(e.clientX - r.left, e.clientY - r.top);
        };
        (app.view as HTMLCanvasElement).addEventListener("pointermove", onMove);

        modelRef.current = model;
        setLoaded(true);
        setStatus("✅ 就绪");
      })
      .catch((err) => {
        if (cancelled) return;
        const msg = err instanceof Error ? err.message : String(err);
        console.error("[Live2D] 加载失败:", msg, err);
        setStatus("❌ " + msg);
      });

    return () => {
      cancelled = true;
      app.destroy(true, { children: true });
      modelRef.current = null;
    };
  }, [modelPath, width, height]);

  // 状态 → 动作
  useEffect(() => {
    if (!modelRef.current || !loaded) return;
    const m = STATE_MAPPINGS[state];
    if (m?.motion) {
      try { modelRef.current.motion(m.motion); } catch { /* */ }
    }
  }, [state, loaded]);

  return (
    <div className="relative" style={{ width, height }}>
      <div ref={containerRef} className="w-full h-full" />

      {/* 状态条 */}
      <div
        style={{
          position: "absolute",
          bottom: 0,
          left: 0,
          right: 0,
          background: "rgba(0,0,0,0.8)",
          color: "#4ade80",
          fontSize: 10,
          fontFamily: "monospace",
          padding: "2px 6px",
          zIndex: 999,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
        }}
      >
        {status}
      </div>
    </div>
  );
}
