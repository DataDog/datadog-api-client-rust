// Create an AWS WIF intake mapping returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_awswif::AWSWIFAPI;
use datadog_api_client::datadogV2::model::AwsWifIntakeMappingAttributes;
use datadog_api_client::datadogV2::model::AwsWifIntakeMappingCreateData;
use datadog_api_client::datadogV2::model::AwsWifIntakeMappingCreateRequest;
use datadog_api_client::datadogV2::model::AwsWifIntakeMappingType;

#[tokio::main]
async fn main() {
    let body = AwsWifIntakeMappingCreateRequest::new(AwsWifIntakeMappingCreateData::new(
        AwsWifIntakeMappingAttributes::new(
            "arn:aws:iam::123456789012:role/my-agent-role".to_string(),
        ),
        AwsWifIntakeMappingType::AWS_WIF_INTAKE_MAPPING,
    ));
    let configuration = datadog::Configuration::new();
    let api = AWSWIFAPI::with_config(configuration);
    let resp = api.create_aws_wif_intake_mapping(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
