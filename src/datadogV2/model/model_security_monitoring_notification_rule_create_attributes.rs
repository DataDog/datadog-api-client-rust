// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a notification rule to be created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringNotificationRuleCreateAttributes {
    /// Whether the notification rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The name of the notification rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Selectors describing the notification rule.
    #[serde(rename = "selectors")]
    pub selectors: crate::datadogV2::model::SecurityMonitoringNotificationRuleSelectors,
    /// Set of targets to notify.
    #[serde(rename = "targets")]
    pub targets: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringNotificationRuleCreateAttributes {
    pub fn new(
        enabled: bool,
        name: String,
        selectors: crate::datadogV2::model::SecurityMonitoringNotificationRuleSelectors,
        targets: Vec<String>,
    ) -> SecurityMonitoringNotificationRuleCreateAttributes {
        SecurityMonitoringNotificationRuleCreateAttributes {
            enabled,
            name,
            selectors,
            targets,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringNotificationRuleCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringNotificationRuleCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringNotificationRuleCreateAttributesVisitor {
            type Value = SecurityMonitoringNotificationRuleCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut selectors: Option<
                    crate::datadogV2::model::SecurityMonitoringNotificationRuleSelectors,
                > = None;
                let mut targets: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selectors" => {
                            selectors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targets" => {
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let selectors = selectors.ok_or_else(|| M::Error::missing_field("selectors"))?;
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;

                let content = SecurityMonitoringNotificationRuleCreateAttributes {
                    enabled,
                    name,
                    selectors,
                    targets,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringNotificationRuleCreateAttributesVisitor)
    }
}
