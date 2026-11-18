// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A feature detected in the application, with the journeys users take to use it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsFunnelSuggestionsFeature {
    /// The suggested journey that users follow to use the feature. Contains one journey.
    #[serde(rename = "core_user_journeys")]
    pub core_user_journeys: Option<Vec<crate::datadogV2::model::ProductAnalyticsCoreUserJourney>>,
    /// Description of the feature.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// ID of the feature.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of the feature.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Relevance score of the feature. Higher is more relevant.
    #[serde(rename = "score")]
    pub score: Option<i64>,
    /// Number of sessions that used the feature.
    #[serde(rename = "support")]
    pub support: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsFunnelSuggestionsFeature {
    pub fn new() -> ProductAnalyticsFunnelSuggestionsFeature {
        ProductAnalyticsFunnelSuggestionsFeature {
            core_user_journeys: None,
            description: None,
            id: None,
            name: None,
            score: None,
            support: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn core_user_journeys(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsCoreUserJourney>,
    ) -> Self {
        self.core_user_journeys = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn score(mut self, value: i64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn support(mut self, value: i64) -> Self {
        self.support = Some(value);
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

impl Default for ProductAnalyticsFunnelSuggestionsFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsFunnelSuggestionsFeature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsFunnelSuggestionsFeatureVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsFunnelSuggestionsFeatureVisitor {
            type Value = ProductAnalyticsFunnelSuggestionsFeature;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut core_user_journeys: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsCoreUserJourney>,
                > = None;
                let mut description: Option<String> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut score: Option<i64> = None;
                let mut support: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "core_user_journeys" => {
                            if v.is_null() {
                                continue;
                            }
                            core_user_journeys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "score" => {
                            if v.is_null() {
                                continue;
                            }
                            score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "support" => {
                            if v.is_null() {
                                continue;
                            }
                            support = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsFunnelSuggestionsFeature {
                    core_user_journeys,
                    description,
                    id,
                    name,
                    score,
                    support,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsFunnelSuggestionsFeatureVisitor)
    }
}
