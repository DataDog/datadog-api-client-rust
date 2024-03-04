// Search tests events returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_tests::CIVisibilityTestsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        CIAppTestEventsRequest::new()
            .filter(
                CIAppTestsQueryFilter::new()
                    .from("now-15m".to_string())
                    .query("@test.service:web-ui-tests AND @test.status:skip".to_string())
                    .to("now".to_string()),
            )
            .options(CIAppQueryOptions::new().timezone("GMT".to_string()))
            .page(CIAppQueryPageOptions::new().limit(25))
            .sort(CIAppSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = CIVisibilityTestsAPI::with_config(configuration);
    let resp = api.search_ci_app_test_events(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
