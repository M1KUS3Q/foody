export interface MealView {
  id: number;
  name: string;
  dayparts: string[];
  ingredients: string[];
  recipe: string | null;
}

export interface IngredientView {
  id: number;
  name: string;
  meals: string[];
  categories: string[];
}

export interface DaypartView {
  id: number;
  name: string;
  meals: string[];
}

export interface CategoryView {
  id: number;
  name: string;
  ingredients: string[];
}

export interface PlanItem {
  day_index: number;
  daypart_name: string;
  meal_name: string;
}

export interface PlanView {
  id: number;
  name: string;
  items: PlanItem[];
}
