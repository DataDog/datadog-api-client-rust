// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request containing a list of dashboards to delete.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardListDeleteItemsRequest {
    /// List of dashboards to delete from the dashboard list.
    #[serde(rename = "dashboards")]
    pub dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemRequest>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardListDeleteItemsRequest {
    pub fn new() -> DashboardListDeleteItemsRequest {
        DashboardListDeleteItemsRequest {
            dashboards: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dashboards(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardListItemRequest>,
    ) -> Self {
        self.dashboards = Some(value);
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

impl Default for DashboardListDeleteItemsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardListDeleteItemsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListDeleteItemsRequestVisitor;
        impl<'a> Visitor<'a> for DashboardListDeleteItemsRequestVisitor {
            type Value = DashboardListDeleteItemsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemRequest>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dashboards" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboards = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DashboardListDeleteItemsRequest {
                    dashboards,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListDeleteItemsRequestVisitor)
    }
}
