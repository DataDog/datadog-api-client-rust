// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for updating the ChatGPT Enterprise integration account. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChatGPTEnterpriseIntegrationAccountSettingsUpdate {
    /// OpenAI API Platform organization ID (`org-...`) that the Costs API uses for your Enterprise usage. This is not the ChatGPT workspace UUID or your Datadog organization ID. A value that does not match `^org-[A-Za-z0-9]+$` is rejected with a `422` response.
    #[serde(rename = "openai_org_id")]
    pub openai_org_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChatGPTEnterpriseIntegrationAccountSettingsUpdate {
    pub fn new() -> ChatGPTEnterpriseIntegrationAccountSettingsUpdate {
        ChatGPTEnterpriseIntegrationAccountSettingsUpdate {
            openai_org_id: None,
            _unparsed: false,
        }
    }

    pub fn openai_org_id(mut self, value: String) -> Self {
        self.openai_org_id = Some(value);
        self
    }
}

impl Default for ChatGPTEnterpriseIntegrationAccountSettingsUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChatGPTEnterpriseIntegrationAccountSettingsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChatGPTEnterpriseIntegrationAccountSettingsUpdateVisitor;
        impl<'a> Visitor<'a> for ChatGPTEnterpriseIntegrationAccountSettingsUpdateVisitor {
            type Value = ChatGPTEnterpriseIntegrationAccountSettingsUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut openai_org_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "openai_org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            openai_org_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = ChatGPTEnterpriseIntegrationAccountSettingsUpdate {
                    openai_org_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChatGPTEnterpriseIntegrationAccountSettingsUpdateVisitor)
    }
}
