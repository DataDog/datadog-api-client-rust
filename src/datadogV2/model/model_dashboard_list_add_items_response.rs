// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing a list of added dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardListAddItemsResponse {
    /// List of dashboards added to the dashboard list.
    #[serde(rename = "added_dashboards_to_list")]
    pub added_dashboards_to_list: Option<Vec<crate::datadogV2::model::DashboardListItemResponse>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardListAddItemsResponse {
    pub fn new() -> DashboardListAddItemsResponse {
        DashboardListAddItemsResponse {
            added_dashboards_to_list: None,
            _unparsed: false,
        }
    }

    pub fn added_dashboards_to_list(
        mut self,
        value: Vec<crate::datadogV2::model::DashboardListItemResponse>,
    ) -> Self {
        self.added_dashboards_to_list = Some(value);
        self
    }
}

impl Default for DashboardListAddItemsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardListAddItemsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListAddItemsResponseVisitor;
        impl<'a> Visitor<'a> for DashboardListAddItemsResponseVisitor {
            type Value = DashboardListAddItemsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut added_dashboards_to_list: Option<
                    Vec<crate::datadogV2::model::DashboardListItemResponse>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "added_dashboards_to_list" => {
                            if v.is_null() {
                                continue;
                            }
                            added_dashboards_to_list =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DashboardListAddItemsResponse {
                    added_dashboards_to_list,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListAddItemsResponseVisitor)
    }
}
