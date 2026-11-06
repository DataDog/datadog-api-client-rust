// List archive configurations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_managed_archives::ListRUMArchiveConfigsOptionalParams;
use datadog_api_client::datadogV2::api_rum_managed_archives::RUMManagedArchivesAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMManagedArchivesAPI::with_config(configuration);
    let resp = api
        .list_rum_archive_configs(
            ListRUMArchiveConfigsOptionalParams::default()
                .page_number(0)
                .page_size(1),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
