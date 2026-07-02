import { useState } from "react";

function App() {
  const [count, setCount] = useState(0);

  return (
    <div className="min-h-screen flex items-center justify-center">
      <div className="bg-white/90 backdrop-blur-sm rounded-xl p-8 shadow-lg text-center">
        <h1 className="text-2xl font-bold text-gray-800 mb-4">
          🐾 Companion
        </h1>
        <p className="text-gray-600 mb-4">AI 小伙伴正在启动中...</p>
        <button
          onClick={() => setCount((c) => c + 1)}
          className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition"
        >
          点击 {count} 次
        </button>
      </div>
    </div>
  );
}

export default App;
