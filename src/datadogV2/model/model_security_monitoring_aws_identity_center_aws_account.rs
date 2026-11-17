// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An integrated AWS account the AWS IAM Identity Center integration reads users from.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringAWSIdentityCenterAWSAccount {
    /// The ID of the AWS account integration configuration.
    #[serde(rename = "account_config_id")]
    pub account_config_id: String,
    /// The AWS account ID.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The name of the AWS account integration.
    #[serde(rename = "account_name")]
    pub account_name: String,
    /// Whether AWS IAM Identity Center users were collected from this AWS account.
    #[serde(rename = "collecting_users")]
    pub collecting_users: bool,
    /// Whether resource collection is enabled for this AWS account.
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringAWSIdentityCenterAWSAccount {
    pub fn new(
        account_config_id: String,
        account_id: String,
        account_name: String,
        collecting_users: bool,
        resource_collection_enabled: bool,
    ) -> SecurityMonitoringAWSIdentityCenterAWSAccount {
        SecurityMonitoringAWSIdentityCenterAWSAccount {
            account_config_id,
            account_id,
            account_name,
            collecting_users,
            resource_collection_enabled,
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

impl<'de> Deserialize<'de> for SecurityMonitoringAWSIdentityCenterAWSAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringAWSIdentityCenterAWSAccountVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringAWSIdentityCenterAWSAccountVisitor {
            type Value = SecurityMonitoringAWSIdentityCenterAWSAccount;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_config_id: Option<String> = None;
                let mut account_id: Option<String> = None;
                let mut account_name: Option<String> = None;
                let mut collecting_users: Option<bool> = None;
                let mut resource_collection_enabled: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_config_id" => {
                            account_config_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_name" => {
                            account_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "collecting_users" => {
                            collecting_users =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_collection_enabled" => {
                            resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_config_id = account_config_id
                    .ok_or_else(|| M::Error::missing_field("account_config_id"))?;
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let account_name =
                    account_name.ok_or_else(|| M::Error::missing_field("account_name"))?;
                let collecting_users =
                    collecting_users.ok_or_else(|| M::Error::missing_field("collecting_users"))?;
                let resource_collection_enabled = resource_collection_enabled
                    .ok_or_else(|| M::Error::missing_field("resource_collection_enabled"))?;

                let content = SecurityMonitoringAWSIdentityCenterAWSAccount {
                    account_config_id,
                    account_id,
                    account_name,
                    collecting_users,
                    resource_collection_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringAWSIdentityCenterAWSAccountVisitor)
    }
}
