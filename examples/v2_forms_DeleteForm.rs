// Delete a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "form" in the system
    let form_data_id =
        uuid::Uuid::parse_str(&std::env::var("FORM_DATA_ID").unwrap()).expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api.delete_form(form_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
