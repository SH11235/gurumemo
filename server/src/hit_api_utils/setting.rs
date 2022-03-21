use dotenv::dotenv;
use std::env;

pub fn yelp_api_base_url() -> String {
    "https://api.yelp.com/v3".to_string()
}

pub fn yelp_businesses_search_path() -> String {
    "/businesses/search".to_string()
}

pub fn yelp_api_key() -> String {
    dotenv().ok();
    env::var("YELP_API_KEY").expect("YELP_API_KEY is not found in .env.")
}

pub const LIMIT_BUSINESS_SEARCH_RESULTS_NUM: i32 = 50;
pub const MAX_RETRY_NUM: i32 = 20;
pub const MAX_TOTAL_NUM: i32 = LIMIT_BUSINESS_SEARCH_RESULTS_NUM * MAX_RETRY_NUM;
