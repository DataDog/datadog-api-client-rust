// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the new mute properties of the findings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BulkMuteFindingsRequestProperties {
    /// Additional information about the reason why those findings are muted or unmuted. This field has a maximum limit of 280 characters.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The expiration date of the mute or unmute action (Unix ms). It must be set to a value greater than the current timestamp.
    /// If this field is not provided, the finding will be muted or unmuted indefinitely, which is equivalent to setting the expiration date to 9999999999999.
    ///
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<i64>,
    /// Whether those findings should be muted or unmuted.
    #[serde(rename = "muted")]
    pub muted: bool,
    /// The reason why this finding is muted or unmuted.
    #[serde(rename = "reason")]
    pub reason: crate::datadogV2::model::FindingMuteReason,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BulkMuteFindingsRequestProperties {
    pub fn new(
        muted: bool,
        reason: crate::datadogV2::model::FindingMuteReason,
    ) -> BulkMuteFindingsRequestProperties {
        BulkMuteFindingsRequestProperties {
            description: None,
            expiration_date: None,
            muted,
            reason,
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
}

impl<'de> Deserialize<'de> for BulkMuteFindingsRequestProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BulkMuteFindingsRequestPropertiesVisitor;
        impl<'a> Visitor<'a> for BulkMuteFindingsRequestPropertiesVisitor {
            type Value = BulkMuteFindingsRequestProperties;

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
                            muted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
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
                        &_ => {
                            _unparsed = true;
                        }
                    }
                }
                let muted = muted.ok_or_else(|| M::Error::missing_field("muted"))?;
                let reason = reason.ok_or_else(|| M::Error::missing_field("reason"))?;

                let content = BulkMuteFindingsRequestProperties {
                    description,
                    expiration_date,
                    muted,
                    reason,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BulkMuteFindingsRequestPropertiesVisitor)
    }
}
