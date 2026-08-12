// Patch a deployment event by version returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dora_metrics::DORAMetricsAPI;
use datadog_api_client::datadogV2::model::DORADeploymentPatchByVersionRemediation;
use datadog_api_client::datadogV2::model::DORADeploymentPatchByVersionRemediationByVersion;
use datadog_api_client::datadogV2::model::DORADeploymentPatchByVersionRequest;
use datadog_api_client::datadogV2::model::DORADeploymentPatchByVersionRequestAttributes;
use datadog_api_client::datadogV2::model::DORADeploymentPatchByVersionRequestData;
use datadog_api_client::datadogV2::model::DORADeploymentPatchRemediationType;
use datadog_api_client::datadogV2::model::DORADeploymentPatchRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        DORADeploymentPatchByVersionRequest::new(
            DORADeploymentPatchByVersionRequestData::new(
                DORADeploymentPatchByVersionRequestAttributes::new(
                    true,
                    "production".to_string(),
                    "my-service".to_string(),
                    "v1.2.3".to_string(),
                ).remediation(
                    DORADeploymentPatchByVersionRemediation::DORADeploymentPatchByVersionRemediationByVersion(
                        Box::new(
                            DORADeploymentPatchByVersionRemediationByVersion::new(
                                DORADeploymentPatchRemediationType::ROLLBACK,
                                "v1.2.2".to_string(),
                            ),
                        ),
                    ),
                ),
                DORADeploymentPatchRequestDataType::DORA_DEPLOYMENT_PATCH_REQUEST,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.PatchDORADeploymentByVersion", true);
    let api = DORAMetricsAPI::with_config(configuration);
    let resp = api.patch_dora_deployment_by_version(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
