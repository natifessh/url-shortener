use sqlx::sqlite::SqlitePool;
use sqlx::*;
use uuid::Uuid;
#[derive(Debug,Clone)]
pub struct Db{
    con:SqlitePool
}
impl Db {
   pub  async fn new()->Result<Self,sqlx::Error>{
        let conn=sqlite::SqlitePool::connect("sqlite://url.db").await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS URLS(ID INTEGER PRIMARY KEY,SHORTENED TEXT NOT NULL UNIQUE,ORGINAL TEXT NOT NULL)",).execute(&conn).await?;

        Ok(Db { con: conn })
        
    }
   pub  async fn insert_url(&self,  orginal:String)->Result<String,sqlx::Error> {
    let shortened=Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO URLS(SHORTENED,ORGINAL) VALUES(?1,?2)").bind(&shortened).bind(orginal).execute(&self.con).await?;
    Ok(shortened)
        
    }
    pub async fn get_orginal(&self,shortened:String)->Result<String,sqlx::Error>{
        let row=sqlx::query("SELECT ORGINAL FROM URLS WHERE SHORTENED=?1").bind(shortened).fetch_one(&self.con).await?;
        let orginal:String=row.get(0);
        Ok(orginal)
    }
    
}