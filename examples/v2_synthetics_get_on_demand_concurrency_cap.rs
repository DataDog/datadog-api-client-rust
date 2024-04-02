// Get the on-demand concurrency cap returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.get_on_demand_concurrency_cap().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
