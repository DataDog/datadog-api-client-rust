// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The RUM data collection settings for the Synthetic browser test.
/// **Note:** There are 3 ways to format RUM settings:
///
/// `{ isEnabled: false }`
/// RUM data is not collected.
///
/// `{ isEnabled: true }`
/// RUM data is collected from the Synthetic test's default application.
///
/// `{ isEnabled: true, applicationId: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", clientTokenId: 12345 }`
/// RUM data is collected using the specified application.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserTestRumSettings {
    /// RUM application ID used to collect RUM data for the browser test.
    #[serde(rename = "applicationId")]
    pub application_id: Option<String>,
    /// RUM application API key ID used to collect RUM data for the browser test.
    #[serde(rename = "clientTokenId")]
    pub client_token_id: Option<i64>,
    /// Determines whether RUM data is collected during test runs.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserTestRumSettings {
    pub fn new(is_enabled: bool) -> SyntheticsBrowserTestRumSettings {
        SyntheticsBrowserTestRumSettings {
            application_id: None,
            client_token_id: None,
            is_enabled,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: String) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn client_token_id(mut self, value: i64) -> Self {
        self.client_token_id = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsBrowserTestRumSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserTestRumSettingsVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserTestRumSettingsVisitor {
            type Value = SyntheticsBrowserTestRumSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut client_token_id: Option<i64> = None;
                let mut is_enabled: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "applicationId" => {
                            if v.is_null() {
                                continue;
                            }
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "clientTokenId" => {
                            if v.is_null() {
                                continue;
                            }
                            client_token_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isEnabled" => {
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;

                let content = SyntheticsBrowserTestRumSettings {
                    application_id,
                    client_token_id,
                    is_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserTestRumSettingsVisitor)
    }
}
