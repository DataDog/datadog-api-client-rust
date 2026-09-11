// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for updating the Snowflake integration account. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeIntegrationAccountSettingsUpdate {
    /// Identifier of the Snowflake account to monitor, either as `organization-account` or as the legacy `account_name.region_id.cloud_provider` account locator. An account identifier can be configured once per Datadog organization; reusing one is rejected with a `422` response. Accounts reached through AWS PrivateLink are not supported.
    #[serde(rename = "snowflake_account_identifier")]
    pub snowflake_account_identifier: Option<String>,
    /// Snowflake user Datadog authenticates as. Create a dedicated user for Datadog and grant it a role with access to the data you want to collect.
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeIntegrationAccountSettingsUpdate {
    pub fn new() -> SnowflakeIntegrationAccountSettingsUpdate {
        SnowflakeIntegrationAccountSettingsUpdate {
            snowflake_account_identifier: None,
            username: None,
            _unparsed: false,
        }
    }

    pub fn snowflake_account_identifier(mut self, value: String) -> Self {
        self.snowflake_account_identifier = Some(value);
        self
    }

    pub fn username(mut self, value: String) -> Self {
        self.username = Some(value);
        self
    }
}

impl Default for SnowflakeIntegrationAccountSettingsUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeIntegrationAccountSettingsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeIntegrationAccountSettingsUpdateVisitor;
        impl<'a> Visitor<'a> for SnowflakeIntegrationAccountSettingsUpdateVisitor {
            type Value = SnowflakeIntegrationAccountSettingsUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut snowflake_account_identifier: Option<String> = None;
                let mut username: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "snowflake_account_identifier" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_account_identifier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "username" => {
                            if v.is_null() {
                                continue;
                            }
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SnowflakeIntegrationAccountSettingsUpdate {
                    snowflake_account_identifier,
                    username,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnowflakeIntegrationAccountSettingsUpdateVisitor)
    }
}
