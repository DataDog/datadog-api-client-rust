// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes to update on a usage quota. Omitting a property leaves its current value unchanged.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageQuotaUpdateAttributes {
    /// Whether to actively block usage above the limit. Omit this field to leave the current enforcement setting unchanged.
    #[serde(
        rename = "enforced",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub enforced: Option<Option<bool>>,
    /// The new quota limit in the usage units defined by the quota namespace. For an organization-wide quota (empty scope), the limit must be greater than the usage already recorded in the current period. Omit this field to leave the current limit unchanged.
    #[serde(
        rename = "usage_limit",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub usage_limit: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageQuotaUpdateAttributes {
    pub fn new() -> UsageQuotaUpdateAttributes {
        UsageQuotaUpdateAttributes {
            enforced: None,
            usage_limit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enforced(mut self, value: Option<bool>) -> Self {
        self.enforced = Some(value);
        self
    }

    pub fn usage_limit(mut self, value: Option<i64>) -> Self {
        self.usage_limit = Some(value);
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

impl Default for UsageQuotaUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageQuotaUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageQuotaUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for UsageQuotaUpdateAttributesVisitor {
            type Value = UsageQuotaUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enforced: Option<Option<bool>> = None;
                let mut usage_limit: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enforced" => {
                            enforced = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_limit" => {
                            usage_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageQuotaUpdateAttributes {
                    enforced,
                    usage_limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageQuotaUpdateAttributesVisitor)
    }
}
