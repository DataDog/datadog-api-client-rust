// Get commitments list returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::api_cloud_cost_management::GetCommitmentsCommitmentListOptionalParams;
use datadog_api_client::datadogV2::model::CommitmentsCommitmentType;
use datadog_api_client::datadogV2::model::CommitmentsProvider;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_commitments_commitment_list", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .get_commitments_commitment_list(
            CommitmentsProvider::AWS,
            "ec2".to_string(),
            1788220800000,
            1790812799000,
            GetCommitmentsCommitmentListOptionalParams::default()
                .commitment_type(CommitmentsCommitmentType::RESERVED_INSTANCES),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
