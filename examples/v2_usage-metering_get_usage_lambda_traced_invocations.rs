// Get hourly usage for lambda traced invocations returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_usage_metering::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp =
        api
            .get_usage_lambda_traced_invocations(
                (Utc::now() + chrono::Duration::days(-5)).to_rfc3339(),
                GetUsageLambdaTracedInvocationsOptionalParams
                ::default().end_hr((Utc::now() + chrono::Duration::days(-3)).to_rfc3339()),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
