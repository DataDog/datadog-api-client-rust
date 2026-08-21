// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the steps of the journey and the filters applied to it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneySearch {
    /// Expression combining the node aliases in order, for example `A -> B -> C`.
    #[serde(rename = "expression")]
    pub expression: String,
    /// Filters applied on top of the journey step expression.
    #[serde(rename = "filters")]
    pub filters: Option<crate::datadogV2::model::ProductAnalyticsJourneySearchFilters>,
    /// Identity join keys used to stitch events belonging to the same user or session.
    #[serde(rename = "join_keys")]
    pub join_keys: Option<crate::datadogV2::model::ProductAnalyticsJoinKeys>,
    /// Map of node alias to the query matching that step of the journey.
    /// Every alias used in `expression` must have an entry here.
    #[serde(rename = "node_objects")]
    pub node_objects:
        std::collections::BTreeMap<String, crate::datadogV2::model::ProductAnalyticsBaseQuery>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneySearch {
    pub fn new(
        expression: String,
        node_objects: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::ProductAnalyticsBaseQuery,
        >,
    ) -> ProductAnalyticsJourneySearch {
        ProductAnalyticsJourneySearch {
            expression,
            filters: None,
            join_keys: None,
            node_objects,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filters(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsJourneySearchFilters,
    ) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn join_keys(mut self, value: crate::datadogV2::model::ProductAnalyticsJoinKeys) -> Self {
        self.join_keys = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneySearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneySearchVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneySearchVisitor {
            type Value = ProductAnalyticsJourneySearch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expression: Option<String> = None;
                let mut filters: Option<
                    crate::datadogV2::model::ProductAnalyticsJourneySearchFilters,
                > = None;
                let mut join_keys: Option<crate::datadogV2::model::ProductAnalyticsJoinKeys> = None;
                let mut node_objects: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::ProductAnalyticsBaseQuery,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expression" => {
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            join_keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "node_objects" => {
                            node_objects =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let node_objects =
                    node_objects.ok_or_else(|| M::Error::missing_field("node_objects"))?;

                let content = ProductAnalyticsJourneySearch {
                    expression,
                    filters,
                    join_keys,
                    node_objects,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneySearchVisitor)
    }
}
