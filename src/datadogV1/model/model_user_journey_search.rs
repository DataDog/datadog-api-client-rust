// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// User journey search configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserJourneySearch {
    /// Expression string.
    #[serde(rename = "expression")]
    pub expression: String,
    /// Filters for user journey search.
    #[serde(rename = "filters")]
    pub filters: Option<crate::datadogV1::model::UserJourneySearchFilters>,
    /// Join keys for user journey queries.
    #[serde(rename = "join_keys")]
    pub join_keys: Option<crate::datadogV1::model::UserJourneyJoinKeys>,
    /// Node objects mapping.
    #[serde(rename = "node_objects")]
    pub node_objects:
        std::collections::BTreeMap<String, crate::datadogV1::model::ProductAnalyticsBaseQuery>,
    /// Step aliases mapping.
    #[serde(rename = "step_aliases")]
    pub step_aliases: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserJourneySearch {
    pub fn new(
        expression: String,
        node_objects: std::collections::BTreeMap<
            String,
            crate::datadogV1::model::ProductAnalyticsBaseQuery,
        >,
    ) -> UserJourneySearch {
        UserJourneySearch {
            expression,
            filters: None,
            join_keys: None,
            node_objects,
            step_aliases: None,
            _unparsed: false,
        }
    }

    pub fn filters(mut self, value: crate::datadogV1::model::UserJourneySearchFilters) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn join_keys(mut self, value: crate::datadogV1::model::UserJourneyJoinKeys) -> Self {
        self.join_keys = Some(value);
        self
    }

    pub fn step_aliases(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.step_aliases = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for UserJourneySearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserJourneySearchVisitor;
        impl<'a> Visitor<'a> for UserJourneySearchVisitor {
            type Value = UserJourneySearch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expression: Option<String> = None;
                let mut filters: Option<crate::datadogV1::model::UserJourneySearchFilters> = None;
                let mut join_keys: Option<crate::datadogV1::model::UserJourneyJoinKeys> = None;
                let mut node_objects: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV1::model::ProductAnalyticsBaseQuery,
                    >,
                > = None;
                let mut step_aliases: Option<std::collections::BTreeMap<String, String>> = None;
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
                        "step_aliases" => {
                            if v.is_null() {
                                continue;
                            }
                            step_aliases =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let node_objects =
                    node_objects.ok_or_else(|| M::Error::missing_field("node_objects"))?;

                let content = UserJourneySearch {
                    expression,
                    filters,
                    join_keys,
                    node_objects,
                    step_aliases,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserJourneySearchVisitor)
    }
}
