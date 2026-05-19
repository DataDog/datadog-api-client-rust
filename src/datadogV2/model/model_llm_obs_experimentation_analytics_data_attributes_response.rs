// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an analytics response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentationAnalyticsDataAttributesResponse {
    /// Total number of events matched by the query before grouping.
    #[serde(rename = "hit_count")]
    pub hit_count: i64,
    /// Analytics query result containing all buckets.
    #[serde(rename = "result")]
    pub result: crate::datadogV2::model::LLMObsExperimentationAnalyticsResult,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentationAnalyticsDataAttributesResponse {
    pub fn new(
        hit_count: i64,
        result: crate::datadogV2::model::LLMObsExperimentationAnalyticsResult,
    ) -> LLMObsExperimentationAnalyticsDataAttributesResponse {
        LLMObsExperimentationAnalyticsDataAttributesResponse {
            hit_count,
            result,
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

impl<'de> Deserialize<'de> for LLMObsExperimentationAnalyticsDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentationAnalyticsDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentationAnalyticsDataAttributesResponseVisitor {
            type Value = LLMObsExperimentationAnalyticsDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hit_count: Option<i64> = None;
                let mut result: Option<
                    crate::datadogV2::model::LLMObsExperimentationAnalyticsResult,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hit_count" => {
                            hit_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result" => {
                            result = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let hit_count = hit_count.ok_or_else(|| M::Error::missing_field("hit_count"))?;
                let result = result.ok_or_else(|| M::Error::missing_field("result"))?;

                let content = LLMObsExperimentationAnalyticsDataAttributesResponse {
                    hit_count,
                    result,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentationAnalyticsDataAttributesResponseVisitor)
    }
}
