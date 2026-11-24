// Set a model pricing override by canonical ID returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingCustomRequestConfig;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingCustomRequestSource;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingCustomRequestSourceType;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRates;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequest;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequestAttributes;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequestConfig;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequestData;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequiredRates;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingTier;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingType;

#[tokio::main]
async fn main() {
    let body = AiGatewayModelPricingRequest::new(AiGatewayModelPricingRequestData::new(
        AiGatewayModelPricingRequestAttributes::new(
            AiGatewayModelPricingRequestConfig::AiGatewayModelPricingCustomRequestConfig(Box::new(
                AiGatewayModelPricingCustomRequestConfig::new(
                    AiGatewayModelPricingRequiredRates::new(0.0, 0.0, 0.0, 0.0),
                    AiGatewayModelPricingCustomRequestSource::new(
                        AiGatewayModelPricingCustomRequestSourceType::CUSTOM,
                    ),
                    1,
                )
                .tiers(vec![AiGatewayModelPricingTier::new(
                    AiGatewayModelPricingRates::new()
                        .cache_read_cost_per_mtok(0.6 as f64)
                        .cache_write_cost_per_mtok(7.5 as f64)
                        .non_cached_input_cost_per_mtok(6.0 as f64)
                        .output_cost_per_mtok(22.5 as f64),
                )
                .context_min_tokens(200000)
                .service_tier("priority".to_string())]),
            )),
        ),
        "gpt-4o".to_string(),
        AiGatewayModelPricingType::MODEL_PRICING,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_ai_gateway_model_pricing_by_canonical", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_model_pricing_by_canonical(
            9223372036854775807,
            "canonical".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
