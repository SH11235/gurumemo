extern crate server;
use dotenv::dotenv;
use mongodb::{bson::doc, error::Result, Client, IndexModel};
// use serde::{Deserialize, Serialize};
use server::driver::hit_yelp_api::YelpApiDriver;
// use server::hit_api_utils::error::YelpAPIAccessError;
use server::usecase::hit_yelp_api::hit_business_search_api::{self, RequestParams};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("*** 開始 ***");
    dotenv().ok();
    let mongo_db_url = env::var("MongoDBURL").expect("MongoDBURL is not found in .env.");
    let client = Client::with_uri_str(mongo_db_url).await?;

    let coll = client.database("yelp").collection("business");

    let params = RequestParams {
        term: None,
        location: None,
        latitude: Some("35.69059985184279".to_string()),
        longitude: Some("139.70279058434141".to_string()),
        radius: None,
        categories: None,
        locale: None,
        limit: None,
        offset: None,
        sort_by: None,
        price: None,
        open_now: None,
        open_at: None,
        attributes: None,
    };

    let yelp_api_driver = YelpApiDriver::new();
    // TODO error handling
    let res = hit_business_search_api::execute(yelp_api_driver, params)
        .await
        .unwrap();

    coll.insert_many(res.businesses, None).await?;

    // struct Soooo {
    //     keys: doc,
    //     options: None,
    // };
    // coll.create_index(
    //     ,
    //     None,
    // );
    let update_result = coll
        .update_many(
            doc! { "key": None },
            doc! {
            "$set": {"population": pp_in, "date_mod": date_mod.to_string()}
            },
            None,
        )
        .await?;
    // println!("Updated {} document", update_result.modified_count);

    eprintln!("*** 終了 ***");
    Ok(())
}

// db.business.createIndex({id: 1}, { unique: true })
