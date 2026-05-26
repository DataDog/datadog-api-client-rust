// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request body for filtering cost recommendations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecommendationsFilterRequest {
    /// Filter expression applied to the recommendations.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Ordered list of sort clauses applied to the result set.
    #[serde(rename = "sort")]
    pub sort: Option<Vec<crate::datadogV2::model::RecommendationsFilterRequestSortItems>>,
    /// Active view name (for example, `active`, `dismissed`, `open`, `in-progress`, or `completed`).
    #[serde(rename = "view")]
    pub view: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RecommendationsFilterRequest {
    pub fn new() -> RecommendationsFilterRequest {
        RecommendationsFilterRequest {
            filter: None,
            sort: None,
            view: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn sort(
        mut self,
        value: Vec<crate::datadogV2::model::RecommendationsFilterRequestSortItems>,
    ) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn view(mut self, value: String) -> Self {
        self.view = Some(value);
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

impl Default for RecommendationsFilterRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RecommendationsFilterRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RecommendationsFilterRequestVisitor;
        impl<'a> Visitor<'a> for RecommendationsFilterRequestVisitor {
            type Value = RecommendationsFilterRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<String> = None;
                let mut sort: Option<
                    Vec<crate::datadogV2::model::RecommendationsFilterRequestSortItems>,
                > = None;
                let mut view: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view" => {
                            if v.is_null() {
                                continue;
                            }
                            view = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RecommendationsFilterRequest {
                    filter,
                    sort,
                    view,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RecommendationsFilterRequestVisitor)
    }
}
