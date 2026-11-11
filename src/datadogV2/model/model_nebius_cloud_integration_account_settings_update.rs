// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for updating the Nebius Cloud integration account. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationAccountSettingsUpdate {
    /// Tags applied to all metrics, logs, and resources collected from this Nebius Cloud account. Sending it replaces the whole list; send an empty array to remove all tags, or omit it to keep the stored list.
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// ID of the Nebius tenant to monitor. An empty value is ignored and the stored tenant ID is kept.
    #[serde(rename = "tenant_id")]
    pub tenant_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationAccountSettingsUpdate {
    pub fn new() -> NebiusCloudIntegrationAccountSettingsUpdate {
        NebiusCloudIntegrationAccountSettingsUpdate {
            account_tags: None,
            tenant_id: None,
            _unparsed: false,
        }
    }

    pub fn account_tags(mut self, value: Vec<String>) -> Self {
        self.account_tags = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: String) -> Self {
        self.tenant_id = Some(value);
        self
    }
}

impl Default for NebiusCloudIntegrationAccountSettingsUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudIntegrationAccountSettingsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationAccountSettingsUpdateVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationAccountSettingsUpdateVisitor {
            type Value = NebiusCloudIntegrationAccountSettingsUpdate;

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
                            if v.is_null() {
                                continue;
                            }
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = NebiusCloudIntegrationAccountSettingsUpdate {
                    account_tags,
                    tenant_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationAccountSettingsUpdateVisitor)
    }
}
