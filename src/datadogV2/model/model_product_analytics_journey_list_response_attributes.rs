// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a journey list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyListResponseAttributes {
    /// The kind of entity returned by a journey list query.
    #[serde(rename = "entity")]
    pub entity: crate::datadogV2::model::ProductAnalyticsJourneyEntity,
    /// The returned rows.
    #[serde(rename = "records")]
    pub records: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Total number of rows matching the query, ignoring `limit`.
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyListResponseAttributes {
    pub fn new(
        entity: crate::datadogV2::model::ProductAnalyticsJourneyEntity,
        records: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
        total_count: i64,
    ) -> ProductAnalyticsJourneyListResponseAttributes {
        ProductAnalyticsJourneyListResponseAttributes {
            entity,
            records,
            total_count,
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyListResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyListResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyListResponseAttributesVisitor {
            type Value = ProductAnalyticsJourneyListResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut entity: Option<crate::datadogV2::model::ProductAnalyticsJourneyEntity> =
                    None;
                let mut records: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut total_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "entity" => {
                            entity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _entity) = entity {
                                match _entity {
                                    crate::datadogV2::model::ProductAnalyticsJourneyEntity::UnparsedObject(_entity) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "records" => {
                            records = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let entity = entity.ok_or_else(|| M::Error::missing_field("entity"))?;
                let records = records.ok_or_else(|| M::Error::missing_field("records"))?;
                let total_count =
                    total_count.ok_or_else(|| M::Error::missing_field("total_count"))?;

                let content = ProductAnalyticsJourneyListResponseAttributes {
                    entity,
                    records,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyListResponseAttributesVisitor)
    }
}
