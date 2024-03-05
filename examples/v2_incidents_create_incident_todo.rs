// Create an incident todo returns "CREATED" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();
    let body =
        IncidentTodoCreateRequest::new(
            IncidentTodoCreateData::new(
                IncidentTodoAttributes::new(
                    vec![IncidentTodoAssignee::IncidentTodoAssigneeHandle("@test.user@test.com".to_string())],
                    "Restore lost data.".to_string(),
                ),
                IncidentTodoType::INCIDENT_TODOS,
            ),
        );
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentTodo", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_todo(incident_data_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
