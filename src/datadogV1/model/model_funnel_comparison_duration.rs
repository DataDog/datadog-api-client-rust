// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Comparison time configuration for funnel widgets.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunnelComparisonDuration {
    /// Custom timeframe for funnel comparison.
    #[serde(rename = "custom_timeframe")]
    pub custom_timeframe: Option<crate::datadogV1::model::FunnelComparisonCustomTimeframe>,
    /// Type of comparison duration.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::FunnelComparisonDurationType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FunnelComparisonDuration {
    pub fn new(
        type_: crate::datadogV1::model::FunnelComparisonDurationType,
    ) -> FunnelComparisonDuration {
        FunnelComparisonDuration {
            custom_timeframe: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn custom_timeframe(
        mut self,
        value: crate::datadogV1::model::FunnelComparisonCustomTimeframe,
    ) -> Self {
        self.custom_timeframe = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for FunnelComparisonDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FunnelComparisonDurationVisitor;
        impl<'a> Visitor<'a> for FunnelComparisonDurationVisitor {
            type Value = FunnelComparisonDuration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_timeframe: Option<
                    crate::datadogV1::model::FunnelComparisonCustomTimeframe,
                > = None;
                let mut type_: Option<crate::datadogV1::model::FunnelComparisonDurationType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_timeframe" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_timeframe =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::FunnelComparisonDurationType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = FunnelComparisonDuration {
                    custom_timeframe,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FunnelComparisonDurationVisitor)
    }
}
