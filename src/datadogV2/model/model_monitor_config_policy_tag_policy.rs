// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Tag attributes of a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorConfigPolicyTagPolicy {
    /// The key of the tag.
    #[serde(rename = "tag_key")]
    pub tag_key: Option<String>,
    /// If a tag key is required for monitor creation.
    #[serde(rename = "tag_key_required")]
    pub tag_key_required: Option<bool>,
    /// Valid values for the tag.
    #[serde(rename = "valid_tag_values")]
    pub valid_tag_values: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorConfigPolicyTagPolicy {
    pub fn new() -> MonitorConfigPolicyTagPolicy {
        MonitorConfigPolicyTagPolicy {
            tag_key: None,
            tag_key_required: None,
            valid_tag_values: None,
            _unparsed: false,
        }
    }

    pub fn tag_key(mut self, value: String) -> Self {
        self.tag_key = Some(value);
        self
    }

    pub fn tag_key_required(mut self, value: bool) -> Self {
        self.tag_key_required = Some(value);
        self
    }

    pub fn valid_tag_values(mut self, value: Vec<String>) -> Self {
        self.valid_tag_values = Some(value);
        self
    }
}

impl Default for MonitorConfigPolicyTagPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorConfigPolicyTagPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorConfigPolicyTagPolicyVisitor;
        impl<'a> Visitor<'a> for MonitorConfigPolicyTagPolicyVisitor {
            type Value = MonitorConfigPolicyTagPolicy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tag_key: Option<String> = None;
                let mut tag_key_required: Option<bool> = None;
                let mut valid_tag_values: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tag_key" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_key_required" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_key_required =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid_tag_values" => {
                            if v.is_null() {
                                continue;
                            }
                            valid_tag_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorConfigPolicyTagPolicy {
                    tag_key,
                    tag_key_required,
                    valid_tag_values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorConfigPolicyTagPolicyVisitor)
    }
}
