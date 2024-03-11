// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's attributes for an update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUpdateAttributes {
    /// Timestamp when customers were no longer impacted by the incident.
    #[serde(
        rename = "customer_impact_end",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub customer_impact_end: Option<Option<String>>,
    /// A summary of the impact customers experienced during the incident.
    #[serde(rename = "customer_impact_scope")]
    pub customer_impact_scope: Option<String>,
    /// Timestamp when customers began being impacted by the incident.
    #[serde(
        rename = "customer_impact_start",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub customer_impact_start: Option<Option<String>>,
    /// A flag indicating whether the incident caused customer impact.
    #[serde(rename = "customer_impacted")]
    pub customer_impacted: Option<bool>,
    /// Timestamp when the incident was detected.
    #[serde(
        rename = "detected",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub detected: Option<Option<String>>,
    /// A condensed view of the user-defined fields for which to update selections.
    #[serde(rename = "fields")]
    pub fields: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    >,
    /// Notification handles that will be notified of the incident during update.
    #[serde(rename = "notification_handles")]
    pub notification_handles: Option<Vec<crate::datadogV2::model::IncidentNotificationHandle>>,
    /// The title of the incident, which summarizes what happened.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUpdateAttributes {
    pub fn new() -> IncidentUpdateAttributes {
        IncidentUpdateAttributes {
            customer_impact_end: None,
            customer_impact_scope: None,
            customer_impact_start: None,
            customer_impacted: None,
            detected: None,
            fields: None,
            notification_handles: None,
            title: None,
            _unparsed: false,
        }
    }

    pub fn customer_impact_end(&mut self, value: Option<String>) -> &mut Self {
        self.customer_impact_end = Some(value);
        self
    }

    pub fn customer_impact_scope(&mut self, value: String) -> &mut Self {
        self.customer_impact_scope = Some(value);
        self
    }

    pub fn customer_impact_start(&mut self, value: Option<String>) -> &mut Self {
        self.customer_impact_start = Some(value);
        self
    }

    pub fn customer_impacted(&mut self, value: bool) -> &mut Self {
        self.customer_impacted = Some(value);
        self
    }

    pub fn detected(&mut self, value: Option<String>) -> &mut Self {
        self.detected = Some(value);
        self
    }

    pub fn fields(
        &mut self,
        value: std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    ) -> &mut Self {
        self.fields = Some(value);
        self
    }

    pub fn notification_handles(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentNotificationHandle>,
    ) -> &mut Self {
        self.notification_handles = Some(value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }
}

impl Default for IncidentUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentUpdateAttributesVisitor {
            type Value = IncidentUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut customer_impact_end: Option<Option<String>> = None;
                let mut customer_impact_scope: Option<String> = None;
                let mut customer_impact_start: Option<Option<String>> = None;
                let mut customer_impacted: Option<bool> = None;
                let mut detected: Option<Option<String>> = None;
                let mut fields: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::IncidentFieldAttributes,
                    >,
                > = None;
                let mut notification_handles: Option<
                    Vec<crate::datadogV2::model::IncidentNotificationHandle>,
                > = None;
                let mut title: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "customer_impact_end" => {
                            customer_impact_end =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_impact_scope" => {
                            if v.is_null() {
                                continue;
                            }
                            customer_impact_scope =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_impact_start" => {
                            customer_impact_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_impacted" => {
                            if v.is_null() {
                                continue;
                            }
                            customer_impacted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detected" => {
                            detected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notification_handles" => {
                            if v.is_null() {
                                continue;
                            }
                            notification_handles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IncidentUpdateAttributes {
                    customer_impact_end,
                    customer_impact_scope,
                    customer_impact_start,
                    customer_impacted,
                    detected,
                    fields,
                    notification_handles,
                    title,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUpdateAttributesVisitor)
    }
}
