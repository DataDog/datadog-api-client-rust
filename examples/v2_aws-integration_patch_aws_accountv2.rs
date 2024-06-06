// AWS Integration - Patch account returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSAccountPatch;
use datadog_api_client::datadogV2::model::AWSAccountPatchAttributes;
use datadog_api_client::datadogV2::model::AWSAccountPatchRequest;
use datadog_api_client::datadogV2::model::AWSAccountType;
use datadog_api_client::datadogV2::model::AWSAuthConfig;
use datadog_api_client::datadogV2::model::AWSLambdaForwarder;
use datadog_api_client::datadogV2::model::AWSLogs;
use datadog_api_client::datadogV2::model::AWSMetrics;
use datadog_api_client::datadogV2::model::AWSNamespaceTagFilter;
use datadog_api_client::datadogV2::model::AWSNamespacesList;
use datadog_api_client::datadogV2::model::AWSRegionsList;
use datadog_api_client::datadogV2::model::AWSResources;
use datadog_api_client::datadogV2::model::AWSTraces;
use datadog_api_client::datadogV2::model::AWSXRayServicesList;

#[tokio::main]
async fn main() {
    let body = AWSAccountPatchRequest::new().data(
        AWSAccountPatch::new()
            .attributes(
                AWSAccountPatchAttributes::new()
                    .account_tags(vec![])
                    .auth_config(AWSAuthConfig::new())
                    .aws_account_id("123456789012".to_string())
                    .aws_regions(AWSRegionsList::new().include_only(vec!["us-east-1".to_string()]))
                    .logs_config(
                        AWSLogs::new().lambda_forwarder(
                            AWSLambdaForwarder::new()
                                .lambdas(vec![])
                                .sources(vec!["s3".to_string()]),
                        ),
                    )
                    .metrics_config(
                        AWSMetrics::new()
                            .namespace_filters(
                                AWSNamespacesList::new()
                                    .exclude_only(vec!["AWS/EC2".to_string()])
                                    .include_only(vec!["AWS/EC2".to_string()]),
                            )
                            .tag_filters(vec![AWSNamespaceTagFilter::new()
                                .namespace("AWS/EC2".to_string())
                                .tags(vec![])]),
                    )
                    .resources_config(AWSResources::new())
                    .traces_config(AWSTraces::new().xray_services(
                        AWSXRayServicesList::new().include_only(vec!["AWS/AppSync".to_string()]),
                    )),
            )
            .type_(AWSAccountType::AWS_ACCOUNT),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.PatchAWSAccountv2", true);
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api
        .patch_aws_accountv2("aws_account_config_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
