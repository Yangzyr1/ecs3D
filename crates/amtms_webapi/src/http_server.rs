use std::sync::mpsc;
use std::thread;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::DefaultHeaders;

use bevy::prelude::Plugin;
pub struct WebApiPlugin;
use clap::{Parser};
use crate::controller::geo::gtp::{post_generate_gtp, post_terrain_data_file};
use crate::controller::tool::status::post_change_tool;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct HttpServerCliArgs {
    #[arg(long = "http")]
    /// HTTP服务器监听地址
    http_bind_address: Option<String>,
}

impl Plugin for WebApiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(start_http_server_system);
    }
}
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}
#[actix_web::main]
async fn main(cli: HttpServerCliArgs) -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .route("/", web::get().to(index))
            .route("/user", web::post().to(index))
            .service(post_generate_gtp)
            .service(post_terrain_data_file)
            .service(post_change_tool)
            .wrap(cors)
    })
        .bind((cli.http_bind_address.unwrap()))?
        .run()
        .await
}

fn async_start_server(cli_args: HttpServerCliArgs)  {
    thread::spawn(||{
        main(cli_args)
    });
}
fn get_cli_args() -> HttpServerCliArgs {
    HttpServerCliArgs::parse()
}
pub fn start_http_server_system(){
    let mut cli = get_cli_args();
    if cli.http_bind_address.is_none() {
        cli.http_bind_address.insert(String::from("127.0.0.1:7443"));
    }
    async_start_server(cli);
}
