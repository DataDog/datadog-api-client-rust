// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Mute properties to apply to the findings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MuteFindingsMuteAttributes {
    /// Additional information about the reason why the findings are muted or unmuted. This field has a limit of 280 characters.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The expiration date of the mute action (Unix ms). It must be set to a value greater than the current timestamp. If this field is not provided, the findings remain muted indefinitely.
    #[serde(rename = "expire_at")]
    pub expire_at: Option<i64>,
    /// Whether the findings should be muted or unmuted.
    #[serde(rename = "is_muted")]
    pub is_muted: bool,
    /// The reason why the findings are muted or unmuted.
    #[serde(rename = "reason")]
    pub reason: crate::datadogV2::model::MuteFindingsReason,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MuteFindingsMuteAttributes {
    pub fn new(
        is_muted: bool,
        reason: crate::datadogV2::model::MuteFindingsReason,
    ) -> MuteFindingsMuteAttributes {
        MuteFindingsMuteAttributes {
            description: None,
            expire_at: None,
            is_muted,
            reason,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn expire_at(mut self, value: i64) -> Self {
        self.expire_at = Some(value);
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

impl<'de> Deserialize<'de> for MuteFindingsMuteAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MuteFindingsMuteAttributesVisitor;
        impl<'a> Visitor<'a> for MuteFindingsMuteAttributesVisitor {
            type Value = MuteFindingsMuteAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut expire_at: Option<i64> = None;
                let mut is_muted: Option<bool> = None;
                let mut reason: Option<crate::datadogV2::model::MuteFindingsReason> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expire_at" => {
                            if v.is_null() {
                                continue;
                            }
                            expire_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_muted" => {
                            is_muted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reason) = reason {
                                match _reason {
                                    crate::datadogV2::model::MuteFindingsReason::UnparsedObject(
                                        _reason,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let is_muted = is_muted.ok_or_else(|| M::Error::missing_field("is_muted"))?;
                let reason = reason.ok_or_else(|| M::Error::missing_field("reason"))?;

                let content = MuteFindingsMuteAttributes {
                    description,
                    expire_at,
                    is_muted,
                    reason,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MuteFindingsMuteAttributesVisitor)
    }
}
