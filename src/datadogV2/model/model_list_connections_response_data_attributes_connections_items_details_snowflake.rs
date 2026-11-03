// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration of a Snowflake connection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake {
    /// Identifier of the Snowflake account.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// Name of the Snowflake database.
    #[serde(rename = "database_name")]
    pub database_name: Option<String>,
    /// Name of the Snowflake schema.
    #[serde(rename = "schema")]
    pub schema: Option<String>,
    /// Name of the Snowflake table.
    #[serde(rename = "table_name")]
    pub table_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake {
    pub fn new() -> ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake {
        ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake {
            account_id: None,
            database_name: None,
            schema: None,
            table_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn database_name(mut self, value: String) -> Self {
        self.database_name = Some(value);
        self
    }

    pub fn schema(mut self, value: String) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn table_name(mut self, value: String) -> Self {
        self.table_name = Some(value);
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

impl Default for ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflakeVisitor;
        impl<'a> Visitor<'a>
            for ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflakeVisitor
        {
            type Value = ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut database_name: Option<String> = None;
                let mut schema: Option<String> = None;
                let mut table_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "database_name" => {
                            if v.is_null() {
                                continue;
                            }
                            database_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            if v.is_null() {
                                continue;
                            }
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_name" => {
                            if v.is_null() {
                                continue;
                            }
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content =
                    ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake {
                        account_id,
                        database_name,
                        schema,
                        table_name,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflakeVisitor,
        )
    }
}
