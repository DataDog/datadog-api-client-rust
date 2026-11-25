// Search RUM operations returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_operations::ListRUMOperationsOptionalParams;
use datadog_api_client::datadogV2::api_rum_operations::RUMOperationsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMOperationsAPI::with_config(configuration);
    let response =
        api.list_rum_operations_with_pagination(ListRUMOperationsOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
