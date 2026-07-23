// Revert a form to a prior version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.RevertFormVersion", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .revert_form_version(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            9223372036854775807,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
