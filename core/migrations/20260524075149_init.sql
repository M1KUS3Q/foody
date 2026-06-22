-- Add migration script here
CREATE TABLE IF NOT EXISTS dayparts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE
);

CREATE TABLE IF NOT EXISTS meals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,
    recipe TEXT
);

CREATE TABLE IF NOT EXISTS ingredients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE
);

CREATE TABLE IF NOT EXISTS meal_ingredients (
    meal_id INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    PRIMARY KEY (meal_id, ingredient_id),
    FOREIGN KEY (meal_id) REFERENCES meals(id) ON DELETE CASCADE,
    FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS meal_dayparts (
    meal_id INTEGER NOT NULL,
    daypart_id INTEGER NOT NULL,
    PRIMARY KEY (meal_id, daypart_id),
    FOREIGN KEY (meal_id) REFERENCES meals(id) ON DELETE CASCADE,
    FOREIGN KEY (daypart_id) REFERENCES dayparts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS meal_plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE
);

CREATE TABLE IF NOT EXISTS meal_plan_days (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    meal_plan_id INTEGER NOT NULL,
    day_index INTEGER NOT NULL,
    FOREIGN KEY (meal_plan_id) REFERENCES meal_plans(id) ON DELETE CASCADE,
    UNIQUE (meal_plan_id, day_index)
);

CREATE TABLE IF NOT EXISTS meal_plan_day_items (
    meal_plan_day_id INTEGER NOT NULL,
    daypart_id INTEGER NOT NULL,
    meal_id INTEGER NOT NULL,
    PRIMARY KEY (meal_plan_day_id, daypart_id),
    FOREIGN KEY (meal_plan_day_id) REFERENCES meal_plan_days(id) ON DELETE CASCADE,
    FOREIGN KEY (meal_id, daypart_id) REFERENCES meal_dayparts(meal_id, daypart_id) ON DELETE CASCADE
);