// Create a Temporal Cloud OpenMetrics account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_temporal_cloud_open_metrics_integration::TemporalCloudOpenMetricsIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountSettingsRequest;

#[tokio::main]
async fn main() {
    let body =
        TemporalCloudOpenMetricsIntegrationAccountCreateRequest::new(
            TemporalCloudOpenMetricsIntegrationAccountCreateData::new(
                TemporalCloudOpenMetricsIntegrationAccountCreateAttributes::new(
                    TemporalCloudOpenMetricsIntegrationAccountAuthenticationRequest
                    ::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest::new(
                                TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "temporal-cloud-prod".to_string(),
                    TemporalCloudOpenMetricsIntegrationAccountSettingsRequest::new()
                        .include_opt_in_labels(false)
                        .namespaces_to_include("prod-*,billing.a1b2c".to_string()),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.create_temporal_cloud_open_metrics_integration_account",
        true,
    );
    let api = TemporalCloudOpenMetricsIntegrationAPI::with_config(configuration);
    let resp = api
        .create_temporal_cloud_open_metrics_integration_account(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
