// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A change indicator that compares the current value to a historical period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QueryValueWidgetComparison {
    /// Color-coding direction: `increase_better` (green on rise), `decrease_better` (green on drop), or `neutral` (no color).
    #[serde(rename = "directionality")]
    pub directionality: Option<crate::datadogV1::model::QueryValueWidgetComparisonDirectionality>,
    /// The comparison period. Use a preset `type` value or set `type` to `custom_timeframe` and provide `custom_timeframe` with explicit millisecond epoch bounds.
    #[serde(rename = "duration")]
    pub duration: crate::datadogV1::model::ComparisonDuration,
    /// How the delta is expressed: `absolute` (raw difference), `relative` (percentage), or `both`.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::QueryValueWidgetComparisonType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl QueryValueWidgetComparison {
    pub fn new(
        duration: crate::datadogV1::model::ComparisonDuration,
    ) -> QueryValueWidgetComparison {
        QueryValueWidgetComparison {
            directionality: None,
            duration,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn directionality(
        mut self,
        value: crate::datadogV1::model::QueryValueWidgetComparisonDirectionality,
    ) -> Self {
        self.directionality = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::QueryValueWidgetComparisonType) -> Self {
        self.type_ = Some(value);
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

impl<'de> Deserialize<'de> for QueryValueWidgetComparison {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryValueWidgetComparisonVisitor;
        impl<'a> Visitor<'a> for QueryValueWidgetComparisonVisitor {
            type Value = QueryValueWidgetComparison;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut directionality: Option<
                    crate::datadogV1::model::QueryValueWidgetComparisonDirectionality,
                > = None;
                let mut duration: Option<crate::datadogV1::model::ComparisonDuration> = None;
                let mut type_: Option<crate::datadogV1::model::QueryValueWidgetComparisonType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "directionality" => {
                            if v.is_null() {
                                continue;
                            }
                            directionality =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _directionality) = directionality {
                                match _directionality {
                                    crate::datadogV1::model::QueryValueWidgetComparisonDirectionality::UnparsedObject(_directionality) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "duration" => {
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::QueryValueWidgetComparisonType::UnparsedObject(_type_) => {
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
                let duration = duration.ok_or_else(|| M::Error::missing_field("duration"))?;

                let content = QueryValueWidgetComparison {
                    directionality,
                    duration,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryValueWidgetComparisonVisitor)
    }
}
