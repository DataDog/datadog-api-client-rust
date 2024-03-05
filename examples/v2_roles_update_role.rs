// Update a role returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_roles::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "role" in the system
    let role_data_id = std::env::var("ROLE_DATA_ID").unwrap();

    // there is a valid "permission" in the system
    let permission_id = std::env::var("PERMISSION_ID").unwrap();
    let body =
        RoleUpdateRequest::new(
            RoleUpdateData::new(
                RoleUpdateAttributes::new().name("developers-updated".to_string()),
                role_data_id,
                RolesType::ROLES,
            ).relationships(
                RoleRelationships
                ::new().permissions(
                    RelationshipToPermissions
                    ::new().data(
                        vec![
                            RelationshipToPermissionData::new().id(permission_id).type_(PermissionsType::PERMISSIONS)
                        ],
                    ),
                ),
            ),
        );
    let configuration = Configuration::new();
    let api = RolesAPI::with_config(configuration);
    let resp = api.update_role(role_data_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
