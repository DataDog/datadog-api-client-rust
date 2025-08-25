// Delete Org Connection returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_connections::OrgConnectionsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "org_connection" in the system
    let org_connection_data_id =
        uuid::Uuid::parse_str(&std::env::var("ORG_CONNECTION_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = OrgConnectionsAPI::with_config(configuration);
    let resp = api
        .delete_org_connections(org_connection_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
