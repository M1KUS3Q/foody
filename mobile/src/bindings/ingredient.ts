import { invoke } from "@tauri-apps/api/core";
import type { IngredientView } from "./types";

export const ingredient = {
  async add(name: string): Promise<void> {
    await invoke("ingredient", { action: "Add", name });
  },

  async remove(name: string, force: boolean = false): Promise<void> {
    await invoke("ingredient", { action: "Remove", name, force });
  },

  async view(name: string): Promise<IngredientView> {
    return await invoke("ingredient", { action: "View", name });
  },

  async list(): Promise<string[]> {
    return await invoke("ingredient", { action: "List" });
  },

  async rename(name: string, newName: string): Promise<void> {
    await invoke("ingredient", {
      action: "Rename",
      name,
      new_name: newName,
    });
  },

  async assign(mealName: string, ingredients: string[]): Promise<void> {
    await invoke("ingredient", {
      action: "Assign",
      mealname: mealName,
      ingredients,
    });
  },

  async unassign(mealName: string, ingredients: string[]): Promise<void> {
    await invoke("ingredient", {
      action: "Unassign",
      mealname: mealName,
      ingredients,
    });
  },
};
