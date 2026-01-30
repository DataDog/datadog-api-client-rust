// Patch a deployment event returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dora_metrics::DORAMetricsAPI;
use datadog_api_client::datadogV2::model::DORADeploymentPatchRemediation;
use datadog_api_client::datadogV2::model::DORADeploymentPatchRemediationType;
use datadog_api_client::datadogV2::model::DORADeploymentPatchRequest;
use datadog_api_client::datadogV2::model::DORADeploymentPatchRequestAttributes;
use datadog_api_client::datadogV2::model::DORADeploymentPatchRequestData;
use datadog_api_client::datadogV2::model::DORADeploymentPatchRequestDataType;

#[tokio::main]
async fn main() {
    let body = DORADeploymentPatchRequest::new(DORADeploymentPatchRequestData::new(
        DORADeploymentPatchRequestAttributes::new()
            .change_failure(true)
            .remediation(DORADeploymentPatchRemediation::new(
                "eG42zNIkVjM".to_string(),
                DORADeploymentPatchRemediationType::ROLLBACK,
            )),
        "z_RwVLi7v4Y".to_string(),
        DORADeploymentPatchRequestDataType::DORA_DEPLOYMENT_PATCH_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = DORAMetricsAPI::with_config(configuration);
    let resp = api
        .patch_dora_deployment("deployment_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
