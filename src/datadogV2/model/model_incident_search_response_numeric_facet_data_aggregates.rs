// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregate information for numeric incident data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentSearchResponseNumericFacetDataAggregates {
    /// Maximum value of the numeric aggregates.
    #[serde(rename = "max", default, with = "::serde_with::rust::double_option")]
    pub max: Option<Option<f64>>,
    /// Minimum value of the numeric aggregates.
    #[serde(rename = "min", default, with = "::serde_with::rust::double_option")]
    pub min: Option<Option<f64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentSearchResponseNumericFacetDataAggregates {
    pub fn new() -> IncidentSearchResponseNumericFacetDataAggregates {
        IncidentSearchResponseNumericFacetDataAggregates {
            max: None,
            min: None,
            _unparsed: false,
        }
    }

    pub fn max(mut self, value: Option<f64>) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(mut self, value: Option<f64>) -> Self {
        self.min = Some(value);
        self
    }
}

impl Default for IncidentSearchResponseNumericFacetDataAggregates {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentSearchResponseNumericFacetDataAggregates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentSearchResponseNumericFacetDataAggregatesVisitor;
        impl<'a> Visitor<'a> for IncidentSearchResponseNumericFacetDataAggregatesVisitor {
            type Value = IncidentSearchResponseNumericFacetDataAggregates;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max: Option<Option<f64>> = None;
                let mut min: Option<Option<f64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max" => {
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min" => {
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IncidentSearchResponseNumericFacetDataAggregates {
                    max,
                    min,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentSearchResponseNumericFacetDataAggregatesVisitor)
    }
}
