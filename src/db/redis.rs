use std::sync::Arc;
use redis::{AsyncCommands, Connection};
use log::{self, warn, info};
use lazy_static::lazy_static;

lazy_static! {
    static ref CLIENT: Arc<redis::Client> = Arc::new(redis::Client::open("redis://127.0.0.1/").unwrap());
}


pub async fn get_from(key: &str) -> Result<i64, redis::RedisError> {
    let mut con = CLIENT.get_async_connection().await?;
    let result: i64 = redis::cmd("GET").arg(key).query_async(&mut con).await?;    
    Ok(result)
}


pub async fn write_to<T:redis::ToRedisArgs + Send + Sync>(key: &str, value: T){
    let mut con:Connection;
    match CLIENT.get_async_connection().await{
        Ok(mut con) =>{
            match con.set(key, value).await as redis::RedisResult<i64> {
                Ok(_) => info!("Success to set"),
                Err(reason) => warn!("Failed to write, reason {reason}"),
            }
        },
        Err(reason) =>{
            warn!("failed to get connection, {reason}");
            ()
        }
    }
}