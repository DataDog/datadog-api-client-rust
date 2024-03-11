// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Parameters for the TOTP/MFA variable
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsGlobalVariableTOTPParameters {
    /// Number of digits for the OTP code.
    #[serde(rename = "digits")]
    pub digits: Option<i32>,
    /// Interval for which to refresh the token (in seconds).
    #[serde(rename = "refresh_interval")]
    pub refresh_interval: Option<i32>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsGlobalVariableTOTPParameters {
    pub fn new() -> SyntheticsGlobalVariableTOTPParameters {
        SyntheticsGlobalVariableTOTPParameters {
            digits: None,
            refresh_interval: None,
            _unparsed: false,
        }
    }

    pub fn digits(&mut self, value: i32) -> &mut Self {
        self.digits = Some(value);
        self
    }

    pub fn refresh_interval(&mut self, value: i32) -> &mut Self {
        self.refresh_interval = Some(value);
        self
    }
}

impl Default for SyntheticsGlobalVariableTOTPParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsGlobalVariableTOTPParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsGlobalVariableTOTPParametersVisitor;
        impl<'a> Visitor<'a> for SyntheticsGlobalVariableTOTPParametersVisitor {
            type Value = SyntheticsGlobalVariableTOTPParameters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut digits: Option<i32> = None;
                let mut refresh_interval: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "digits" => {
                            if v.is_null() {
                                continue;
                            }
                            digits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "refresh_interval" => {
                            if v.is_null() {
                                continue;
                            }
                            refresh_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsGlobalVariableTOTPParameters {
                    digits,
                    refresh_interval,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsGlobalVariableTOTPParametersVisitor)
    }
}
