// Remove a user from a role returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_roles::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "role" in the system
    let role_data_id = std::env::var("ROLE_DATA_ID").unwrap();

    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body = RelationshipToUser::new(RelationshipToUserData::new(
        user_data_id.clone(),
        UsersType::USERS,
    ));
    let configuration = Configuration::new();
    let api = RolesAPI::with_config(configuration);
    let resp = api.remove_user_from_role(role_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
