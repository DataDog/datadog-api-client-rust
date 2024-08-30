// AWS Integration - Create account config returns "AWS Account object" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSAccountCreateRequest;
use datadog_api_client::datadogV2::model::AWSAccountCreateRequestAttributes;
use datadog_api_client::datadogV2::model::AWSAccountCreateRequestData;
use datadog_api_client::datadogV2::model::AWSAccountPartition;
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
    let body = AWSAccountCreateRequest::new().data(
        AWSAccountCreateRequestData::new(
            AWSAccountCreateRequestAttributes::new(
                AWSAuthConfig::AWSAuthConfigRole(Box::new(AWSAuthConfigRole::new(
                    "test".to_string(),
                ))),
                "123456789012".to_string(),
                AWSAccountPartition::AWS,
            )
            .account_tags(Some(vec![]))
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
        .id("123456789012".to_string())
        .type_("account".to_string()),
    );
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
