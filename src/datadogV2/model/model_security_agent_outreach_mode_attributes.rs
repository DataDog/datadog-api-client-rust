// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a mode-only policy update.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentOutreachModeAttributes {
    /// The organization's outreach mode. `off` disables user outreach. `manual` only sends outreach an analyst
    /// triggers. `automated` auto-sends automated interviews for rules with outreach enabled, to users whose email
    /// domain is allowed.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::SecurityAgentOutreachMode,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentOutreachModeAttributes {
    pub fn new(
        mode: crate::datadogV2::model::SecurityAgentOutreachMode,
    ) -> SecurityAgentOutreachModeAttributes {
        SecurityAgentOutreachModeAttributes {
            mode,
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

impl<'de> Deserialize<'de> for SecurityAgentOutreachModeAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentOutreachModeAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityAgentOutreachModeAttributesVisitor {
            type Value = SecurityAgentOutreachModeAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mode: Option<crate::datadogV2::model::SecurityAgentOutreachMode> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::SecurityAgentOutreachMode::UnparsedObject(_mode) => {
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
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;

                let content = SecurityAgentOutreachModeAttributes {
                    mode,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentOutreachModeAttributesVisitor)
    }
}
