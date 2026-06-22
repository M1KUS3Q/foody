fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = std::path::Path::new(&manifest_dir).parent().unwrap();
    let env_path = workspace_root.join(".env");

    println!("cargo:rerun-if-changed={}", env_path.display());
    dotenv::from_path(&env_path).ok();
    for (key, value) in dotenv::vars() {
        if key == "DATABASE_URL" {
            // Emit an absolute path so sqlx macros find the DB regardless of CWD.
            // sqlx macros run with CWD = workspace root (where cargo was invoked),
            // not the core/ crate directory.
            let db_path = workspace_root.join("data.db");
            println!("cargo:rustc-env=DATABASE_URL=sqlite://{}", db_path.display());
        } else {
            println!("cargo:rustc-env={}={}", key, value);
        }
    }
}
