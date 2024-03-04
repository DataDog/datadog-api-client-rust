// Create an incident returns "CREATED" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body =
        IncidentCreateRequest::new(
            IncidentCreateData::new(
                IncidentCreateAttributes::new(
                    false,
                    "Example-Incident".to_string(),
                ).fields(std::collections::BTreeMap::from([])),
                IncidentType::INCIDENTS,
            ).relationships(
                IncidentCreateRelationships::new(
                    Some(
                        NullableRelationshipToUser::new(
                            Some(NullableRelationshipToUserData::new(user_data_id, UsersType::USERS)),
                        ),
                    ),
                ),
            ),
        );
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncident", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
