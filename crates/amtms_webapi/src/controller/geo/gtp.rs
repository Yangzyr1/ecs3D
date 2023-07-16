use actix_web::{post, web, HttpResponse, Error};
use amtms_geo::api::gtp::{set_csv_path, update_gtp};

#[post("/gtp/data/terrain")]
pub async fn post_terrain_data_file(paths: web::Json<Vec<String>>) -> Result<HttpResponse, Error> {
    if !paths.0.is_empty() {
        set_csv_path(paths.0.first().unwrap().to_string());
    }
    Ok(HttpResponse::Ok().body(paths.0.first().unwrap().to_string()))
}
#[post("/gtp/generate")]
pub async fn post_generate_gtp() -> Result<HttpResponse, Error> {
    update_gtp();
    Ok(HttpResponse::Ok().body("Updating"))
}