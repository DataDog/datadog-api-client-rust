// Update an AWS integration returns "AWS Account object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSAccountPartition;
use datadog_api_client::datadogV2::model::AWSAccountType;
use datadog_api_client::datadogV2::model::AWSAccountUpdateRequest;
use datadog_api_client::datadogV2::model::AWSAccountUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::AWSAccountUpdateRequestData;
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
    // there is a valid "aws_account_v2" in the system
    let aws_account_v2_data_id = std::env::var("AWS_ACCOUNT_V2_DATA_ID").unwrap();
    let body =
        AWSAccountUpdateRequest::new(
            AWSAccountUpdateRequestData::new(
                AWSAccountUpdateRequestAttributes::new("123456789012".to_string())
                    .account_tags(Some(vec!["key:value".to_string()]))
                    .auth_config(
                        AWSAuthConfig::AWSAuthConfigRole(
                            Box::new(AWSAuthConfigRole::new("DatadogIntegrationRole".to_string())),
                        ),
                    )
                    .aws_partition(AWSAccountPartition::AWS)
                    .logs_config(
                        AWSLogsConfig
                        ::new().lambda_forwarder(
                            AWSLambdaForwarderConfig::new()
                                .lambdas(
                                    vec![
                                        "arn:aws:lambda:us-east-1:123456789012:function:DatadogLambdaLogForwarder".to_string()
                                    ],
                                )
                                .log_source_config(
                                    AWSLambdaForwarderConfigLogSourceConfig
                                    ::new().tag_filters(
                                        vec![
                                            AWSLogSourceTagFilter::new()
                                                .source("s3".to_string())
                                                .tags(Some(vec!["test:test".to_string()]))
                                        ],
                                    ),
                                )
                                .sources(vec!["s3".to_string()]),
                        ),
                    )
                    .metrics_config(
                        AWSMetricsConfig::new()
                            .automute_enabled(true)
                            .collect_cloudwatch_alarms(true)
                            .collect_custom_metrics(true)
                            .enabled(true)
                            .tag_filters(
                                vec![
                                    AWSNamespaceTagFilter::new()
                                        .namespace("AWS/EC2".to_string())
                                        .tags(Some(vec!["key:value".to_string()]))
                                ],
                            ),
                    )
                    .resources_config(
                        AWSResourcesConfig::new()
                            .cloud_security_posture_management_collection(false)
                            .extended_collection(false),
                    )
                    .traces_config(AWSTracesConfig::new()),
                AWSAccountType::ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateAWSAccount", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api
        .update_aws_account(aws_account_v2_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
