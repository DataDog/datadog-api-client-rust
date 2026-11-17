// Update a CoreWeave integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_core_weave_integration::CoreWeaveIntegrationAPI;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthType;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::CoreWeaveCloudMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        CoreWeaveCloudIntegrationAccountUpdateRequest::new(
            CoreWeaveCloudIntegrationAccountUpdateData::new(
                CoreWeaveCloudIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        CoreWeaveCloudIntegrationAccountAuthenticationUpdate
                        ::CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate(
                            Box::new(
                                CoreWeaveCloudIntegrationAccountAPITokenAuthUpdate::new(
                                    CoreWeaveCloudIntegrationAccountAPITokenAuthType::COREWEAVE_API_TOKEN,
                                ).api_token("CW-SECRET-xxxxxxxxxxxxxxxxxxxx".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
                        CoreWeaveCloudIntegrationDataflowsRequest::new()
                            .coreweave_cloud_logs(CoreWeaveCloudLogsIntegrationDataflowRequest::new().enabled(true))
                            .coreweave_cloud_metrics(
                                CoreWeaveCloudMetricsIntegrationDataflowRequest::new().enabled(true),
                            ),
                    )
                    .name("coreweave-prod".to_string())
                    .settings(
                        CoreWeaveCloudIntegrationAccountSettingsUpdate::new()
                            .account_tags(vec!["env:prod".to_string(), "team:gpu".to_string()])
                            .cluster_filters(vec!["cluster:prod-*".to_string(), "!cluster:dev".to_string()]),
                    ),
                "f7a8b9c0-d1e2-4f3a-9b4c-5d6e7f8091a2".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_core_weave_cloud_integration_account", true);
    let api = CoreWeaveIntegrationAPI::with_config(configuration);
    let resp = api
        .update_core_weave_cloud_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
