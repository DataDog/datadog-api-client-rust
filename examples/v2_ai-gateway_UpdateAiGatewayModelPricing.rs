// Set a model pricing override returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingCustomRequestConfig;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingCustomRequestSource;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingCustomRequestSourceType;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequest;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequestAttributes;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequestConfig;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequestData;
use datadog_api_client::datadogV2::model::AiGatewayModelPricingRequiredRates;
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
                ),
            )),
        ),
        "gpt-4o".to_string(),
        AiGatewayModelPricingType::MODEL_PRICING,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_ai_gateway_model_pricing", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_model_pricing(9223372036854775807, "canonical".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
