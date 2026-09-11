// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings configured on the Snowflake integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeIntegrationAccountSettingsResponse {
    /// Identifier of the Snowflake account being monitored.
    #[serde(rename = "snowflake_account_identifier")]
    pub snowflake_account_identifier: String,
    /// Snowflake user Datadog authenticates as.
    #[serde(rename = "username")]
    pub username: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeIntegrationAccountSettingsResponse {
    pub fn new(
        snowflake_account_identifier: String,
        username: String,
    ) -> SnowflakeIntegrationAccountSettingsResponse {
        SnowflakeIntegrationAccountSettingsResponse {
            snowflake_account_identifier,
            username,
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

impl<'de> Deserialize<'de> for SnowflakeIntegrationAccountSettingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeIntegrationAccountSettingsResponseVisitor;
        impl<'a> Visitor<'a> for SnowflakeIntegrationAccountSettingsResponseVisitor {
            type Value = SnowflakeIntegrationAccountSettingsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut snowflake_account_identifier: Option<String> = None;
                let mut username: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "snowflake_account_identifier" => {
                            snowflake_account_identifier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "username" => {
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let snowflake_account_identifier = snowflake_account_identifier
                    .ok_or_else(|| M::Error::missing_field("snowflake_account_identifier"))?;
                let username = username.ok_or_else(|| M::Error::missing_field("username"))?;

                let content = SnowflakeIntegrationAccountSettingsResponse {
                    snowflake_account_identifier,
                    username,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnowflakeIntegrationAccountSettingsResponseVisitor)
    }
}
