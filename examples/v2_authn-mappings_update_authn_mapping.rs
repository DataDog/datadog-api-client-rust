// Edit an AuthN Mapping returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_authn_mappings::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "authn_mapping" in the system
    let authn_mapping_data_id = std::env::var("AUTHN_MAPPING_DATA_ID").unwrap();

    // there is a valid "role" in the system
    let role_data_id = std::env::var("ROLE_DATA_ID").unwrap();
    let body = AuthNMappingUpdateRequest::new(
        AuthNMappingUpdateData::new(
            authn_mapping_data_id.clone(),
            AuthNMappingsType::AUTHN_MAPPINGS,
        )
        .attributes(
            AuthNMappingUpdateAttributes::new()
                .attribute_key("member-of".to_string())
                .attribute_value("Development".to_string()),
        )
        .relationships(
            AuthNMappingUpdateRelationships::new().role(
                RelationshipToRole::new().data(
                    RelationshipToRoleData::new()
                        .id(role_data_id.clone())
                        .type_(RolesType::ROLES),
                ),
            ),
        ),
    );
    let configuration = Configuration::new();
    let api = AuthNMappingsAPI::with_config(configuration);
    let resp = api
        .update_authn_mapping(authn_mapping_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}