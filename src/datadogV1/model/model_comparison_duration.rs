// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The comparison period. Use a preset `type` value or set `type` to `custom_timeframe` and provide `custom_timeframe` with explicit millisecond epoch bounds.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ComparisonDuration {
    /// Fixed time range for a `custom_timeframe` comparison.
    #[serde(rename = "custom_timeframe")]
    pub custom_timeframe: Option<crate::datadogV1::model::ComparisonCustomTimeframe>,
    /// The comparison window type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ComparisonDurationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ComparisonDuration {
    pub fn new(type_: crate::datadogV1::model::ComparisonDurationType) -> ComparisonDuration {
        ComparisonDuration {
            custom_timeframe: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_timeframe(
        mut self,
        value: crate::datadogV1::model::ComparisonCustomTimeframe,
    ) -> Self {
        self.custom_timeframe = Some(value);
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

impl<'de> Deserialize<'de> for ComparisonDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ComparisonDurationVisitor;
        impl<'a> Visitor<'a> for ComparisonDurationVisitor {
            type Value = ComparisonDuration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_timeframe: Option<
                    crate::datadogV1::model::ComparisonCustomTimeframe,
                > = None;
                let mut type_: Option<crate::datadogV1::model::ComparisonDurationType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                                    crate::datadogV1::model::ComparisonDurationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ComparisonDuration {
                    custom_timeframe,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ComparisonDurationVisitor)
    }
}
