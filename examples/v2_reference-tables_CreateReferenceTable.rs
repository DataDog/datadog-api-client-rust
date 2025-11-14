// Create reference table returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reference_tables::ReferenceTablesAPI;
use datadog_api_client::datadogV2::model::CreateTableRequest;
use datadog_api_client::datadogV2::model::CreateTableRequestData;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesFileMetadata;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesFileMetadataCloudStorage;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesSchema;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesSchemaFieldsItems;
use datadog_api_client::datadogV2::model::CreateTableRequestDataType;
use datadog_api_client::datadogV2::model::ReferenceTableCreateSourceType;
use datadog_api_client::datadogV2::model::ReferenceTableSchemaFieldType;

#[tokio::main]
async fn main() {
    let body =
        CreateTableRequest
        ::new().data(
            CreateTableRequestData::new(
                CreateTableRequestDataType::REFERENCE_TABLE,
            ).attributes(
                CreateTableRequestDataAttributes::new(
                    CreateTableRequestDataAttributesSchema::new(
                        vec![
                            CreateTableRequestDataAttributesSchemaFieldsItems::new(
                                "field_1".to_string(),
                                ReferenceTableSchemaFieldType::STRING,
                            )
                        ],
                        vec!["field_1".to_string()],
                    ),
                    ReferenceTableCreateSourceType::LOCAL_FILE,
                    "table_1".to_string(),
                )
                    .file_metadata(
                        CreateTableRequestDataAttributesFileMetadata
                        ::CreateTableRequestDataAttributesFileMetadataCloudStorage(
                            Box::new(
                                CreateTableRequestDataAttributesFileMetadataCloudStorage::new(
                                    CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails::new()
                                        .aws_detail(
                                            CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail
                                            ::new(
                                                "123456789000".to_string(),
                                                "example-data-bucket".to_string(),
                                                "reference-tables/users.csv".to_string(),
                                            ),
                                        )
                                        .azure_detail(
                                            CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail
                                            ::new(
                                                "aaaaaaaa-1111-2222-3333-bbbbbbbbbbbb".to_string(),
                                                "reference-data".to_string(),
                                                "examplestorageaccount".to_string(),
                                                "cccccccc-4444-5555-6666-dddddddddddd".to_string(),
                                                "tables/users.csv".to_string(),
                                            ),
                                        )
                                        .gcp_detail(
                                            CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail
                                            ::new(
                                                "data/reference_tables/users.csv".to_string(),
                                                "example-data-bucket".to_string(),
                                                "example-gcp-project-12345".to_string(),
                                                "example-service@example-gcp-project-12345.iam.gserviceaccount.com".to_string(),
                                            ),
                                        ),
                                    false,
                                ),
                            ),
                        ),
                    )
                    .tags(vec!["tag_1".to_string(), "tag_2".to_string()]),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ReferenceTablesAPI::with_config(configuration);
    let resp = api.create_reference_table(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
