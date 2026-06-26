import { invoke } from "@tauri-apps/api/core";
import type { DaypartView } from "./types";

export const daypart = {
  async add(name: string): Promise<void> {
    await invoke("daypart", { action: "Add", name });
  },

  async remove(name: string): Promise<void> {
    await invoke("daypart", { action: "Remove", name });
  },

  async view(name: string): Promise<DaypartView> {
    return await invoke("daypart", { action: "View", name });
  },

  async list(): Promise<string[]> {
    return await invoke("daypart", { action: "List" });
  },

  async assign(mealName: string, dayparts: string[]): Promise<void> {
    await invoke("daypart", {
      action: "Assign",
      mealname: mealName,
      dayparts,
    });
  },

  async unassign(mealName: string, dayparts: string[]): Promise<void> {
    await invoke("daypart", {
      action: "Unassign",
      mealname: mealName,
      dayparts,
    });
  },
};
