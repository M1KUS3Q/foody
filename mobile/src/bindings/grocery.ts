import { invoke } from "@tauri-apps/api/core";

export const grocery = {
  async plan(name: string): Promise<string> {
    return await invoke("grocery", { action: "Plan", name });
  },

  async meal(name: string): Promise<string> {
    return await invoke("grocery", { action: "Meal", name });
  },
};
