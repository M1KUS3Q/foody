import { invoke } from "@tauri-apps/api/core";
import type { PlanView } from "./types";

export const plan = {
  async add(name: string): Promise<void> {
    await invoke("plan", { command: { action: "Add", name } });
  },

  async remove(name: string): Promise<void> {
    await invoke("plan", { command: { action: "Remove", name } });
  },

  async view(name: string): Promise<PlanView> {
    return await invoke("plan", { command: { action: "View", name } });
  },

  async list(): Promise<string[]> {
    return await invoke("plan", { command: { action: "List" } });
  },

  async rename(name: string, newName: string): Promise<void> {
    await invoke("plan", { command: { action: "Rename", name, new_name: newName } });
  },

  async assign(
    planName: string,
    indexName: string,
    daypartName: string,
    mealName: string,
  ): Promise<void> {
    await invoke("plan", {
      command: {
        action: "Assign",
        planname: planName,
        indexname: indexName,
        daypartname: daypartName,
        mealname: mealName,
      },
    });
  },

  async unassign(
    planName: string,
    indexName: string,
    daypartName: string,
  ): Promise<void> {
    await invoke("plan", {
      command: {
        action: "Unassign",
        planname: planName,
        indexname: indexName,
        daypartname: daypartName,
      },
    });
  },

  async fill(planName: string, days: number = 7): Promise<void> {
    await invoke("plan", { command: { action: "Fill", planname: planName, days } });
  },
};
