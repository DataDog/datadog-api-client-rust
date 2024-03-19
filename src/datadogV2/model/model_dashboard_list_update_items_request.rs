// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request containing the list of dashboards to update to.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardListUpdateItemsRequest {
    /// List of dashboards to update the dashboard list to.
    #[serde(rename = "dashboards")]
    pub dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemRequest>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardListUpdateItemsRequest {
    pub fn new() -> DashboardListUpdateItemsRequest {
        DashboardListUpdateItemsRequest {
            dashboards: None,
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
}

impl Default for DashboardListUpdateItemsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardListUpdateItemsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListUpdateItemsRequestVisitor;
        impl<'a> Visitor<'a> for DashboardListUpdateItemsRequestVisitor {
            type Value = DashboardListUpdateItemsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemRequest>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dashboards" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboards = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DashboardListUpdateItemsRequest {
                    dashboards,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListUpdateItemsRequestVisitor)
    }
}
