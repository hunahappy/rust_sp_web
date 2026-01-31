use crate::AppState;
use crate::comm::db::select_json;
use askama::Template;
use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde_json::Value;
use tower_sessions::Session;

#[derive(Template)]
#[template(path = "login.html")]
pub struct IndexTemplate {
    pid: String,
    data: String,
}

// Askama 템플릿을 axum 응답으로 변환
impl IntoResponse for IndexTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
        }
    }
}

pub async fn get_login() -> IndexTemplate {
    IndexTemplate {
        pid: "login".to_string(),
        data: "-".to_string(),
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////

pub async fn post_login(
    Path(id): Path<String>,
    session: Session,
    State(state): State<AppState>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    match id.as_str() {
        "post_login" => {
            let str_query = "select count(아이디) as 인증 from 사용자 where 아이디 = $1 and 비밀번호 = $2";
            let vec_sql_jo = vec![
                v["user_id"].as_str().unwrap_or("???"),
                v["user_password"].as_str().unwrap_or("???"),
            ];

            let json: Vec<Value> = select_json(State(state), str_query, vec_sql_jo).await;

            if json[0]["인증"].as_i64().unwrap_or(0) == 1 {
                session
                    .insert("user_id", v["user_id"].as_str().unwrap_or("???"))
                    .await
                    .unwrap();
            }

            Json(json).into_response()
        }
        // "post_check" => {
        //     let logged_in = session.get::<String>("user_id").await.unwrap().is_some();
        //     let str_user_check;

        //     if logged_in {
        //         str_user_check = r#"{"check":"세션있음"}"#;
        //     } else {
        //         str_user_check = r#"{"check":"세션없음"}"#;
        //     }

        //     let v: Value = serde_json::from_str(str_user_check).unwrap();
        //     Json(v).into_response()
        // }
        // "post_clear" => {
        //     session.remove::<String>("user_id").await.unwrap();

        //     let s = r#"{"clear":"삭제"}"#;
        //     let u: Value = serde_json::from_str(s).unwrap();
        //     Json(u).into_response()
        // }
        _ => {
            let s = r#"{"인증":"???"}"#;
            let u: Value = serde_json::from_str(s).unwrap();
            Json(u).into_response()
        }
    }
}
