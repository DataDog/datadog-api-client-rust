// List role templates returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_roles::RolesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListRoleTemplates", true);
    let api = RolesAPI::with_config(configuration);
    let resp = api.list_role_templates().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
