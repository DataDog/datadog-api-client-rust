// Create a new Opsgenie account returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_opsgenie_integration::OpsgenieIntegrationAPI;
use datadog_api_client::datadogV2::model::OpsgenieAccountCreateAttributes;
use datadog_api_client::datadogV2::model::OpsgenieAccountCreateData;
use datadog_api_client::datadogV2::model::OpsgenieAccountCreateRequest;
use datadog_api_client::datadogV2::model::OpsgenieAccountType;
use datadog_api_client::datadogV2::model::OpsgenieServiceRegionType;

#[tokio::main]
async fn main() {
    let body = OpsgenieAccountCreateRequest::new(OpsgenieAccountCreateData::new(
        OpsgenieAccountCreateAttributes::new(
            "00000000-0000-0000-0000-000000000000".to_string(),
            OpsgenieServiceRegionType::US,
        ),
        OpsgenieAccountType::OPSGENIE_ACCOUNT,
    ));
    let configuration = datadog::Configuration::new();
    let api = OpsgenieIntegrationAPI::with_config(configuration);
    let resp = api.create_opsgenie_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
