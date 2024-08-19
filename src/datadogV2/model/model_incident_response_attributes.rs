// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's attributes from a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentResponseAttributes {
    /// Timestamp of when the incident was archived.
    #[serde(
        rename = "archived",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub archived: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The incident case id.
    #[serde(
        rename = "case_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub case_id: Option<Option<i64>>,
    /// Timestamp when the incident was created.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// Length of the incident's customer impact in seconds.
    /// Equals the difference between `customer_impact_start` and `customer_impact_end`.
    #[serde(rename = "customer_impact_duration")]
    pub customer_impact_duration: Option<i64>,
    /// Timestamp when customers were no longer impacted by the incident.
    #[serde(
        rename = "customer_impact_end",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub customer_impact_end: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A summary of the impact customers experienced during the incident.
    #[serde(
        rename = "customer_impact_scope",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub customer_impact_scope: Option<Option<String>>,
    /// Timestamp when customers began being impacted by the incident.
    #[serde(
        rename = "customer_impact_start",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub customer_impact_start: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A flag indicating whether the incident caused customer impact.
    #[serde(rename = "customer_impacted")]
    pub customer_impacted: Option<bool>,
    /// Timestamp when the incident was detected.
    #[serde(
        rename = "detected",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub detected: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A condensed view of the user-defined fields attached to incidents.
    #[serde(rename = "fields")]
    pub fields: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    >,
    /// Timestamp when the incident was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// Incident's non Datadog creator.
    #[serde(
        rename = "non_datadog_creator",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub non_datadog_creator: Option<Option<crate::datadogV2::model::IncidentNonDatadogCreator>>,
    /// Notification handles that will be notified of the incident during update.
    #[serde(
        rename = "notification_handles",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub notification_handles:
        Option<Option<Vec<crate::datadogV2::model::IncidentNotificationHandle>>>,
    /// The monotonically increasing integer ID for the incident.
    #[serde(rename = "public_id")]
    pub public_id: Option<i64>,
    /// Timestamp when the incident's state was last changed from active or stable to resolved or completed.
    #[serde(
        rename = "resolved",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub resolved: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The incident severity.
    #[serde(rename = "severity")]
    pub severity: Option<crate::datadogV2::model::IncidentSeverity>,
    /// The state incident.
    #[serde(rename = "state", default, with = "::serde_with::rust::double_option")]
    pub state: Option<Option<String>>,
    /// The amount of time in seconds to detect the incident.
    /// Equals the difference between `customer_impact_start` and `detected`.
    #[serde(rename = "time_to_detect")]
    pub time_to_detect: Option<i64>,
    /// The amount of time in seconds to call incident after detection. Equals the difference of `detected` and `created`.
    #[serde(rename = "time_to_internal_response")]
    pub time_to_internal_response: Option<i64>,
    /// The amount of time in seconds to resolve customer impact after detecting the issue. Equals the difference between `customer_impact_end` and `detected`.
    #[serde(rename = "time_to_repair")]
    pub time_to_repair: Option<i64>,
    /// The amount of time in seconds to resolve the incident after it was created. Equals the difference between `created` and `resolved`.
    #[serde(rename = "time_to_resolve")]
    pub time_to_resolve: Option<i64>,
    /// The title of the incident, which summarizes what happened.
    #[serde(rename = "title")]
    pub title: String,
    /// The incident visibility status.
    #[serde(
        rename = "visibility",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub visibility: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentResponseAttributes {
    pub fn new(title: String) -> IncidentResponseAttributes {
        IncidentResponseAttributes {
            archived: None,
            case_id: None,
            created: None,
            customer_impact_duration: None,
            customer_impact_end: None,
            customer_impact_scope: None,
            customer_impact_start: None,
            customer_impacted: None,
            detected: None,
            fields: None,
            modified: None,
            non_datadog_creator: None,
            notification_handles: None,
            public_id: None,
            resolved: None,
            severity: None,
            state: None,
            time_to_detect: None,
            time_to_internal_response: None,
            time_to_repair: None,
            time_to_resolve: None,
            title,
            visibility: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn archived(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn case_id(mut self, value: Option<i64>) -> Self {
        self.case_id = Some(value);
        self
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn customer_impact_duration(mut self, value: i64) -> Self {
        self.customer_impact_duration = Some(value);
        self
    }

    pub fn customer_impact_end(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.customer_impact_end = Some(value);
        self
    }

    pub fn customer_impact_scope(mut self, value: Option<String>) -> Self {
        self.customer_impact_scope = Some(value);
        self
    }

    pub fn customer_impact_start(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.customer_impact_start = Some(value);
        self
    }

    pub fn customer_impacted(mut self, value: bool) -> Self {
        self.customer_impacted = Some(value);
        self
    }

    pub fn detected(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.detected = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn non_datadog_creator(
        mut self,
        value: Option<crate::datadogV2::model::IncidentNonDatadogCreator>,
    ) -> Self {
        self.non_datadog_creator = Some(value);
        self
    }

    pub fn notification_handles(
        mut self,
        value: Option<Vec<crate::datadogV2::model::IncidentNotificationHandle>>,
    ) -> Self {
        self.notification_handles = Some(value);
        self
    }

    pub fn public_id(mut self, value: i64) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn resolved(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.resolved = Some(value);
        self
    }

    pub fn severity(mut self, value: crate::datadogV2::model::IncidentSeverity) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn state(mut self, value: Option<String>) -> Self {
        self.state = Some(value);
        self
    }

    pub fn time_to_detect(mut self, value: i64) -> Self {
        self.time_to_detect = Some(value);
        self
    }

    pub fn time_to_internal_response(mut self, value: i64) -> Self {
        self.time_to_internal_response = Some(value);
        self
    }

    pub fn time_to_repair(mut self, value: i64) -> Self {
        self.time_to_repair = Some(value);
        self
    }

    pub fn time_to_resolve(mut self, value: i64) -> Self {
        self.time_to_resolve = Some(value);
        self
    }

    pub fn visibility(mut self, value: Option<String>) -> Self {
        self.visibility = Some(value);
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

impl<'de> Deserialize<'de> for IncidentResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentResponseAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentResponseAttributesVisitor {
            type Value = IncidentResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archived: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut case_id: Option<Option<i64>> = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut customer_impact_duration: Option<i64> = None;
                let mut customer_impact_end: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut customer_impact_scope: Option<Option<String>> = None;
                let mut customer_impact_start: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut customer_impacted: Option<bool> = None;
                let mut detected: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut fields: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::IncidentFieldAttributes,
                    >,
                > = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut non_datadog_creator: Option<
                    Option<crate::datadogV2::model::IncidentNonDatadogCreator>,
                > = None;
                let mut notification_handles: Option<
                    Option<Vec<crate::datadogV2::model::IncidentNotificationHandle>>,
                > = None;
                let mut public_id: Option<i64> = None;
                let mut resolved: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut severity: Option<crate::datadogV2::model::IncidentSeverity> = None;
                let mut state: Option<Option<String>> = None;
                let mut time_to_detect: Option<i64> = None;
                let mut time_to_internal_response: Option<i64> = None;
                let mut time_to_repair: Option<i64> = None;
                let mut time_to_resolve: Option<i64> = None;
                let mut title: Option<String> = None;
                let mut visibility: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archived" => {
                            archived = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "case_id" => {
                            case_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_impact_duration" => {
                            if v.is_null() {
                                continue;
                            }
                            customer_impact_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_impact_end" => {
                            customer_impact_end =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_impact_scope" => {
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
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "non_datadog_creator" => {
                            non_datadog_creator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notification_handles" => {
                            notification_handles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolved" => {
                            resolved = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            if v.is_null() {
                                continue;
                            }
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _severity) = severity {
                                match _severity {
                                    crate::datadogV2::model::IncidentSeverity::UnparsedObject(
                                        _severity,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_to_detect" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_detect =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_to_internal_response" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_internal_response =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_to_repair" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_repair =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_to_resolve" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_resolve =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "visibility" => {
                            visibility = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = IncidentResponseAttributes {
                    archived,
                    case_id,
                    created,
                    customer_impact_duration,
                    customer_impact_end,
                    customer_impact_scope,
                    customer_impact_start,
                    customer_impacted,
                    detected,
                    fields,
                    modified,
                    non_datadog_creator,
                    notification_handles,
                    public_id,
                    resolved,
                    severity,
                    state,
                    time_to_detect,
                    time_to_internal_response,
                    time_to_repair,
                    time_to_resolve,
                    title,
                    visibility,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentResponseAttributesVisitor)
    }
}
