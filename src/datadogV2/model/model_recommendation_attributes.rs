// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the SPA Recommendation resource. Contains recommendations for both driver and executor components.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecommendationAttributes {
    #[serde(rename = "confidence_level")]
    pub confidence_level: Option<f64>,
    /// Resource recommendation for a single Spark component (driver or executor). Contains estimation data used to patch Spark job specs.
    #[serde(rename = "driver")]
    pub driver: crate::datadogV2::model::ComponentRecommendation,
    /// Resource recommendation for a single Spark component (driver or executor). Contains estimation data used to patch Spark job specs.
    #[serde(rename = "executor")]
    pub executor: crate::datadogV2::model::ComponentRecommendation,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RecommendationAttributes {
    pub fn new(
        driver: crate::datadogV2::model::ComponentRecommendation,
        executor: crate::datadogV2::model::ComponentRecommendation,
    ) -> RecommendationAttributes {
        RecommendationAttributes {
            confidence_level: None,
            driver,
            executor,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn confidence_level(mut self, value: f64) -> Self {
        self.confidence_level = Some(value);
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

impl<'de> Deserialize<'de> for RecommendationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RecommendationAttributesVisitor;
        impl<'a> Visitor<'a> for RecommendationAttributesVisitor {
            type Value = RecommendationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_level: Option<f64> = None;
                let mut driver: Option<crate::datadogV2::model::ComponentRecommendation> = None;
                let mut executor: Option<crate::datadogV2::model::ComponentRecommendation> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confidence_level" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "driver" => {
                            driver = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "executor" => {
                            executor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let driver = driver.ok_or_else(|| M::Error::missing_field("driver"))?;
                let executor = executor.ok_or_else(|| M::Error::missing_field("executor"))?;

                let content = RecommendationAttributes {
                    confidence_level,
                    driver,
                    executor,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RecommendationAttributesVisitor)
    }
}
