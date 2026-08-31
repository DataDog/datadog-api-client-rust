// List usage quotas returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::ListQuotasOptionalParams;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListQuotas", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let response = api.list_quotas_with_pagination(
        "ai_credits".to_string(),
        ListQuotasOptionalParams::default(),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
