// Create an AuthN Mapping returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_authn_mappings::AuthNMappingsAPI;
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
        AuthNMappingCreateRequest::new(
            AuthNMappingCreateData::new(AuthNMappingsType::AUTHN_MAPPINGS)
                .attributes(
                    AuthNMappingCreateAttributes::new()
                        .attribute_key("exampleauthnmapping".to_string())
                        .attribute_value("Example-AuthN-Mapping".to_string()),
                )
                .relationships(
                    AuthNMappingCreateRelationships
                    ::new().role(
                        RelationshipToRole
                        ::new().data(RelationshipToRoleData::new().id(role_data_id).type_(RolesType::ROLES)),
                    ),
                ),
        );
    let configuration = Configuration::new();
    let api = AuthNMappingsAPI::with_config(configuration);
    let resp = api.create_authn_mapping(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
