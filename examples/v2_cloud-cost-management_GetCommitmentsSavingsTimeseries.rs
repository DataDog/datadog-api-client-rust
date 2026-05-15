// Get commitments savings (timeseries) returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::api_cloud_cost_management::GetCommitmentsSavingsTimeseriesOptionalParams;
use datadog_api_client::datadogV2::model::CommitmentsProvider;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetCommitmentsSavingsTimeseries", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .get_commitments_savings_timeseries(
            CommitmentsProvider::AWS,
            "product".to_string(),
            9223372036854775807,
            9223372036854775807,
            GetCommitmentsSavingsTimeseriesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
