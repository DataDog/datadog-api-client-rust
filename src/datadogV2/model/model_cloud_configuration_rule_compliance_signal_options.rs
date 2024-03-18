// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// How to generate compliance signals. Useful for cloud_configuration rules only.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudConfigurationRuleComplianceSignalOptions {
    /// The default activation status.
    #[serde(
        rename = "defaultActivationStatus",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_activation_status: Option<Option<bool>>,
    /// The default group by fields.
    #[serde(
        rename = "defaultGroupByFields",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_group_by_fields: Option<Option<Vec<String>>>,
    /// Whether signals will be sent.
    #[serde(
        rename = "userActivationStatus",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub user_activation_status: Option<Option<bool>>,
    /// Fields to use to group findings by when sending signals.
    #[serde(
        rename = "userGroupByFields",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub user_group_by_fields: Option<Option<Vec<String>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudConfigurationRuleComplianceSignalOptions {
    pub fn new() -> CloudConfigurationRuleComplianceSignalOptions {
        CloudConfigurationRuleComplianceSignalOptions {
            default_activation_status: None,
            default_group_by_fields: None,
            user_activation_status: None,
            user_group_by_fields: None,
            _unparsed: false,
        }
    }

    pub fn default_activation_status(mut self, value: Option<bool>) -> Self {
        self.default_activation_status = Some(value);
        self
    }

    pub fn default_group_by_fields(mut self, value: Option<Vec<String>>) -> Self {
        self.default_group_by_fields = Some(value);
        self
    }

    pub fn user_activation_status(mut self, value: Option<bool>) -> Self {
        self.user_activation_status = Some(value);
        self
    }

    pub fn user_group_by_fields(mut self, value: Option<Vec<String>>) -> Self {
        self.user_group_by_fields = Some(value);
        self
    }
}

impl Default for CloudConfigurationRuleComplianceSignalOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudConfigurationRuleComplianceSignalOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudConfigurationRuleComplianceSignalOptionsVisitor;
        impl<'a> Visitor<'a> for CloudConfigurationRuleComplianceSignalOptionsVisitor {
            type Value = CloudConfigurationRuleComplianceSignalOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_activation_status: Option<Option<bool>> = None;
                let mut default_group_by_fields: Option<Option<Vec<String>>> = None;
                let mut user_activation_status: Option<Option<bool>> = None;
                let mut user_group_by_fields: Option<Option<Vec<String>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "defaultActivationStatus" => {
                            default_activation_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaultGroupByFields" => {
                            default_group_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userActivationStatus" => {
                            user_activation_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userGroupByFields" => {
                            user_group_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CloudConfigurationRuleComplianceSignalOptions {
                    default_activation_status,
                    default_group_by_fields,
                    user_activation_status,
                    user_group_by_fields,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudConfigurationRuleComplianceSignalOptionsVisitor)
    }
}
