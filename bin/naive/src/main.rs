use dotenv::dotenv;
use ethers::providers::{Http, Middleware, Provider};
use sqlx::{Pool, Postgres};
use std::env;

mod storage;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    println!("Hello, world!");

    let http_rpc_url = env::var("RPC_URL").expect("RPC_URL env variable should be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable should be set");

    let provider =
        Provider::<Http>::try_from(http_rpc_url).expect("RPC_URL should be an http provider url");

    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");
}
