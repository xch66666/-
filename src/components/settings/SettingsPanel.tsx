import { useState, useEffect } from "react";
import * as api from "../../api";
import type { PersonaConfig, LlmConfig } from "../../types";

interface SettingsPanelProps {
  onClose: () => void;
}

export function SettingsPanel({ onClose }: SettingsPanelProps) {
  const [activeTab, setActiveTab] = useState<"persona" | "llm" | "about">("persona");
  const [persona, setPersona] = useState<PersonaConfig>({
    name: "小伴",
    personality: "温柔体贴，偶尔俏皮",
    style: "口语化，像朋友聊天",
    nickname: "你",
  });
  const [llm, setLlm] = useState<LlmConfig>({
    provider: "openai",
    model: "gpt-4o-mini",
    max_context_messages: 20,
    temperature: 0.8,
    api_key: "",
    api_base: "",
  });

  // 提供商预设
  const PROVIDERS = [
    { id: "openai" as const, name: "OpenAI (GPT)", base: "https://api.openai.com/v1", models: ["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo"] },
    { id: "deepseek" as const, name: "DeepSeek", base: "https://api.deepseek.com/v1", models: ["deepseek-chat", "deepseek-reasoner"] },
    { id: "qwen" as const, name: "通义千问 (Qwen)", base: "https://dashscope.aliyuncs.com/compatible-mode/v1", models: ["qwen-turbo", "qwen-plus", "qwen-max", "qwen-long"] },
    { id: "claude" as const, name: "Claude (Anthropic)", base: "https://api.anthropic.com/v1", models: ["claude-sonnet-4-20250514", "claude-3-5-haiku-20241022"] },
    { id: "custom" as const, name: "自定义 (OpenAI 兼容)", base: "", models: [] },
  ];

  const currentProvider = PROVIDERS.find((p) => p.id === llm.provider) || PROVIDERS[4]!;

  const handleProviderChange = (providerId: LlmConfig["provider"]) => {
    const p = PROVIDERS.find((pr) => pr.id === providerId);
    if (p) {
      setLlm({
        ...llm,
        provider: providerId,
        api_base: p.base,
        model: p.models[0] || llm.model,
      });
    }
  };
  const [saved, setSaved] = useState(false);

  // 加载配置
  useEffect(() => {
    api.getParsedConfig<PersonaConfig>("persona").then((p) => {
      if (p) setPersona(p);
    });
    api.getParsedConfig<LlmConfig>("llm").then((l) => {
      if (l) setLlm(l);
    });
  }, []);

  const savePersona = async () => {
    await api.setConfig("persona", persona);
    showSaved();
  };

  const saveLlm = async () => {
    await api.setConfig("llm", llm);
    showSaved();
  };

  const showSaved = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  return (
    <div className="flex flex-col h-full bg-gray-50/95 backdrop-blur-md rounded-2xl shadow-xl border border-gray-200/50 overflow-hidden">
      {/* 头部 */}
      <div className="flex items-center justify-between px-4 py-3 bg-white/80 border-b border-gray-100">
        <h2 className="text-sm font-semibold text-gray-700">⚙️ 设置</h2>
        <button
          onClick={onClose}
          className="p-1.5 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      {/* Tab 导航 */}
      <div className="flex border-b border-gray-100">
        {(["persona", "llm", "about"] as const).map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`flex-1 py-2 text-xs font-medium transition ${
              activeTab === tab
                ? "text-indigo-600 border-b-2 border-indigo-500"
                : "text-gray-500 hover:text-gray-700"
            }`}
          >
            {tab === "persona" ? "人设" : tab === "llm" ? "AI 模型" : "关于"}
          </button>
        ))}
      </div>

      {/* 内容区 */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {activeTab === "persona" && (
          <>
            <Field label="名字" value={persona.name} onChange={(v) => setPersona({ ...persona, name: v })} />
            <Field label="性格" value={persona.personality} onChange={(v) => setPersona({ ...persona, personality: v })} />
            <Field label="说话风格" value={persona.style} onChange={(v) => setPersona({ ...persona, style: v })} />
            <Field label="称呼用户" value={persona.nickname} onChange={(v) => setPersona({ ...persona, nickname: v })} />
            <button onClick={savePersona} className="w-full py-2 bg-indigo-500 text-white rounded-xl text-sm hover:bg-indigo-600 transition">
              保存人设
            </button>
          </>
        )}

        {activeTab === "llm" && (
          <>
            {/* 提供商选择 */}
            <div>
              <label className="block text-xs text-gray-500 mb-1">服务商</label>
              <select
                value={llm.provider}
                onChange={(e) => handleProviderChange(e.target.value as LlmConfig["provider"])}
                className="w-full px-3 py-2 text-sm border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-300 bg-white/90"
              >
                {PROVIDERS.map((p) => (
                  <option key={p.id} value={p.id}>{p.name}</option>
                ))}
              </select>
            </div>

            {/* 模型选择 */}
            {currentProvider.models.length > 0 ? (
              <div>
                <label className="block text-xs text-gray-500 mb-1">模型</label>
                <select
                  value={llm.model}
                  onChange={(e) => setLlm({ ...llm, model: e.target.value })}
                  className="w-full px-3 py-2 text-sm border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-300 bg-white/90"
                >
                  {currentProvider.models.map((m) => (
                    <option key={m} value={m}>{m}</option>
                  ))}
                </select>
              </div>
            ) : (
              <Field label="模型名称" value={llm.model} onChange={(v) => setLlm({ ...llm, model: v })} placeholder="model-name" />
            )}

            <Field label="API Base URL" value={llm.api_base || ""} onChange={(v) => setLlm({ ...llm, api_base: v })} placeholder={currentProvider.base || "https://your-api.com/v1"} />
            <Field label="API Key" value={llm.api_key || ""} onChange={(v) => setLlm({ ...llm, api_key: v })} type="password" placeholder="sk-..." />
            <div>
              <label className="block text-xs text-gray-500 mb-1">Temperature: {llm.temperature}</label>
              <input
                type="range"
                min="0"
                max="2"
                step="0.1"
                value={llm.temperature}
                onChange={(e) => setLlm({ ...llm, temperature: parseFloat(e.target.value) })}
                className="w-full"
              />
            </div>
            <button onClick={saveLlm} className="w-full py-2 bg-indigo-500 text-white rounded-xl text-sm hover:bg-indigo-600 transition">
              保存模型设置
            </button>
          </>
        )}

        {activeTab === "about" && (
          <div className="text-center text-gray-500 text-sm space-y-3 pt-8">
            <span className="text-5xl block">🐾</span>
            <p className="font-semibold text-gray-700">Companion</p>
            <p>版本 0.1.0</p>
            <p className="text-xs">你的桌面 AI 小伙伴</p>
            <p className="text-xs text-gray-400">Made with ❤️ and Tauri</p>
          </div>
        )}
      </div>

      {/* 保存提示 */}
      {saved && (
        <div className="absolute bottom-4 left-1/2 -translate-x-1/2 bg-green-500 text-white text-xs px-3 py-1.5 rounded-full shadow-lg animate-bounce">
          ✅ 已保存
        </div>
      )}
    </div>
  );
}

function Field({
  label,
  value,
  onChange,
  type = "text",
  placeholder,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
  type?: string;
  placeholder?: string;
}) {
  return (
    <div>
      <label className="block text-xs text-gray-500 mb-1">{label}</label>
      <input
        type={type}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="w-full px-3 py-2 text-sm border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-300 bg-white/90"
      />
    </div>
  );
}
