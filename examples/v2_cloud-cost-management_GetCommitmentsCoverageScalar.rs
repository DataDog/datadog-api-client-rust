// Get commitments coverage (scalar) returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::api_cloud_cost_management::GetCommitmentsCoverageScalarOptionalParams;
use datadog_api_client::datadogV2::model::CommitmentsProvider;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetCommitmentsCoverageScalar", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .get_commitments_coverage_scalar(
            CommitmentsProvider::AWS,
            "product".to_string(),
            9223372036854775807,
            9223372036854775807,
            GetCommitmentsCoverageScalarOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
