// List degradation templates returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::ListDegradationTemplatesOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .list_degradation_templates(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            ListDegradationTemplatesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
