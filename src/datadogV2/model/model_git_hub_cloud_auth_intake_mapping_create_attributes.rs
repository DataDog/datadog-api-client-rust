// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a GitHub cloud authentication intake mapping
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GitHubCloudAuthIntakeMappingCreateAttributes {
    /// GitHub Actions OIDC claims to match against. Each field is a regular expression.
    /// The `sub` claim is required; all other claims are optional. A token matches only when
    /// all provided patterns match simultaneously (AND semantics).
    #[serde(rename = "claim_matchers")]
    pub claim_matchers: crate::datadogV2::model::GitHubOIDCClaimPatterns,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GitHubCloudAuthIntakeMappingCreateAttributes {
    pub fn new(
        claim_matchers: crate::datadogV2::model::GitHubOIDCClaimPatterns,
    ) -> GitHubCloudAuthIntakeMappingCreateAttributes {
        GitHubCloudAuthIntakeMappingCreateAttributes {
            claim_matchers,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for GitHubCloudAuthIntakeMappingCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GitHubCloudAuthIntakeMappingCreateAttributesVisitor;
        impl<'a> Visitor<'a> for GitHubCloudAuthIntakeMappingCreateAttributesVisitor {
            type Value = GitHubCloudAuthIntakeMappingCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut claim_matchers: Option<crate::datadogV2::model::GitHubOIDCClaimPatterns> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "claim_matchers" => {
                            claim_matchers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let claim_matchers =
                    claim_matchers.ok_or_else(|| M::Error::missing_field("claim_matchers"))?;

                let content = GitHubCloudAuthIntakeMappingCreateAttributes {
                    claim_matchers,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GitHubCloudAuthIntakeMappingCreateAttributesVisitor)
    }
}
