// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a usage quota bulk write result. On success, all fields except `error` are present. On failure, only `error` is present and the other fields are omitted.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageQuotaBulkResultAttributes {
    /// Whether usage above the limit is actively blocked instead of only tracked or alerted on. Omitted if this item failed to write.
    #[serde(rename = "enforced")]
    pub enforced: Option<bool>,
    /// An error message describing why this item failed to write. Omitted if this item was written successfully.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// The public ID of the organization that owns the quota. Omitted if this item failed to write.
    #[serde(rename = "org_public_id")]
    pub org_public_id: Option<String>,
    /// A namespace-specific key and value identifying what the quota applies to within an organization. The object contains exactly one entry. A value of `"*"` identifies the default quota applied to entities without a specific quota. This field is omitted for an organization-wide quota.
    #[serde(rename = "scope")]
    pub scope: Option<std::collections::BTreeMap<String, String>>,
    /// The quota limit in the usage units defined by the quota namespace. May be fractional for quotas configured before public writes required whole units. Omitted if this item failed to write.
    #[serde(rename = "usage_limit")]
    pub usage_limit: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageQuotaBulkResultAttributes {
    pub fn new() -> UsageQuotaBulkResultAttributes {
        UsageQuotaBulkResultAttributes {
            enforced: None,
            error: None,
            org_public_id: None,
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

    pub fn error(mut self, value: String) -> Self {
        self.error = Some(value);
        self
    }

    pub fn org_public_id(mut self, value: String) -> Self {
        self.org_public_id = Some(value);
        self
    }

    pub fn scope(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn usage_limit(mut self, value: f64) -> Self {
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

impl Default for UsageQuotaBulkResultAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageQuotaBulkResultAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageQuotaBulkResultAttributesVisitor;
        impl<'a> Visitor<'a> for UsageQuotaBulkResultAttributesVisitor {
            type Value = UsageQuotaBulkResultAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enforced: Option<bool> = None;
                let mut error: Option<String> = None;
                let mut org_public_id: Option<String> = None;
                let mut scope: Option<std::collections::BTreeMap<String, String>> = None;
                let mut usage_limit: Option<f64> = None;
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
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_public_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_limit" => {
                            if v.is_null() || v.as_str() == Some("") {
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

                let content = UsageQuotaBulkResultAttributes {
                    enforced,
                    error,
                    org_public_id,
                    scope,
                    usage_limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageQuotaBulkResultAttributesVisitor)
    }
}
