pub mod common;
pub mod info;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use info::get_stats;
use lazy_static::lazy_static;
use std::{env, sync::Mutex, thread::sleep};
use sysinfo::{Networks, System};

lazy_static! {
    static ref SYS: Mutex<System> = Mutex::new(System::new_all());
    static ref NETWORKS: Mutex<Networks> = Mutex::new(Networks::new_with_refreshed_list());
}

#[get("/")]
async fn get_sysinfo() -> impl Responder {
    let res = get_stats();
    let res = serde_json::to_string_pretty(&res).unwrap();
    HttpResponse::Ok()
        .append_header(("content-type", "application/json"))
        .append_header(("access-control-allow-origin", "*"))
        .body(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SYS.lock().unwrap().refresh_all();
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    SYS.lock().unwrap().refresh_all();
    NETWORKS.lock().unwrap().refresh_list();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let port = env::var("PORT")
        .unwrap_or("8080".into())
        .parse::<u16>()
        .unwrap();
    HttpServer::new(|| App::new().wrap(Logger::default()).service(get_sysinfo))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
