// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes describing a single cost recommendation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationDataAttributes {
    /// Datadog resource key identifying the recommended resource.
    #[serde(rename = "dd_resource_key")]
    pub dd_resource_key: Option<String>,
    /// Estimated daily savings if the recommendation is applied.
    #[serde(rename = "potential_daily_savings")]
    pub potential_daily_savings:
        Option<crate::datadogV2::model::CostRecommendationDataAttributesPotentialDailySavings>,
    /// The kind of recommendation (for example, `terminate` or `rightsize`).
    #[serde(rename = "recommendation_type")]
    pub recommendation_type: Option<String>,
    /// Cloud provider identifier of the resource.
    #[serde(rename = "resource_id")]
    pub resource_id: Option<String>,
    /// Resource type (for example, `aws_ec2_instance`).
    #[serde(rename = "resource_type")]
    pub resource_type: Option<String>,
    /// Tags attached to the recommended resource.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationDataAttributes {
    pub fn new() -> CostRecommendationDataAttributes {
        CostRecommendationDataAttributes {
            dd_resource_key: None,
            potential_daily_savings: None,
            recommendation_type: None,
            resource_id: None,
            resource_type: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dd_resource_key(mut self, value: String) -> Self {
        self.dd_resource_key = Some(value);
        self
    }

    pub fn potential_daily_savings(
        mut self,
        value: crate::datadogV2::model::CostRecommendationDataAttributesPotentialDailySavings,
    ) -> Self {
        self.potential_daily_savings = Some(value);
        self
    }

    pub fn recommendation_type(mut self, value: String) -> Self {
        self.recommendation_type = Some(value);
        self
    }

    pub fn resource_id(mut self, value: String) -> Self {
        self.resource_id = Some(value);
        self
    }

    pub fn resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl Default for CostRecommendationDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CostRecommendationDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationDataAttributesVisitor;
        impl<'a> Visitor<'a> for CostRecommendationDataAttributesVisitor {
            type Value = CostRecommendationDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dd_resource_key: Option<String> = None;
                let mut potential_daily_savings: Option<
                    crate::datadogV2::model::CostRecommendationDataAttributesPotentialDailySavings,
                > = None;
                let mut recommendation_type: Option<String> = None;
                let mut resource_id: Option<String> = None;
                let mut resource_type: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dd_resource_key" => {
                            if v.is_null() {
                                continue;
                            }
                            dd_resource_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "potential_daily_savings" => {
                            if v.is_null() {
                                continue;
                            }
                            potential_daily_savings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recommendation_type" => {
                            if v.is_null() {
                                continue;
                            }
                            recommendation_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CostRecommendationDataAttributes {
                    dd_resource_key,
                    potential_daily_savings,
                    recommendation_type,
                    resource_id,
                    resource_type,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostRecommendationDataAttributesVisitor)
    }
}
