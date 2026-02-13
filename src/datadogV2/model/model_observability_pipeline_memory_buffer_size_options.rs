// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options for configuring a memory buffer by queue length.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineMemoryBufferSizeOptions {
    /// Maximum events for the memory buffer.
    #[serde(rename = "max_events")]
    pub max_events: i64,
    /// The type of the buffer that will be configured, a memory buffer.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptionsMemoryType>,
    /// Behavior when the buffer is full (block and stop accepting new events, or drop new events)
    #[serde(rename = "when_full")]
    pub when_full: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptionsWhenFull>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineMemoryBufferSizeOptions {
    pub fn new(max_events: i64) -> ObservabilityPipelineMemoryBufferSizeOptions {
        ObservabilityPipelineMemoryBufferSizeOptions {
            max_events,
            type_: None,
            when_full: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptionsMemoryType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn when_full(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptionsWhenFull,
    ) -> Self {
        self.when_full = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineMemoryBufferSizeOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineMemoryBufferSizeOptionsVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineMemoryBufferSizeOptionsVisitor {
            type Value = ObservabilityPipelineMemoryBufferSizeOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max_events: Option<i64> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptionsMemoryType,
                > = None;
                let mut when_full: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptionsWhenFull,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max_events" => {
                            max_events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineBufferOptionsMemoryType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "when_full" => {
                            if v.is_null() {
                                continue;
                            }
                            when_full = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _when_full) = when_full {
                                match _when_full {
                                    crate::datadogV2::model::ObservabilityPipelineBufferOptionsWhenFull::UnparsedObject(_when_full) => {
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
                let max_events = max_events.ok_or_else(|| M::Error::missing_field("max_events"))?;

                let content = ObservabilityPipelineMemoryBufferSizeOptions {
                    max_events,
                    type_,
                    when_full,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineMemoryBufferSizeOptionsVisitor)
    }
}
