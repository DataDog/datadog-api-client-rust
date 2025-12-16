// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Phone notification channel configuration
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotificationChannelPhoneConfig {
    /// The formatted international version of Number (e.g. +33 7 1 23 45 67).
    #[serde(rename = "formatted_number")]
    pub formatted_number: String,
    /// The E-164 formatted phone number (e.g. +3371234567)
    #[serde(rename = "number")]
    pub number: String,
    /// The ISO 3166-1 alpha-2 two-letter country code.
    #[serde(rename = "region")]
    pub region: String,
    /// If present, the date the user subscribed this number to SMS messages
    #[serde(
        rename = "sms_subscribed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub sms_subscribed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Indicates that the notification channel is a phone
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::NotificationChannelPhoneConfigType,
    /// Indicates whether this phone has been verified by the user in Datadog On-Call
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotificationChannelPhoneConfig {
    pub fn new(
        formatted_number: String,
        number: String,
        region: String,
        type_: crate::datadogV2::model::NotificationChannelPhoneConfigType,
        verified: bool,
    ) -> NotificationChannelPhoneConfig {
        NotificationChannelPhoneConfig {
            formatted_number,
            number,
            region,
            sms_subscribed_at: None,
            type_,
            verified,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn sms_subscribed_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.sms_subscribed_at = Some(value);
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

impl<'de> Deserialize<'de> for NotificationChannelPhoneConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotificationChannelPhoneConfigVisitor;
        impl<'a> Visitor<'a> for NotificationChannelPhoneConfigVisitor {
            type Value = NotificationChannelPhoneConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formatted_number: Option<String> = None;
                let mut number: Option<String> = None;
                let mut region: Option<String> = None;
                let mut sms_subscribed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut type_: Option<crate::datadogV2::model::NotificationChannelPhoneConfigType> =
                    None;
                let mut verified: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formatted_number" => {
                            formatted_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "number" => {
                            number = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sms_subscribed_at" => {
                            sms_subscribed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::NotificationChannelPhoneConfigType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "verified" => {
                            verified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let formatted_number =
                    formatted_number.ok_or_else(|| M::Error::missing_field("formatted_number"))?;
                let number = number.ok_or_else(|| M::Error::missing_field("number"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let verified = verified.ok_or_else(|| M::Error::missing_field("verified"))?;

                let content = NotificationChannelPhoneConfig {
                    formatted_number,
                    number,
                    region,
                    sms_subscribed_at,
                    type_,
                    verified,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotificationChannelPhoneConfigVisitor)
    }
}
