// Create or replace a budget's custom forecast returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::CustomForecastEntry;
use datadog_api_client::datadogV2::model::CustomForecastEntryTagFilter;
use datadog_api_client::datadogV2::model::CustomForecastType;
use datadog_api_client::datadogV2::model::CustomForecastUpsertRequest;
use datadog_api_client::datadogV2::model::CustomForecastUpsertRequestData;
use datadog_api_client::datadogV2::model::CustomForecastUpsertRequestDataAttributes;

#[tokio::main]
async fn main() {
    let body = CustomForecastUpsertRequest::new(
        CustomForecastUpsertRequestData::new(
            CustomForecastUpsertRequestDataAttributes::new(
                "00000000-0000-0000-0000-000000000001".to_string(),
                vec![CustomForecastEntry::new(
                    400.0,
                    202501,
                    vec![CustomForecastEntryTagFilter::new(
                        "service".to_string(),
                        "ec2".to_string(),
                    )],
                )],
            ),
            CustomForecastType::CUSTOM_FORECAST,
        )
        .id("".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertCustomForecast", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.upsert_custom_forecast(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
