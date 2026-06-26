import { invoke } from "@tauri-apps/api/core";
import type { CategoryView } from "./types";

export const category = {
  async add(name: string): Promise<void> {
    await invoke("category", {
      command: { action: "Add", name }
    });
  },

  async remove(name: string): Promise<void> {
    await invoke("category", { command: { action: "Remove", name } });
  },

  async view(name: string): Promise<CategoryView> {
    return await invoke("category", { command: { action: "View", name } });
  },

  async list(): Promise<string[]> {
    return await invoke("category", { command: { action: "List" } });
  },

  async assign(ingredientName: string, categories: string[]): Promise<void> {
    await invoke("category", {
      command: {
        action: "Assign",
        ingredientname: ingredientName,
        categories,
      },
    });
  },

  async unassign(ingredientName: string, categories: string[]): Promise<void> {
    await invoke("category", {
      command: {
        action: "Unassign",
        ingredientname: ingredientName,
        categories,
      },
    });
  },
};
