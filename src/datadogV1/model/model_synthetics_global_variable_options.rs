// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options for the Global Variable for MFA.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsGlobalVariableOptions {
    /// Parameters for the TOTP/MFA variable
    #[serde(rename = "totp_parameters")]
    pub totp_parameters: Option<crate::datadogV1::model::SyntheticsGlobalVariableTOTPParameters>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsGlobalVariableOptions {
    pub fn new() -> SyntheticsGlobalVariableOptions {
        SyntheticsGlobalVariableOptions {
            totp_parameters: None,
            _unparsed: false,
        }
    }

    pub fn totp_parameters(
        mut self,
        value: crate::datadogV1::model::SyntheticsGlobalVariableTOTPParameters,
    ) -> Self {
        self.totp_parameters = Some(value);
        self
    }
}

impl Default for SyntheticsGlobalVariableOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsGlobalVariableOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsGlobalVariableOptionsVisitor;
        impl<'a> Visitor<'a> for SyntheticsGlobalVariableOptionsVisitor {
            type Value = SyntheticsGlobalVariableOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut totp_parameters: Option<
                    crate::datadogV1::model::SyntheticsGlobalVariableTOTPParameters,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "totp_parameters" => {
                            if v.is_null() {
                                continue;
                            }
                            totp_parameters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsGlobalVariableOptions {
                    totp_parameters,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsGlobalVariableOptionsVisitor)
    }
}
