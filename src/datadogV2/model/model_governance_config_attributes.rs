// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a Governance Console configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceConfigAttributes {
    /// Whether notifications are sent to users when detections are assigned to them.
    #[serde(rename = "assignment_notifications_enabled")]
    pub assignment_notifications_enabled: bool,
    /// Whether the Governance Console is enabled for the organization.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Whether usage attribution is configured for the organization.
    #[serde(rename = "usage_attribution_configured")]
    pub usage_attribution_configured: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceConfigAttributes {
    pub fn new(
        assignment_notifications_enabled: bool,
        enabled: bool,
        usage_attribution_configured: bool,
    ) -> GovernanceConfigAttributes {
        GovernanceConfigAttributes {
            assignment_notifications_enabled,
            enabled,
            usage_attribution_configured,
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

impl<'de> Deserialize<'de> for GovernanceConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceConfigAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceConfigAttributesVisitor {
            type Value = GovernanceConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignment_notifications_enabled: Option<bool> = None;
                let mut enabled: Option<bool> = None;
                let mut usage_attribution_configured: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignment_notifications_enabled" => {
                            assignment_notifications_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_attribution_configured" => {
                            usage_attribution_configured =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let assignment_notifications_enabled = assignment_notifications_enabled
                    .ok_or_else(|| M::Error::missing_field("assignment_notifications_enabled"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let usage_attribution_configured = usage_attribution_configured
                    .ok_or_else(|| M::Error::missing_field("usage_attribution_configured"))?;

                let content = GovernanceConfigAttributes {
                    assignment_notifications_enabled,
                    enabled,
                    usage_attribution_configured,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceConfigAttributesVisitor)
    }
}
