// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for creating the Nebius Cloud integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationAccountSettingsRequest {
    /// Tags applied to all metrics, logs, and resources collected from this Nebius Cloud account. Defaults to no tags.
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// ID of the Nebius tenant to monitor, the organizational workspace identifier. Found in the Nebius console under Administration > Organization.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationAccountSettingsRequest {
    pub fn new(tenant_id: String) -> NebiusCloudIntegrationAccountSettingsRequest {
        NebiusCloudIntegrationAccountSettingsRequest {
            account_tags: None,
            tenant_id,
            _unparsed: false,
        }
    }

    pub fn account_tags(mut self, value: Vec<String>) -> Self {
        self.account_tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for NebiusCloudIntegrationAccountSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationAccountSettingsRequestVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationAccountSettingsRequestVisitor {
            type Value = NebiusCloudIntegrationAccountSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_tags: Option<Vec<String>> = None;
                let mut tenant_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            account_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let tenant_id = tenant_id.ok_or_else(|| M::Error::missing_field("tenant_id"))?;

                let content = NebiusCloudIntegrationAccountSettingsRequest {
                    account_tags,
                    tenant_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationAccountSettingsRequestVisitor)
    }
}
