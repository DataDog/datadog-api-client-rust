// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Facet data for the incident property fields.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentSearchResponsePropertyFieldFacetData {
    /// Aggregate information for numeric incident data.
    #[serde(rename = "aggregates")]
    pub aggregates:
        Option<crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates>,
    /// Facet data for the property field of an incident.
    #[serde(rename = "facets")]
    pub facets: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    /// Name of the incident property field.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentSearchResponsePropertyFieldFacetData {
    pub fn new(
        facets: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
        name: String,
    ) -> IncidentSearchResponsePropertyFieldFacetData {
        IncidentSearchResponsePropertyFieldFacetData {
            aggregates: None,
            facets,
            name,
            _unparsed: false,
        }
    }

    pub fn aggregates(
        mut self,
        value: crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates,
    ) -> Self {
        self.aggregates = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for IncidentSearchResponsePropertyFieldFacetData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentSearchResponsePropertyFieldFacetDataVisitor;
        impl<'a> Visitor<'a> for IncidentSearchResponsePropertyFieldFacetDataVisitor {
            type Value = IncidentSearchResponsePropertyFieldFacetData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregates: Option<
                    crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates,
                > = None;
                let mut facets: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
                > = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregates" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "facets" => {
                            facets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let facets = facets.ok_or_else(|| M::Error::missing_field("facets"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = IncidentSearchResponsePropertyFieldFacetData {
                    aggregates,
                    facets,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentSearchResponsePropertyFieldFacetDataVisitor)
    }
}
