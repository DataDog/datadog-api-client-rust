// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the AWS IAM Identity Center AWS account prerequisites.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringAWSIdentityCenterAWSAccountsAttributes {
    /// The integrated AWS accounts that users are read from, including AWS Organization management accounts
    /// and any accounts that users were collected from.
    #[serde(rename = "aws_accounts")]
    pub aws_accounts: Vec<crate::datadogV2::model::SecurityMonitoringAWSIdentityCenterAWSAccount>,
    /// Whether AWS IAM Identity Center users were collected for the organization.
    #[serde(rename = "has_valid_prerequisite")]
    pub has_valid_prerequisite: bool,
    /// The missing setup step when `has_valid_prerequisite` is `false`.
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringAWSIdentityCenterAWSAccountsAttributes {
    pub fn new(
        aws_accounts: Vec<crate::datadogV2::model::SecurityMonitoringAWSIdentityCenterAWSAccount>,
        has_valid_prerequisite: bool,
    ) -> SecurityMonitoringAWSIdentityCenterAWSAccountsAttributes {
        SecurityMonitoringAWSIdentityCenterAWSAccountsAttributes {
            aws_accounts,
            has_valid_prerequisite,
            reason: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringAWSIdentityCenterAWSAccountsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringAWSIdentityCenterAWSAccountsAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringAWSIdentityCenterAWSAccountsAttributesVisitor {
            type Value = SecurityMonitoringAWSIdentityCenterAWSAccountsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aws_accounts: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringAWSIdentityCenterAWSAccount>,
                > = None;
                let mut has_valid_prerequisite: Option<bool> = None;
                let mut reason: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aws_accounts" => {
                            aws_accounts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_valid_prerequisite" => {
                            has_valid_prerequisite =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let aws_accounts =
                    aws_accounts.ok_or_else(|| M::Error::missing_field("aws_accounts"))?;
                let has_valid_prerequisite = has_valid_prerequisite
                    .ok_or_else(|| M::Error::missing_field("has_valid_prerequisite"))?;

                let content = SecurityMonitoringAWSIdentityCenterAWSAccountsAttributes {
                    aws_accounts,
                    has_valid_prerequisite,
                    reason,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SecurityMonitoringAWSIdentityCenterAWSAccountsAttributesVisitor)
    }
}
