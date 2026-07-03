import { defineConfig, type Plugin } from "vite";
import react from "@vitejs/plugin-react";

const host = process.env.TAURI_DEV_HOST;

/**
 * 修复 PixiJS v6 在 Tauri WebView2 下
 * checkMaxIfStatementsInShader 抛 0 的 bug。
 * 直接改写预打包后的源码，把 throw 替换为 Math.max(1, ...)。
 */
function fixPixiWebGLCheck(): Plugin {
  return {
    name: "fix-pixi-webgl-check",
    transform(code, id) {
      // 命中 Vite 预打包的 pixi chunk
      if (!id.includes(".vite") || !code.includes("checkMaxIfStatementsInShader")) {
        return null;
      }
      console.log("[fix-pixi-webgl-check] 补丁命中:", id);
      const patched = code.replace(
        /if\s*\(\s*maxIfs\s*===?\s*0\s*\)\s*\{[^}]*\}/,
        "if (maxIfs === 0) { maxIfs = 1; }"
      );
      return { code: patched, map: null };
    },
  };
}

export default defineConfig({
  plugins: [fixPixiWebGLCheck(), react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
