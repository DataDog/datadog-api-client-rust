// Create an Anthropic Usage and Costs account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_anthropic_usage_and_costs_integration::AnthropicUsageAndCostsIntegrationAPI;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        AnthropicUsageAndCostsIntegrationAccountCreateRequest::new(
            AnthropicUsageAndCostsIntegrationAccountCreateData::new(
                AnthropicUsageAndCostsIntegrationAccountCreateAttributes::new(
                    AnthropicUsageAndCostsIntegrationAccountAuthenticationRequest
                    ::AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthRequest::new(
                                AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "anthropic-acme".to_string(),
                ).dataflows(
                    AnthropicUsageAndCostsIntegrationDataflowsRequest
                    ::new().anthropic_cloud_cost_metrics(
                        AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowRequest::new().enabled(true),
                    ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.create_anthropic_usage_and_costs_integration_account",
        true,
    );
    let api = AnthropicUsageAndCostsIntegrationAPI::with_config(configuration);
    let resp = api
        .create_anthropic_usage_and_costs_integration_account(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
