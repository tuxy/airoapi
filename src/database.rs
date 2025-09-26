use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use log::error;
use redis::AsyncCommands;

pub async fn init_conn(url: String) -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(url).unwrap();
    let pool = bb8::Pool::builder()
        .build(manager)
        .await
        .unwrap();

    // Check if database is online
    {
        let mut conn = pool.get().await.unwrap();
        conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
        let result: String = conn.get("foo").await.unwrap();
        if result != "bar" {
            error!("Redis check failed");
            panic!();
        }
    }
    pool
}