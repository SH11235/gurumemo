use crate::domain::entity::yelp_businesses_search_api::YelpBusinessSearchResult;
use crate::driver::hit_yelp_api::YelpApiDriver;
use crate::hit_api_utils::error::YelpAPIAccessError;
use crate::usecase::hit_yelp_api::hit_business_search_api::{self, RequestParams};
use axum::{extract::Query, response};

pub async fn businesses_search_handler(
    Query(params): Query<RequestParams>,
) -> response::Json<YelpBusinessSearchResult> {
    let yelp_api_driver = YelpApiDriver::new();
    let res = hit_business_search_api::execute(yelp_api_driver, params)
        .await
        .map_err(|e| YelpAPIAccessError::InternalErrorWithMessage(e.to_string()))
        .unwrap();
    response::Json(res)
}
