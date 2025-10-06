// Create a page from an incident returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentCreatePageAttributes;
use datadog_api_client::datadogV2::model::IncidentCreatePageFromIncidentData;
use datadog_api_client::datadogV2::model::IncidentCreatePageFromIncidentRequest;
use datadog_api_client::datadogV2::model::IncidentPageTarget;
use datadog_api_client::datadogV2::model::IncidentPageTargetType;
use datadog_api_client::datadogV2::model::IncidentPageType;

#[tokio::main]
async fn main() {
    let body = IncidentCreatePageFromIncidentRequest::new(IncidentCreatePageFromIncidentData::new(
        IncidentCreatePageAttributes::new(
            IncidentPageTarget::new(
                "team-handle".to_string(),
                IncidentPageTargetType::TEAM_HANDLE,
            ),
            "Incident Response Page".to_string(),
        )
        .description(Some("Page created for incident response".to_string()))
        .services(Some(vec![
            "web-service".to_string(),
            "api-service".to_string(),
        ]))
        .tags(Some(vec!["urgent".to_string(), "production".to_string()])),
        IncidentPageType::PAGE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreatePageFromIncident", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_page_from_incident("incident_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
