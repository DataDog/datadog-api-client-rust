// Update deployment rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::model::DeploymentRuleDataType;
use datadog_api_client::datadogV2::model::DeploymentRuleOptionsFaultyDeploymentDetection;
use datadog_api_client::datadogV2::model::DeploymentRulesOptions;
use datadog_api_client::datadogV2::model::UpdateDeploymentRuleParams;
use datadog_api_client::datadogV2::model::UpdateDeploymentRuleParamsData;
use datadog_api_client::datadogV2::model::UpdateDeploymentRuleParamsDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "deployment_gate" in the system
    let deployment_gate_data_id = std::env::var("DEPLOYMENT_GATE_DATA_ID").unwrap();

    // there is a valid "deployment_rule" in the system
    let deployment_rule_data_id = std::env::var("DEPLOYMENT_RULE_DATA_ID").unwrap();
    let body = UpdateDeploymentRuleParams::new(UpdateDeploymentRuleParamsData::new(
        UpdateDeploymentRuleParamsDataAttributes::new(
            false,
            "Updated deployment rule".to_string(),
            DeploymentRulesOptions::DeploymentRuleOptionsFaultyDeploymentDetection(Box::new(
                DeploymentRuleOptionsFaultyDeploymentDetection::new().excluded_resources(vec![]),
            )),
        ),
        DeploymentRuleDataType::DEPLOYMENT_RULE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateDeploymentRule", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api
        .update_deployment_rule(
            deployment_gate_data_id.clone(),
            deployment_rule_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
