// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Type-specific configuration of the connection. Only the field matching the connection type is set.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListConnectionsResponseDataAttributesConnectionsItemsDetails {
    /// Configuration of a reference table connection.
    #[serde(rename = "ref_table")]
    pub ref_table: Option<crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsDetailsRefTable>,
    /// Configuration of a Snowflake connection.
    #[serde(rename = "snowflake")]
    pub snowflake: Option<crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ListConnectionsResponseDataAttributesConnectionsItemsDetails {
    pub fn new() -> ListConnectionsResponseDataAttributesConnectionsItemsDetails {
        ListConnectionsResponseDataAttributesConnectionsItemsDetails {
            ref_table: None,
            snowflake: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ref_table(
        mut self,
        value: crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsDetailsRefTable,
    ) -> Self {
        self.ref_table = Some(value);
        self
    }

    pub fn snowflake(
        mut self,
        value: crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake,
    ) -> Self {
        self.snowflake = Some(value);
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

impl Default for ListConnectionsResponseDataAttributesConnectionsItemsDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListConnectionsResponseDataAttributesConnectionsItemsDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListConnectionsResponseDataAttributesConnectionsItemsDetailsVisitor;
        impl<'a> Visitor<'a> for ListConnectionsResponseDataAttributesConnectionsItemsDetailsVisitor {
            type Value = ListConnectionsResponseDataAttributesConnectionsItemsDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ref_table: Option<crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsDetailsRefTable> = None;
                let mut snowflake: Option<crate::datadogV2::model::ListConnectionsResponseDataAttributesConnectionsItemsDetailsSnowflake> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ref_table" => {
                            if v.is_null() {
                                continue;
                            }
                            ref_table = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snowflake" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ListConnectionsResponseDataAttributesConnectionsItemsDetails {
                    ref_table,
                    snowflake,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ListConnectionsResponseDataAttributesConnectionsItemsDetailsVisitor)
    }
}
