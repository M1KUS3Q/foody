import { invoke } from "@tauri-apps/api/core";
import type { DaypartView } from "./types";

export const daypart = {
  async add(name: string): Promise<void> {
    await invoke("daypart", { command: { action: "Add", name } });
  },

  async remove(name: string): Promise<void> {
    await invoke("daypart", { command: { action: "Remove", name } });
  },

  async view(name: string): Promise<DaypartView> {
    return await invoke("daypart", { command: { action: "View", name } });
  },

  async list(): Promise<string[]> {
    return await invoke("daypart", { command: { action: "List" } });
  },

  async assign(mealName: string, dayparts: string[]): Promise<void> {
    await invoke("daypart", {
      command: {
        action: "Assign",
        mealname: mealName,
        dayparts,
      },
    });
  },

  async unassign(mealName: string, dayparts: string[]): Promise<void> {
    await invoke("daypart", {
      command: {
        action: "Unassign",
        mealname: mealName,
        dayparts,
      },
    });
  },
};
