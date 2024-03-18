// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about a service definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn github_html_url(mut self, value: String) -> Self {
        self.github_html_url = Some(value);
        self
    }

    pub fn ingested_schema_version(mut self, value: String) -> Self {
        self.ingested_schema_version = Some(value);
        self
    }

    pub fn ingestion_source(mut self, value: String) -> Self {
        self.ingestion_source = Some(value);
        self
    }

    pub fn last_modified_time(mut self, value: String) -> Self {
        self.last_modified_time = Some(value);
        self
    }

    pub fn origin(mut self, value: String) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn origin_detail(mut self, value: String) -> Self {
        self.origin_detail = Some(value);
        self
    }

    pub fn warnings(
        mut self,
        value: Vec<crate::datadogV2::model::ServiceDefinitionMetaWarnings>,
    ) -> Self {
        self.warnings = Some(value);
        self
    }
}

impl Default for ServiceDefinitionMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionMetaVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionMetaVisitor {
            type Value = ServiceDefinitionMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut github_html_url: Option<String> = None;
                let mut ingested_schema_version: Option<String> = None;
                let mut ingestion_source: Option<String> = None;
                let mut last_modified_time: Option<String> = None;
                let mut origin: Option<String> = None;
                let mut origin_detail: Option<String> = None;
                let mut warnings: Option<
                    Vec<crate::datadogV2::model::ServiceDefinitionMetaWarnings>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "github-html-url" => {
                            if v.is_null() {
                                continue;
                            }
                            github_html_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingested-schema-version" => {
                            if v.is_null() {
                                continue;
                            }
                            ingested_schema_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingestion-source" => {
                            if v.is_null() {
                                continue;
                            }
                            ingestion_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last-modified-time" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "origin" => {
                            if v.is_null() {
                                continue;
                            }
                            origin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "origin-detail" => {
                            if v.is_null() {
                                continue;
                            }
                            origin_detail =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warnings" => {
                            if v.is_null() {
                                continue;
                            }
                            warnings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ServiceDefinitionMeta {
                    github_html_url,
                    ingested_schema_version,
                    ingestion_source,
                    last_modified_time,
                    origin,
                    origin_detail,
                    warnings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionMetaVisitor)
    }
}
