// Update an Anthropic Usage and Costs account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_anthropic_usage_and_costs_integration::AnthropicUsageAndCostsIntegrationAPI;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::AnthropicUsageAndCostsIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        AnthropicUsageAndCostsIntegrationAccountUpdateRequest::new(
            AnthropicUsageAndCostsIntegrationAccountUpdateData::new(
                AnthropicUsageAndCostsIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        AnthropicUsageAndCostsIntegrationAccountAuthenticationUpdate
                        ::AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthUpdate::new(
                                    AnthropicUsageAndCostsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
                        AnthropicUsageAndCostsIntegrationDataflowsRequest
                        ::new().anthropic_cloud_cost_metrics(
                            AnthropicUsageAndCostsCloudCostMetricsIntegrationDataflowRequest::new().enabled(true),
                        ),
                    )
                    .name("anthropic-acme".to_string()),
                "9b4c7e2a-1d6f-4a3b-8e5c-0f2d7a9b6c18".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.update_anthropic_usage_and_costs_integration_account",
        true,
    );
    let api = AnthropicUsageAndCostsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_anthropic_usage_and_costs_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
