// List Org Connections returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_connections::ListOrgConnectionsOptionalParams;
use datadog_api_client::datadogV2::api_org_connections::OrgConnectionsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OrgConnectionsAPI::with_config(configuration);
    let resp = api
        .list_org_connections(ListOrgConnectionsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
