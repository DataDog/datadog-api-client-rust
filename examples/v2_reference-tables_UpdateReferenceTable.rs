// Update reference table returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_reference_tables::ReferenceTablesAPI;
use datadog_api_client::datadogV2::model::PatchTableRequest;
use datadog_api_client::datadogV2::model::PatchTableRequestData;
use datadog_api_client::datadogV2::model::PatchTableRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchTableRequestDataAttributesFileMetadata;
use datadog_api_client::datadogV2::model::PatchTableRequestDataAttributesFileMetadataCloudStorage;
use datadog_api_client::datadogV2::model::PatchTableRequestDataAttributesFileMetadataOneOfAccessDetails;
use datadog_api_client::datadogV2::model::PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail;
use datadog_api_client::datadogV2::model::PatchTableRequestDataAttributesSchema;
use datadog_api_client::datadogV2::model::PatchTableRequestDataAttributesSchemaFieldsItems;
use datadog_api_client::datadogV2::model::PatchTableRequestDataType;
use datadog_api_client::datadogV2::model::ReferenceTableSchemaFieldType;

#[tokio::main]
async fn main() {
    let body =
        PatchTableRequest
        ::new().data(
            PatchTableRequestData::new(PatchTableRequestDataType::REFERENCE_TABLE)
                .attributes(
                    PatchTableRequestDataAttributes::new()
                        .description("this is a cloud table generated via a cloud bucket sync".to_string())
                        .file_metadata(
                            PatchTableRequestDataAttributesFileMetadata
                            ::PatchTableRequestDataAttributesFileMetadataCloudStorage(
                                Box::new(
                                    PatchTableRequestDataAttributesFileMetadataCloudStorage::new()
                                        .access_details(
                                            PatchTableRequestDataAttributesFileMetadataOneOfAccessDetails
                                            ::new().aws_detail(
                                                PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail
                                                ::new()
                                                    .aws_account_id("test-account-id".to_string())
                                                    .aws_bucket_name("test-bucket".to_string())
                                                    .file_path("test_rt.csv".to_string()),
                                            ),
                                        )
                                        .sync_enabled(true),
                                ),
                            ),
                        )
                        .schema(
                            PatchTableRequestDataAttributesSchema::new(
                                vec![
                                    PatchTableRequestDataAttributesSchemaFieldsItems::new(
                                        "id".to_string(),
                                        ReferenceTableSchemaFieldType::INT32,
                                    ),
                                    PatchTableRequestDataAttributesSchemaFieldsItems::new(
                                        "name".to_string(),
                                        ReferenceTableSchemaFieldType::STRING,
                                    )
                                ],
                                vec!["id".to_string()],
                            ),
                        )
                        .sync_enabled(false)
                        .tags(vec!["test_tag".to_string()]),
                )
                .id("00000000-0000-0000-0000-000000000000".to_string()),
        );
    let configuration = datadog::Configuration::new();
    let api = ReferenceTablesAPI::with_config(configuration);
    let resp = api.update_reference_table("id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
