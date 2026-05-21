// Create an incident Jira template returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentJiraTemplateDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentJiraTemplateDataRequest;
use datadog_api_client::datadogV2::model::IncidentJiraTemplateFieldConfiguration;
use datadog_api_client::datadogV2::model::IncidentJiraTemplateIncidentTypeRelationship;
use datadog_api_client::datadogV2::model::IncidentJiraTemplateIncidentTypeRelationshipData;
use datadog_api_client::datadogV2::model::IncidentJiraTemplateRelationships;
use datadog_api_client::datadogV2::model::IncidentJiraTemplateRequest;
use datadog_api_client::datadogV2::model::IncidentJiraTemplateType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentJiraTemplateRequest::new(
        IncidentJiraTemplateDataRequest::new(
            IncidentJiraTemplateDataAttributesRequest::new(
                "123456".to_string(),
                "10001".to_string(),
                "10000".to_string(),
                "INC".to_string(),
            )
            .field_configurations(vec![IncidentJiraTemplateFieldConfiguration::new(
                "summary".to_string(),
                "bidirectional".to_string(),
            )
            .incident_field(Some("title".to_string()))
            .jira_field_type(Some("string".to_string()))])
            .is_default(false)
            .name("Default Jira Template".to_string())
            .sync_enabled(true)
            .type_("jira".to_string()),
            IncidentJiraTemplateType::INCIDENTS_JIRA_TEMPLATES,
        )
        .relationships(IncidentJiraTemplateRelationships::new().incident_type(
            IncidentJiraTemplateIncidentTypeRelationship::new(
                IncidentJiraTemplateIncidentTypeRelationshipData::new(
                    Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
                    "incident_types".to_string(),
                ),
            ),
        )),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentJiraTemplate", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_jira_template(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
