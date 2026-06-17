// List AWS WIF persona mappings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_awswif::AWSWIFAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = AWSWIFAPI::with_config(configuration);
    let resp = api.list_aws_wif_persona_mappings().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
