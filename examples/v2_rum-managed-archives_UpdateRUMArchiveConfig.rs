// Create or update an archive configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_managed_archives::RUMManagedArchivesAPI;
use datadog_api_client::datadogV2::model::RumArchiveConfigType;
use datadog_api_client::datadogV2::model::RumArchiveConfigUpsertAttributes;
use datadog_api_client::datadogV2::model::RumArchiveConfigUpsertData;
use datadog_api_client::datadogV2::model::RumArchiveConfigUpsertRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = RumArchiveConfigUpsertRequest::new(RumArchiveConfigUpsertData::new(
        RumArchiveConfigUpsertAttributes::new(true).retention_days(90),
        "a33671aa-24fd-4dcd-ba4b-5bbdbafe7690".to_string(),
        RumArchiveConfigType::RUM_ARCHIVE_CONFIG,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMManagedArchivesAPI::with_config(configuration);
    let resp = api
        .update_rum_archive_config(
            Uuid::parse_str("a33671aa-24fd-4dcd-ba4b-5bbdbafe7690").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
