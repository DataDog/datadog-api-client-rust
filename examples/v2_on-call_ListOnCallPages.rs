// List On-Call pages returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::ListOnCallPagesOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .list_on_call_pages(
            ListOnCallPagesOptionalParams::default().filter_query("key:1".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
