// List org authorized clients returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_authorized_clients::ListOrgAuthorizedClientsOptionalParams;
use datadog_api_client::datadogV2::api_org_authorized_clients::OrgAuthorizedClientsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OrgAuthorizedClientsAPI::with_config(configuration);
    let response = api.list_org_authorized_clients_with_pagination(
        ListOrgAuthorizedClientsOptionalParams::default(),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
