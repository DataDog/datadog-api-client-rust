// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Operation that finds an object in a `source` array using a `filter`, and then extracts a specific value into the `target` attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArrayProcessorOperationSelect {
    /// Filter condition expressed as `key:value` used to find the matching element.
    #[serde(rename = "filter")]
    pub filter: String,
    /// Attribute path of the array to search into.
    #[serde(rename = "source")]
    pub source: String,
    /// Attribute that receives the extracted value.
    #[serde(rename = "target")]
    pub target: String,
    /// Operation type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsArrayProcessorOperationSelectType,
    /// Key of the value to extract from the matching element.
    #[serde(rename = "value_to_extract")]
    pub value_to_extract: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArrayProcessorOperationSelect {
    pub fn new(
        filter: String,
        source: String,
        target: String,
        type_: crate::datadogV1::model::LogsArrayProcessorOperationSelectType,
        value_to_extract: String,
    ) -> LogsArrayProcessorOperationSelect {
        LogsArrayProcessorOperationSelect {
            filter,
            source,
            target,
            type_,
            value_to_extract,
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

impl<'de> Deserialize<'de> for LogsArrayProcessorOperationSelect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArrayProcessorOperationSelectVisitor;
        impl<'a> Visitor<'a> for LogsArrayProcessorOperationSelectVisitor {
            type Value = LogsArrayProcessorOperationSelect;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<String> = None;
                let mut source: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV1::model::LogsArrayProcessorOperationSelectType,
                > = None;
                let mut value_to_extract: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsArrayProcessorOperationSelectType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value_to_extract" => {
                            value_to_extract =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let filter = filter.ok_or_else(|| M::Error::missing_field("filter"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let value_to_extract =
                    value_to_extract.ok_or_else(|| M::Error::missing_field("value_to_extract"))?;

                let content = LogsArrayProcessorOperationSelect {
                    filter,
                    source,
                    target,
                    type_,
                    value_to_extract,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArrayProcessorOperationSelectVisitor)
    }
}
