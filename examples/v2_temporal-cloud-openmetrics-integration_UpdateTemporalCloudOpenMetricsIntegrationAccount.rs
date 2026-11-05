// Update a Temporal Cloud OpenMetrics account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_temporal_cloud_open_metrics_integration::TemporalCloudOpenMetricsIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountUpdateRequest;

#[tokio::main]
async fn main() {
    let body =
        TemporalCloudOpenMetricsIntegrationAccountUpdateRequest::new(
            TemporalCloudOpenMetricsIntegrationAccountUpdateData::new(
                TemporalCloudOpenMetricsIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        TemporalCloudOpenMetricsIntegrationAccountAuthenticationUpdate
                        ::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthUpdate::new(
                                    TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .name("temporal-cloud-prod".to_string())
                    .settings(
                        TemporalCloudOpenMetricsIntegrationAccountSettingsUpdate::new()
                            .include_opt_in_labels(false)
                            .namespaces_to_include("prod-*,billing.a1b2c".to_string()),
                    ),
                "7c2d4e91-5a3b-4f68-b0e2-9d1f6a8c3e57".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.update_temporal_cloud_open_metrics_integration_account",
        true,
    );
    let api = TemporalCloudOpenMetricsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_temporal_cloud_open_metrics_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
