// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Tag attributes of a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorConfigPolicyTagPolicyCreateRequest {
    /// The key of the tag.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// If a tag key is required for monitor creation.
    #[serde(rename = "tag_key_required")]
    pub tag_key_required: bool,
    /// Valid values for the tag.
    #[serde(rename = "valid_tag_values")]
    pub valid_tag_values: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorConfigPolicyTagPolicyCreateRequest {
    pub fn new(
        tag_key: String,
        tag_key_required: bool,
        valid_tag_values: Vec<String>,
    ) -> MonitorConfigPolicyTagPolicyCreateRequest {
        MonitorConfigPolicyTagPolicyCreateRequest {
            tag_key,
            tag_key_required,
            valid_tag_values,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MonitorConfigPolicyTagPolicyCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorConfigPolicyTagPolicyCreateRequestVisitor;
        impl<'a> Visitor<'a> for MonitorConfigPolicyTagPolicyCreateRequestVisitor {
            type Value = MonitorConfigPolicyTagPolicyCreateRequest;

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
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_key_required" => {
                            tag_key_required =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid_tag_values" => {
                            valid_tag_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let tag_key_required =
                    tag_key_required.ok_or_else(|| M::Error::missing_field("tag_key_required"))?;
                let valid_tag_values =
                    valid_tag_values.ok_or_else(|| M::Error::missing_field("valid_tag_values"))?;

                let content = MonitorConfigPolicyTagPolicyCreateRequest {
                    tag_key,
                    tag_key_required,
                    valid_tag_values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorConfigPolicyTagPolicyCreateRequestVisitor)
    }
}
