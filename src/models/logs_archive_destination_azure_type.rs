/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsArchiveDestinationAzureType : Type of the Azure archive destination.

/// Type of the Azure archive destination.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsArchiveDestinationAzureType {
    #[serde(rename = "azure")]
    AZURE,

}

impl ToString for LogsArchiveDestinationAzureType {
    fn to_string(&self) -> String {
        match self {
            Self::AZURE => String::from("azure"),
        }
    }
}

impl Default for LogsArchiveDestinationAzureType {
    fn default() -> LogsArchiveDestinationAzureType {
        Self::AZURE
    }
}




