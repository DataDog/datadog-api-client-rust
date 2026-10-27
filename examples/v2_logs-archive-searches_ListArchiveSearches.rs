// List Archive Searches returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_archive_searches::LogsArchiveSearchesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_archive_searches", true);
    let api = LogsArchiveSearchesAPI::with_config(configuration);
    let resp = api.list_archive_searches().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
