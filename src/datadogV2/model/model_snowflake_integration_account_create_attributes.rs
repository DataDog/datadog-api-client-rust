// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Writable attributes used to create a Snowflake integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeIntegrationAccountCreateAttributes {
    /// RSA key pair authentication, the only method Snowflake integration accounts support. Generate an RSA key pair and assign the public key to the Snowflake user named in `settings.username`. Because an update replaces this object as a whole, every required field must be sent again on each update, even when only one of them is changing.
    #[serde(rename = "authentication")]
    pub authentication: crate::datadogV2::model::SnowflakeIntegrationAccountAuthenticationRequest,
    /// Data Datadog collects from Snowflake, keyed by dataflow id. Each dataflow turns on a distinct kind of collection: set `enabled` to start or stop it, and use `settings` to configure what it collects. Defaults listed on each dataflow apply when the account is created; on update, omitted fields keep their current values. Every dataflow reads from Snowflake as the user in `settings.username`, so that user's role must be granted access to the underlying views; a dataflow enabled without those grants is stored but collects no data.
    #[serde(rename = "dataflows")]
    pub dataflows: Option<crate::datadogV2::model::SnowflakeIntegrationDataflowsRequest>,
    /// Human-readable name of the Snowflake integration account. Must be
    /// unique within your Datadog organization.
    #[serde(rename = "name")]
    pub name: String,
    /// Settings for creating the Snowflake integration account.
    #[serde(rename = "settings")]
    pub settings: crate::datadogV2::model::SnowflakeIntegrationAccountSettingsRequest,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeIntegrationAccountCreateAttributes {
    pub fn new(
        authentication: crate::datadogV2::model::SnowflakeIntegrationAccountAuthenticationRequest,
        name: String,
        settings: crate::datadogV2::model::SnowflakeIntegrationAccountSettingsRequest,
    ) -> SnowflakeIntegrationAccountCreateAttributes {
        SnowflakeIntegrationAccountCreateAttributes {
            authentication,
            dataflows: None,
            name,
            settings,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataflows(
        mut self,
        value: crate::datadogV2::model::SnowflakeIntegrationDataflowsRequest,
    ) -> Self {
        self.dataflows = Some(value);
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

impl<'de> Deserialize<'de> for SnowflakeIntegrationAccountCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeIntegrationAccountCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SnowflakeIntegrationAccountCreateAttributesVisitor {
            type Value = SnowflakeIntegrationAccountCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<
                    crate::datadogV2::model::SnowflakeIntegrationAccountAuthenticationRequest,
                > = None;
                let mut dataflows: Option<
                    crate::datadogV2::model::SnowflakeIntegrationDataflowsRequest,
                > = None;
                let mut name: Option<String> = None;
                let mut settings: Option<
                    crate::datadogV2::model::SnowflakeIntegrationAccountSettingsRequest,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataflows" => {
                            if v.is_null() {
                                continue;
                            }
                            dataflows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let authentication =
                    authentication.ok_or_else(|| M::Error::missing_field("authentication"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let settings = settings.ok_or_else(|| M::Error::missing_field("settings"))?;

                let content = SnowflakeIntegrationAccountCreateAttributes {
                    authentication,
                    dataflows,
                    name,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnowflakeIntegrationAccountCreateAttributesVisitor)
    }
}
