/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsArchiveIntegrationAzure : The Azure archive's integration destination.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogsArchiveIntegrationAzure {
    /// A client ID.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// A tenant ID.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
}

impl LogsArchiveIntegrationAzure {
    /// The Azure archive's integration destination.
    pub fn new(client_id: String, tenant_id: String) -> LogsArchiveIntegrationAzure {
        LogsArchiveIntegrationAzure {
            client_id,
            tenant_id,
        }
    }
}


