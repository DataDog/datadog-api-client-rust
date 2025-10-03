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
                                "name".to_string(),
                                ReferenceTableSchemaFieldType::STRING,
                            ),
                            CreateTableRequestDataAttributesSchemaFieldsItems::new(
                                "account_id".to_string(),
                                ReferenceTableSchemaFieldType::STRING,
                            )
                        ],
                        vec!["account_id".to_string()],
                    ),
                    ReferenceTableCreateSourceType::S3,
                    "test_reference_table".to_string(),
                )
                    .description("this is a cloud table generated via a cloud bucket sync".to_string())
                    .file_metadata(
                        CreateTableRequestDataAttributesFileMetadata
                        ::CreateTableRequestDataAttributesFileMetadataCloudStorage(
                            Box::new(
                                CreateTableRequestDataAttributesFileMetadataCloudStorage::new(
                                    CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails
                                    ::new().aws_detail(
                                        CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail::new(
                                            "test-account-id".to_string(),
                                            "test-bucket".to_string(),
                                            "test_rt.csv".to_string(),
                                        ),
                                    ),
                                    true,
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
