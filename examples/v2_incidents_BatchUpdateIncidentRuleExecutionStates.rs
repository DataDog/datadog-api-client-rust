// Batch update incident rule execution states returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentBatchUpdateRuleExecutionStatesData;
use datadog_api_client::datadogV2::model::IncidentBatchUpdateRuleExecutionStatesDataAttributes;
use datadog_api_client::datadogV2::model::IncidentBatchUpdateRuleExecutionStatesRequest;
use datadog_api_client::datadogV2::model::IncidentRuleExecutionStateRule;
use datadog_api_client::datadogV2::model::IncidentRuleExecutionStateType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentBatchUpdateRuleExecutionStatesRequest::new(
        IncidentBatchUpdateRuleExecutionStatesData::new(
            IncidentBatchUpdateRuleExecutionStatesDataAttributes::new(vec![
                IncidentRuleExecutionStateRule::new(
                    Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
                )
                .last_executed_at(Some(
                    DateTime::parse_from_rfc3339("2024-01-01T00:00:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                )),
            ]),
            IncidentRuleExecutionStateType::INCIDENT_RULE_EXECUTION_STATES,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.BatchUpdateIncidentRuleExecutionStates", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .batch_update_incident_rule_execution_states("incident_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
