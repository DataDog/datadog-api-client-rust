// List org authorized clients returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_authorized_clients::ListOrgAuthorizedClientsOptionalParams;
use datadog_api_client::datadogV2::api_org_authorized_clients::OrgAuthorizedClientsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OrgAuthorizedClientsAPI::with_config(configuration);
    let resp = api
        .list_org_authorized_clients(ListOrgAuthorizedClientsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
