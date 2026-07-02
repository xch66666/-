import { useEffect, useRef, useState } from "react";
import * as PIXI from "pixi.js";
import { Live2DModel } from "pixi-live2d-display";

// pixi-live2d-display 需要全局 PIXI
(window as unknown as Record<string, unknown>).PIXI = PIXI;

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

// 状态 → Live2D 表情/动作映射
const STATE_MAPPINGS: Record<
  CompanionState,
  { expression?: string; motion?: string }
> = {
  idle: { expression: undefined, motion: undefined },
  speaking: { expression: undefined, motion: "talk" },
  thinking: { expression: "thinking", motion: undefined },
  sleeping: { expression: "sleeping", motion: undefined },
  excited: { expression: "happy", motion: undefined },
  concerned: { expression: "sad", motion: undefined },
};

export function Live2DView({
  modelPath = "/models/haru/haru.model3.json",
  state = "idle",
  width = 300,
  height = 400,
}: Live2DViewProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const appRef = useRef<PIXI.Application | null>(null);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const modelRef = useRef<any>(null);
  const [loaded, setLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 初始化 PixiJS + Live2D
  useEffect(() => {
    if (!canvasRef.current) return;

    const canvas = canvasRef.current;

    const app = new PIXI.Application({
      view: canvas,
      width,
      height,
      backgroundAlpha: 0,
      antialias: true,
      resolution: window.devicePixelRatio || 1,
      autoDensity: true,
    });

    appRef.current = app;

    // 加载 Live2D 模型
    Live2DModel.from(modelPath)
      .then((model) => {
        if (!appRef.current) return;

        modelRef.current = model;

        // 调整模型大小适配画布
        const scale = Math.min(
          width / model.width,
          height / model.height
        ) * 0.8;
        model.scale.set(scale);
        model.anchor.set(0.5, 0.5);
        model.x = width / 2;
        model.y = height / 2;

        app.stage.addChild(model as unknown as PIXI.Container);

        // 鼠标追踪：pointermove 时让模型注视鼠标
        const onPointerMove = (e: PointerEvent) => {
          const rect = canvas.getBoundingClientRect();
          const x = e.clientX - rect.left;
          const y = e.clientY - rect.top;
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          (model as any).focus(x, y);
        };
        canvas.addEventListener("pointermove", onPointerMove);

        setLoaded(true);
        console.log("Live2D 模型加载成功:", modelPath);

        // 清理函数
        const origDestroy = app.destroy.bind(app);
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (app as any).destroy = (...args: any[]) => {
          canvas.removeEventListener("pointermove", onPointerMove);
          origDestroy(...args);
        };
      })
      .catch((err) => {
        console.warn("Live2D 模型加载失败:", err);
        setError("模型加载失败，请放置 Live2D 模型到 public/models/");
      });

    return () => {
      app.destroy(true, { children: true });
      appRef.current = null;
      modelRef.current = null;
    };
  }, [modelPath, width, height]);

  // 状态变化时切换表情/动作
  useEffect(() => {
    const model = modelRef.current;
    if (!model || !loaded) return;

    const mapping = STATE_MAPPINGS[state];
    if (!mapping) return;

    try {
      // 尝试设置表情
      if (mapping.expression) {
        try {
          model.expression(mapping.expression);
        } catch {
          // 表情不存在，忽略
        }
      }

      // 尝试播放动作
      if (mapping.motion) {
        try {
          model.motion(mapping.motion);
        } catch {
          // 动作组不存在，忽略
        }
      }
    } catch (e) {
      console.debug("Live2D 状态切换:", e);
    }
  }, [state, loaded]);

  return (
    <div className="relative" style={{ width, height }}>
      <canvas ref={canvasRef} className="w-full h-full" />

      {/* 加载失败时显示占位 */}
      {error && (
        <div className="absolute inset-0 flex flex-col items-center justify-center text-gray-400">
          <span className="text-6xl mb-2">🐾</span>
          <p className="text-xs text-center px-4">{error}</p>
        </div>
      )}
    </div>
  );
}
