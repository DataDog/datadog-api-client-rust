// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for a postmortem template stored in Confluence. Required when `location` is `confluence`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConfluencePostmortemSettings {
    /// The ID of the Confluence integration account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The ID of the parent Confluence page under which postmortems are created.
    #[serde(
        rename = "parent_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub parent_id: Option<Option<String>>,
    /// The ID of the Confluence space where postmortems are created.
    #[serde(rename = "space_id")]
    pub space_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConfluencePostmortemSettings {
    pub fn new(account_id: String, space_id: String) -> ConfluencePostmortemSettings {
        ConfluencePostmortemSettings {
            account_id,
            parent_id: None,
            space_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn parent_id(mut self, value: Option<String>) -> Self {
        self.parent_id = Some(value);
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

impl<'de> Deserialize<'de> for ConfluencePostmortemSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfluencePostmortemSettingsVisitor;
        impl<'a> Visitor<'a> for ConfluencePostmortemSettingsVisitor {
            type Value = ConfluencePostmortemSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut parent_id: Option<Option<String>> = None;
                let mut space_id: Option<String> = None;
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
                        "parent_id" => {
                            parent_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_id" => {
                            space_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let space_id = space_id.ok_or_else(|| M::Error::missing_field("space_id"))?;

                let content = ConfluencePostmortemSettings {
                    account_id,
                    parent_id,
                    space_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConfluencePostmortemSettingsVisitor)
    }
}
