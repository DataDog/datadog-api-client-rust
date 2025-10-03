// Create reference table with upload returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reference_tables::ReferenceTablesAPI;
use datadog_api_client::datadogV2::model::CreateTableRequest;
use datadog_api_client::datadogV2::model::CreateTableRequestData;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesFileMetadata;
use datadog_api_client::datadogV2::model::CreateTableRequestDataAttributesFileMetadataLocalFile;
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
                                "id".to_string(),
                                ReferenceTableSchemaFieldType::STRING,
                            ),
                            CreateTableRequestDataAttributesSchemaFieldsItems::new(
                                "name".to_string(),
                                ReferenceTableSchemaFieldType::STRING,
                            ),
                            CreateTableRequestDataAttributesSchemaFieldsItems::new(
                                "value".to_string(),
                                ReferenceTableSchemaFieldType::INT32,
                            )
                        ],
                        vec!["id".to_string()],
                    ),
                    ReferenceTableCreateSourceType::LOCAL_FILE,
                    "test_reference_table_Example-Reference-Table".to_string(),
                )
                    .description("Test reference table created via BDD test Example-Reference-Table".to_string())
                    .file_metadata(
                        CreateTableRequestDataAttributesFileMetadata
                        ::CreateTableRequestDataAttributesFileMetadataLocalFile(
                            Box::new(
                                CreateTableRequestDataAttributesFileMetadataLocalFile::new(
                                    "test-upload-id-Example-Reference-Table".to_string(),
                                ),
                            ),
                        ),
                    )
                    .tags(vec!["test_tag".to_string()]),
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
