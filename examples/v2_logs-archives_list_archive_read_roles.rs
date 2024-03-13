// List read roles for an archive returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs_archives::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = LogsArchivesAPI::with_config(configuration);
    let resp = api.list_archive_read_roles("archive_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}