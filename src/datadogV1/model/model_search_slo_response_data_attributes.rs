// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchSLOResponseDataAttributes {
    /// Facets
    #[serde(rename = "facets")]
    pub facets: Option<crate::datadogV1::model::SearchSLOResponseDataAttributesFacets>,
    /// SLOs
    #[serde(rename = "slos")]
    pub slos: Option<Vec<crate::datadogV1::model::SearchServiceLevelObjective>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SearchSLOResponseDataAttributes {
    pub fn new() -> SearchSLOResponseDataAttributes {
        SearchSLOResponseDataAttributes {
            facets: None,
            slos: None,
            _unparsed: false,
        }
    }

    pub fn facets(
        mut self,
        value: crate::datadogV1::model::SearchSLOResponseDataAttributesFacets,
    ) -> Self {
        self.facets = Some(value);
        self
    }

    pub fn slos(
        mut self,
        value: Vec<crate::datadogV1::model::SearchServiceLevelObjective>,
    ) -> Self {
        self.slos = Some(value);
        self
    }
}

impl Default for SearchSLOResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SearchSLOResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchSLOResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for SearchSLOResponseDataAttributesVisitor {
            type Value = SearchSLOResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facets: Option<
                    crate::datadogV1::model::SearchSLOResponseDataAttributesFacets,
                > = None;
                let mut slos: Option<Vec<crate::datadogV1::model::SearchServiceLevelObjective>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facets" => {
                            if v.is_null() {
                                continue;
                            }
                            facets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slos" => {
                            if v.is_null() {
                                continue;
                            }
                            slos = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SearchSLOResponseDataAttributes {
                    facets,
                    slos,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SearchSLOResponseDataAttributesVisitor)
    }
}
