// Update global incident handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateGlobalIncidentHandleOptionalParams;
use datadog_api_client::datadogV2::model::IncidentHandleAttributesFields;
use datadog_api_client::datadogV2::model::IncidentHandleAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentHandleDataRequest;
use datadog_api_client::datadogV2::model::IncidentHandleRelationship;
use datadog_api_client::datadogV2::model::IncidentHandleRelationshipData;
use datadog_api_client::datadogV2::model::IncidentHandleRelationshipsRequest;
use datadog_api_client::datadogV2::model::IncidentHandleRequest;
use datadog_api_client::datadogV2::model::IncidentHandleType;

#[tokio::main]
async fn main() {
    let body = IncidentHandleRequest::new(
        IncidentHandleDataRequest::new(
            IncidentHandleAttributesRequest::new("@incident-sev-1".to_string())
                .fields(IncidentHandleAttributesFields::new().severity(vec!["SEV-1".to_string()])),
            IncidentHandleType::INCIDENTS_HANDLES,
        )
        .id("b2494081-cdf0-4205-b366-4e1dd4fdf0bf".to_string())
        .relationships(Some(
            IncidentHandleRelationshipsRequest::new(IncidentHandleRelationship::new(
                IncidentHandleRelationshipData::new(
                    "f7b538b1-ed7c-4e84-82de-fdf84a539d40".to_string(),
                    "incident_types".to_string(),
                ),
            ))
            .commander_user(IncidentHandleRelationship::new(
                IncidentHandleRelationshipData::new(
                    "f7b538b1-ed7c-4e84-82de-fdf84a539d40".to_string(),
                    "incident_types".to_string(),
                ),
            )),
        )),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateGlobalIncidentHandle", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_global_incident_handle(body, UpdateGlobalIncidentHandleOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
