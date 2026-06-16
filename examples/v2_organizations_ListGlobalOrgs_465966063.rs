// List global orgs returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_organizations::ListGlobalOrgsOptionalParams;
use datadog_api_client::datadogV2::api_organizations::OrganizationsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OrganizationsAPI::with_config(configuration);
    let response = api.list_global_orgs_with_pagination(
        "user@example.com".to_string(),
        ListGlobalOrgsOptionalParams::default(),
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
