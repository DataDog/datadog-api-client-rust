// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionMeta {
    /// GitHub HTML URL.
    #[serde(rename = "github-html-url", skip_serializing_if = "Option::is_none")]
    pub github_html_url: String,
    /// Ingestion schema version.
    #[serde(rename = "ingested-schema-version", skip_serializing_if = "Option::is_none")]
    pub ingested_schema_version: String,
    /// Ingestion source of the service definition.
    #[serde(rename = "ingestion-source", skip_serializing_if = "Option::is_none")]
    pub ingestion_source: String,
    /// Last modified time of the service definition.
    #[serde(rename = "last-modified-time", skip_serializing_if = "Option::is_none")]
    pub last_modified_time: String,
    /// User defined origin of the service definition.
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: String,
    /// User defined origin's detail of the service definition.
    #[serde(rename = "origin-detail", skip_serializing_if = "Option::is_none")]
    pub origin_detail: String,
    /// A list of schema validation warnings.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Vec<ServiceDefinitionMetaWarnings>,
}

