// Get a list of spans returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_spans::ListSpansGetOptionalParams;
use datadog_api_client::datadogV2::api::api_spans::SpansAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SpansAPI::with_config(configuration);
    let resp = api
        .list_spans_get(ListSpansGetOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
