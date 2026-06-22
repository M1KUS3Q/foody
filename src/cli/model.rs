use clap::{Parser, Subcommand};
use clap_complete::Shell;

/// A meal planning and grocery management tool.
#[derive(Parser)]
#[command(name = "foody", author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Category,
}

/// Top-level resource categories.
#[derive(Subcommand, Debug)]
pub enum Category {
    /// Manage meals (add, remove, view, list, rename).
    #[command(alias = "m")]
    Meal {
        #[command(subcommand)]
        action: MealAction,
    },
    /// Manage ingredients and assign them to meals.
    #[command(aliases = ["ing", "i"])]
    Ingredient {
        #[command(subcommand)]
        action: IngredientAction,
    },
    /// Manage dayparts (breakfast, lunch, dinner, etc.) and assign them to meals.
    #[command(aliases = ["dp", "d"])]
    Daypart {
        #[command(subcommand)]
        action: DaypartAction,
    },
    /// Manage grocery categories (dairy, produce, meat, etc.) and assign them to ingredients.
    #[command(name = "category", aliases = ["cat", "c"])]
    GroceryCategory {
        #[command(subcommand)]
        action: GroceryCategoryAction,
    },
    /// Manage meal plans and assign meals to plan slots.
    #[command(alias = "p")]
    Plan {
        #[command(subcommand)]
        action: PlanAction,
    },
    /// Generate grocery lists from a plan or individual meal.
    #[command(aliases = ["gr", "g"])]
    Grocery {
        #[command(subcommand)]
        action: GroceryAction,
    },

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        shell: Shell,
    },

    /// Self-update foody to the latest version from GitHub releases.
    Upgrade {
        /// Skip confirmation prompt.
        #[arg(short, long)]
        force: bool,
    },

    /// Send feedback to the developer via a Discord webhook.
    #[command(aliases = ["fb"])]
    Feedback {
        /// Feedback content to send to the developer.
        content: String,
    },

    /// Manage recipe text for meals (set, view, remove).
    #[command(alias = "r")]
    Recipe {
        #[command(subcommand)]
        action: RecipeAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum RecipeAction {
    /// Set or update the recipe for a meal.
    #[command(aliases = ["s", "a", "add"])]
    Set {
        /// Name of the meal.
        name: String,
        /// Recipe text.
        recipe: String,
    },
    /// View the recipe for a meal.
    #[command(aliases = ["v", "show", "get"])]
    View {
        /// Name of the meal.
        name: String,
    },
    /// Remove the recipe from a meal.
    #[command(aliases = ["rm", "delete", "del"])]
    Remove {
        /// Name of the meal.
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum MealAction {
    /// Create a new meal.
    #[command(aliases = ["a", "new", "create"])]
    Add {
        /// Name of the meal to create.
        name: String,
    },
    /// Delete a meal by name.
    #[command(aliases = ["rm", "delete", "del"])]
    Remove {
        /// Skip confirmation prompt.
        #[arg(short, long)]
        force: bool,
        /// Name of the meal to remove.
        name: String,
    },
    /// Display details of a specific meal.
    #[command(aliases = ["v", "show", "get"])]
    View {
        /// Name of the meal to view.
        name: String,
    },
    /// List all meals.
    #[command(aliases = ["ls", "l"])]
    List,
    /// Rename an existing meal.
    #[command(aliases = ["mv", "rn"])]
    Rename {
        /// Current name of the meal.
        name: String,
        /// New name to assign.
        new_name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum IngredientAction {
    /// Create a new ingredient.
    #[command(aliases = ["a", "new", "create"])]
    Add {
        /// Name of the ingredient to create.
        name: String,
    },
    /// Delete an ingredient by name.
    #[command(aliases = ["rm", "delete", "del"])]
    Remove {
        /// Skip confirmation prompt.
        #[arg(short, long)]
        force: bool,
        /// Name of the ingredient to remove.
        name: String,
    },
    /// Display details of a specific ingredient.
    #[command(aliases = ["v", "show", "get"])]
    View {
        /// Name of the ingredient to view.
        name: String,
    },
    /// List all ingredients.
    #[command(aliases = ["ls", "l"])]
    List,
    /// Rename an existing ingredient.
    #[command(aliases = ["mv", "rn"])]
    Rename {
        /// Current name of the ingredient.
        name: String,
        /// New name to assign.
        new_name: String,
    },
    /// Assign one or more ingredients to a meal.
    ///
    /// Accepts a comma-separated list of ingredient names.
    /// Example: foody ingredient assign "Pasta Bolognese" pasta,beef,tomato
    #[command(aliases = ["as", "attach", "link"])]
    Assign {
        /// Name of the target meal.
        mealname: String,
        /// Comma-separated list of ingredients to assign.
        #[arg(value_delimiter = ',')]
        ingredients: Vec<String>,
    },
    /// Remove one or more ingredients from a meal.
    ///
    /// Accepts a comma-separated list of ingredient names.
    #[command(aliases = ["un", "detach", "unlink"])]
    Unassign {
        /// Name of the target meal.
        mealname: String,
        /// Comma-separated list of ingredients to remove.
        #[arg(value_delimiter = ',')]
        ingredients: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum DaypartAction {
    /// Create a new daypart (e.g. breakfast, lunch, dinner).
    #[command(aliases = ["a", "new", "create"])]
    Add {
        /// Name of the daypart to create.
        name: String,
    },
    /// Delete a daypart by name.
    #[command(aliases = ["rm", "delete", "del"])]
    Remove {
        /// Name of the daypart to remove.
        name: String,
    },
    /// Display details of a specific daypart, e.g. which meals are assigned to it.
    #[command(aliases = ["v", "show", "get"])]
    View {
        /// Name of the daypart to view.
        name: String,
    },
    /// List all dayparts.
    #[command(aliases = ["ls", "l"])]
    List,
    /// Assign one or more dayparts to a meal.
    ///
    /// Example: foody daypart assign "Omelette" breakfast,brunch
    #[command(aliases = ["as", "attach", "link"])]
    Assign {
        /// Name of the target meal.
        mealname: String,
        /// Comma-separated list of dayparts to assign.
        #[arg(value_delimiter = ',')]
        dayparts: Vec<String>,
    },
    /// Remove one or more dayparts from a meal.
    #[command(aliases = ["un", "detach", "unlink"])]
    Unassign {
        /// Name of the target meal.
        mealname: String,
        /// Comma-separated list of dayparts to remove.
        #[arg(value_delimiter = ',')]
        dayparts: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum GroceryCategoryAction {
    /// Create a new grocery category (e.g. dairy, produce, meat).
    #[command(aliases = ["a", "new", "create"])]
    Add {
        /// Name of the category to create.
        name: String,
    },
    /// Delete a category by name.
    #[command(aliases = ["rm", "delete", "del"])]
    Remove {
        /// Name of the category to remove.
        name: String,
    },
    /// Display details of a specific category, e.g. which ingredients belong to it.
    #[command(aliases = ["v", "show", "get"])]
    View {
        /// Name of the category to view.
        name: String,
    },
    /// List all categories.
    #[command(aliases = ["ls", "l"])]
    List,
    /// Assign one or more categories to an ingredient.
    ///
    /// Example: foody category assign "Egg" dairy,produce
    #[command(aliases = ["as", "attach", "link"])]
    Assign {
        /// Name of the target ingredient.
        ingredientname: String,
        /// Comma-separated list of categories to assign.
        #[arg(value_delimiter = ',')]
        categories: Vec<String>,
    },
    /// Remove one or more categories from an ingredient.
    #[command(aliases = ["un", "detach", "unlink"])]
    Unassign {
        /// Name of the target ingredient.
        ingredientname: String,
        /// Comma-separated list of categories to remove.
        #[arg(value_delimiter = ',')]
        categories: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum PlanAction {
    /// Create a new meal plan.
    #[command(aliases = ["a", "new", "create"])]
    Add {
        /// Name of the plan to create.
        name: String,
    },
    /// Delete a meal plan by name.
    #[command(aliases = ["rm", "delete", "del"])]
    Remove {
        /// Name of the plan to remove.
        name: String,
    },
    /// Display all slots in a plan.
    #[command(aliases = ["v", "show", "get"])]
    View {
        /// Name of the plan to view.
        name: String,
    },
    /// List all plans.
    #[command(aliases = ["ls", "l"])]
    List,
    /// Rename an existing plan.
    #[command(aliases = ["mv", "rn"])]
    Rename {
        /// Current name of the plan.
        name: String,
        /// New name to assign.
        new_name: String,
    },
    /// Assign a meal to a specific slot in a plan.
    ///
    /// Example: foody plan assign "Week1" "Day1" dinner "Pasta Bolognese"
    #[command(aliases = ["as", "attach", "link", "set"])]
    Assign {
        /// Name of the target plan.
        planname: String,
        /// Day or index label for the slot (e.g. "Day1", "Monday").
        indexname: String,
        /// Daypart for the slot (e.g. dinner).
        daypartname: String,
        /// Name of the meal to assign.
        mealname: String,
    },
    /// Remove a meal from a specific plan slot.
    #[command(aliases = ["un", "detach", "unlink", "clear"])]
    Unassign {
        /// Name of the target plan.
        planname: String,
        /// Day or index label of the slot to clear.
        indexname: String,
        /// Daypart of the slot to clear.
        daypartname: String,
    },
    /// Auto-fill a plan with random eligible meals.
    #[command(aliases = ["f", "auto", "generate", "gen"])]
    Fill {
        /// Name of the plan to fill.
        planname: String,
        /// Number of days to fill [default: 7].
        #[arg(short, long, default_value_t = 7)]
        days: usize,
    },
}

#[derive(Subcommand, Debug)]
pub enum GroceryAction {
    /// Generate a grocery list from an entire meal plan.
    ///
    /// Aggregates all ingredients across every slot in the plan.
    /// Optionally export to a file.
    #[command(aliases = ["p", "from-plan"])]
    Plan {
        /// Name of the plan to generate a grocery list for.
        name: String,
        /// Write output to this file path instead of stdout.
        #[arg(short, long, value_name = "FILE")]
        export: Option<String>,
    },
    /// Generate a grocery list for a single meal.
    #[command(aliases = ["m", "from-meal"])]
    Meal {
        /// Name of the meal to generate a grocery list for.
        name: String,
        /// Write output to this file path instead of stdout.
        #[arg(short, long, value_name = "FILE")]
        export: Option<String>,
    },
}
