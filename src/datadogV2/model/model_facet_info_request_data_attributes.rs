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
pub struct FacetInfoRequestDataAttributes {
    #[serde(rename = "facet_id")]
    pub facet_id: String,
    #[serde(rename = "limit")]
    pub limit: i64,
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV2::model::FacetInfoRequestDataAttributesSearch>,
    #[serde(rename = "term_search")]
    pub term_search: Option<crate::datadogV2::model::FacetInfoRequestDataAttributesTermSearch>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FacetInfoRequestDataAttributes {
    pub fn new(facet_id: String, limit: i64) -> FacetInfoRequestDataAttributes {
        FacetInfoRequestDataAttributes {
            facet_id,
            limit,
            search: None,
            term_search: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn search(
        mut self,
        value: crate::datadogV2::model::FacetInfoRequestDataAttributesSearch,
    ) -> Self {
        self.search = Some(value);
        self
    }

    pub fn term_search(
        mut self,
        value: crate::datadogV2::model::FacetInfoRequestDataAttributesTermSearch,
    ) -> Self {
        self.term_search = Some(value);
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

impl<'de> Deserialize<'de> for FacetInfoRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FacetInfoRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for FacetInfoRequestDataAttributesVisitor {
            type Value = FacetInfoRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet_id: Option<String> = None;
                let mut limit: Option<i64> = None;
                let mut search: Option<
                    crate::datadogV2::model::FacetInfoRequestDataAttributesSearch,
                > = None;
                let mut term_search: Option<
                    crate::datadogV2::model::FacetInfoRequestDataAttributesTermSearch,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet_id" => {
                            facet_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "term_search" => {
                            if v.is_null() {
                                continue;
                            }
                            term_search =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let facet_id = facet_id.ok_or_else(|| M::Error::missing_field("facet_id"))?;
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;

                let content = FacetInfoRequestDataAttributes {
                    facet_id,
                    limit,
                    search,
                    term_search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FacetInfoRequestDataAttributesVisitor)
    }
}
