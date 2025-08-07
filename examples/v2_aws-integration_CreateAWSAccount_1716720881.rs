// Create an AWS account returns "AWS Account object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSAccountCreateRequest;
use datadog_api_client::datadogV2::model::AWSAccountCreateRequestAttributes;
use datadog_api_client::datadogV2::model::AWSAccountCreateRequestData;
use datadog_api_client::datadogV2::model::AWSAccountPartition;
use datadog_api_client::datadogV2::model::AWSAccountType;
use datadog_api_client::datadogV2::model::AWSAuthConfig;
use datadog_api_client::datadogV2::model::AWSAuthConfigRole;
use datadog_api_client::datadogV2::model::AWSLambdaForwarderConfig;
use datadog_api_client::datadogV2::model::AWSLambdaForwarderConfigLogSourceConfig;
use datadog_api_client::datadogV2::model::AWSLogSourceTagFilter;
use datadog_api_client::datadogV2::model::AWSLogsConfig;
use datadog_api_client::datadogV2::model::AWSMetricsConfig;
use datadog_api_client::datadogV2::model::AWSNamespaceTagFilter;
use datadog_api_client::datadogV2::model::AWSResourcesConfig;
use datadog_api_client::datadogV2::model::AWSTracesConfig;

#[tokio::main]
async fn main() {
    let body = AWSAccountCreateRequest::new(AWSAccountCreateRequestData::new(
        AWSAccountCreateRequestAttributes::new(
            AWSAuthConfig::AWSAuthConfigRole(Box::new(AWSAuthConfigRole::new(
                "DatadogIntegrationRole".to_string(),
            ))),
            "123456789012".to_string(),
            AWSAccountPartition::AWS,
        )
        .account_tags(Some(vec!["key:value".to_string()]))
        .logs_config(
            AWSLogsConfig::new().lambda_forwarder(
                AWSLambdaForwarderConfig::new()
                    .lambdas(vec![
                        "arn:aws:lambda:us-east-1:123456789012:function:DatadogLambdaLogForwarder"
                            .to_string(),
                    ])
                    .log_source_config(AWSLambdaForwarderConfigLogSourceConfig::new().tag_filters(
                        vec![
                                            AWSLogSourceTagFilter::new()
                                                .source("s3".to_string())
                                                .tags(Some(vec!["test:test".to_string()]))
                                        ],
                    ))
                    .sources(vec!["s3".to_string()]),
            ),
        )
        .metrics_config(
            AWSMetricsConfig::new()
                .automute_enabled(true)
                .collect_cloudwatch_alarms(true)
                .collect_custom_metrics(true)
                .enabled(true)
                .tag_filters(vec![AWSNamespaceTagFilter::new()
                    .namespace("AWS/EC2".to_string())
                    .tags(Some(vec!["key:value".to_string()]))]),
        )
        .resources_config(
            AWSResourcesConfig::new()
                .cloud_security_posture_management_collection(false)
                .extended_collection(false),
        )
        .traces_config(AWSTracesConfig::new()),
        AWSAccountType::ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAWSAccount", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.create_aws_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
