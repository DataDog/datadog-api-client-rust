// Create a configuration deployment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::model::FleetDeploymentConfigureV2Attributes;
use datadog_api_client::datadogV2::model::FleetDeploymentConfigureV2Create;
use datadog_api_client::datadogV2::model::FleetDeploymentConfigureV2CreateRequest;
use datadog_api_client::datadogV2::model::FleetDeploymentFileOp;
use datadog_api_client::datadogV2::model::FleetDeploymentOperation;
use datadog_api_client::datadogV2::model::FleetDeploymentResourceType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = FleetDeploymentConfigureV2CreateRequest::new(FleetDeploymentConfigureV2Create::new(
        FleetDeploymentConfigureV2Attributes::new(
            vec![FleetDeploymentOperation::new(
                FleetDeploymentFileOp::MERGE_PATCH,
                "/datadog.yaml".to_string(),
            )
            .patch(BTreeMap::from([(
                "log_level".to_string(),
                Value::from("info"),
            )]))],
            "env:prod AND service:example-fleet-automation".to_string(),
        )
        .dry_run(true),
        FleetDeploymentResourceType::DEPLOYMENT,
    ));
    let configuration = datadog::Configuration::new();
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.create_fleet_deployment_configure_v2(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
