// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a usage quota.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageQuotaResponseAttributes {
    /// Whether usage above the limit is actively blocked instead of only tracked or alerted on.
    #[serde(rename = "enforced")]
    pub enforced: bool,
    /// The public ID of the organization that owns the quota.
    #[serde(rename = "org_public_id")]
    pub org_public_id: String,
    /// A namespace-specific key and value identifying what the quota applies to within an organization. The object contains exactly one entry. A value of `"*"` identifies the default quota applied to entities without a specific quota. This field is omitted for an organization-wide quota.
    #[serde(rename = "scope")]
    pub scope: Option<std::collections::BTreeMap<String, String>>,
    /// The quota limit in the usage units defined by the quota namespace. May be fractional for quotas configured before public writes required whole units.
    #[serde(rename = "usage_limit")]
    pub usage_limit: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageQuotaResponseAttributes {
    pub fn new(
        enforced: bool,
        org_public_id: String,
        usage_limit: f64,
    ) -> UsageQuotaResponseAttributes {
        UsageQuotaResponseAttributes {
            enforced,
            org_public_id,
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

impl<'de> Deserialize<'de> for UsageQuotaResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageQuotaResponseAttributesVisitor;
        impl<'a> Visitor<'a> for UsageQuotaResponseAttributesVisitor {
            type Value = UsageQuotaResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enforced: Option<bool> = None;
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
                            enforced = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_public_id" => {
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
                let org_public_id =
                    org_public_id.ok_or_else(|| M::Error::missing_field("org_public_id"))?;
                let usage_limit =
                    usage_limit.ok_or_else(|| M::Error::missing_field("usage_limit"))?;

                let content = UsageQuotaResponseAttributes {
                    enforced,
                    org_public_id,
                    scope,
                    usage_limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageQuotaResponseAttributesVisitor)
    }
}
