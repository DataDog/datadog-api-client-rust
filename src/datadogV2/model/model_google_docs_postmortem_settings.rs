// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for a postmortem template stored in Google Docs. Required when `location` is `google_docs`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GoogleDocsPostmortemSettings {
    /// The ID of the Google Drive integration account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The ID of the Google Drive folder where postmortems are created.
    #[serde(rename = "parent_folder_id")]
    pub parent_folder_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GoogleDocsPostmortemSettings {
    pub fn new(account_id: String, parent_folder_id: String) -> GoogleDocsPostmortemSettings {
        GoogleDocsPostmortemSettings {
            account_id,
            parent_folder_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for GoogleDocsPostmortemSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoogleDocsPostmortemSettingsVisitor;
        impl<'a> Visitor<'a> for GoogleDocsPostmortemSettingsVisitor {
            type Value = GoogleDocsPostmortemSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut parent_folder_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_folder_id" => {
                            parent_folder_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let parent_folder_id =
                    parent_folder_id.ok_or_else(|| M::Error::missing_field("parent_folder_id"))?;

                let content = GoogleDocsPostmortemSettings {
                    account_id,
                    parent_folder_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GoogleDocsPostmortemSettingsVisitor)
    }
}
