// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Each log event is separated using the specified delimiter character.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSocketDestinationFramingCharacterDelimited {
    /// A single ASCII character used as a delimiter.
    #[serde(rename = "delimiter")]
    pub delimiter: String,
    /// The definition of `ObservabilityPipelineSocketDestinationFramingCharacterDelimitedMethod` object.
    #[serde(rename = "method")]
    pub method: crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingCharacterDelimitedMethod,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSocketDestinationFramingCharacterDelimited {
    pub fn new(
        delimiter: String,
        method: crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingCharacterDelimitedMethod,
    ) -> ObservabilityPipelineSocketDestinationFramingCharacterDelimited {
        ObservabilityPipelineSocketDestinationFramingCharacterDelimited {
            delimiter,
            method,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSocketDestinationFramingCharacterDelimited {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSocketDestinationFramingCharacterDelimitedVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSocketDestinationFramingCharacterDelimitedVisitor {
            type Value = ObservabilityPipelineSocketDestinationFramingCharacterDelimited;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut delimiter: Option<String> = None;
                let mut method: Option<crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingCharacterDelimitedMethod> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "delimiter" => {
                            delimiter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "method" => {
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _method) = method {
                                match _method {
                                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingCharacterDelimitedMethod::UnparsedObject(_method) => {
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
                let delimiter = delimiter.ok_or_else(|| M::Error::missing_field("delimiter"))?;
                let method = method.ok_or_else(|| M::Error::missing_field("method"))?;

                let content = ObservabilityPipelineSocketDestinationFramingCharacterDelimited {
                    delimiter,
                    method,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineSocketDestinationFramingCharacterDelimitedVisitor)
    }
}
