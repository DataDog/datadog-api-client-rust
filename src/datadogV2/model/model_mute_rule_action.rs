// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The action to take when the mute rule matches a finding.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MuteRuleAction {
    /// The Unix timestamp in milliseconds at which the mute expires. If omitted, the mute does not expire.
    #[serde(rename = "expire_at")]
    pub expire_at: Option<i64>,
    /// The reason for muting a security finding.
    #[serde(rename = "reason")]
    pub reason: crate::datadogV2::model::MuteReason,
    /// An optional description providing more context for the mute reason.
    #[serde(rename = "reason_description")]
    pub reason_description: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MuteRuleAction {
    pub fn new(reason: crate::datadogV2::model::MuteReason) -> MuteRuleAction {
        MuteRuleAction {
            expire_at: None,
            reason,
            reason_description: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn expire_at(mut self, value: i64) -> Self {
        self.expire_at = Some(value);
        self
    }

    pub fn reason_description(mut self, value: String) -> Self {
        self.reason_description = Some(value);
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

impl<'de> Deserialize<'de> for MuteRuleAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MuteRuleActionVisitor;
        impl<'a> Visitor<'a> for MuteRuleActionVisitor {
            type Value = MuteRuleAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expire_at: Option<i64> = None;
                let mut reason: Option<crate::datadogV2::model::MuteReason> = None;
                let mut reason_description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expire_at" => {
                            if v.is_null() {
                                continue;
                            }
                            expire_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reason) = reason {
                                match _reason {
                                    crate::datadogV2::model::MuteReason::UnparsedObject(
                                        _reason,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "reason_description" => {
                            if v.is_null() {
                                continue;
                            }
                            reason_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let reason = reason.ok_or_else(|| M::Error::missing_field("reason"))?;

                let content = MuteRuleAction {
                    expire_at,
                    reason,
                    reason_description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MuteRuleActionVisitor)
    }
}
