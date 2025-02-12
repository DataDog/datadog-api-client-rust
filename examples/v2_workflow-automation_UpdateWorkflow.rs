// Update an existing Workflow returns "Successfully updated a workflow." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::Connection;
use datadog_api_client::datadogV2::model::ConnectionEnv;
use datadog_api_client::datadogV2::model::ConnectionEnvEnv;
use datadog_api_client::datadogV2::model::GithubWebhookTrigger;
use datadog_api_client::datadogV2::model::GithubWebhookTriggerWrapper;
use datadog_api_client::datadogV2::model::InputSchema;
use datadog_api_client::datadogV2::model::InputSchemaParameters;
use datadog_api_client::datadogV2::model::InputSchemaParametersType;
use datadog_api_client::datadogV2::model::MonitorTrigger;
use datadog_api_client::datadogV2::model::MonitorTriggerWrapper;
use datadog_api_client::datadogV2::model::OutboundEdge;
use datadog_api_client::datadogV2::model::OutputSchema;
use datadog_api_client::datadogV2::model::OutputSchemaParameters;
use datadog_api_client::datadogV2::model::OutputSchemaParametersType;
use datadog_api_client::datadogV2::model::Parameter;
use datadog_api_client::datadogV2::model::Spec;
use datadog_api_client::datadogV2::model::Step;
use datadog_api_client::datadogV2::model::Trigger;
use datadog_api_client::datadogV2::model::TriggerRateLimit;
use datadog_api_client::datadogV2::model::UpdateWorkflowRequest;
use datadog_api_client::datadogV2::model::WorkflowDataType;
use datadog_api_client::datadogV2::model::WorkflowDataUpdate;
use datadog_api_client::datadogV2::model::WorkflowDataUpdateAttributes;
use serde_json::Value;

#[tokio::main]
async fn main() {
    // there is a valid "workflow" in the system
    let workflow_data_id = std::env::var("WORKFLOW_DATA_ID").unwrap();
    let body = UpdateWorkflowRequest::new(
        WorkflowDataUpdate::new(
            WorkflowDataUpdateAttributes::new()
                .description("A sample workflow.".to_string())
                .name("Example Workflow".to_string())
                .published(true)
                .spec(
                    Spec::new()
                        .connection_envs(vec![ConnectionEnv::new(ConnectionEnvEnv::DEFAULT)
                            .connections(vec![Connection::new(
                                "11111111-1111-1111-1111-111111111111".to_string(),
                                "INTEGRATION_DATADOG".to_string(),
                            )])])
                        .handle("my-handle".to_string())
                        .input_schema(InputSchema::new().parameters(vec![
                                        InputSchemaParameters::new(
                                            "input".to_string(),
                                            InputSchemaParametersType::STRING,
                                        ).default_value(Value::from("default"))
                                    ]))
                        .output_schema(OutputSchema::new().parameters(vec![
                                        OutputSchemaParameters::new(
                                            "output".to_string(),
                                            OutputSchemaParametersType::ARRAY_OBJECT,
                                        ).value(Value::from("outputValue"))
                                    ]))
                        .steps(vec![
                            Step::new(
                                "com.datadoghq.dd.monitor.listMonitors".to_string(),
                                "Step1".to_string(),
                            )
                            .connection_label("INTEGRATION_DATADOG".to_string())
                            .outbound_edges(vec![OutboundEdge::new(
                                "main".to_string(),
                                "Step2".to_string(),
                            )])
                            .parameters(vec![Parameter::new(
                                "tags".to_string(),
                                Value::from("service:monitoring"),
                            )]),
                            Step::new("com.datadoghq.core.noop".to_string(), "Step2".to_string()),
                        ])
                        .triggers(vec![
                            Trigger::MonitorTriggerWrapper(Box::new(
                                MonitorTriggerWrapper::new(
                                    MonitorTrigger::new().rate_limit(
                                        TriggerRateLimit::new()
                                            .count(1)
                                            .interval("3600s".to_string()),
                                    ),
                                )
                                .start_step_names(vec!["Step1".to_string()]),
                            )),
                            Trigger::GithubWebhookTriggerWrapper(Box::new(
                                GithubWebhookTriggerWrapper::new(GithubWebhookTrigger::new())
                                    .start_step_names(vec!["Step1".to_string()]),
                            )),
                        ]),
                )
                .tags(vec![
                    "team:infra".to_string(),
                    "service:monitoring".to_string(),
                    "foo:bar".to_string(),
                ]),
            WorkflowDataType::WORKFLOWS,
        )
        .id("22222222-2222-2222-2222-222222222222".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api.update_workflow(workflow_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
