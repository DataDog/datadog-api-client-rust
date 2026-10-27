// Rehydrate an Archive Search returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_archive_searches::LogsArchiveSearchesAPI;
use datadog_api_client::datadogV2::model::ArchiveSearchRehydrateRequest;
use datadog_api_client::datadogV2::model::ArchiveSearchRehydrateRequestAttributes;
use datadog_api_client::datadogV2::model::ArchiveSearchRehydrateRequestData;
use datadog_api_client::datadogV2::model::ArchiveSearchRehydrateRequestRehydration;
use datadog_api_client::datadogV2::model::ArchiveSearchRehydrationTier;
use datadog_api_client::datadogV2::model::ArchiveSearchType;

#[tokio::main]
async fn main() {
    let body = ArchiveSearchRehydrateRequest::new(ArchiveSearchRehydrateRequestData::new(
        ArchiveSearchRehydrateRequestAttributes::new(
            ArchiveSearchRehydrateRequestRehydration::new(
                15,
                ArchiveSearchRehydrationTier::STANDARD,
            ),
        ),
        ArchiveSearchType::ARCHIVE_SEARCH,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.rehydrate_archive_search", true);
    let api = LogsArchiveSearchesAPI::with_config(configuration);
    let resp = api
        .rehydrate_archive_search("archive_search_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
