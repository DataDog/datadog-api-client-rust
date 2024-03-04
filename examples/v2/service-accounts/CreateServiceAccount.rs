// Create a service account returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_accounts::ServiceAccountsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "role" in the system
    let role_data_id = std::env::var("ROLE_DATA_ID").unwrap();
    let body =
        ServiceAccountCreateRequest::new(
            ServiceAccountCreateData::new(
                ServiceAccountCreateAttributes::new(
                    "Example-Service-Account@datadoghq.com".to_string(),
                    true,
                ).name("Test API Client".to_string()),
                UsersType::USERS,
            ).relationships(
                UserRelationships
                ::new().roles(
                    RelationshipToRoles
                    ::new().data(vec![RelationshipToRoleData::new().id(role_data_id).type_(RolesType::ROLES)]),
                ),
            ),
        );
    let configuration = Configuration::new();
    let api = ServiceAccountsAPI::with_config(configuration);
    let resp = api.create_service_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
