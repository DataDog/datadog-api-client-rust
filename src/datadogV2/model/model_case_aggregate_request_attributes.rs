// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the aggregation request, including the search query and grouping configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseAggregateRequestAttributes {
    /// Configuration for grouping aggregated results by one or more case fields.
    #[serde(rename = "group_by")]
    pub group_by: crate::datadogV2::model::CaseAggregateGroupBy,
    /// A search query to filter which cases are included in the aggregation. Uses the same syntax as the Case Management search bar.
    #[serde(rename = "query_filter")]
    pub query_filter: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseAggregateRequestAttributes {
    pub fn new(
        group_by: crate::datadogV2::model::CaseAggregateGroupBy,
        query_filter: String,
    ) -> CaseAggregateRequestAttributes {
        CaseAggregateRequestAttributes {
            group_by,
            query_filter,
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

impl<'de> Deserialize<'de> for CaseAggregateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseAggregateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CaseAggregateRequestAttributesVisitor {
            type Value = CaseAggregateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group_by: Option<crate::datadogV2::model::CaseAggregateGroupBy> = None;
                let mut query_filter: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_filter" => {
                            query_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let query_filter =
                    query_filter.ok_or_else(|| M::Error::missing_field("query_filter"))?;

                let content = CaseAggregateRequestAttributes {
                    group_by,
                    query_filter,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseAggregateRequestAttributesVisitor)
    }
}
