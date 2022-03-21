use crate::{domain::entity::yelp_businesses_search_api::YelpBusinessSearchResult, hit_api_utils::setting::MAX_RETRY_NUM};
use crate::hit_api_utils::error::YelpAPIAccessError;
use crate::hit_api_utils::setting::LIMIT_BUSINESS_SEARCH_RESULTS_NUM;
use async_trait::async_trait;
use serde::{de, Deserialize, Serialize, Deserializer};
use std::{fmt, str::FromStr};

// reference https://www.yelp.com/developers/documentation/v3/business_search
/// [`serde_with`]: https://docs.rs/serde_with/1.11.0/serde_with/rust/string_empty_as_none/index.html
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub term: Option<String>, // Optional. Search term, for example "food" or "restaurants". The term may also be business names, such as "Starbucks". If term is not included the endpoint will default to searching across businesses from a small number of popular categories.
    pub location: Option<String>, // Required if either latitude or longitude is not provided. This string indicates the geographic area to be used when searching for businesses. Examples: "New York City", "NYC", "350 5th Ave, New York, NY 10118". Businesses returned in the response may not be strictly within the specified location.
    pub latitude: Option<String>, // Required if location is not provided. Latitude of the location you want to search nearby.
    pub longitude: Option<String>, // Required if location is not provided. Longitude of the location you want to search nearby.
    pub radius: Option<String>, // Optional. A suggested search radius in meters. This field is used as a suggestion to the search. The actual search radius may be lower than the suggested radius in dense urban areas, and higher in regions of less business density. If the specified value is too large, a AREA_TOO_LARGE error may be returned. The max value is 40000 meters (about 25 miles).
    pub categories: Option<String>, // Optional. Categories to filter the search results with. See the list of supported categories. The category filter can be a list of comma delimited categories. For example, "bars,french" will filter by Bars OR French. The category identifier should be used (for example "discgolf", not "Disc Golf").
    pub locale: Option<String>, // Optional. Specify the locale into which to localize the business information. See the list of supported locales. Defaults to en_US.
    pub limit: Option<i32>, // Optional. Number of business results to return. By default, it will return 20. Maximum is 50.
    pub offset: Option<i32>, // Optional. Offset the list of returned business results by this amount.
    pub sort_by: Option<String>, // Optional. Suggestion to the search algorithm that the results be sorted by one of the these modes: best_match, rating, review_count or distance. The default is best_match. Note that specifying the sort_by is a suggestion (not strictly enforced) to Yelp's search, which considers multiple input parameters to return the most relevant results. For example, the rating sort is not strictly sorted by the rating value, but by an adjusted rating value that takes into account the number of ratings, similar to a Bayesian average. This is to prevent skewing results to businesses with a single review.
    pub price: Option<String>, // Optional. Pricing levels to filter the search result with: 1 = $, 2 = $$, 3 = $$$, 4 = $$$$. The price filter can be a list of comma delimited pricing levels. For example, "1, 2, 3" will filter the results to show the ones that are $, $$, or $$$.
    pub open_now: Option<bool>, // Optional. Default to false. When set to true, only return the businesses open now. Notice that open_at and open_now cannot be used together.
    pub open_at: Option<i32>, // Optional. An integer represending the Unix time in the same timezone of the search location. If specified, it will return business open at the given time. Notice that open_at and open_now cannot be used together.
    pub attributes: Option<String>, // Optional. Try these additional filters to return specific search results!
                                    // ・hot_and_new - popular businesses which recently joined Yelp
                                    // ・request_a_quote - businesses which actively reply to Request a Quote inquiries
                                    // ・reservation - businesses with Yelp Reservations bookings enabled on their profile page
                                    // ・waitlist_reservation - businesses with Yelp Waitlist bookings enabled on their profile screen (iOS/Android)
                                    // ・deals - businesses offering Yelp Deals on their profile page
                                    // ・gender_neutral_restrooms - businesses which provide gender neutral restrooms
                                    // ・open_to_all - businesses which are Open To All
                                    // ・wheelchair_accessible - businesses which are Wheelchair Accessible
                                    // You can combine multiple attributes by providing a comma separated like "attribute1,attribute2".
                                    // If multiple attributes are used, only businesses that satisfy ALL attributes will be returned in search results.
                                    // For example, the attributes "hot_and_new,request_a_quote" will return businesses that are Hot and New AND offer Request a Quote.
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub fn hit_business_seatch_api_num(count: i32) -> i32 {
    let request_num: i32 = if (count % LIMIT_BUSINESS_SEARCH_RESULTS_NUM) == 0 {
        count / LIMIT_BUSINESS_SEARCH_RESULTS_NUM
    } else {
        count / LIMIT_BUSINESS_SEARCH_RESULTS_NUM + 1
    };
    if request_num > MAX_RETRY_NUM {
        MAX_RETRY_NUM
    } else {
        request_num
    }
}

#[test]
fn check_req_num() {
    assert_eq!(hit_business_seatch_api_num(200), 4);
    assert_eq!(hit_business_seatch_api_num(201), 5);
}

#[async_trait]
pub trait HitBusinessSearchAPIUseCase {
    async fn hit_business_search_api(
        &self,
        params: RequestParams,
    ) -> Result<YelpBusinessSearchResult, YelpAPIAccessError>;
}

pub async fn execute<T>(
    api_access: T,
    params: RequestParams,
) -> Result<YelpBusinessSearchResult, YelpAPIAccessError>
where
    T: HitBusinessSearchAPIUseCase,
{
    api_access.hit_business_search_api(params).await
}
