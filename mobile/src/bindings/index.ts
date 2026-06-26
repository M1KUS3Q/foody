export { meal } from "./meal";
export { ingredient } from "./ingredient";
export { daypart } from "./daypart";
export { category } from "./category";
export { plan } from "./plan";
export { recipe } from "./recipe";
export { grocery } from "./grocery";
export type * from "./types";

import { meal } from "./meal";
import { ingredient } from "./ingredient";
import { daypart } from "./daypart";
import { category } from "./category";
import { plan } from "./plan";
import { recipe } from "./recipe";
import { grocery } from "./grocery";

export const foody = {
  meal,
  ingredient,
  daypart,
  category,
  plan,
  recipe,
  grocery,
} as const;
