// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Each log event is delimited by a newline character.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSocketDestinationFramingNewlineDelimited {
    /// The definition of `ObservabilityPipelineSocketDestinationFramingNewlineDelimitedMethod` object.
    #[serde(rename = "method")]
    pub method: crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingNewlineDelimitedMethod,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSocketDestinationFramingNewlineDelimited {
    pub fn new(
        method: crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingNewlineDelimitedMethod,
    ) -> ObservabilityPipelineSocketDestinationFramingNewlineDelimited {
        ObservabilityPipelineSocketDestinationFramingNewlineDelimited {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSocketDestinationFramingNewlineDelimited {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSocketDestinationFramingNewlineDelimitedVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSocketDestinationFramingNewlineDelimitedVisitor {
            type Value = ObservabilityPipelineSocketDestinationFramingNewlineDelimited;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut method: Option<crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingNewlineDelimitedMethod> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "method" => {
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _method) = method {
                                match _method {
                                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingNewlineDelimitedMethod::UnparsedObject(_method) => {
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
                let method = method.ok_or_else(|| M::Error::missing_field("method"))?;

                let content = ObservabilityPipelineSocketDestinationFramingNewlineDelimited {
                    method,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineSocketDestinationFramingNewlineDelimitedVisitor)
    }
}
