// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a usage quota by scope.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageQuotaCreateAttributes {
    /// Whether to actively block usage above the limit instead of only tracking or alerting on it.
    #[serde(rename = "enforced")]
    pub enforced: bool,
    /// A namespace-specific key and value identifying what the quota applies to within an organization. The object must contain exactly one entry. Use `"*"` as the value for the default quota applied to entities without a specific quota, or omit the scope for an organization-wide quota. A specific value must identify an existing user handle in the caller's organization when `include_descendants` is false. When `include_descendants` is true, the handle must exist in the caller's organization or in at least one targeted descendant organization; the quota is then applied only to the organizations where that handle exists, and the request fails only if the handle exists in none of them.
    #[serde(rename = "scope")]
    pub scope: Option<std::collections::BTreeMap<String, String>>,
    /// The quota limit to set in the usage units defined by the quota namespace. For an organization-wide quota (scope omitted), the limit must be greater than the usage already recorded in the current period.
    #[serde(rename = "usage_limit")]
    pub usage_limit: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageQuotaCreateAttributes {
    pub fn new(enforced: bool, usage_limit: i64) -> UsageQuotaCreateAttributes {
        UsageQuotaCreateAttributes {
            enforced,
            scope: None,
            usage_limit,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn scope(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.scope = Some(value);
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

impl<'de> Deserialize<'de> for UsageQuotaCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageQuotaCreateAttributesVisitor;
        impl<'a> Visitor<'a> for UsageQuotaCreateAttributesVisitor {
            type Value = UsageQuotaCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enforced: Option<bool> = None;
                let mut scope: Option<std::collections::BTreeMap<String, String>> = None;
                let mut usage_limit: Option<i64> = None;
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
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let enforced = enforced.ok_or_else(|| M::Error::missing_field("enforced"))?;
                let usage_limit =
                    usage_limit.ok_or_else(|| M::Error::missing_field("usage_limit"))?;

                let content = UsageQuotaCreateAttributes {
                    enforced,
                    scope,
                    usage_limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageQuotaCreateAttributesVisitor)
    }
}
