// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metadata about a service definition.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionMeta {
    /// GitHub HTML URL.
    #[serde(rename = "github-html-url")]
    pub github_html_url: Option<String>,
    /// Ingestion schema version.
    #[serde(rename = "ingested-schema-version")]
    pub ingested_schema_version: Option<String>,
    /// Ingestion source of the service definition.
    #[serde(rename = "ingestion-source")]
    pub ingestion_source: Option<String>,
    /// Last modified time of the service definition.
    #[serde(rename = "last-modified-time")]
    pub last_modified_time: Option<String>,
    /// User defined origin of the service definition.
    #[serde(rename = "origin")]
    pub origin: Option<String>,
    /// User defined origin's detail of the service definition.
    #[serde(rename = "origin-detail")]
    pub origin_detail: Option<String>,
    /// A list of schema validation warnings.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::ServiceDefinitionMetaWarnings>>,
}

impl ServiceDefinitionMeta {
    pub fn new() -> ServiceDefinitionMeta {
        ServiceDefinitionMeta {
            github_html_url: None,
            ingested_schema_version: None,
            ingestion_source: None,
            last_modified_time: None,
            origin: None,
            origin_detail: None,
            warnings: None,
        }
    }
}
impl Default for ServiceDefinitionMeta {
    fn default() -> Self {
        Self::new()
    }
}
