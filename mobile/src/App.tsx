import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [mealName, setMealName] = useState("");
  const [status, setStatus] = useState<{ type: "ok" | "err"; msg: string } | null>(null);
  const [loading, setLoading] = useState(false);

  async function addMeal() {
    const name = mealName.trim();
    if (!name) return;

    setLoading(true);
    setStatus(null);

    try {
      await invoke("add_meal", { name });
      setStatus({ type: "ok", msg: `Added "${name}"!` });
      setMealName("");
    } catch (e) {
      setStatus({ type: "err", msg: String(e) });
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="container">
      <h1>Foody</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          addMeal();
        }}
      >
        <input
          id="meal-input"
          value={mealName}
          onChange={(e) => setMealName(e.currentTarget.value)}
          placeholder="What did you eat?"
          disabled={loading}
        />
        <button type="submit" disabled={loading}>
          {loading ? "Saving…" : "Add Meal"}
        </button>
      </form>

      {status && (
        <p className={status.type === "err" ? "error" : "success"}>
          {status.msg}
        </p>
      )}
    </main>
  );
}

export default App;
