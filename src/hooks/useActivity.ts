import { useState, useEffect, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import * as api from "../api";
import type { ActivityType, ActivityChangedEvent } from "../types";

interface UseActivityReturn {
  currentActivity: ActivityType | null;
  currentProcess: string | null;
  currentTitle: string | null;
  isIdle: boolean;
  isFullscreen: boolean;
  getStats: (since?: number) => Promise<Array<{ activity: string; total_seconds: number }>>;
}

export function useActivity(): UseActivityReturn {
  const [currentActivity, setCurrentActivity] = useState<ActivityType | null>(null);
  const [currentProcess, setCurrentProcess] = useState<string | null>(null);
  const [currentTitle, setCurrentTitle] = useState<string | null>(null);
  const [isIdle, setIsIdle] = useState(false);
  const [isFullscreen, setIsFullscreen] = useState(false);

  // 初始加载当前活动
  useEffect(() => {
    api.getCurrentActivity().then((log) => {
      if (log) {
        setCurrentActivity(log.activity as ActivityType);
        setCurrentProcess(log.process_name);
        setCurrentTitle(log.window_title);
      }
    }).catch(console.error);
  }, []);

  // 监听活动变化事件
  useEffect(() => {
    const unlistenChanged = listen<ActivityChangedEvent>(
      "activity:changed",
      (event) => {
        const { activity, process_name, window_title } = event.payload;
        setCurrentActivity(activity as ActivityType);
        setCurrentProcess(process_name);
        setCurrentTitle(window_title);
        setIsIdle(activity === "idle");
        console.log("[activity] 活动变化:", activity, process_name);
      }
    );

    // 监听全屏事件
    const unlistenFullscreen = listen<{ process_name: string }>(
      "activity:fullscreen",
      () => {
        setIsFullscreen(true);
        console.log("[activity] 全屏检测");
        // 5 秒后恢复（全屏是瞬时状态）
        setTimeout(() => setIsFullscreen(false), 5000);
      }
    );

    return () => {
      unlistenChanged.then((fn) => fn());
      unlistenFullscreen.then((fn) => fn());
    };
  }, []);

  const getStats = useCallback(
    (since?: number) => api.getActivityStats(since),
    []
  );

  return {
    currentActivity,
    currentProcess,
    currentTitle,
    isIdle,
    isFullscreen,
    getStats,
  };
}
