// Search archived sessions returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_recovery::RUMRecoveryAPI;
use datadog_api_client::datadogV2::model::RUMArchiveSessionsSearchAttributes;
use datadog_api_client::datadogV2::model::RUMArchiveSessionsSearchData;
use datadog_api_client::datadogV2::model::RUMArchiveSessionsSearchRequest;

#[tokio::main]
async fn main() {
    let body = RUMArchiveSessionsSearchRequest::new(
        RUMArchiveSessionsSearchData::new(
            RUMArchiveSessionsSearchAttributes::new(
                1789905600000,
                "service:web".to_string(),
                1789992000000,
            )
            .cursor("example".to_string())
            .hide_retained(false)
            .limit(0),
        )
        .id("11111111-1111-4111-8111-111111111111".to_string())
        .type_("rum_archive_sessions_request".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = RUMRecoveryAPI::with_config(configuration);
    let resp = api.search_rum_archive_sessions(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
