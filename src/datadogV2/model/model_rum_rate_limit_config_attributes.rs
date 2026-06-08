// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The RUM rate limit configuration properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRateLimitConfigAttributes {
    /// The configuration used when `mode` is `adaptive`.
    #[serde(rename = "adaptive")]
    pub adaptive: Option<crate::datadogV2::model::RumRateLimitAdaptiveConfig>,
    /// The configuration used when `mode` is `custom`.
    #[serde(rename = "custom")]
    pub custom: Option<crate::datadogV2::model::RumRateLimitCustomConfig>,
    /// The rate limit mode. `custom` enforces a fixed session limit, while
    /// `adaptive` dynamically adjusts retention.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::RumRateLimitMode,
    /// The ID of the organization the rate limit configuration belongs to.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// The date the rate limit configuration was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// The handle of the user who last updated the rate limit configuration.
    #[serde(rename = "updated_by")]
    pub updated_by: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRateLimitConfigAttributes {
    pub fn new(
        mode: crate::datadogV2::model::RumRateLimitMode,
        org_id: i64,
    ) -> RumRateLimitConfigAttributes {
        RumRateLimitConfigAttributes {
            adaptive: None,
            custom: None,
            mode,
            org_id,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn adaptive(mut self, value: crate::datadogV2::model::RumRateLimitAdaptiveConfig) -> Self {
        self.adaptive = Some(value);
        self
    }

    pub fn custom(mut self, value: crate::datadogV2::model::RumRateLimitCustomConfig) -> Self {
        self.custom = Some(value);
        self
    }

    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(mut self, value: String) -> Self {
        self.updated_by = Some(value);
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

impl<'de> Deserialize<'de> for RumRateLimitConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRateLimitConfigAttributesVisitor;
        impl<'a> Visitor<'a> for RumRateLimitConfigAttributesVisitor {
            type Value = RumRateLimitConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut adaptive: Option<crate::datadogV2::model::RumRateLimitAdaptiveConfig> =
                    None;
                let mut custom: Option<crate::datadogV2::model::RumRateLimitCustomConfig> = None;
                let mut mode: Option<crate::datadogV2::model::RumRateLimitMode> = None;
                let mut org_id: Option<i64> = None;
                let mut updated_at: Option<String> = None;
                let mut updated_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "adaptive" => {
                            if v.is_null() {
                                continue;
                            }
                            adaptive = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom" => {
                            if v.is_null() {
                                continue;
                            }
                            custom = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::RumRateLimitMode::UnparsedObject(
                                        _mode,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;

                let content = RumRateLimitConfigAttributes {
                    adaptive,
                    custom,
                    mode,
                    org_id,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRateLimitConfigAttributesVisitor)
    }
}
