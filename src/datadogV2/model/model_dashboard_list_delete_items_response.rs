// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing a list of deleted dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardListDeleteItemsResponse {
    /// List of dashboards deleted from the dashboard list.
    #[serde(rename = "deleted_dashboards_from_list")]
    pub deleted_dashboards_from_list:
        Option<Vec<crate::datadogV2::model::DashboardListItemResponse>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardListDeleteItemsResponse {
    pub fn new() -> DashboardListDeleteItemsResponse {
        DashboardListDeleteItemsResponse {
            deleted_dashboards_from_list: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn deleted_dashboards_from_list(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardListItemResponse>,
    ) -> Self {
        self.deleted_dashboards_from_list = Some(value);
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

impl Default for DashboardListDeleteItemsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardListDeleteItemsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListDeleteItemsResponseVisitor;
        impl<'a> Visitor<'a> for DashboardListDeleteItemsResponseVisitor {
            type Value = DashboardListDeleteItemsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_dashboards_from_list: Option<
                    Vec<crate::datadogV2::model::DashboardListItemResponse>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted_dashboards_from_list" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_dashboards_from_list =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DashboardListDeleteItemsResponse {
                    deleted_dashboards_from_list,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListDeleteItemsResponseVisitor)
    }
}
