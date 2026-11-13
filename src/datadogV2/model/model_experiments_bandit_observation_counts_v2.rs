// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// How many observations the latest saved refresh saw, whatever the range. Null when its counts can't be read.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditObservationCountsV2 {
    /// Observations the refresh couldn't use.
    #[serde(rename = "invalid")]
    pub invalid: i64,
    /// Every observation. The sum of the other three.
    #[serde(rename = "total")]
    pub total: i64,
    /// Valid observations whose aggregation window is still open.
    #[serde(rename = "valid_immature")]
    pub valid_immature: i64,
    /// Valid observations whose aggregation window has closed. The estimates use these.
    #[serde(rename = "valid_mature")]
    pub valid_mature: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditObservationCountsV2 {
    pub fn new(
        invalid: i64,
        total: i64,
        valid_immature: i64,
        valid_mature: i64,
    ) -> ExperimentsBanditObservationCountsV2 {
        ExperimentsBanditObservationCountsV2 {
            invalid,
            total,
            valid_immature,
            valid_mature,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ExperimentsBanditObservationCountsV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditObservationCountsV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditObservationCountsV2Visitor {
            type Value = ExperimentsBanditObservationCountsV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut invalid: Option<i64> = None;
                let mut total: Option<i64> = None;
                let mut valid_immature: Option<i64> = None;
                let mut valid_mature: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "invalid" => {
                            invalid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid_immature" => {
                            valid_immature =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid_mature" => {
                            valid_mature =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let invalid = invalid.ok_or_else(|| M::Error::missing_field("invalid"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;
                let valid_immature =
                    valid_immature.ok_or_else(|| M::Error::missing_field("valid_immature"))?;
                let valid_mature =
                    valid_mature.ok_or_else(|| M::Error::missing_field("valid_mature"))?;

                let content = ExperimentsBanditObservationCountsV2 {
                    invalid,
                    total,
                    valid_immature,
                    valid_mature,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditObservationCountsV2Visitor)
    }
}
