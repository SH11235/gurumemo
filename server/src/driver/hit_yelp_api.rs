use crate::domain::entity::yelp_businesses_search_api::{Center, Region, YelpBusinessSearchResult};
use crate::hit_api_utils::error::{UseCase, YelpAPIAccessError};
use crate::hit_api_utils::setting::{
    yelp_api_base_url, yelp_api_key, yelp_businesses_search_path, LIMIT_BUSINESS_SEARCH_RESULTS_NUM,
};
use crate::usecase::hit_yelp_api::hit_business_search_api;
use async_trait::async_trait;

pub struct YelpApiDriver {}

impl YelpApiDriver {
    pub fn new() -> YelpApiDriver {
        YelpApiDriver {}
    }
}

impl Default for YelpApiDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl UseCase for YelpApiDriver {}

#[async_trait]
impl hit_business_search_api::HitBusinessSearchAPIUseCase for YelpApiDriver {
    async fn hit_business_search_api(
        &self,
        params: hit_business_search_api::RequestParams,
    ) -> Result<YelpBusinessSearchResult, YelpAPIAccessError> {
        let endpoint = format!("{}{}", yelp_api_base_url(), yelp_businesses_search_path());
        let bearer_token = format!("Bearer {}", yelp_api_key());
        let mut api_result = YelpBusinessSearchResult {
            total: 0,
            businesses: vec![],
            region: Region {
                center: Center {
                    latitude: 0.0,
                    longitude: 0.0,
                },
            },
        };
        let param_limit = params.limit;
        let mut p = params.clone();
        p.limit = match param_limit {
            Some(lim) => {
                // Yelp APIへ1回のリクエストのlimitの最大値がLIMIT_BUSINESS_SEARCH_RESULTS_NUMなので、これを超えないようにする
                if lim > LIMIT_BUSINESS_SEARCH_RESULTS_NUM {
                    Some(LIMIT_BUSINESS_SEARCH_RESULTS_NUM)
                } else {
                    Some(lim)
                }
            },
            None => None
        };
        let res = reqwest::Client::new()
            .get(&endpoint)
            .query(&p)
            .header("authorization", &bearer_token)
            .send()
            .await
            .map_err(|e| YelpAPIAccessError::InternalErrorWithMessage(e.to_string()))?;
        let res_json: YelpBusinessSearchResult = res
            .json()
            .await
            .map_err(|e| YelpAPIAccessError::InternalErrorWithMessage(e.to_string()))?;
        api_result.total = res_json.total;
        let limit_num_for_request_num = if api_result.total > params.limit.unwrap_or(LIMIT_BUSINESS_SEARCH_RESULTS_NUM) {
            params.limit.unwrap_or(LIMIT_BUSINESS_SEARCH_RESULTS_NUM)
        } else {
            api_result.total
        };
        let request_num = hit_business_search_api::hit_business_seatch_api_num(limit_num_for_request_num);
        println!("request_num is {}", request_num);
        api_result.region = res_json.region;

        let business_info_vec = res_json.businesses;
        api_result.businesses.extend(business_info_vec);
        for idx in 1..request_num {
            let offset = Some(idx * LIMIT_BUSINESS_SEARCH_RESULTS_NUM);
            let limit = Some(LIMIT_BUSINESS_SEARCH_RESULTS_NUM);
            let mut p = params.clone();
            p.offset = offset;
            p.limit = limit; 
            let res = reqwest::Client::new()
                .get(&endpoint)
                .query(&p)
                .header("authorization", &bearer_token)
                .send()
                .await
                .map_err(|e| YelpAPIAccessError::InternalErrorWithMessage(e.to_string()))?;
            let res_json: YelpBusinessSearchResult = res
                .json()
                .await
                .map_err(|e| YelpAPIAccessError::InternalErrorWithMessage(e.to_string()))?;
            api_result.total = res_json.total;
            api_result.region = res_json.region;
            let business_info_vec = res_json.businesses;
            api_result.businesses.extend(business_info_vec);
        }
        Ok(api_result)
    }
}
