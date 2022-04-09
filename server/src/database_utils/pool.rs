use dotenv::dotenv;
use mongodb::{error::Result, Client};
use std::env;

pub type MongoResult<T> = Result<T>;

pub async fn mongo_db_client() -> Result<Client> {
    Client::with_uri_str(database_url()).await
}

fn database_url() -> String {
    dotenv().ok();
    env::var("MongoDBURL").expect("MongoDBURL is not found in .env.")
}
