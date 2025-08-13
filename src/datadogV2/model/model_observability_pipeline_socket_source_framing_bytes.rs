// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Byte frames are passed through as-is according to the underlying I/O boundaries (for example, split between messages or stream segments).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSocketSourceFramingBytes {
    /// Byte frames are passed through as-is according to the underlying I/O boundaries (for example, split between messages or stream segments).
    #[serde(rename = "method")]
    pub method: crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingBytesMethod,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSocketSourceFramingBytes {
    pub fn new(
        method: crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingBytesMethod,
    ) -> ObservabilityPipelineSocketSourceFramingBytes {
        ObservabilityPipelineSocketSourceFramingBytes {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSocketSourceFramingBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSocketSourceFramingBytesVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSocketSourceFramingBytesVisitor {
            type Value = ObservabilityPipelineSocketSourceFramingBytes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut method: Option<
                    crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingBytesMethod,
                > = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingBytesMethod::UnparsedObject(_method) => {
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

                let content = ObservabilityPipelineSocketSourceFramingBytes {
                    method,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineSocketSourceFramingBytesVisitor)
    }
}
