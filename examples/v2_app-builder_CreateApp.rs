// Create App returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
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

#[tokio::main]
async fn main() {
    let body = CreateAppRequest::new().data(
        CreateAppRequestData::new(AppDefinitionType::APPDEFINITIONS).attributes(
            CreateAppRequestDataAttributes::new()
                .components(vec![ComponentGrid::new(
                    "grid0".to_string(),
                    ComponentGridProperties::new().children(vec![Component::new(
                        "gridCell0".to_string(),
                        ComponentProperties::new()
                            .children(vec![Component::new(
                                "calloutValue0".to_string(),
                                ComponentProperties::new()
                                    .is_visible(ComponentPropertiesIsVisible::Bool(true)),
                                ComponentType::CALLOUTVALUE,
                            )
                            .events(vec![])])
                            .is_visible(ComponentPropertiesIsVisible::String("true".to_string())),
                        ComponentType::GRIDCELL,
                    )
                    .events(vec![])]),
                    ComponentGridType::GRID,
                )
                .events(vec![])])
                .description("This is a simple example app".to_string())
                .name("Example App".to_string())
                .queries(vec![])
                .root_instance_name("grid0".to_string()),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateApp", true);
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api.create_app(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
