// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Facets
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchSLOResponseDataAttributesFacets {
    /// All tags associated with an SLO.
    #[serde(rename = "all_tags")]
    pub all_tags:
        Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Creator of an SLO.
    #[serde(rename = "creator_name")]
    pub creator_name:
        Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Tags with the `env` tag key.
    #[serde(rename = "env_tags")]
    pub env_tags:
        Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Tags with the `service` tag key.
    #[serde(rename = "service_tags")]
    pub service_tags:
        Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Type of SLO.
    #[serde(rename = "slo_type")]
    pub slo_type:
        Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectInt>>,
    /// SLO Target
    #[serde(rename = "target")]
    pub target:
        Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectInt>>,
    /// Tags with the `team` tag key.
    #[serde(rename = "team_tags")]
    pub team_tags:
        Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Timeframes of SLOs.
    #[serde(rename = "timeframe")]
    pub timeframe:
        Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SearchSLOResponseDataAttributesFacets {
    pub fn new() -> SearchSLOResponseDataAttributesFacets {
        SearchSLOResponseDataAttributesFacets {
            all_tags: None,
            creator_name: None,
            env_tags: None,
            service_tags: None,
            slo_type: None,
            target: None,
            team_tags: None,
            timeframe: None,
            _unparsed: false,
        }
    }

    pub fn all_tags(
        mut self,
        value: Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
    ) -> Self {
        self.all_tags = Some(value);
        self
    }

    pub fn creator_name(
        mut self,
        value: Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
    ) -> Self {
        self.creator_name = Some(value);
        self
    }

    pub fn env_tags(
        mut self,
        value: Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
    ) -> Self {
        self.env_tags = Some(value);
        self
    }

    pub fn service_tags(
        mut self,
        value: Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
    ) -> Self {
        self.service_tags = Some(value);
        self
    }

    pub fn slo_type(
        mut self,
        value: Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectInt>,
    ) -> Self {
        self.slo_type = Some(value);
        self
    }

    pub fn target(
        mut self,
        value: Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectInt>,
    ) -> Self {
        self.target = Some(value);
        self
    }

    pub fn team_tags(
        mut self,
        value: Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
    ) -> Self {
        self.team_tags = Some(value);
        self
    }

    pub fn timeframe(
        mut self,
        value: Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
    ) -> Self {
        self.timeframe = Some(value);
        self
    }
}

impl Default for SearchSLOResponseDataAttributesFacets {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SearchSLOResponseDataAttributesFacets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchSLOResponseDataAttributesFacetsVisitor;
        impl<'a> Visitor<'a> for SearchSLOResponseDataAttributesFacetsVisitor {
            type Value = SearchSLOResponseDataAttributesFacets;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut all_tags: Option<
                    Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
                > = None;
                let mut creator_name: Option<
                    Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
                > = None;
                let mut env_tags: Option<
                    Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
                > = None;
                let mut service_tags: Option<
                    Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
                > = None;
                let mut slo_type: Option<
                    Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectInt>,
                > = None;
                let mut target: Option<
                    Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectInt>,
                > = None;
                let mut team_tags: Option<
                    Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
                > = None;
                let mut timeframe: Option<
                    Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "all_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            all_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator_name" => {
                            if v.is_null() {
                                continue;
                            }
                            creator_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            env_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            service_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slo_type" => {
                            if v.is_null() {
                                continue;
                            }
                            slo_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            team_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe" => {
                            if v.is_null() {
                                continue;
                            }
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SearchSLOResponseDataAttributesFacets {
                    all_tags,
                    creator_name,
                    env_tags,
                    service_tags,
                    slo_type,
                    target,
                    team_tags,
                    timeframe,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SearchSLOResponseDataAttributesFacetsVisitor)
    }
}
