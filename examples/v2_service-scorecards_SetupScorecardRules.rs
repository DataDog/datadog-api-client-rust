// Set up rules for organization returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_scorecards::ServiceScorecardsAPI;
use datadog_api_client::datadogV2::model::SetupRulesRequest;
use datadog_api_client::datadogV2::model::SetupRulesRequestAttributes;
use datadog_api_client::datadogV2::model::SetupRulesRequestData;
use datadog_api_client::datadogV2::model::SetupRulesRequestDataType;

#[tokio::main]
async fn main() {
    let body = SetupRulesRequest::new(SetupRulesRequestData::new(
        SetupRulesRequestAttributes::new().disabled_default_rules(vec![
            "q8MQxk8TCqrHnWkx".to_string(),
            "r9NRyl9UDrsIoXly".to_string(),
        ]),
        SetupRulesRequestDataType::SETUP,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SetupScorecardRules", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api.setup_scorecard_rules(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
