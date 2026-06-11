// Delete a user's authorizations for a client returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_authorized_clients::OrgAuthorizedClientsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OrgAuthorizedClientsAPI::with_config(configuration);
    let resp = api
        .delete_org_authorized_client_all_user_authorizations(
            "00000000-0000-0000-0000-000000000001".to_string(),
            "00000000-0000-9999-0000-000000000001".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
