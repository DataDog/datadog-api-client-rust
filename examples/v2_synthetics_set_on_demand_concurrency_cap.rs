// Save new value for on-demand concurrency cap returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::OnDemandConcurrencyCapAttributes;

#[tokio::main]
async fn main() {
    let body = OnDemandConcurrencyCapAttributes::new().on_demand_concurrency_cap(20 as f64);
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.set_on_demand_concurrency_cap(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
