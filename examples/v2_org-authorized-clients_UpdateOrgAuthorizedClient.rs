// Update an org authorized client returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_authorized_clients::OrgAuthorizedClientsAPI;
use datadog_api_client::datadogV2::model::OrgAuthorizedClientType;
use datadog_api_client::datadogV2::model::OrgAuthorizedClientUpdateAttributes;
use datadog_api_client::datadogV2::model::OrgAuthorizedClientUpdateData;
use datadog_api_client::datadogV2::model::OrgAuthorizedClientUpdateRequest;

#[tokio::main]
async fn main() {
    let body = OrgAuthorizedClientUpdateRequest::new(
        OrgAuthorizedClientUpdateData::new(
            "00000000-0000-0000-0000-000000000001".to_string(),
            OrgAuthorizedClientType::ORG_AUTHORIZED_CLIENTS,
        )
        .attributes(OrgAuthorizedClientUpdateAttributes::new().disabled(true)),
    );
    let configuration = datadog::Configuration::new();
    let api = OrgAuthorizedClientsAPI::with_config(configuration);
    let resp = api
        .update_org_authorized_client("00000000-0000-0000-0000-000000000001".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
