use actix_web::*;
use db::Db;

use actix_web::HttpResponse;

use dev::WebService;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct OrginalUrl{
    original:String
}
#[derive(Serialize)]
struct ShortenedUrl{
    shortened:String
}

#[post("/shorten")]
pub async fn shorten_url(db:web::Data<Db>,req:web::Json<OrginalUrl>)->impl Responder{
    match db.insert_url(req.original.clone()).await {
        Ok(shortened)=>HttpResponse::Ok().json(ShortenedUrl{shortened}),
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
    let db=Db::new().await.unwrap();
    HttpServer::new(move ||{
        
        App::new()
        .app_data(web::Data::new(db.clone()))
       
            .configure(config)
    }).bind("127.0.0.1:8080")?
    .run()
    .await
   
}

