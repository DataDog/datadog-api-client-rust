// Create role with a permission returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_roles::RolesAPI;
use datadog_api_client::datadogV2::model::PermissionsType;
use datadog_api_client::datadogV2::model::RelationshipToPermissionData;
use datadog_api_client::datadogV2::model::RelationshipToPermissions;
use datadog_api_client::datadogV2::model::RoleCreateAttributes;
use datadog_api_client::datadogV2::model::RoleCreateData;
use datadog_api_client::datadogV2::model::RoleCreateRequest;
use datadog_api_client::datadogV2::model::RoleRelationships;
use datadog_api_client::datadogV2::model::RolesType;

#[tokio::main]
async fn main() {
    let body = RoleCreateRequest::new(
        RoleCreateData::new(RoleCreateAttributes::new("Example-Role".to_string()))
            .relationships(RoleRelationships::new().permissions(
                RelationshipToPermissions::new().data(vec![
                                RelationshipToPermissionData::new()
                                    .id("d90f6831-d3d8-11e9-a77a-4fd230ddbc6a".to_string())
                                    .type_(PermissionsType::PERMISSIONS)
                            ]),
            ))
            .type_(RolesType::ROLES),
    );
    let configuration = datadog::Configuration::new();
    let api = RolesAPI::with_config(configuration);
    let resp = api.create_role(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
