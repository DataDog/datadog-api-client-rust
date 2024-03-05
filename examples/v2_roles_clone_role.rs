// Create a new role by cloning an existing role returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_roles::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "role" in the system
    let role_data_id = std::env::var("ROLE_DATA_ID").unwrap();
    let body =
        RoleCloneRequest::new(
            RoleClone::new(RoleCloneAttributes::new("Example-Role clone".to_string()), RolesType::ROLES),
        );
    let configuration = Configuration::new();
    let api = RolesAPI::with_config(configuration);
    let resp = api.clone_role(role_data_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
