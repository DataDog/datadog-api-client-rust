// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about the mute status of this finding.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FindingMute {
    /// Additional information about the reason why this finding is muted or unmuted.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The expiration date of the mute or unmute action (Unix ms).
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<i64>,
    /// Whether this finding is muted or unmuted.
    #[serde(rename = "muted")]
    pub muted: Option<bool>,
    /// The reason why this finding is muted or unmuted.
    #[serde(rename = "reason")]
    pub reason: Option<crate::datadogV2::model::FindingMuteReason>,
    /// The start of the mute period.
    #[serde(rename = "start_date")]
    pub start_date: Option<i64>,
    /// The ID of the user who muted or unmuted this finding.
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FindingMute {
    pub fn new() -> FindingMute {
        FindingMute {
            description: None,
            expiration_date: None,
            muted: None,
            reason: None,
            start_date: None,
            uuid: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: i64) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn muted(mut self, value: bool) -> Self {
        self.muted = Some(value);
        self
    }

    pub fn reason(mut self, value: crate::datadogV2::model::FindingMuteReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn start_date(mut self, value: i64) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
}

impl Default for FindingMute {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FindingMute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FindingMuteVisitor;
        impl<'a> Visitor<'a> for FindingMuteVisitor {
            type Value = FindingMute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut expiration_date: Option<i64> = None;
                let mut muted: Option<bool> = None;
                let mut reason: Option<crate::datadogV2::model::FindingMuteReason> = None;
                let mut start_date: Option<i64> = None;
                let mut uuid: Option<String> = None;
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
                        "expiration_date" => {
                            if v.is_null() {
                                continue;
                            }
                            expiration_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "muted" => {
                            if v.is_null() {
                                continue;
                            }
                            muted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reason) = reason {
                                match _reason {
                                    crate::datadogV2::model::FindingMuteReason::UnparsedObject(
                                        _reason,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            _unparsed = true;
                        }
                    }
                }

                let content = FindingMute {
                    description,
                    expiration_date,
                    muted,
                    reason,
                    start_date,
                    uuid,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FindingMuteVisitor)
    }
}
