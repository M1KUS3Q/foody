import { invoke } from "@tauri-apps/api/core";

export const recipe = {
  async set(mealName: string, recipe: string): Promise<void> {
    await invoke("recipe", { action: "Set", meal_name: mealName, recipe });
  },

  async view(mealName: string): Promise<string | null> {
    return await invoke("recipe", { action: "View", meal_name: mealName });
  },

  async remove(mealName: string): Promise<void> {
    await invoke("recipe", { action: "Remove", meal_name: mealName });
  },
};
