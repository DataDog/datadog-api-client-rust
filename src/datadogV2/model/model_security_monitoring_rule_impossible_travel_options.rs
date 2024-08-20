// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options on impossible travel rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleImpossibleTravelOptions {
    /// If true, signals are suppressed for the first 24 hours. In that time, Datadog learns the user's regular
    /// access locations. This can be helpful to reduce noise and infer VPN usage or credentialed API access.
    #[serde(rename = "baselineUserLocations")]
    pub baseline_user_locations: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleImpossibleTravelOptions {
    pub fn new() -> SecurityMonitoringRuleImpossibleTravelOptions {
        SecurityMonitoringRuleImpossibleTravelOptions {
            baseline_user_locations: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn baseline_user_locations(mut self, value: bool) -> Self {
        self.baseline_user_locations = Some(value);
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

impl Default for SecurityMonitoringRuleImpossibleTravelOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleImpossibleTravelOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleImpossibleTravelOptionsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleImpossibleTravelOptionsVisitor {
            type Value = SecurityMonitoringRuleImpossibleTravelOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut baseline_user_locations: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "baselineUserLocations" => {
                            if v.is_null() {
                                continue;
                            }
                            baseline_user_locations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleImpossibleTravelOptions {
                    baseline_user_locations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleImpossibleTravelOptionsVisitor)
    }
}
