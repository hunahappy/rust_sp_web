use crate::comm::cpu_temp::get_cpu_temperature;
use crate::comm::util;
use askama::Template;
use axum::Json;
use axum::extract::Path;
use axum::response::{Html, IntoResponse, Redirect, Response};
use serde_json::{Map, Value, json};
use sysinfo::{Disks, System};
use tower_sessions::Session;

#[derive(Template)]
#[template(path = "system_info.html")]
pub struct IndexTemplate {
    uid: String,
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

pub async fn get_system_info(session: Session) -> Response {
    let logged_in = session.get::<String>("user_id").await.unwrap().is_some();
    if !logged_in {
        return Redirect::to("/").into_response();
    } else {
        IndexTemplate {
            uid: session.get::<String>("user_id").await.unwrap().unwrap().to_string(),
            data: "-".to_string(),
        }
        .into_response()
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////

pub async fn post_system_info(Path(id): Path<String>, session: Session) -> impl IntoResponse {
    let logged_in = session.get::<String>("user_id").await.unwrap().is_some();
    if !logged_in {
        return Redirect::to("/").into_response();
    }

    match id.as_str() {
        "get_system_info" => {
            let mut sys = System::new_all();
            sys.refresh_all();

            let mut map = Map::new();

            for (i, cpu) in sys.cpus().iter().enumerate() {
                map.insert(
                    i.to_string(),
                    json!(((cpu.cpu_usage() * 100.0).round() / 100.0).to_string()),
                );
            }

            let cpu_list = Value::Object(map);

            let cpu_global = ((sys.global_cpu_usage() * 100.0).round() / 100.0).to_string();

            // ---------------- Memory ----------------
            let mem_total = util::format_with_commas(sys.total_memory() / 1024 / 1024); // MB            
            let mem_used = util::format_with_commas(sys.used_memory() / 1024 / 1024); // MB

            // ---------------- Disk ----------------
            let disks = Disks::new_with_refreshed_list();

            let disk_list: Vec<Value> = disks
                .iter()
                .map(|d| {
                    json!({
                        "3.mount": d.mount_point().to_string_lossy(),
                        "2.avail_mb": util::format_with_commas(d.available_space() / 1024 / 1024),
                        "1.total_mb": util::format_with_commas(d.total_space() / 1024 / 1024)
                    })
                })
                .collect();

            // ---------------- Temperature ----------------
            let temp_val = match get_cpu_temperature() {
                Ok(t) => json!(t),
                Err(_) => json!("지원안함"),
            };

            // ---------------- Final JSON ----------------
            let status_json = json!({
                "cpu": {
                    "global_usage": cpu_global,
                    "cores": cpu_list
                },
                "memory": {
                    "1.total_mb": mem_total,
                    "2.used_mb": mem_used
                },
                "disks": disk_list,
                "temperature": temp_val
            });

            // axum handler에서
            Json(status_json).into_response()
        }
        _ => {
            let s = r#"{"report_th_1":"???"}"#;
            let u: Value = serde_json::from_str(s).unwrap();
            Json(u).into_response()
        }
    }
}
