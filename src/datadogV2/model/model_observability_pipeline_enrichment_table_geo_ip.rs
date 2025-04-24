// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Uses a GeoIP database to enrich logs based on an IP field.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineEnrichmentTableGeoIp {
    /// Path to the IP field in the log.
    #[serde(rename = "key_field")]
    pub key_field: String,
    /// Locale used to resolve geographical names.
    #[serde(rename = "locale")]
    pub locale: String,
    /// Path to the GeoIP database file.
    #[serde(rename = "path")]
    pub path: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineEnrichmentTableGeoIp {
    pub fn new(
        key_field: String,
        locale: String,
        path: String,
    ) -> ObservabilityPipelineEnrichmentTableGeoIp {
        ObservabilityPipelineEnrichmentTableGeoIp {
            key_field,
            locale,
            path,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineEnrichmentTableGeoIp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineEnrichmentTableGeoIpVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineEnrichmentTableGeoIpVisitor {
            type Value = ObservabilityPipelineEnrichmentTableGeoIp;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut key_field: Option<String> = None;
                let mut locale: Option<String> = None;
                let mut path: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "key_field" => {
                            key_field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locale" => {
                            locale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let key_field = key_field.ok_or_else(|| M::Error::missing_field("key_field"))?;
                let locale = locale.ok_or_else(|| M::Error::missing_field("locale"))?;
                let path = path.ok_or_else(|| M::Error::missing_field("path"))?;

                let content = ObservabilityPipelineEnrichmentTableGeoIp {
                    key_field,
                    locale,
                    path,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineEnrichmentTableGeoIpVisitor)
    }
}
