// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes returned by an incident search.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentSearchResponseAttributes {
    /// Facet data for incidents returned by a search query.
    #[serde(rename = "facets")]
    pub facets: crate::datadogV2::model::IncidentSearchResponseFacetsData,
    /// Incidents returned by the search.
    #[serde(rename = "incidents")]
    pub incidents: Vec<crate::datadogV2::model::IncidentSearchResponseIncidentsData>,
    /// Number of incidents returned by the search.
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentSearchResponseAttributes {
    pub fn new(
        facets: crate::datadogV2::model::IncidentSearchResponseFacetsData,
        incidents: Vec<crate::datadogV2::model::IncidentSearchResponseIncidentsData>,
        total: i32,
    ) -> IncidentSearchResponseAttributes {
        IncidentSearchResponseAttributes {
            facets,
            incidents,
            total,
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

impl<'de> Deserialize<'de> for IncidentSearchResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentSearchResponseAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentSearchResponseAttributesVisitor {
            type Value = IncidentSearchResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facets: Option<crate::datadogV2::model::IncidentSearchResponseFacetsData> =
                    None;
                let mut incidents: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseIncidentsData>,
                > = None;
                let mut total: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facets" => {
                            facets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incidents" => {
                            incidents = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let facets = facets.ok_or_else(|| M::Error::missing_field("facets"))?;
                let incidents = incidents.ok_or_else(|| M::Error::missing_field("incidents"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = IncidentSearchResponseAttributes {
                    facets,
                    incidents,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentSearchResponseAttributesVisitor)
    }
}
