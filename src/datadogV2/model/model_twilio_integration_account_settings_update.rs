// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for updating the Twilio integration account. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TwilioIntegrationAccountSettingsUpdate {
    /// Twilio Account SID that uniquely identifies your Twilio account.
    #[serde(rename = "account_sid")]
    pub account_sid: Option<String>,
    /// When enabled, Twilio phone numbers in the `to` field and SMS message bodies are censored for privacy.
    #[serde(rename = "censor_logs")]
    pub censor_logs: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TwilioIntegrationAccountSettingsUpdate {
    pub fn new() -> TwilioIntegrationAccountSettingsUpdate {
        TwilioIntegrationAccountSettingsUpdate {
            account_sid: None,
            censor_logs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_sid(mut self, value: String) -> Self {
        self.account_sid = Some(value);
        self
    }

    pub fn censor_logs(mut self, value: bool) -> Self {
        self.censor_logs = Some(value);
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

impl Default for TwilioIntegrationAccountSettingsUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TwilioIntegrationAccountSettingsUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwilioIntegrationAccountSettingsUpdateVisitor;
        impl<'a> Visitor<'a> for TwilioIntegrationAccountSettingsUpdateVisitor {
            type Value = TwilioIntegrationAccountSettingsUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_sid: Option<String> = None;
                let mut censor_logs: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_sid" => {
                            if v.is_null() {
                                continue;
                            }
                            account_sid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "censor_logs" => {
                            if v.is_null() {
                                continue;
                            }
                            censor_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TwilioIntegrationAccountSettingsUpdate {
                    account_sid,
                    censor_logs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TwilioIntegrationAccountSettingsUpdateVisitor)
    }
}
