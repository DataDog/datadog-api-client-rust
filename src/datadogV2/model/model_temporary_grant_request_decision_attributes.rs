// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Binary decision. Overrides, reason fields, and unknown attributes are rejected, including null values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemporaryGrantRequestDecisionAttributes {
    /// Approve the exact submitted total and expiry, or deny the request.
    #[serde(rename = "decision")]
    pub decision: crate::datadogV2::model::TemporaryGrantRequestDecision,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TemporaryGrantRequestDecisionAttributes {
    pub fn new(
        decision: crate::datadogV2::model::TemporaryGrantRequestDecision,
    ) -> TemporaryGrantRequestDecisionAttributes {
        TemporaryGrantRequestDecisionAttributes {
            decision,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for TemporaryGrantRequestDecisionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporaryGrantRequestDecisionAttributesVisitor;
        impl<'a> Visitor<'a> for TemporaryGrantRequestDecisionAttributesVisitor {
            type Value = TemporaryGrantRequestDecisionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut decision: Option<crate::datadogV2::model::TemporaryGrantRequestDecision> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "decision" => {
                            decision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _decision) = decision {
                                match _decision {
                                    crate::datadogV2::model::TemporaryGrantRequestDecision::UnparsedObject(_decision) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let decision = decision.ok_or_else(|| M::Error::missing_field("decision"))?;

                let content = TemporaryGrantRequestDecisionAttributes {
                    decision,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TemporaryGrantRequestDecisionAttributesVisitor)
    }
}
