// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Tracing configuration for the Vercel integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct VercelTraceConfig {
    /// Whether tracing is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Whether the configuration uses the deprecated OpenTelemetry tracing setup.
    #[serde(rename = "isDeprecatedOtel")]
    pub is_deprecated_otel: bool,
    /// Percentage of traces to forward to Datadog, between 0 and 100.
    #[serde(rename = "samplingPercentage")]
    pub sampling_percentage: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl VercelTraceConfig {
    pub fn new(
        enabled: bool,
        is_deprecated_otel: bool,
        sampling_percentage: i32,
    ) -> VercelTraceConfig {
        VercelTraceConfig {
            enabled,
            is_deprecated_otel,
            sampling_percentage,
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

impl<'de> Deserialize<'de> for VercelTraceConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VercelTraceConfigVisitor;
        impl<'a> Visitor<'a> for VercelTraceConfigVisitor {
            type Value = VercelTraceConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut is_deprecated_otel: Option<bool> = None;
                let mut sampling_percentage: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isDeprecatedOtel" => {
                            is_deprecated_otel =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "samplingPercentage" => {
                            sampling_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let is_deprecated_otel = is_deprecated_otel
                    .ok_or_else(|| M::Error::missing_field("is_deprecated_otel"))?;
                let sampling_percentage = sampling_percentage
                    .ok_or_else(|| M::Error::missing_field("sampling_percentage"))?;

                let content = VercelTraceConfig {
                    enabled,
                    is_deprecated_otel,
                    sampling_percentage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(VercelTraceConfigVisitor)
    }
}
