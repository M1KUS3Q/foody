import { invoke } from "@tauri-apps/api/core";
import type { MealView } from "./types";

export const meal = {
  async add(name: string): Promise<void> {
    await invoke("meal", { action: "Add", name });
  },

  async remove(name: string, force: boolean = false): Promise<void> {
    await invoke("meal", { action: "Remove", name, force });
  },

  async view(name: string): Promise<MealView> {
    return await invoke("meal", { action: "View", name });
  },

  async list(): Promise<string[]> {
    return await invoke("meal", { action: "List" });
  },

  async rename(name: string, newName: string): Promise<void> {
    await invoke("meal", { action: "Rename", name, new_name: newName });
  },
};
