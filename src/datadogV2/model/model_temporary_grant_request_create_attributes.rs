// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Immutable terms and justification for a temporary grant request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemporaryGrantRequestCreateAttributes {
    /// Exclusive expiry. Must be a future timestamp with a timezone; date-only values are rejected. Omitted or null defaults to the next UTC month boundary. Approval accepts this deadline unchanged.
    #[serde(
        rename = "expires_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub expires_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Reason for requesting temporary credits. Trimmed and limited to 4096 UTF-8 bytes.
    #[serde(rename = "justification")]
    pub justification: String,
    /// Target monthly AI-credit limit in positive whole credits, not additional credits. Fractional credits are not accepted.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TemporaryGrantRequestCreateAttributes {
    pub fn new(justification: String, total: i64) -> TemporaryGrantRequestCreateAttributes {
        TemporaryGrantRequestCreateAttributes {
            expires_at: None,
            justification,
            total,
            _unparsed: false,
        }
    }

    pub fn expires_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.expires_at = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TemporaryGrantRequestCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporaryGrantRequestCreateAttributesVisitor;
        impl<'a> Visitor<'a> for TemporaryGrantRequestCreateAttributesVisitor {
            type Value = TemporaryGrantRequestCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expires_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut justification: Option<String> = None;
                let mut total: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expires_at" => {
                            expires_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "justification" => {
                            justification =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let justification =
                    justification.ok_or_else(|| M::Error::missing_field("justification"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = TemporaryGrantRequestCreateAttributes {
                    expires_at,
                    justification,
                    total,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TemporaryGrantRequestCreateAttributesVisitor)
    }
}
