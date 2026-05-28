// Update an Opsgenie account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_opsgenie_integration::OpsgenieIntegrationAPI;
use datadog_api_client::datadogV2::model::OpsgenieAccountType;
use datadog_api_client::datadogV2::model::OpsgenieAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::OpsgenieAccountUpdateData;
use datadog_api_client::datadogV2::model::OpsgenieAccountUpdateRequest;
use datadog_api_client::datadogV2::model::OpsgenieServiceRegionType;

#[tokio::main]
async fn main() {
    let body = OpsgenieAccountUpdateRequest::new(OpsgenieAccountUpdateData::new(
        OpsgenieAccountUpdateAttributes::new()
            .api_key("00000000-0000-0000-0000-000000000000".to_string())
            .region(OpsgenieServiceRegionType::US),
        "596da4af-0563-4097-90ff-07230c3f9db3".to_string(),
        OpsgenieAccountType::OPSGENIE_ACCOUNT,
    ));
    let configuration = datadog::Configuration::new();
    let api = OpsgenieIntegrationAPI::with_config(configuration);
    let resp = api
        .update_opsgenie_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
