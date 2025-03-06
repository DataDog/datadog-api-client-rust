// Post an AWS on demand task returns "AWS on demand task created successfully."
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;
use datadog_api_client::datadogV2::model::AwsOnDemandCreateAttributes;
use datadog_api_client::datadogV2::model::AwsOnDemandCreateData;
use datadog_api_client::datadogV2::model::AwsOnDemandCreateRequest;
use datadog_api_client::datadogV2::model::AwsOnDemandType;

#[tokio::main]
async fn main() {
    let body = AwsOnDemandCreateRequest::new(AwsOnDemandCreateData::new(
        AwsOnDemandCreateAttributes::new(
            "arn:aws:lambda:eu-west-3:376334461865:function:This-Is-An-Api-Spec-Test".to_string(),
        ),
        AwsOnDemandType::AWS_RESOURCE,
    ));
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api.create_aws_on_demand_task(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
