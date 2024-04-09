// Create role returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_roles::RolesAPI;
use datadog_api_client::datadogV2::model::RoleCreateAttributes;
use datadog_api_client::datadogV2::model::RoleCreateData;
use datadog_api_client::datadogV2::model::RoleCreateRequest;
use datadog_api_client::datadogV2::model::RolesType;

#[tokio::main]
async fn main() {
    let body = RoleCreateRequest::new(
        RoleCreateData::new(RoleCreateAttributes::new("Example-Role".to_string()))
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
