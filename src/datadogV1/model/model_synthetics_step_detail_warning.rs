// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object collecting warnings for a given step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsStepDetailWarning {
    /// Message for the warning.
    #[serde(rename = "message")]
    pub message: String,
    /// User locator used.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsWarningType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsStepDetailWarning {
    pub fn new(
        message: String,
        type_: crate::datadogV1::model::SyntheticsWarningType,
    ) -> SyntheticsStepDetailWarning {
        SyntheticsStepDetailWarning {
            message,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsStepDetailWarning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsStepDetailWarningVisitor;
        impl<'a> Visitor<'a> for SyntheticsStepDetailWarningVisitor {
            type Value = SyntheticsStepDetailWarning;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut message: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsWarningType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsWarningType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsStepDetailWarning {
                    message,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsStepDetailWarningVisitor)
    }
}
