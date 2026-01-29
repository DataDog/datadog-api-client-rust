// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Auto-close inactive cases settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AutoCloseInactiveCases {
    /// Whether auto-close is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Maximum inactive time in seconds before auto-closing
    #[serde(rename = "max_inactive_time_in_secs")]
    pub max_inactive_time_in_secs: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AutoCloseInactiveCases {
    pub fn new() -> AutoCloseInactiveCases {
        AutoCloseInactiveCases {
            enabled: None,
            max_inactive_time_in_secs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn max_inactive_time_in_secs(mut self, value: i64) -> Self {
        self.max_inactive_time_in_secs = Some(value);
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

impl Default for AutoCloseInactiveCases {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AutoCloseInactiveCases {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AutoCloseInactiveCasesVisitor;
        impl<'a> Visitor<'a> for AutoCloseInactiveCasesVisitor {
            type Value = AutoCloseInactiveCases;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut max_inactive_time_in_secs: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_inactive_time_in_secs" => {
                            if v.is_null() {
                                continue;
                            }
                            max_inactive_time_in_secs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AutoCloseInactiveCases {
                    enabled,
                    max_inactive_time_in_secs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AutoCloseInactiveCasesVisitor)
    }
}
