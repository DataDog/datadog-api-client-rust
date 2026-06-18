// Get an AWS WIF persona mapping returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_awswif::AWSWIFAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = AWSWIFAPI::with_config(configuration);
    let resp = api
        .get_aws_wif_persona_mapping(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
