// Search error tracking issues returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_error_tracking::ErrorTrackingAPI;
use datadog_api_client::datadogV2::api_error_tracking::SearchIssuesOptionalParams;
use datadog_api_client::datadogV2::model::IssuesSearchRequest;
use datadog_api_client::datadogV2::model::IssuesSearchRequestData;
use datadog_api_client::datadogV2::model::IssuesSearchRequestDataAttributes;
use datadog_api_client::datadogV2::model::IssuesSearchRequestDataAttributesTrack;
use datadog_api_client::datadogV2::model::IssuesSearchRequestDataType;

#[tokio::main]
async fn main() {
    let body = IssuesSearchRequest::new(IssuesSearchRequestData::new(
        IssuesSearchRequestDataAttributes::new(
            1671612804000,
            "service:orders-* AND @language:go".to_string(),
            1671620004000,
        )
        .track(IssuesSearchRequestDataAttributesTrack::TRACE),
        IssuesSearchRequestDataType::SEARCH_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = ErrorTrackingAPI::with_config(configuration);
    let resp = api
        .search_issues(body, SearchIssuesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
