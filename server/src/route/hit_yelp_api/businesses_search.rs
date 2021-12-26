use crate::domain::entity::yelp_businesses_search_api::YelpBusinessSearchResult;
use crate::driver::hit_yelp_api::YelpApiDriver;
use crate::hit_api_utils::error::YelpAPIAccessError;
use crate::usecase::hit_yelp_api::hit_business_search_api::{self, RequestParams};
use axum::{extract::Query, response};
use std::collections::HashMap;

pub async fn businesses_search_handler(
    Query(params): Query<HashMap<String, String>>,
    // ) -> Result<response::Json<YelpBusinessSearchResult>, YelpAPIAccessError> {
) -> response::Json<YelpBusinessSearchResult> {
    let yelp_api_driver = YelpApiDriver::new();
    let item = RequestParams {
        term: None,
        location: None,
        latitude: match params.get(&"latitude".to_string()) {
            Some(s) => Some(s.to_string()),
            _ => None,
        },
        longitude: match params.get(&"longitude".to_string()) {
            Some(s) => Some(s.to_string()),
            _ => None,
        },
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
    // TODO error handling
    let res = hit_business_search_api::execute(yelp_api_driver, item)
        .await
        .unwrap();
    response::Json(res)
}
