// Create role returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_roles::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = RoleCreateRequest::new(
        RoleCreateData::new(RoleCreateAttributes::new("Example-Role".to_string()))
            .type_(RolesType::ROLES),
    );
    let configuration = Configuration::new();
    let api = RolesAPI::with_config(configuration);
    let resp = api.create_role(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
