// List archive configurations returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_managed_archives::ListRUMArchiveConfigsOptionalParams;
use datadog_api_client::datadogV2::api_rum_managed_archives::RUMManagedArchivesAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMManagedArchivesAPI::with_config(configuration);
    let response = api
        .list_rum_archive_configs_with_pagination(ListRUMArchiveConfigsOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
