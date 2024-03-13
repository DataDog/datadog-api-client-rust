// Search tests events returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_tests::*;
use datadog_api_client::datadogV2::model::*;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let body = CIAppTestEventsRequest::new()
        .filter(
            CIAppTestsQueryFilter::new()
                .from("now-15m".to_string())
                .query("@test.status:pass AND -@language:python".to_string())
                .to("now".to_string()),
        )
        .page(CIAppQueryPageOptions::new().limit(2))
        .sort(CIAppSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = CIVisibilityTestsAPI::with_config(configuration);
    let response = api.search_ci_app_test_events_with_pagination(
        SearchCIAppTestEventsOptionalParams::default().body(body),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}