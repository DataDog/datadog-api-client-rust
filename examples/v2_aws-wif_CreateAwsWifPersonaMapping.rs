// Create an AWS WIF persona mapping returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_awswif::AWSWIFAPI;
use datadog_api_client::datadogV2::model::AwsWifPersonaMappingCreateAttributes;
use datadog_api_client::datadogV2::model::AwsWifPersonaMappingCreateData;
use datadog_api_client::datadogV2::model::AwsWifPersonaMappingCreateRequest;
use datadog_api_client::datadogV2::model::AwsWifPersonaMappingType;

#[tokio::main]
async fn main() {
    let body = AwsWifPersonaMappingCreateRequest::new(AwsWifPersonaMappingCreateData::new(
        AwsWifPersonaMappingCreateAttributes::new(
            "user@example.com".to_string(),
            "arn:aws:iam::123456789012:role/my-workload-role".to_string(),
        ),
        AwsWifPersonaMappingType::AWS_WIF_CONFIG,
    ));
    let configuration = datadog::Configuration::new();
    let api = AWSWIFAPI::with_config(configuration);
    let resp = api.create_aws_wif_persona_mapping(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
