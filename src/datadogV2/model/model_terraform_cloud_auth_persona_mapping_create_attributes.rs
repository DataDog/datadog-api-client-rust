// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an HCP Terraform persona mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TerraformCloudAuthPersonaMappingCreateAttributes {
    /// Handle or email address of the Datadog user or service account that matching tokens authenticate as. Its permissions must not exceed the caller's.
    #[serde(rename = "account_identifier")]
    pub account_identifier: String,
    /// HCP Terraform workload identity token issuer. Use `<https://app.terraform.io`> for HCP Terraform in the US or `<https://app.eu.terraform.io`> for HCP Terraform in Europe.
    #[serde(rename = "issuer_url")]
    pub issuer_url: crate::datadogV2::model::TerraformCloudAuthIssuerURL,
    /// Pattern matched against the `sub` claim of HCP Terraform workload identity tokens. Use `organization:<org>:project:<project>:workspace:<workspace>:run_phase:<plan|apply>` for workspace runs, or `organization:<org>:module:<module>:operation:test_run` for module test runs. Segments are HCP Terraform names, not IDs.
    /// Any segment except the organization can be `*` to match any value.
    #[serde(rename = "sub_pattern")]
    pub sub_pattern: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TerraformCloudAuthPersonaMappingCreateAttributes {
    pub fn new(
        account_identifier: String,
        issuer_url: crate::datadogV2::model::TerraformCloudAuthIssuerURL,
        sub_pattern: String,
    ) -> TerraformCloudAuthPersonaMappingCreateAttributes {
        TerraformCloudAuthPersonaMappingCreateAttributes {
            account_identifier,
            issuer_url,
            sub_pattern,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for TerraformCloudAuthPersonaMappingCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TerraformCloudAuthPersonaMappingCreateAttributesVisitor;
        impl<'a> Visitor<'a> for TerraformCloudAuthPersonaMappingCreateAttributesVisitor {
            type Value = TerraformCloudAuthPersonaMappingCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_identifier: Option<String> = None;
                let mut issuer_url: Option<crate::datadogV2::model::TerraformCloudAuthIssuerURL> =
                    None;
                let mut sub_pattern: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_identifier" => {
                            account_identifier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issuer_url" => {
                            issuer_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _issuer_url) = issuer_url {
                                match _issuer_url {
                                    crate::datadogV2::model::TerraformCloudAuthIssuerURL::UnparsedObject(_issuer_url) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sub_pattern" => {
                            sub_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let account_identifier = account_identifier
                    .ok_or_else(|| M::Error::missing_field("account_identifier"))?;
                let issuer_url = issuer_url.ok_or_else(|| M::Error::missing_field("issuer_url"))?;
                let sub_pattern =
                    sub_pattern.ok_or_else(|| M::Error::missing_field("sub_pattern"))?;

                let content = TerraformCloudAuthPersonaMappingCreateAttributes {
                    account_identifier,
                    issuer_url,
                    sub_pattern,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TerraformCloudAuthPersonaMappingCreateAttributesVisitor)
    }
}
