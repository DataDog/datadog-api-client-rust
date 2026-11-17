// Create a CoreWeave integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_core_weave_integration::CoreWeaveIntegrationAPI;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthType;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        CoreWeaveCloudIntegrationAccountCreateRequest::new(
            CoreWeaveCloudIntegrationAccountCreateData::new(
                CoreWeaveCloudIntegrationAccountCreateAttributes::new(
                    CoreWeaveCloudIntegrationAccountAuthenticationRequest
                    ::CoreWeaveCloudIntegrationAccountAPITokenAuthRequest(
                        Box::new(
                            CoreWeaveCloudIntegrationAccountAPITokenAuthRequest::new(
                                "CW-SECRET-xxxxxxxxxxxxxxxxxxxx".to_string(),
                                CoreWeaveCloudIntegrationAccountAPITokenAuthType::COREWEAVE_API_TOKEN,
                            ),
                        ),
                    ),
                    "coreweave-prod".to_string(),
                    CoreWeaveCloudIntegrationAccountSettingsRequest::new()
                        .account_tags(vec!["env:prod".to_string(), "team:gpu".to_string()])
                        .cluster_filters(vec!["cluster:prod-*".to_string(), "!cluster:dev".to_string()]),
                ).dataflows(
                    CoreWeaveCloudIntegrationDataflowsRequest::new()
                        .coreweave_cloud_logs(CoreWeaveCloudLogsIntegrationDataflowRequest::new().enabled(true))
                        .coreweave_cloud_metrics(
                            CoreWeaveCloudMetricsIntegrationDataflowRequest::new().enabled(true),
                        ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_core_weave_cloud_integration_account", true);
    let api = CoreWeaveIntegrationAPI::with_config(configuration);
    let resp = api.create_core_weave_cloud_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
