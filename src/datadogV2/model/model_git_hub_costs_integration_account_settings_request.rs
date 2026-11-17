// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for creating the GitHub Costs integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GitHubCostsIntegrationAccountSettingsRequest {
    /// Your GitHub enterprise slug, found on the enterprise settings page (e.g. enterprise-name), or an organization login (e.g. organization-name). An enterprise already covers every organization it owns.
    #[serde(rename = "enterprise_name")]
    pub enterprise_name: String,
    /// Comma-separated tags added to the collected costs, for example `env:prod,team:example`.
    #[serde(rename = "tags")]
    pub tags: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GitHubCostsIntegrationAccountSettingsRequest {
    pub fn new(enterprise_name: String) -> GitHubCostsIntegrationAccountSettingsRequest {
        GitHubCostsIntegrationAccountSettingsRequest {
            enterprise_name,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn tags(mut self, value: String) -> Self {
        self.tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for GitHubCostsIntegrationAccountSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GitHubCostsIntegrationAccountSettingsRequestVisitor;
        impl<'a> Visitor<'a> for GitHubCostsIntegrationAccountSettingsRequestVisitor {
            type Value = GitHubCostsIntegrationAccountSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enterprise_name: Option<String> = None;
                let mut tags: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enterprise_name" => {
                            enterprise_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let enterprise_name =
                    enterprise_name.ok_or_else(|| M::Error::missing_field("enterprise_name"))?;

                let content = GitHubCostsIntegrationAccountSettingsRequest {
                    enterprise_name,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GitHubCostsIntegrationAccountSettingsRequestVisitor)
    }
}
