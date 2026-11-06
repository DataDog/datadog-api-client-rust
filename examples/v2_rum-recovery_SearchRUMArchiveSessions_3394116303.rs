// Search archived sessions returns canonical data and first-page metadata
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_recovery::RUMRecoveryAPI;
use datadog_api_client::datadogV2::model::RumArchiveSessionsSearchAttributes;
use datadog_api_client::datadogV2::model::RumArchiveSessionsSearchData;
use datadog_api_client::datadogV2::model::RumArchiveSessionsSearchRequest;

#[tokio::main]
async fn main() {
    let body = RumArchiveSessionsSearchRequest::new(
        RumArchiveSessionsSearchData::new(
            RumArchiveSessionsSearchAttributes::new(
                1636024271000,
                "service:rum18600-fixture".to_string(),
                1636629071000,
            )
            .limit(1),
        )
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
