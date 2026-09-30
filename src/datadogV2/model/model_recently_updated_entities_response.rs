// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response from the recently updated entities endpoint, containing the entities with the most recent updates in the requested time range, ordered from most to least recently updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecentlyUpdatedEntitiesResponse {
    /// The list of entities with the most recent updates, ordered from most to least recently updated.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::EntityContextEntity>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RecentlyUpdatedEntitiesResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::EntityContextEntity>,
    ) -> RecentlyUpdatedEntitiesResponse {
        RecentlyUpdatedEntitiesResponse {
            data,
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

impl<'de> Deserialize<'de> for RecentlyUpdatedEntitiesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RecentlyUpdatedEntitiesResponseVisitor;
        impl<'a> Visitor<'a> for RecentlyUpdatedEntitiesResponseVisitor {
            type Value = RecentlyUpdatedEntitiesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::EntityContextEntity>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = RecentlyUpdatedEntitiesResponse {
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RecentlyUpdatedEntitiesResponseVisitor)
    }
}
