// Create App returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::ActionQuery;
use datadog_api_client::datadogV2::model::ActionQueryProperties;
use datadog_api_client::datadogV2::model::ActionQuerySpec;
use datadog_api_client::datadogV2::model::ActionQuerySpecInputs;
use datadog_api_client::datadogV2::model::ActionQuerySpecObject;
use datadog_api_client::datadogV2::model::ActionQueryType;
use datadog_api_client::datadogV2::model::AppBuilderEvent;
use datadog_api_client::datadogV2::model::AppBuilderEventName;
use datadog_api_client::datadogV2::model::AppBuilderEventType;
use datadog_api_client::datadogV2::model::AppDefinitionType;
use datadog_api_client::datadogV2::model::Component;
use datadog_api_client::datadogV2::model::ComponentGrid;
use datadog_api_client::datadogV2::model::ComponentGridProperties;
use datadog_api_client::datadogV2::model::ComponentGridType;
use datadog_api_client::datadogV2::model::ComponentProperties;
use datadog_api_client::datadogV2::model::ComponentPropertiesIsVisible;
use datadog_api_client::datadogV2::model::ComponentType;
use datadog_api_client::datadogV2::model::CreateAppRequest;
use datadog_api_client::datadogV2::model::CreateAppRequestData;
use datadog_api_client::datadogV2::model::CreateAppRequestDataAttributes;
use datadog_api_client::datadogV2::model::DataTransform;
use datadog_api_client::datadogV2::model::DataTransformProperties;
use datadog_api_client::datadogV2::model::DataTransformType;
use datadog_api_client::datadogV2::model::Query;
use datadog_api_client::datadogV2::model::StateVariable;
use datadog_api_client::datadogV2::model::StateVariableProperties;
use datadog_api_client::datadogV2::model::StateVariableType;
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        CreateAppRequest
        ::new().data(
            CreateAppRequestData::new(
                AppDefinitionType::APPDEFINITIONS,
            ).attributes(
                CreateAppRequestDataAttributes::new()
                    .components(
                        vec![
                            ComponentGrid::new(
                                "grid0".to_string(),
                                ComponentGridProperties::new()
                                    .background_color("default".to_string())
                                    .children(
                                        vec![
                                            Component::new(
                                                "gridCell0".to_string(),
                                                ComponentProperties::new()
                                                    .children(
                                                        vec![
                                                            Component::new(
                                                                "text0".to_string(),
                                                                ComponentProperties
                                                                ::new().is_visible(
                                                                    ComponentPropertiesIsVisible::Bool(true),
                                                                ),
                                                                ComponentType::TEXT,
                                                            ).events(vec![])
                                                        ],
                                                    )
                                                    .is_visible(
                                                        ComponentPropertiesIsVisible::String("true".to_string()),
                                                    ),
                                                ComponentType::GRIDCELL,
                                            ).events(vec![]),
                                            Component::new(
                                                "gridCell2".to_string(),
                                                ComponentProperties::new()
                                                    .children(
                                                        vec![
                                                            Component::new(
                                                                "table0".to_string(),
                                                                ComponentProperties
                                                                ::new().is_visible(
                                                                    ComponentPropertiesIsVisible::Bool(true),
                                                                ),
                                                                ComponentType::TABLE,
                                                            ).events(vec![])
                                                        ],
                                                    )
                                                    .is_visible(
                                                        ComponentPropertiesIsVisible::String("true".to_string()),
                                                    ),
                                                ComponentType::GRIDCELL,
                                            ).events(vec![]),
                                            Component::new(
                                                "gridCell1".to_string(),
                                                ComponentProperties::new()
                                                    .children(
                                                        vec![
                                                            Component::new(
                                                                "text1".to_string(),
                                                                ComponentProperties
                                                                ::new().is_visible(
                                                                    ComponentPropertiesIsVisible::Bool(true),
                                                                ),
                                                                ComponentType::TEXT,
                                                            ).events(vec![])
                                                        ],
                                                    )
                                                    .is_visible(
                                                        ComponentPropertiesIsVisible::String("true".to_string()),
                                                    ),
                                                ComponentType::GRIDCELL,
                                            ).events(vec![]),
                                            Component::new(
                                                "gridCell3".to_string(),
                                                ComponentProperties::new()
                                                    .children(
                                                        vec![
                                                            Component::new(
                                                                "button0".to_string(),
                                                                ComponentProperties
                                                                ::new().is_visible(
                                                                    ComponentPropertiesIsVisible::Bool(true),
                                                                ),
                                                                ComponentType::BUTTON,
                                                            ).events(
                                                                vec![
                                                                    AppBuilderEvent::new()
                                                                        .name(AppBuilderEventName::CLICK)
                                                                        .type_(
                                                                            AppBuilderEventType::SETSTATEVARIABLEVALUE,
                                                                        )
                                                                ],
                                                            )
                                                        ],
                                                    )
                                                    .is_visible(
                                                        ComponentPropertiesIsVisible::String("true".to_string()),
                                                    ),
                                                ComponentType::GRIDCELL,
                                            ).events(vec![]),
                                            Component::new(
                                                "gridCell4".to_string(),
                                                ComponentProperties::new()
                                                    .children(
                                                        vec![
                                                            Component::new(
                                                                "button1".to_string(),
                                                                ComponentProperties
                                                                ::new().is_visible(
                                                                    ComponentPropertiesIsVisible::Bool(true),
                                                                ),
                                                                ComponentType::BUTTON,
                                                            ).events(
                                                                vec![
                                                                    AppBuilderEvent::new()
                                                                        .name(AppBuilderEventName::CLICK)
                                                                        .type_(
                                                                            AppBuilderEventType::SETSTATEVARIABLEVALUE,
                                                                        )
                                                                ],
                                                            )
                                                        ],
                                                    )
                                                    .is_visible(
                                                        ComponentPropertiesIsVisible::String("true".to_string()),
                                                    ),
                                                ComponentType::GRIDCELL,
                                            ).events(vec![])
                                        ],
                                    ),
                                ComponentGridType::GRID,
                            ).events(vec![])
                        ],
                    )
                    .description(
                        "This is a slightly complicated example app that fetches and displays cat facts".to_string(),
                    )
                    .name("Example Cat Facts Viewer".to_string())
                    .queries(
                        vec![
                            Query::ActionQuery(
                                Box::new(
                                    ActionQuery::new(
                                        Uuid::parse_str(
                                            "92ff0bb8-553b-4f31-87c7-ef5bd16d47d5",
                                        ).expect("invalid UUID"),
                                        "fetchFacts".to_string(),
                                        ActionQueryProperties::new(
                                            ActionQuerySpec::ActionQuerySpecObject(
                                                Box::new(
                                                    ActionQuerySpecObject::new(
                                                        "com.datadoghq.http.request".to_string(),
                                                    )
                                                        .connection_id(
                                                            "5e63f4a8-4ce6-47de-ba11-f6617c1d54f3".to_string(),
                                                        )
                                                        .inputs(
                                                            ActionQuerySpecInputs::ActionQuerySpecInput(
                                                                BTreeMap::from(
                                                                    [
                                                                        ("verb".to_string(), Value::from("GET")),
                                                                        (
                                                                            "url".to_string(),
                                                                            Value::from(
                                                                                "https://catfact.ninja/facts",
                                                                            ),
                                                                        ),
                                                                    ],
                                                                ),
                                                            ),
                                                        ),
                                                ),
                                            ),
                                        ),
                                        ActionQueryType::ACTION,
                                    ).events(vec![]),
                                ),
                            ),
                            Query::StateVariable(
                                Box::new(
                                    StateVariable::new(
                                        Uuid::parse_str(
                                            "afd03c81-4075-4432-8618-ba09d52d2f2d",
                                        ).expect("invalid UUID"),
                                        "pageSize".to_string(),
                                        StateVariableProperties::new().default_value(Value::from("${20}")),
                                        StateVariableType::STATEVARIABLE,
                                    ),
                                ),
                            ),
                            Query::DataTransform(
                                Box::new(
                                    DataTransform::new(
                                        Uuid::parse_str(
                                            "0fb22859-47dc-4137-9e41-7b67d04c525c",
                                        ).expect("invalid UUID"),
                                        "randomFact".to_string(),
                                        DataTransformProperties
                                        ::new().outputs(
                                            r#"${(() => {const facts = fetchFacts.outputs.body.data
return facts[Math.floor(Math.random()*facts.length)]
})()}"#.to_string(),
                                        ),
                                        DataTransformType::DATATRANSFORM,
                                    ),
                                ),
                            )
                        ],
                    )
                    .root_instance_name("grid0".to_string()),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api.create_app(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
