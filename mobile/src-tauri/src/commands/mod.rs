use anyhow::Result;
use serde::Serialize;
use std::future::Future;

/// Await a query call and serialize the result to JSON, mapping errors to String.
pub(crate) async fn serialize_json<T: Serialize>(
    f: impl Future<Output = anyhow::Result<T>>,
) -> Result<serde_json::Value, String> {
    f.await
        .map_err(|e| e.to_string())
        .and_then(|v| serde_json::to_value(v).map_err(|e| e.to_string()))
}

pub mod category;
pub mod daypart;
pub mod grocery;
pub mod ingredient;
pub mod meal;
pub mod plan;
pub mod recipe;
