// Create bits deployment analysis deployment rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::model::CreateDeploymentRuleParams;
use datadog_api_client::datadogV2::model::CreateDeploymentRuleParamsData;
use datadog_api_client::datadogV2::model::CreateDeploymentRuleParamsDataAttributes;
use datadog_api_client::datadogV2::model::DeploymentRuleDataType;
use datadog_api_client::datadogV2::model::DeploymentRuleOptionsFaultyDeploymentDetection;
use datadog_api_client::datadogV2::model::DeploymentRulesOptions;

#[tokio::main]
async fn main() {
    // there is a valid "deployment_gate" in the system
    let deployment_gate_data_id = std::env::var("DEPLOYMENT_GATE_DATA_ID").unwrap();
    let body = CreateDeploymentRuleParams::new().data(CreateDeploymentRuleParamsData::new(
        CreateDeploymentRuleParamsDataAttributes::new(
            "Bits deployment analysis rule".to_string(),
            DeploymentRulesOptions::DeploymentRuleOptionsFaultyDeploymentDetection(Box::new(
                DeploymentRuleOptionsFaultyDeploymentDetection::new().duration(900),
            )),
            "bits_deployment_analysis".to_string(),
        )
        .dry_run(false),
        DeploymentRuleDataType::DEPLOYMENT_RULE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_deployment_rule", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api
        .create_deployment_rule(deployment_gate_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
