// Create an Archive Search returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_archive_searches::LogsArchiveSearchesAPI;
use datadog_api_client::datadogV2::model::ArchiveSearchCreateRehydration;
use datadog_api_client::datadogV2::model::ArchiveSearchCreateRequest;
use datadog_api_client::datadogV2::model::ArchiveSearchCreateRequestAttributes;
use datadog_api_client::datadogV2::model::ArchiveSearchCreateRequestData;
use datadog_api_client::datadogV2::model::ArchiveSearchRehydrationTier;
use datadog_api_client::datadogV2::model::ArchiveSearchType;

#[tokio::main]
async fn main() {
    let body = ArchiveSearchCreateRequest::new(ArchiveSearchCreateRequestData::new(
        ArchiveSearchCreateRequestAttributes::new(
            "mhmyYmyLTOaFYKvhNadu1w".to_string(),
            DateTime::parse_from_rfc3339("2026-01-01T00:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            "checkout-latency-investigation".to_string(),
            "service:checkout status:error".to_string(),
            DateTime::parse_from_rfc3339("2026-01-02T00:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )
        .description("Investigating the checkout latency spike.".to_string())
        .rehydration(ArchiveSearchCreateRehydration::new(
            1000000,
            15,
            ArchiveSearchRehydrationTier::STANDARD,
        )),
        ArchiveSearchType::ARCHIVE_SEARCH,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_archive_search", true);
    let api = LogsArchiveSearchesAPI::with_config(configuration);
    let resp = api.create_archive_search(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
