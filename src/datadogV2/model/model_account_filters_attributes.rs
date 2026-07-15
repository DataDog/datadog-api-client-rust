// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the account filters of a cloud account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AccountFiltersAttributes {
    /// The account filtering configuration.
    #[serde(rename = "account_filters")]
    pub account_filters: Option<crate::datadogV2::model::AccountFilteringConfig>,
    /// The cloud account ID.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The cloud provider of the account, for example `aws`, `aws_cur2`, or `oci`.
    #[serde(rename = "cloud")]
    pub cloud: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AccountFiltersAttributes {
    pub fn new() -> AccountFiltersAttributes {
        AccountFiltersAttributes {
            account_filters: None,
            account_id: None,
            cloud: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_filters(
        mut self,
        value: crate::datadogV2::model::AccountFilteringConfig,
    ) -> Self {
        self.account_filters = Some(value);
        self
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn cloud(mut self, value: String) -> Self {
        self.cloud = Some(value);
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

impl Default for AccountFiltersAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AccountFiltersAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AccountFiltersAttributesVisitor;
        impl<'a> Visitor<'a> for AccountFiltersAttributesVisitor {
            type Value = AccountFiltersAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_filters: Option<crate::datadogV2::model::AccountFilteringConfig> =
                    None;
                let mut account_id: Option<String> = None;
                let mut cloud: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            account_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AccountFiltersAttributes {
                    account_filters,
                    account_id,
                    cloud,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AccountFiltersAttributesVisitor)
    }
}
