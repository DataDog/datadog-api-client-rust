// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ObservabilityPipelineEnrichmentTableFileKeyItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineEnrichmentTableFileKeyItems {
    /// The `items` `column`.
    #[serde(rename = "column")]
    pub column: String,
    /// The definition of `ObservabilityPipelineEnrichmentTableFileKeyItemsComparison` object.
    #[serde(rename = "comparison")]
    pub comparison:
        crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItemsComparison,
    /// The `items` `field`.
    #[serde(rename = "field")]
    pub field: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineEnrichmentTableFileKeyItems {
    pub fn new(
        column: String,
        comparison: crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItemsComparison,
        field: String,
    ) -> ObservabilityPipelineEnrichmentTableFileKeyItems {
        ObservabilityPipelineEnrichmentTableFileKeyItems {
            column,
            comparison,
            field,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineEnrichmentTableFileKeyItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineEnrichmentTableFileKeyItemsVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineEnrichmentTableFileKeyItemsVisitor {
            type Value = ObservabilityPipelineEnrichmentTableFileKeyItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column: Option<String> = None;
                let mut comparison: Option<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItemsComparison> = None;
                let mut field: Option<String> = None;
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
                        "comparison" => {
                            comparison = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _comparison) = comparison {
                                match _comparison {
                                    crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItemsComparison::UnparsedObject(_comparison) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "field" => {
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let column = column.ok_or_else(|| M::Error::missing_field("column"))?;
                let comparison = comparison.ok_or_else(|| M::Error::missing_field("comparison"))?;
                let field = field.ok_or_else(|| M::Error::missing_field("field"))?;

                let content = ObservabilityPipelineEnrichmentTableFileKeyItems {
                    column,
                    comparison,
                    field,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineEnrichmentTableFileKeyItemsVisitor)
    }
}
