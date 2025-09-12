// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AwsCurConfigResponseDataAttributesAccountFilters` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AwsCurConfigResponseDataAttributesAccountFilters {
    /// The `account_filters` `excluded_accounts`.
    #[serde(rename = "excluded_accounts")]
    pub excluded_accounts: Option<Vec<String>>,
    /// The `account_filters` `include_new_accounts`.
    #[serde(
        rename = "include_new_accounts",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub include_new_accounts: Option<Option<bool>>,
    /// The `account_filters` `included_accounts`.
    #[serde(rename = "included_accounts")]
    pub included_accounts: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AwsCurConfigResponseDataAttributesAccountFilters {
    pub fn new() -> AwsCurConfigResponseDataAttributesAccountFilters {
        AwsCurConfigResponseDataAttributesAccountFilters {
            excluded_accounts: None,
            include_new_accounts: None,
            included_accounts: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn excluded_accounts(mut self, value: Vec<String>) -> Self {
        self.excluded_accounts = Some(value);
        self
    }

    pub fn include_new_accounts(mut self, value: Option<bool>) -> Self {
        self.include_new_accounts = Some(value);
        self
    }

    pub fn included_accounts(mut self, value: Vec<String>) -> Self {
        self.included_accounts = Some(value);
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

impl Default for AwsCurConfigResponseDataAttributesAccountFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AwsCurConfigResponseDataAttributesAccountFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AwsCurConfigResponseDataAttributesAccountFiltersVisitor;
        impl<'a> Visitor<'a> for AwsCurConfigResponseDataAttributesAccountFiltersVisitor {
            type Value = AwsCurConfigResponseDataAttributesAccountFilters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut excluded_accounts: Option<Vec<String>> = None;
                let mut include_new_accounts: Option<Option<bool>> = None;
                let mut included_accounts: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "excluded_accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_accounts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_new_accounts" => {
                            include_new_accounts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included_accounts" => {
                            if v.is_null() {
                                continue;
                            }
                            included_accounts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AwsCurConfigResponseDataAttributesAccountFilters {
                    excluded_accounts,
                    include_new_accounts,
                    included_accounts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AwsCurConfigResponseDataAttributesAccountFiltersVisitor)
    }
}
