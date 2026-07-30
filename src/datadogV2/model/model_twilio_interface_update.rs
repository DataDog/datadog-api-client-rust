// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Partial Twilio interface (source-type) configuration for updates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TwilioInterfaceUpdate {
    /// Authentication methods supported by the Twilio interface. Exactly one is set, selected by its `type`.
    #[serde(rename = "authentication")]
    pub authentication: Option<crate::datadogV2::model::TwilioAuthentication>,
    /// Dataflows for the Twilio interface.
    #[serde(rename = "dataflows")]
    pub dataflows: Option<Vec<crate::datadogV2::model::TwilioDataflow>>,
    /// Partial Twilio interface settings for updates.
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV2::model::TwilioSettingsUpdate>,
    /// Interface discriminator for Twilio.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TwilioInterfaceType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TwilioInterfaceUpdate {
    pub fn new(type_: crate::datadogV2::model::TwilioInterfaceType) -> TwilioInterfaceUpdate {
        TwilioInterfaceUpdate {
            authentication: None,
            dataflows: None,
            settings: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn authentication(mut self, value: crate::datadogV2::model::TwilioAuthentication) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn dataflows(mut self, value: Vec<crate::datadogV2::model::TwilioDataflow>) -> Self {
        self.dataflows = Some(value);
        self
    }

    pub fn settings(mut self, value: crate::datadogV2::model::TwilioSettingsUpdate) -> Self {
        self.settings = Some(value);
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

impl<'de> Deserialize<'de> for TwilioInterfaceUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwilioInterfaceUpdateVisitor;
        impl<'a> Visitor<'a> for TwilioInterfaceUpdateVisitor {
            type Value = TwilioInterfaceUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<crate::datadogV2::model::TwilioAuthentication> =
                    None;
                let mut dataflows: Option<Vec<crate::datadogV2::model::TwilioDataflow>> = None;
                let mut settings: Option<crate::datadogV2::model::TwilioSettingsUpdate> = None;
                let mut type_: Option<crate::datadogV2::model::TwilioInterfaceType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            if v.is_null() {
                                continue;
                            }
                            authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _authentication) = authentication {
                                match _authentication {
                                    crate::datadogV2::model::TwilioAuthentication::UnparsedObject(_authentication) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "dataflows" => {
                            if v.is_null() {
                                continue;
                            }
                            dataflows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TwilioInterfaceType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TwilioInterfaceUpdate {
                    authentication,
                    dataflows,
                    settings,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TwilioInterfaceUpdateVisitor)
    }
}
