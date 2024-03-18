// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Facet data numeric attributes of an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentSearchResponseNumericFacetData {
    /// Aggregate information for numeric incident data.
    #[serde(rename = "aggregates")]
    pub aggregates: crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates,
    /// Name of the incident property field.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentSearchResponseNumericFacetData {
    pub fn new(
        aggregates: crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates,
        name: String,
    ) -> IncidentSearchResponseNumericFacetData {
        IncidentSearchResponseNumericFacetData {
            aggregates,
            name,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for IncidentSearchResponseNumericFacetData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentSearchResponseNumericFacetDataVisitor;
        impl<'a> Visitor<'a> for IncidentSearchResponseNumericFacetDataVisitor {
            type Value = IncidentSearchResponseNumericFacetData;

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
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregates" => {
                            aggregates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let aggregates = aggregates.ok_or_else(|| M::Error::missing_field("aggregates"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = IncidentSearchResponseNumericFacetData {
                    aggregates,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentSearchResponseNumericFacetDataVisitor)
    }
}
