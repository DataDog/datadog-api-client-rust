// Create a Product Analytics launch returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchCreateData;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchCreateRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchEvent;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchKPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchPhase;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchStepsRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchTrackingPlanRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsLaunchType;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsLaunchCreateRequest::new(ProductAnalyticsLaunchCreateData::new(
        ProductAnalyticsLaunchRequestAttributes::new(
            "Let users apply promo codes at checkout.".to_string(),
            "checkout_promo_codes".to_string(),
            ProductAnalyticsLaunchStepsRequest::new(
                ProductAnalyticsLaunchTrackingPlanRequest::new().events(vec![
                    ProductAnalyticsLaunchEvent::new("promo_code_applied".to_string())
                        .kpis(vec!["7c1e6a52-3a8f-4d4e-9a55-2f1d1c8b6a10".to_string()])
                        .trigger("The user applies a valid promo code.".to_string())
                        .views(vec!["/checkout".to_string()]),
                ]),
            ),
        )
        .description(Some("Promo codes at checkout.".to_string()))
        .kpis(vec![ProductAnalyticsLaunchKPI::new(
            "7c1e6a52-3a8f-4d4e-9a55-2f1d1c8b6a10".to_string(),
            "Checkout conversion rate".to_string(),
        )])
        .phase(ProductAnalyticsLaunchPhase::PLAN),
        ProductAnalyticsLaunchType::LIFTOFF,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_product_analytics_launch", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.create_product_analytics_launch(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
