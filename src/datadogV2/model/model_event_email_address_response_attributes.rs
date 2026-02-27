// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an event email address resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventEmailAddressResponseAttributes {
    /// The alert type of events generated from the email address.
    #[serde(
        rename = "alert_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub alert_type: Option<Option<crate::datadogV2::model::EventEmailAddressAlertType>>,
    /// The timestamp of when the event email address was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A description of the event email address.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The generated email address for ingesting events.
    #[serde(rename = "email")]
    pub email: String,
    /// The format of events ingested through the email address.
    #[serde(rename = "format")]
    pub format: crate::datadogV2::model::EventEmailAddressFormat,
    /// The timestamp of when the event email address was last used.
    #[serde(
        rename = "last_used_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_used_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A list of handles to notify when an email is received.
    #[serde(rename = "notify_handles")]
    pub notify_handles: Option<Vec<String>>,
    /// The timestamp of when the event email address was revoked.
    #[serde(
        rename = "revoked_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub revoked_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A list of tags to apply to events generated from the email address.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventEmailAddressResponseAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        email: String,
        format: crate::datadogV2::model::EventEmailAddressFormat,
    ) -> EventEmailAddressResponseAttributes {
        EventEmailAddressResponseAttributes {
            alert_type: None,
            created_at,
            description: None,
            email,
            format,
            last_used_at: None,
            notify_handles: None,
            revoked_at: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alert_type(
        mut self,
        value: Option<crate::datadogV2::model::EventEmailAddressAlertType>,
    ) -> Self {
        self.alert_type = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn last_used_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    pub fn notify_handles(mut self, value: Vec<String>) -> Self {
        self.notify_handles = Some(value);
        self
    }

    pub fn revoked_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for EventEmailAddressResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventEmailAddressResponseAttributesVisitor;
        impl<'a> Visitor<'a> for EventEmailAddressResponseAttributesVisitor {
            type Value = EventEmailAddressResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alert_type: Option<
                    Option<crate::datadogV2::model::EventEmailAddressAlertType>,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<Option<String>> = None;
                let mut email: Option<String> = None;
                let mut format: Option<crate::datadogV2::model::EventEmailAddressFormat> = None;
                let mut last_used_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut notify_handles: Option<Vec<String>> = None;
                let mut revoked_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alert_type" => {
                            alert_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _alert_type) = alert_type {
                                match _alert_type {
                                    Some(crate::datadogV2::model::EventEmailAddressAlertType::UnparsedObject(_alert_type)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "format" => {
                            format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _format) = format {
                                match _format {
                                    crate::datadogV2::model::EventEmailAddressFormat::UnparsedObject(_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "last_used_at" => {
                            last_used_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_handles" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_handles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revoked_at" => {
                            revoked_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let email = email.ok_or_else(|| M::Error::missing_field("email"))?;
                let format = format.ok_or_else(|| M::Error::missing_field("format"))?;

                let content = EventEmailAddressResponseAttributes {
                    alert_type,
                    created_at,
                    description,
                    email,
                    format,
                    last_used_at,
                    notify_handles,
                    revoked_at,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventEmailAddressResponseAttributesVisitor)
    }
}
