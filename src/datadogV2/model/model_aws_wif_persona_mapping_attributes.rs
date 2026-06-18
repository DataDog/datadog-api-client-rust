// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an AWS WIF persona mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AwsWifPersonaMappingAttributes {
    /// The Datadog user handle (email address) to map the AWS principal to.
    #[serde(rename = "account_identifier")]
    pub account_identifier: String,
    /// The Datadog user UUID corresponding to `account_identifier`. Read-only — set by the server.
    #[serde(rename = "account_uuid")]
    pub account_uuid: Option<uuid::Uuid>,
    /// The AWS IAM ARN pattern identifying the role or user that will be mapped.
    /// Supports wildcards (`*`) to match multiple principals within an account.
    #[serde(rename = "arn_pattern")]
    pub arn_pattern: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AwsWifPersonaMappingAttributes {
    pub fn new(account_identifier: String, arn_pattern: String) -> AwsWifPersonaMappingAttributes {
        AwsWifPersonaMappingAttributes {
            account_identifier,
            account_uuid: None,
            arn_pattern,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_uuid(mut self, value: uuid::Uuid) -> Self {
        self.account_uuid = Some(value);
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

impl<'de> Deserialize<'de> for AwsWifPersonaMappingAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AwsWifPersonaMappingAttributesVisitor;
        impl<'a> Visitor<'a> for AwsWifPersonaMappingAttributesVisitor {
            type Value = AwsWifPersonaMappingAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_identifier: Option<String> = None;
                let mut account_uuid: Option<uuid::Uuid> = None;
                let mut arn_pattern: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_identifier" => {
                            account_identifier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            account_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "arn_pattern" => {
                            arn_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_identifier = account_identifier
                    .ok_or_else(|| M::Error::missing_field("account_identifier"))?;
                let arn_pattern =
                    arn_pattern.ok_or_else(|| M::Error::missing_field("arn_pattern"))?;

                let content = AwsWifPersonaMappingAttributes {
                    account_identifier,
                    account_uuid,
                    arn_pattern,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AwsWifPersonaMappingAttributesVisitor)
    }
}
