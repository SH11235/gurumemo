use crate::database_utils::pool;
use crate::domain::entity::yelp_business::Business;
use crate::usecase::hit_yelp_api::hit_business_search_api::RequestParams;
use axum::{extract::Query, response};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

pub async fn mongo_businesses_search_handler(
    Query(params): Query<RequestParams>,
) -> response::Json<Vec<Business>> {
    let client = pool::mongo_db_client().await.unwrap();
    let db = client.database("yelp");

    // Get a handle to a collection of `YelpBusinessSearchResult`.
    let typed_collection = db.collection::<Business>("business");

    // Query the books in the collection with a filter and an option.
    let filter = doc! {
        "longitude_latitude": {
            "$near": {
              "$geometry": {
                "type": "Point",
                "coordinates": [params.longitude.unwrap().parse::<f64>().unwrap(), params.latitude.unwrap().parse::<f64>().unwrap()]
              },
              "$maxDistance": params.radius.unwrap().parse::<i32>().unwrap(),
            //   "$minDistance": 10 //<distance in meters>
            }
          }
    };

    let find_options = FindOptions::builder().build();
    let mut cursor = typed_collection.find(filter, find_options).await.unwrap();

    let mut res: Vec<Business> = [].to_vec();
    // Iterate over the results of the cursor.
    while let Some(business) = cursor.try_next().await.unwrap() {
        println!("title: {:?}", business);
        res.push(business);
    }
    response::Json(res)
}
