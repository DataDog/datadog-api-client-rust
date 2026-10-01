// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines how to map a metric lookup value to a CSV column during enrichment table lookups.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineMetricEnrichmentTableFileKey {
    /// The CSV column name or index to match against the lookup value.
    #[serde(rename = "column")]
    pub column: String,
    /// Specifies the source of the key value used for metric enrichment table lookups.
    /// The lookup key can be either the metric name or a metric tag.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableLookupSource,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineMetricEnrichmentTableFileKey {
    pub fn new(
        column: String,
        source: crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableLookupSource,
    ) -> ObservabilityPipelineMetricEnrichmentTableFileKey {
        ObservabilityPipelineMetricEnrichmentTableFileKey {
            column,
            source,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineMetricEnrichmentTableFileKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineMetricEnrichmentTableFileKeyVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineMetricEnrichmentTableFileKeyVisitor {
            type Value = ObservabilityPipelineMetricEnrichmentTableFileKey;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column: Option<String> = None;
                let mut source: Option<
                    crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableLookupSource,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column" => {
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableLookupSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let column = column.ok_or_else(|| M::Error::missing_field("column"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = ObservabilityPipelineMetricEnrichmentTableFileKey {
                    column,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineMetricEnrichmentTableFileKeyVisitor)
    }
}
