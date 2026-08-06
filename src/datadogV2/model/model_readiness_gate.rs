// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Used to merge multiple branches into a single branch.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReadinessGate {
    /// The definition of `ReadinessGateThresholdType` object.
    #[serde(rename = "thresholdType")]
    pub threshold_type: crate::datadogV2::model::ReadinessGateThresholdType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReadinessGate {
    pub fn new(
        threshold_type: crate::datadogV2::model::ReadinessGateThresholdType,
    ) -> ReadinessGate {
        ReadinessGate {
            threshold_type,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for ReadinessGate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReadinessGateVisitor;
        impl<'a> Visitor<'a> for ReadinessGateVisitor {
            type Value = ReadinessGate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut threshold_type: Option<
                    crate::datadogV2::model::ReadinessGateThresholdType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "thresholdType" => {
                            threshold_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _threshold_type) = threshold_type {
                                match _threshold_type {
                                    crate::datadogV2::model::ReadinessGateThresholdType::UnparsedObject(_threshold_type) => {
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
                let threshold_type =
                    threshold_type.ok_or_else(|| M::Error::missing_field("threshold_type"))?;

                let content = ReadinessGate {
                    threshold_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReadinessGateVisitor)
    }
}
