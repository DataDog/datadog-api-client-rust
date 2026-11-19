// Stop bandit returns "The bandit is stopped and serves the chosen variant."
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsStopBanditV2Request;
use datadog_api_client::datadogV2::model::ExperimentsStopBanditV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsStopBanditV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsStopBanditV2RequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ExperimentsStopBanditV2Request::new(ExperimentsStopBanditV2RequestData::new(
        ExperimentsStopBanditV2RequestDataAttributes::new("green".to_string()),
        ExperimentsStopBanditV2RequestDataType::STOP_BANDIT_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.stop_bandit", true);
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .stop_bandit(
            Uuid::parse_str("4b1f3c2e-8d5a-4f6b-9c7e-1a2b3c4d5e6f").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
