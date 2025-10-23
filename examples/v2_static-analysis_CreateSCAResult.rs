// Post dependencies for analysis returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::ScaRequest;
use datadog_api_client::datadogV2::model::ScaRequestData;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributes;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesCommit;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesDependenciesItems;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItems;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesDependenciesItemsReachableSymbolPropertiesItems;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesFilesItems;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesRelationsItems;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesRepository;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesVulnerabilitiesItems;
use datadog_api_client::datadogV2::model::ScaRequestDataAttributesVulnerabilitiesItemsAffectsItems;
use datadog_api_client::datadogV2::model::ScaRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        ScaRequest
        ::new().data(
            ScaRequestData::new(
                ScaRequestDataType::SCAREQUESTS,
            ).attributes(
                ScaRequestDataAttributes::new()
                    .commit(ScaRequestDataAttributesCommit::new())
                    .dependencies(
                        vec![
                            ScaRequestDataAttributesDependenciesItems::new()
                                .exclusions(vec![])
                                .locations(
                                    vec![
                                        ScaRequestDataAttributesDependenciesItemsLocationsItems::new()
                                            .block(
                                                ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition
                                                ::new()
                                                    .end(
                                                        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition
                                                        ::new(),
                                                    )
                                                    .start(
                                                        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition
                                                        ::new(),
                                                    ),
                                            )
                                            .name(
                                                ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition
                                                ::new()
                                                    .end(
                                                        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition
                                                        ::new(),
                                                    )
                                                    .start(
                                                        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition
                                                        ::new(),
                                                    ),
                                            )
                                            .namespace(
                                                ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition
                                                ::new()
                                                    .end(
                                                        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition
                                                        ::new(),
                                                    )
                                                    .start(
                                                        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition
                                                        ::new(),
                                                    ),
                                            )
                                            .version(
                                                ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition
                                                ::new()
                                                    .end(
                                                        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition
                                                        ::new(),
                                                    )
                                                    .start(
                                                        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition
                                                        ::new(),
                                                    ),
                                            )
                                    ],
                                )
                                .reachable_symbol_properties(
                                    vec![
                                        ScaRequestDataAttributesDependenciesItemsReachableSymbolPropertiesItems::new()
                                    ],
                                )
                        ],
                    )
                    .files(vec![ScaRequestDataAttributesFilesItems::new()])
                    .relations(vec![ScaRequestDataAttributesRelationsItems::new().depends_on(vec![])])
                    .repository(ScaRequestDataAttributesRepository::new())
                    .vulnerabilities(
                        vec![
                            ScaRequestDataAttributesVulnerabilitiesItems
                            ::new().affects(vec![ScaRequestDataAttributesVulnerabilitiesItemsAffectsItems::new()])
                        ],
                    ),
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateSCAResult", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api.create_sca_result(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
