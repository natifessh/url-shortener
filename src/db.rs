use std::env;

use base64::{engine, Engine};
use dotenv::dotenv;
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use sqlx::*;
use uuid::Uuid;

use crate::OrginalUrl;
#[derive(Debug,Clone)]
pub struct Db{
    con:Pool<Postgres>
}
impl Db {
   pub  async fn new()->Result<Self,sqlx::Error>{
        dotenv().ok();
        let db_url=env::var("PG_URL").expect("DATABASE_URL must be set");
        let conn=PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.expect("failed");
        sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id SERIAL PRIMARY KEY,
            shortened TEXT NOT NULL UNIQUE,
            original TEXT NOT NULL
        )"
    )
    .execute(&conn)
    .await?;
        Ok(Db {con:conn})
   }   
   pub  async fn insert_url(&self,  orginal:String)->Result<String,sqlx::Error> {
    let mut  rand=rand::thread_rng();
    let rand_bytes:[u8;6]=rand.gen();
    let shortened=engine::general_purpose::URL_SAFE.encode(rand_bytes);
    sqlx::query("INSERT INTO URLS(shortened,original) VALUES($1,$2)").bind(&shortened).bind(orginal).execute(&self.con).await.expect("qhattt");
    Ok(shortened)
        
    }
    pub async fn get_orginal(&self,shortened:String)->Result<String,sqlx::Error>{
        let row:OrginalUrl=sqlx::query_as::<_,OrginalUrl>("SELECT original FROM URLS WHERE SHORTENED=$1").bind(shortened).fetch_one(&self.con).await?;
        Ok(row.original)
    }
    
}