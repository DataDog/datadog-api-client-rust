// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Sankey request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsSankeyRequestAttributes {
    /// The shape of the Sankey diagram, expressed as the facets to flow between and how many steps to show.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::ProductAnalyticsSankeyDefinition,
    /// Selects the sessions a Sankey diagram is built from.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsSankeySearch,
    /// The time window a Sankey query covers.
    #[serde(rename = "time")]
    pub time: crate::datadogV2::model::ProductAnalyticsSankeyTime,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsSankeyRequestAttributes {
    pub fn new(
        definition: crate::datadogV2::model::ProductAnalyticsSankeyDefinition,
        search: crate::datadogV2::model::ProductAnalyticsSankeySearch,
        time: crate::datadogV2::model::ProductAnalyticsSankeyTime,
    ) -> ProductAnalyticsSankeyRequestAttributes {
        ProductAnalyticsSankeyRequestAttributes {
            definition,
            search,
            time,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsSankeyRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsSankeyRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsSankeyRequestAttributesVisitor {
            type Value = ProductAnalyticsSankeyRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<
                    crate::datadogV2::model::ProductAnalyticsSankeyDefinition,
                > = None;
                let mut search: Option<crate::datadogV2::model::ProductAnalyticsSankeySearch> =
                    None;
                let mut time: Option<crate::datadogV2::model::ProductAnalyticsSankeyTime> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;
                let time = time.ok_or_else(|| M::Error::missing_field("time"))?;

                let content = ProductAnalyticsSankeyRequestAttributes {
                    definition,
                    search,
                    time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsSankeyRequestAttributesVisitor)
    }
}
