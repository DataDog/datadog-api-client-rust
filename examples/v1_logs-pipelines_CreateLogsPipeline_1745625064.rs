// Create a pipeline with Schema Processor and preserve_source true returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::LogsFilter;
use datadog_api_client::datadogV1::model::LogsPipeline;
use datadog_api_client::datadogV1::model::LogsProcessor;
use datadog_api_client::datadogV1::model::LogsSchemaCategoryMapper;
use datadog_api_client::datadogV1::model::LogsSchemaCategoryMapperCategory;
use datadog_api_client::datadogV1::model::LogsSchemaCategoryMapperFallback;
use datadog_api_client::datadogV1::model::LogsSchemaCategoryMapperTargets;
use datadog_api_client::datadogV1::model::LogsSchemaCategoryMapperType;
use datadog_api_client::datadogV1::model::LogsSchemaData;
use datadog_api_client::datadogV1::model::LogsSchemaMapper;
use datadog_api_client::datadogV1::model::LogsSchemaProcessor;
use datadog_api_client::datadogV1::model::LogsSchemaProcessorType;
use datadog_api_client::datadogV1::model::LogsSchemaRemapper;
use datadog_api_client::datadogV1::model::LogsSchemaRemapperType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = LogsPipeline::new("testSchemaProcessor".to_string())
        .filter(LogsFilter::new().query("source:python".to_string()))
        .processors(vec![LogsProcessor::LogsSchemaProcessor(Box::new(
            LogsSchemaProcessor::new(
                vec![
                    LogsSchemaMapper::LogsSchemaCategoryMapper(Box::new(
                        LogsSchemaCategoryMapper::new(
                            vec![
                                LogsSchemaCategoryMapperCategory::new(
                                    LogsFilter::new().query("@eventName:(*Create*)".to_string()),
                                    1,
                                    "Create".to_string(),
                                ),
                                LogsSchemaCategoryMapperCategory::new(
                                    LogsFilter::new().query(
                                        "@eventName:(ChangePassword OR PasswordUpdated)"
                                            .to_string(),
                                    ),
                                    3,
                                    "Password Change".to_string(),
                                ),
                                LogsSchemaCategoryMapperCategory::new(
                                    LogsFilter::new().query("@eventName:(*Attach*)".to_string()),
                                    7,
                                    "Attach Policy".to_string(),
                                ),
                                LogsSchemaCategoryMapperCategory::new(
                                    LogsFilter::new()
                                        .query("@eventName:(*Detach* OR *Remove*)".to_string()),
                                    8,
                                    "Detach Policy".to_string(),
                                ),
                                LogsSchemaCategoryMapperCategory::new(
                                    LogsFilter::new().query("@eventName:(*Delete*)".to_string()),
                                    6,
                                    "Delete".to_string(),
                                ),
                                LogsSchemaCategoryMapperCategory::new(
                                    LogsFilter::new().query("@eventName:*".to_string()),
                                    99,
                                    "Other".to_string(),
                                ),
                            ],
                            "activity_id and activity_name".to_string(),
                            LogsSchemaCategoryMapperTargets::new()
                                .id("ocsf.activity_id".to_string())
                                .name("ocsf.activity_name".to_string()),
                            LogsSchemaCategoryMapperType::SCHEMA_CATEGORY_MAPPER,
                        )
                        .fallback(
                            LogsSchemaCategoryMapperFallback::new()
                                .sources(BTreeMap::from([(
                                    "ocsf.activity_name".to_string(),
                                    vec!["eventName".to_string()],
                                )]))
                                .values(BTreeMap::from([
                                    ("ocsf.activity_id".to_string(), "99".to_string()),
                                    ("ocsf.activity_name".to_string(), "Other".to_string()),
                                ])),
                        ),
                    )),
                    LogsSchemaMapper::LogsSchemaCategoryMapper(Box::new(
                        LogsSchemaCategoryMapper::new(
                            vec![
                                LogsSchemaCategoryMapperCategory::new(
                                    LogsFilter::new().query("-@errorCode:*".to_string()),
                                    1,
                                    "Success".to_string(),
                                ),
                                LogsSchemaCategoryMapperCategory::new(
                                    LogsFilter::new().query("@errorCode:*".to_string()),
                                    2,
                                    "Failure".to_string(),
                                ),
                            ],
                            "status".to_string(),
                            LogsSchemaCategoryMapperTargets::new()
                                .id("ocsf.status_id".to_string())
                                .name("ocsf.status".to_string()),
                            LogsSchemaCategoryMapperType::SCHEMA_CATEGORY_MAPPER,
                        ),
                    )),
                    LogsSchemaMapper::LogsSchemaCategoryMapper(Box::new(
                        LogsSchemaCategoryMapper::new(
                            vec![LogsSchemaCategoryMapperCategory::new(
                                LogsFilter::new().query("@eventName:*".to_string()),
                                1,
                                "Informational".to_string(),
                            )],
                            "Set default severity".to_string(),
                            LogsSchemaCategoryMapperTargets::new()
                                .id("ocsf.severity_id".to_string())
                                .name("ocsf.severity".to_string()),
                            LogsSchemaCategoryMapperType::SCHEMA_CATEGORY_MAPPER,
                        ),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map userIdentity to ocsf.user.uid".to_string(),
                            vec![
                                "userIdentity.principalId".to_string(),
                                "responseElements.role.roleId".to_string(),
                                "responseElements.user.userId".to_string(),
                            ],
                            "ocsf.user.uid".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map userName to ocsf.user.name".to_string(),
                            vec![
                                "requestParameters.userName".to_string(),
                                "responseElements.role.roleName".to_string(),
                                "requestParameters.roleName".to_string(),
                                "responseElements.user.userName".to_string(),
                            ],
                            "ocsf.user.name".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map api to ocsf.api".to_string(),
                            vec!["api".to_string()],
                            "ocsf.api".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map user to ocsf.user".to_string(),
                            vec!["user".to_string()],
                            "ocsf.user".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map actor to ocsf.actor".to_string(),
                            vec!["actor".to_string()],
                            "ocsf.actor".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map cloud to ocsf.cloud".to_string(),
                            vec!["cloud".to_string()],
                            "ocsf.cloud".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map http_request to ocsf.http_request".to_string(),
                            vec!["http_request".to_string()],
                            "ocsf.http_request".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map metadata to ocsf.metadata".to_string(),
                            vec!["metadata".to_string()],
                            "ocsf.metadata".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map time to ocsf.time".to_string(),
                            vec!["time".to_string()],
                            "ocsf.time".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map src_endpoint to ocsf.src_endpoint".to_string(),
                            vec!["src_endpoint".to_string()],
                            "ocsf.src_endpoint".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map severity to ocsf.severity".to_string(),
                            vec!["severity".to_string()],
                            "ocsf.severity".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsSchemaMapper::LogsSchemaRemapper(Box::new(
                        LogsSchemaRemapper::new(
                            "Map severity_id to ocsf.severity_id".to_string(),
                            vec!["severity_id".to_string()],
                            "ocsf.severity_id".to_string(),
                            LogsSchemaRemapperType::SCHEMA_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                ],
                "Apply OCSF schema for 3001".to_string(),
                LogsSchemaData::new(
                    "Account Change".to_string(),
                    3001,
                    "ocsf".to_string(),
                    "1.5.0".to_string(),
                )
                .profiles(vec!["cloud".to_string(), "datetime".to_string()]),
                LogsSchemaProcessorType::SCHEMA_PROCESSOR,
            )
            .is_enabled(true),
        ))])
        .tags(vec![]);
    let configuration = datadog::Configuration::new();
    let api = LogsPipelinesAPI::with_config(configuration);
    let resp = api.create_logs_pipeline(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
