// Get an org authorized client returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_authorized_clients::GetOrgAuthorizedClientOptionalParams;
use datadog_api_client::datadogV2::api_org_authorized_clients::OrgAuthorizedClientsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OrgAuthorizedClientsAPI::with_config(configuration);
    let resp = api
        .get_org_authorized_client(
            "00000000-0000-0000-0000-000000000001".to_string(),
            GetOrgAuthorizedClientOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
