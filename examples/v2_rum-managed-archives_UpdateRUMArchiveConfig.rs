// Create or update an archive configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_managed_archives::RUMManagedArchivesAPI;
use datadog_api_client::datadogV2::model::RumArchiveConfigType;
use datadog_api_client::datadogV2::model::RumArchiveConfigUpsertAttributes;
use datadog_api_client::datadogV2::model::RumArchiveConfigUpsertData;
use datadog_api_client::datadogV2::model::RumArchiveConfigUpsertRequest;

#[tokio::main]
async fn main() {
    // there is a valid "rum_application" in the system
    let rum_application_data_id = std::env::var("RUM_APPLICATION_DATA_ID").unwrap();
    let body = RumArchiveConfigUpsertRequest::new(RumArchiveConfigUpsertData::new(
        RumArchiveConfigUpsertAttributes::new(true).retention_days(90),
        rum_application_data_id.clone(),
        RumArchiveConfigType::RUM_ARCHIVE_CONFIG,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMManagedArchivesAPI::with_config(configuration);
    let resp = api
        .update_rum_archive_config(rum_application_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
