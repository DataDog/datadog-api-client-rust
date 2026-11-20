// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Stored HCP Terraform persona mapping attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TerraformCloudAuthPersonaMappingAttributesResponse {
    /// Handle of the mapped Datadog account, resolved when the mapping was created.
    #[serde(rename = "account_identifier")]
    pub account_identifier: String,
    /// UUID of the mapped Datadog user or service account.
    #[serde(rename = "account_uuid")]
    pub account_uuid: String,
    /// Audience to set as the `TFC_WORKLOAD_IDENTITY_AUDIENCE_DATADOG` environment variable in HCP Terraform. Datadog computes this value as `datadog/<organization UUID>`.
    #[serde(rename = "audience")]
    pub audience: Option<String>,
    /// HCP Terraform workload identity token issuer. Empty for mappings created before the issuer was required.
    #[serde(rename = "issuer_url")]
    pub issuer_url: Option<String>,
    /// Pattern matched against the `sub` claim of HCP Terraform workload identity tokens.
    #[serde(rename = "sub_pattern")]
    pub sub_pattern: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TerraformCloudAuthPersonaMappingAttributesResponse {
    pub fn new(account_identifier: String, account_uuid: String, sub_pattern: String) -> TerraformCloudAuthPersonaMappingAttributesResponse {
        TerraformCloudAuthPersonaMappingAttributesResponse {
            account_identifier,
            account_uuid,
            audience: None,
            issuer_url: None,
            sub_pattern,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn audience(mut self, value: String) -> Self {
        self.audience = Some(value);
        self
    }
    
    pub fn issuer_url(mut self, value: String) -> Self {
        self.issuer_url = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for TerraformCloudAuthPersonaMappingAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TerraformCloudAuthPersonaMappingAttributesResponseVisitor;
        impl<'a> Visitor<'a> for TerraformCloudAuthPersonaMappingAttributesResponseVisitor {
            type Value = TerraformCloudAuthPersonaMappingAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_identifier: Option<String> = None;
                let mut account_uuid: Option<String> = None;
                let mut audience: Option<String> = None;
                let mut issuer_url: Option<String> = None;
                let mut sub_pattern: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_identifier" => {
                            account_identifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "account_uuid" => {
                            account_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "audience" => {
                            if v.is_null() {
                                continue;
                            }
                            audience = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "issuer_url" => {
                            if v.is_null() {
                                continue;
                            }
                            issuer_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sub_pattern" => {
                            sub_pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let account_identifier = account_identifier.ok_or_else(|| M::Error::missing_field("account_identifier"))?;
                let account_uuid = account_uuid.ok_or_else(|| M::Error::missing_field("account_uuid"))?;
                let sub_pattern = sub_pattern.ok_or_else(|| M::Error::missing_field("sub_pattern"))?;

                let content = TerraformCloudAuthPersonaMappingAttributesResponse {
                    account_identifier,
                    account_uuid,
                    audience,
                    issuer_url,
                    sub_pattern,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TerraformCloudAuthPersonaMappingAttributesResponseVisitor)
    }
}
