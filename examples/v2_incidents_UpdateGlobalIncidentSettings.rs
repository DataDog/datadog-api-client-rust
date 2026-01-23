// Update global incident settings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::GlobalIncidentSettingsAttributesRequest;
use datadog_api_client::datadogV2::model::GlobalIncidentSettingsDataRequest;
use datadog_api_client::datadogV2::model::GlobalIncidentSettingsRequest;
use datadog_api_client::datadogV2::model::GlobalIncidentSettingsType;

#[tokio::main]
async fn main() {
    let body = GlobalIncidentSettingsRequest::new(
        GlobalIncidentSettingsDataRequest::new(
            GlobalIncidentSettingsType::INCIDENTS_GLOBAL_SETTINGS,
        )
        .attributes(
            GlobalIncidentSettingsAttributesRequest::new()
                .analytics_dashboard_id("abc-123-def".to_string()),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateGlobalIncidentSettings", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.update_global_incident_settings(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
