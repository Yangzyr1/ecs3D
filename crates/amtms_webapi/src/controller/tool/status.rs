use actix_web::{post, web, HttpResponse, Error};
use actix_web::web::Json;
use amtms_tool::api::status::change_tool_mode;

#[post("/tool/generate")]
pub async fn post_change_tool(mode: Json<String>) -> Result<HttpResponse, Error> {
    change_tool_mode(mode.0);
    Ok(HttpResponse::Ok().body("Updating"))
}