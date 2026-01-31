use crate::AppState;
use crate::comm::cpu_temp::get_cpu_temperature;
use crate::comm::db::select_json;
use askama::Template;
use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use serde_json::Value;
use serde_json::json;
use sysinfo::{Disks, System};
use tower_sessions::Session;

#[derive(Template)]
#[template(path = "report_th_1.html")]
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

pub async fn get_report_th_1(session: Session) -> Response {
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

pub async fn post_report_th_1(
    Path(id): Path<String>,
    session: Session,
    State(state): State<AppState>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let logged_in = session.get::<String>("user_id").await.unwrap().is_some();
    if !logged_in {
        return Redirect::to("/").into_response();
    }

    match id.as_str() {
        "post_chart_1" => {
            let sql_jo = match v["주야"].as_str().unwrap_or("???") {
                "ju" => " and ts::time >= '08:00' and ts::time < '19:00' ",
                "ya" => " and (ts::time >= '19:00' or ts::time < '08:00') ",
                _ => "",
            };
            let sql_head = match v["단계"].as_str().unwrap_or("???") {
                "m1" => "1 minutes",
                "m10" => "10 minutes",
                "h1" => "1 hour",
                "d1" => "1 day",
                _ => "???",
            };

            let sql_text = format!(
                r#"
                select	to_char(ts, 'MM-DD') AS 일자,
                        time_bucket('{}', ts) AS 시간대,
                        ROUND(avg((데이터->>'온도')::numeric), 2) as 온도,
                        ROUND(avg((데이터->>'습도')::numeric), 2) as 습도,
                        ROUND(avg((데이터->>'조도')::numeric), 2) as 조도
                from    센서데이터
                where   일자 between $1::date and $2::date
                {}
                group by to_char(ts, 'MM-DD'), 시간대
                order by 일자, 시간대
                "#,
                sql_head, sql_jo,
            );

            let vec_sql_jo = vec![
                v["시작일"].as_str().unwrap_or("???"),
                v["종료일"].as_str().unwrap_or("???"),
            ];

            let json: Vec<Value> = select_json(State(state), sql_text.as_str(), vec_sql_jo).await;
            Json(json).into_response()
        }
        "post_chart_2" => {
            let mut sys = System::new_all();
            sys.refresh_all();

            // ---------------- CPU ----------------
            let cpu_list: Vec<Value> = sys
                .cpus()
                .iter()
                .enumerate()
                .map(|(i, cpu)| {
                    json!({
                        "core": i,
                        "usage": cpu.cpu_usage()
                    })
                })
                .collect();

            let cpu_global = sys.global_cpu_usage();

            // ---------------- Memory ----------------
            let mem_total = sys.total_memory() / 1024; // MB
            let mem_used = sys.used_memory() / 1024; // MB

            // ---------------- Disk ----------------
            let disks = Disks::new_with_refreshed_list();

            let disk_list: Vec<Value> = disks
                .iter()
                .map(|d| {
                    json!({
                        "mount": d.mount_point().to_string_lossy(),
                        "avail_gb": d.available_space() / 1024 / 1024 / 1024,
                        "total_gb": d.total_space() / 1024 / 1024 / 1024
                    })
                })
                .collect();

            // ---------------- Temperature ----------------
            let temp_val = match get_cpu_temperature() {
                Ok(t) => json!(t),
                Err(_) => Value::Null,
            };

            // ---------------- Final JSON ----------------
            let status_json = json!({
                "cpu": {
                    "global_usage": cpu_global,
                    "cores": cpu_list
                },
                "memory": {
                    "total_mb": mem_total,
                    "used_mb": mem_used
                },
                "disks": disk_list,
                "temperature": temp_val
            });

            println!("{:#?}", status_json);

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
