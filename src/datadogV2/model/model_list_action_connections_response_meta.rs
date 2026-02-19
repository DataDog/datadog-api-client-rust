// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for the list connections response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListActionConnectionsResponseMeta {
    /// Count of integrations by type.
    #[serde(
        rename = "integration_counts",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub integration_counts: Option<Option<Vec<crate::datadogV2::model::IntegrationCounts>>>,
    /// The total number of connections.
    #[serde(rename = "total")]
    pub total: i64,
    /// The total number of connections that match the specified filters.
    #[serde(rename = "total_filtered")]
    pub total_filtered: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListActionConnectionsResponseMeta {
    pub fn new(total: i64, total_filtered: i64) -> ListActionConnectionsResponseMeta {
        ListActionConnectionsResponseMeta {
            integration_counts: None,
            total,
            total_filtered,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn integration_counts(
        mut self,
        value: Option<Vec<crate::datadogV2::model::IntegrationCounts>>,
    ) -> Self {
        self.integration_counts = Some(value);
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

impl<'de> Deserialize<'de> for ListActionConnectionsResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListActionConnectionsResponseMetaVisitor;
        impl<'a> Visitor<'a> for ListActionConnectionsResponseMetaVisitor {
            type Value = ListActionConnectionsResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut integration_counts: Option<
                    Option<Vec<crate::datadogV2::model::IntegrationCounts>>,
                > = None;
                let mut total: Option<i64> = None;
                let mut total_filtered: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "integration_counts" => {
                            integration_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_filtered" => {
                            total_filtered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;
                let total_filtered =
                    total_filtered.ok_or_else(|| M::Error::missing_field("total_filtered"))?;

                let content = ListActionConnectionsResponseMeta {
                    integration_counts,
                    total,
                    total_filtered,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListActionConnectionsResponseMetaVisitor)
    }
}
