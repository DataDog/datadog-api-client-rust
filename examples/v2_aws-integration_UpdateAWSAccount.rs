// AWS Integration - Patch account config returns "AWS Account object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSAccountPartition;
use datadog_api_client::datadogV2::model::AWSAccountPatchRequest;
use datadog_api_client::datadogV2::model::AWSAccountPatchRequestAttributes;
use datadog_api_client::datadogV2::model::AWSAccountPatchRequestData;
use datadog_api_client::datadogV2::model::AWSAuthConfig;
use datadog_api_client::datadogV2::model::AWSAuthConfigRole;
use datadog_api_client::datadogV2::model::AWSLambdaForwarderConfig;
use datadog_api_client::datadogV2::model::AWSLogsConfig;
use datadog_api_client::datadogV2::model::AWSMetricsConfig;
use datadog_api_client::datadogV2::model::AWSNamespaceFilters;
use datadog_api_client::datadogV2::model::AWSNamespaceFiltersIncludeOnly;
use datadog_api_client::datadogV2::model::AWSNamespaceTagFilter;
use datadog_api_client::datadogV2::model::AWSRegions;
use datadog_api_client::datadogV2::model::AWSRegionsIncludeOnly;
use datadog_api_client::datadogV2::model::AWSResourcesConfig;
use datadog_api_client::datadogV2::model::AWSTracesConfig;
use datadog_api_client::datadogV2::model::XRayServicesIncludeOnly;
use datadog_api_client::datadogV2::model::XRayServicesList;

#[tokio::main]
async fn main() {
    // there is a valid "aws_account_v2" in the system
    let aws_account_v2_data_attributes_aws_account_id =
        std::env::var("AWS_ACCOUNT_V2_DATA_ATTRIBUTES_AWS_ACCOUNT_ID").unwrap();
    let body = AWSAccountPatchRequest::new().data(
        AWSAccountPatchRequestData::new(
            AWSAccountPatchRequestAttributes::new(
                aws_account_v2_data_attributes_aws_account_id.clone(),
            )
            .account_tags(Some(vec![]))
            .auth_config(AWSAuthConfig::AWSAuthConfigRole(Box::new(
                AWSAuthConfigRole::new("test".to_string()),
            )))
            .aws_partition(AWSAccountPartition::AWS)
            .aws_regions(AWSRegions::AWSRegionsIncludeOnly(Box::new(
                AWSRegionsIncludeOnly::new(vec!["us-east-1".to_string()]),
            )))
            .logs_config(
                AWSLogsConfig::new().lambda_forwarder(
                    AWSLambdaForwarderConfig::new()
                        .lambdas(vec![])
                        .sources(vec!["s3".to_string()]),
                ),
            )
            .metrics_config(
                AWSMetricsConfig::new()
                    .namespace_filters(AWSNamespaceFilters::AWSNamespaceFiltersIncludeOnly(
                        Box::new(AWSNamespaceFiltersIncludeOnly::new(vec![
                            "AWS/EC2".to_string()
                        ])),
                    ))
                    .tag_filters(vec![AWSNamespaceTagFilter::new()
                        .namespace("AWS/EC2".to_string())
                        .tags(Some(vec![]))]),
            )
            .resources_config(AWSResourcesConfig::new())
            .traces_config(AWSTracesConfig::new().xray_services(
                XRayServicesList::XRayServicesIncludeOnly(Box::new(XRayServicesIncludeOnly::new(
                    vec!["AWS/AppSync".to_string()],
                ))),
            )),
        )
        .id(aws_account_v2_data_attributes_aws_account_id.clone())
        .type_("account".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateAWSAccount", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api
        .update_aws_account(aws_account_v2_data_attributes_aws_account_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
