// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunnelRequestDataAttributesSearch {
    #[serde(rename = "cross_session_filter")]
    pub cross_session_filter: Option<String>,
    #[serde(rename = "query_string")]
    pub query_string: Option<String>,
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::FunnelRequestDataAttributesSearchStepsItems>>,
    #[serde(rename = "subquery_id")]
    pub subquery_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FunnelRequestDataAttributesSearch {
    pub fn new() -> FunnelRequestDataAttributesSearch {
        FunnelRequestDataAttributesSearch {
            cross_session_filter: None,
            query_string: None,
            steps: None,
            subquery_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cross_session_filter(mut self, value: String) -> Self {
        self.cross_session_filter = Some(value);
        self
    }

    pub fn query_string(mut self, value: String) -> Self {
        self.query_string = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: Vec<crate::datadogV2::model::FunnelRequestDataAttributesSearchStepsItems>,
    ) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn subquery_id(mut self, value: String) -> Self {
        self.subquery_id = Some(value);
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

impl Default for FunnelRequestDataAttributesSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FunnelRequestDataAttributesSearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FunnelRequestDataAttributesSearchVisitor;
        impl<'a> Visitor<'a> for FunnelRequestDataAttributesSearchVisitor {
            type Value = FunnelRequestDataAttributesSearch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cross_session_filter: Option<String> = None;
                let mut query_string: Option<String> = None;
                let mut steps: Option<
                    Vec<crate::datadogV2::model::FunnelRequestDataAttributesSearchStepsItems>,
                > = None;
                let mut subquery_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cross_session_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            cross_session_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_string" => {
                            if v.is_null() {
                                continue;
                            }
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subquery_id" => {
                            if v.is_null() {
                                continue;
                            }
                            subquery_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FunnelRequestDataAttributesSearch {
                    cross_session_filter,
                    query_string,
                    steps,
                    subquery_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FunnelRequestDataAttributesSearchVisitor)
    }
}
