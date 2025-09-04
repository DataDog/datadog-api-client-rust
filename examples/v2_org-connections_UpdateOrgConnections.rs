// Update Org Connection returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_connections::OrgConnectionsAPI;
use datadog_api_client::datadogV2::model::OrgConnectionType;
use datadog_api_client::datadogV2::model::OrgConnectionTypeEnum;
use datadog_api_client::datadogV2::model::OrgConnectionUpdate;
use datadog_api_client::datadogV2::model::OrgConnectionUpdateAttributes;
use datadog_api_client::datadogV2::model::OrgConnectionUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "org_connection" in the system
    let org_connection_data_id =
        uuid::Uuid::parse_str(&std::env::var("ORG_CONNECTION_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = OrgConnectionUpdateRequest::new(OrgConnectionUpdate::new(
        OrgConnectionUpdateAttributes::new(vec![
            OrgConnectionTypeEnum::LOGS,
            OrgConnectionTypeEnum::METRICS,
        ]),
        org_connection_data_id.clone(),
        OrgConnectionType::ORG_CONNECTION,
    ));
    let configuration = datadog::Configuration::new();
    let api = OrgConnectionsAPI::with_config(configuration);
    let resp = api
        .update_org_connections(org_connection_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
