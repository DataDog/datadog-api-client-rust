// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a static enrichment table loaded from a CSV file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineEnrichmentTableFile {
    /// File encoding format.
    #[serde(rename = "encoding")]
    pub encoding: crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileEncoding,
    /// Key fields used to look up enrichment values.
    #[serde(rename = "key")]
    pub key: Vec<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItems>,
    /// Path to the CSV file.
    #[serde(rename = "path")]
    pub path: String,
    /// Schema defining column names and their types.
    #[serde(rename = "schema")]
    pub schema: Vec<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileSchemaItems>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineEnrichmentTableFile {
    pub fn new(
        encoding: crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileEncoding,
        key: Vec<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItems>,
        path: String,
        schema: Vec<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileSchemaItems>,
    ) -> ObservabilityPipelineEnrichmentTableFile {
        ObservabilityPipelineEnrichmentTableFile {
            encoding,
            key,
            path,
            schema,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineEnrichmentTableFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineEnrichmentTableFileVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineEnrichmentTableFileVisitor {
            type Value = ObservabilityPipelineEnrichmentTableFile;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut encoding: Option<
                    crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileEncoding,
                > = None;
                let mut key: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItems>,
                > = None;
                let mut path: Option<String> = None;
                let mut schema: Option<Vec<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileSchemaItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "encoding" => {
                            encoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let encoding = encoding.ok_or_else(|| M::Error::missing_field("encoding"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let path = path.ok_or_else(|| M::Error::missing_field("path"))?;
                let schema = schema.ok_or_else(|| M::Error::missing_field("schema"))?;

                let content = ObservabilityPipelineEnrichmentTableFile {
                    encoding,
                    key,
                    path,
                    schema,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineEnrichmentTableFileVisitor)
    }
}
