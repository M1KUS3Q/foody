import { invoke } from "@tauri-apps/api/core";
import type { CategoryView } from "./types";

export const category = {
  async add(name: string): Promise<void> {
    await invoke("category", { action: "Add", name });
  },

  async remove(name: string): Promise<void> {
    await invoke("category", { action: "Remove", name });
  },

  async view(name: string): Promise<CategoryView> {
    return await invoke("category", { action: "View", name });
  },

  async list(): Promise<string[]> {
    return await invoke("category", { action: "List" });
  },

  async assign(ingredientName: string, categories: string[]): Promise<void> {
    await invoke("category", {
      action: "Assign",
      ingredientname: ingredientName,
      categories,
    });
  },

  async unassign(ingredientName: string, categories: string[]): Promise<void> {
    await invoke("category", {
      action: "Unassign",
      ingredientname: ingredientName,
      categories,
    });
  },
};
