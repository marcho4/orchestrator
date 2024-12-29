use reqwest::Client;
use redis::{Client as RedisClient, Connection};
pub struct Orchestrator {
    pub client: Client,
    pub redis_client: Connection,
}
impl Orchestrator {
    pub fn new() -> Orchestrator {
        let redis = RedisClient::open("redis://redis:6379").unwrap();
        let con = redis.get_connection().unwrap();

        Orchestrator {
            client: Client::new(),
            redis_client: con,
        }
    }
}