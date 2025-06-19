use std::env;

use actix_cors::Cors;
use actix_web::{middleware::Logger, *};
use db::Db;

use actix_web::HttpResponse;

use dev::WebService;
use env_logger::Env;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
#[derive(Deserialize,FromRow,Debug,Default)]
pub struct OrginalUrl{
   // #[serde(rename = "ORIGINAL")]
    original:String,
  
}
#[derive(Serialize)]
struct ShortenedUrl{
    shortened:String
}

#[post("/shorten")]
pub async fn shorten_url(db:web::Data<Db>,req:web::Json<OrginalUrl>)->impl Responder{
    match db.insert_url(req.original.clone()).await {
        Ok(shortened)=>HttpResponse::Ok().json(ShortenedUrl{shortened:shortened}),
        Err(_)=>HttpResponse::InternalServerError().body("Error saving url")  
    }
}
#[get("/{shortened}")]
pub async fn get_full_url(db:web::Data<Db>,req:HttpRequest)->impl Responder{
    let shortened=req.match_info().get("shortened").unwrap_or_default();
    match db.get_orginal(String::from(shortened)).await {
        Ok(orginal)=>HttpResponse::Found().insert_header(("location",orginal)).finish(),
        Err(_)=>HttpResponse::NotFound().body("url not found")   
    }
}
pub fn config(conf:&mut web::ServiceConfig){
    conf.service(get_full_url);
    conf.service(shorten_url);
}



mod db;
#[actix_web::main]
async fn main() ->std::io::Result<()>{
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init_from_env(Env::default().default_filter_or("mehhh"));
    let db=Db::new().await.unwrap();
    HttpServer::new(move ||{
        let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);
        App::new()
        .wrap(cors)
        .wrap(Logger::default()) 
        .app_data(web::Data::new(db.clone()))
       
            .configure(config)
    }).bind(format!("0.0.0.0:{}",port))?
    .run()
    .await
}


