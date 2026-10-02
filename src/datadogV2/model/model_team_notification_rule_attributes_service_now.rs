// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// ServiceNow notification settings for the team.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamNotificationRuleAttributesServiceNow {
    /// ServiceNow template handle names to use for notifications.
    #[serde(rename = "templates")]
    pub templates: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamNotificationRuleAttributesServiceNow {
    pub fn new() -> TeamNotificationRuleAttributesServiceNow {
        TeamNotificationRuleAttributesServiceNow {
            templates: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn templates(mut self, value: Vec<String>) -> Self {
        self.templates = Some(value);
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

impl Default for TeamNotificationRuleAttributesServiceNow {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamNotificationRuleAttributesServiceNow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamNotificationRuleAttributesServiceNowVisitor;
        impl<'a> Visitor<'a> for TeamNotificationRuleAttributesServiceNowVisitor {
            type Value = TeamNotificationRuleAttributesServiceNow;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut templates: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "templates" => {
                            if v.is_null() {
                                continue;
                            }
                            templates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamNotificationRuleAttributesServiceNow {
                    templates,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamNotificationRuleAttributesServiceNowVisitor)
    }
}
