// Link incident to case returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::IncidentRelationshipData;
use datadog_api_client::datadogV2::model::IncidentResourceType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentRequest;

#[tokio::main]
async fn main() {
    let body = RelationshipToIncidentRequest::new(IncidentRelationshipData::new(
        "00000000-0000-0000-0000-000000000000".to_string(),
        IncidentResourceType::INCIDENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.LinkIncident", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.link_incident("case_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
