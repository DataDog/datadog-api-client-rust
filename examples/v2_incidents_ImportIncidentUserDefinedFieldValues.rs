// Import values for an incident user-defined field returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::ImportIncidentUserDefinedFieldValuesOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ImportIncidentUserDefinedFieldValues", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .import_incident_user_defined_field_values(
            "00000000-0000-0000-0000-000000000000".to_string(),
            ImportIncidentUserDefinedFieldValuesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
