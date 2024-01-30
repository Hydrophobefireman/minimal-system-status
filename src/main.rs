pub mod common;
pub mod info;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use info::get_stats;
use lazy_static::lazy_static;
use std::{env, sync::Mutex, thread::sleep};
use sysinfo::{Networks, System};

lazy_static! {
    static ref SYS: Mutex<System> = Mutex::new(System::new_all());
    static ref NETWORKS: Mutex<Networks> = Mutex::new(Networks::new_with_refreshed_list());
}

#[get("/sys")]
async fn get_sysinfo() -> impl Responder {
    let res = get_stats();
    let res = serde_json::to_string_pretty(&res).unwrap();
    HttpResponse::Ok()
        .append_header(("content-type", "application/json"))
        .body(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SYS.lock().unwrap().refresh_all();
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    SYS.lock().unwrap().refresh_all();
    NETWORKS.lock().unwrap().refresh_list();
    let port = env::var("PORT")
        .unwrap_or("8080".into())
        .parse::<u16>()
        .unwrap();
    HttpServer::new(|| App::new().service(get_sysinfo))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
