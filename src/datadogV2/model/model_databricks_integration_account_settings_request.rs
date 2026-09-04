// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for creating the Databricks integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationAccountSettingsRequest {
    /// ID of the SQL warehouse used to query the Databricks system tables.
    #[serde(rename = "system_tables_sql_warehouse_id")]
    pub system_tables_sql_warehouse_id: Option<String>,
    /// URL of the Databricks workspace.
    #[serde(rename = "workspace_url")]
    pub workspace_url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksIntegrationAccountSettingsRequest {
    pub fn new(workspace_url: String) -> DatabricksIntegrationAccountSettingsRequest {
        DatabricksIntegrationAccountSettingsRequest {
            system_tables_sql_warehouse_id: None,
            workspace_url,
            _unparsed: false,
        }
    }

    pub fn system_tables_sql_warehouse_id(mut self, value: String) -> Self {
        self.system_tables_sql_warehouse_id = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksIntegrationAccountSettingsRequestVisitor;
        impl<'a> Visitor<'a> for DatabricksIntegrationAccountSettingsRequestVisitor {
            type Value = DatabricksIntegrationAccountSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut system_tables_sql_warehouse_id: Option<String> = None;
                let mut workspace_url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "system_tables_sql_warehouse_id" => {
                            if v.is_null() {
                                continue;
                            }
                            system_tables_sql_warehouse_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workspace_url" => {
                            workspace_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let workspace_url =
                    workspace_url.ok_or_else(|| M::Error::missing_field("workspace_url"))?;

                let content = DatabricksIntegrationAccountSettingsRequest {
                    system_tables_sql_warehouse_id,
                    workspace_url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksIntegrationAccountSettingsRequestVisitor)
    }
}
