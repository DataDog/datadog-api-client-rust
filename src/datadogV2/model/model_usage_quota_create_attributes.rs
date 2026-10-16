// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a usage quota by scope. Each item must provide `usage_limit`, `pending_usage_limit`, or both. Providing only `pending_usage_limit` updates an existing organization-wide quota, never creates one, requires `enforced` to be omitted, and fails if the quota does not exist.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageQuotaCreateAttributes {
    /// Whether to actively block usage above `usage_limit` instead of only tracking or alerting on it. Required when `usage_limit` is provided and must be omitted when only `pending_usage_limit` is provided.
    #[serde(rename = "enforced")]
    pub enforced: Option<bool>,
    /// The non-negative, whole-number limit to schedule for the organization-wide quota in the usage units defined by the quota namespace. It is not checked against current usage. Each write schedules the value for 00:00 UTC on the first day of the next calendar month and replaces any previously scheduled change; the server computes `pending_effective_from`. Omit this field to leave any scheduled change unchanged, including when raising `usage_limit`. Cancel a scheduled change only by deleting the quota's `/pending` sub-resource.
    #[serde(rename = "pending_usage_limit")]
    pub pending_usage_limit: Option<i64>,
    /// A namespace-specific key and value identifying what the quota applies to within an organization. The object must contain exactly one entry. Use `"*"` as the value for the default quota applied to entities without a specific quota, or omit the scope for an organization-wide quota. A specific value must identify an existing user handle in the caller's organization when `include_descendants` is false. When `include_descendants` is true, the handle must exist in the caller's organization or in at least one targeted descendant organization; the quota is then applied only to the organizations where that handle exists, and the request fails only if the handle exists in none of them.
    #[serde(rename = "scope")]
    pub scope: Option<std::collections::BTreeMap<String, String>>,
    /// The non-negative, whole-number quota limit to set in the usage units defined by the quota namespace. For an organization-wide quota (scope omitted), the limit must be greater than usage already recorded in the current period. When this field is provided, `enforced` is required.
    #[serde(rename = "usage_limit")]
    pub usage_limit: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageQuotaCreateAttributes {
    pub fn new() -> UsageQuotaCreateAttributes {
        UsageQuotaCreateAttributes {
            enforced: None,
            pending_usage_limit: None,
            scope: None,
            usage_limit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enforced(mut self, value: bool) -> Self {
        self.enforced = Some(value);
        self
    }

    pub fn pending_usage_limit(mut self, value: i64) -> Self {
        self.pending_usage_limit = Some(value);
        self
    }

    pub fn scope(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn usage_limit(mut self, value: i64) -> Self {
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

impl Default for UsageQuotaCreateAttributes {
    fn default() -> Self {
        Self::new()
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
                let mut pending_usage_limit: Option<i64> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            enforced = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pending_usage_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            pending_usage_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_limit" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = UsageQuotaCreateAttributes {
                    enforced,
                    pending_usage_limit,
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
