// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A field to group by in the available values query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardAvailableValuesEventsQueryGroupByItems {
    /// The facet to group by.
    #[serde(rename = "facet")]
    pub facet: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardAvailableValuesEventsQueryGroupByItems {
    pub fn new(facet: String) -> DashboardAvailableValuesEventsQueryGroupByItems {
        DashboardAvailableValuesEventsQueryGroupByItems {
            facet,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for DashboardAvailableValuesEventsQueryGroupByItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardAvailableValuesEventsQueryGroupByItemsVisitor;
        impl<'a> Visitor<'a> for DashboardAvailableValuesEventsQueryGroupByItemsVisitor {
            type Value = DashboardAvailableValuesEventsQueryGroupByItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;

                let content = DashboardAvailableValuesEventsQueryGroupByItems { facet, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardAvailableValuesEventsQueryGroupByItemsVisitor)
    }
}
