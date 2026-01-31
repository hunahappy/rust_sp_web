use crate::AppState;
use axum::extract::State;
use serde_json::Value;
use sqlx::Row;

pub async fn select_json(State(state): State<AppState>, str_query: &str, vec_sql_jo: Vec<&str>) -> Vec<Value> {
    let start_query = "select row_to_json(t) as j from (";
    let end_query = ") t";
    let str_query_2 = format!("{}{}{}", start_query, str_query, end_query);

    let mut q = sqlx::query(str_query_2.as_str());

    for p in vec_sql_jo {
        q = q.bind(p);
    }

    let rows = q.fetch_all(&state.db).await.unwrap();

    let json: Vec<Value> = rows
        .into_iter()
        .filter_map(|r| r.try_get::<Value, _>("j").ok())
        .collect();

    return json;
}
