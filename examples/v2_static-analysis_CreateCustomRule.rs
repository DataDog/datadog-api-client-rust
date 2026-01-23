// Create Custom Rule returns "Successfully created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::CustomRuleDataType;
use datadog_api_client::datadogV2::model::CustomRuleRequest;
use datadog_api_client::datadogV2::model::CustomRuleRequestData;
use datadog_api_client::datadogV2::model::CustomRuleRequestDataAttributes;

#[tokio::main]
async fn main() {
    let body = CustomRuleRequest::new().data(
        CustomRuleRequestData::new()
            .attributes(CustomRuleRequestDataAttributes::new())
            .type_(CustomRuleDataType::CUSTOM_RULE),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateCustomRule", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .create_custom_rule("ruleset_name".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
