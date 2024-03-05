// Paginate monthly usage attribution
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_usage_metering::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "monthly_usage_attribution" response
    let monthly_usage_attribution_metadata_pagination_next_record_id =
        std::env::var("MONTHLY_USAGE_ATTRIBUTION_METADATA_PAGINATION_NEXT_RECORD_ID").unwrap();
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp =
        api
            .get_monthly_usage_attribution(
                (Utc::now() + chrono::Duration::days(-3)).to_rfc3339(),
                MonthlyUsageAttributionSupportedMetrics::INFRA_HOST_USAGE,
                GetMonthlyUsageAttributionOptionalParams
                ::default().next_record_id(monthly_usage_attribution_metadata_pagination_next_record_id),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
