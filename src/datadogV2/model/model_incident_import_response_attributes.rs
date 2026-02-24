// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's attributes from an import response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentImportResponseAttributes {
    /// Timestamp when the incident was archived.
    #[serde(
        rename = "archived",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub archived: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The incident case ID.
    #[serde(
        rename = "case_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub case_id: Option<Option<i64>>,
    /// Timestamp when the incident was created.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// UUID of the user who created the incident.
    #[serde(
        rename = "created_by_uuid",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub created_by_uuid: Option<Option<String>>,
    /// A unique key used to ensure idempotent incident creation.
    #[serde(
        rename = "creation_idempotency_key",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub creation_idempotency_key: Option<Option<String>>,
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
    /// Timestamp when customers began to be impacted by the incident.
    #[serde(
        rename = "customer_impact_start",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub customer_impact_start: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Timestamp when the incident was declared.
    #[serde(
        rename = "declared",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub declared: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// UUID of the user who declared the incident.
    #[serde(
        rename = "declared_by_uuid",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub declared_by_uuid: Option<Option<String>>,
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
    /// A unique identifier that represents an incident type.
    #[serde(rename = "incident_type_uuid")]
    pub incident_type_uuid: Option<String>,
    /// A flag indicating whether the incident is a test incident.
    #[serde(rename = "is_test")]
    pub is_test: Option<bool>,
    /// UUID of the user who last modified the incident.
    #[serde(
        rename = "last_modified_by_uuid",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_modified_by_uuid: Option<Option<String>>,
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
    /// Notification handles that are notified of the incident during update.
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
    /// The state of the incident.
    #[serde(rename = "state", default, with = "::serde_with::rust::double_option")]
    pub state: Option<Option<String>>,
    /// The title of the incident that summarizes what happened.
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

impl IncidentImportResponseAttributes {
    pub fn new(title: String) -> IncidentImportResponseAttributes {
        IncidentImportResponseAttributes {
            archived: None,
            case_id: None,
            created: None,
            created_by_uuid: None,
            creation_idempotency_key: None,
            customer_impact_end: None,
            customer_impact_scope: None,
            customer_impact_start: None,
            declared: None,
            declared_by_uuid: None,
            detected: None,
            fields: None,
            incident_type_uuid: None,
            is_test: None,
            last_modified_by_uuid: None,
            modified: None,
            non_datadog_creator: None,
            notification_handles: None,
            public_id: None,
            resolved: None,
            severity: None,
            state: None,
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

    pub fn created_by_uuid(mut self, value: Option<String>) -> Self {
        self.created_by_uuid = Some(value);
        self
    }

    pub fn creation_idempotency_key(mut self, value: Option<String>) -> Self {
        self.creation_idempotency_key = Some(value);
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

    pub fn declared(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.declared = Some(value);
        self
    }

    pub fn declared_by_uuid(mut self, value: Option<String>) -> Self {
        self.declared_by_uuid = Some(value);
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

    pub fn incident_type_uuid(mut self, value: String) -> Self {
        self.incident_type_uuid = Some(value);
        self
    }

    pub fn is_test(mut self, value: bool) -> Self {
        self.is_test = Some(value);
        self
    }

    pub fn last_modified_by_uuid(mut self, value: Option<String>) -> Self {
        self.last_modified_by_uuid = Some(value);
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

impl<'de> Deserialize<'de> for IncidentImportResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentImportResponseAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentImportResponseAttributesVisitor {
            type Value = IncidentImportResponseAttributes;

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
                let mut created_by_uuid: Option<Option<String>> = None;
                let mut creation_idempotency_key: Option<Option<String>> = None;
                let mut customer_impact_end: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut customer_impact_scope: Option<Option<String>> = None;
                let mut customer_impact_start: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut declared: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut declared_by_uuid: Option<Option<String>> = None;
                let mut detected: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut fields: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::IncidentFieldAttributes,
                    >,
                > = None;
                let mut incident_type_uuid: Option<String> = None;
                let mut is_test: Option<bool> = None;
                let mut last_modified_by_uuid: Option<Option<String>> = None;
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
                        "created_by_uuid" => {
                            created_by_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creation_idempotency_key" => {
                            creation_idempotency_key =
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
                        "declared" => {
                            declared = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "declared_by_uuid" => {
                            declared_by_uuid =
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
                        "incident_type_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_type_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_test" => {
                            if v.is_null() {
                                continue;
                            }
                            is_test = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_uuid" => {
                            last_modified_by_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = IncidentImportResponseAttributes {
                    archived,
                    case_id,
                    created,
                    created_by_uuid,
                    creation_idempotency_key,
                    customer_impact_end,
                    customer_impact_scope,
                    customer_impact_start,
                    declared,
                    declared_by_uuid,
                    detected,
                    fields,
                    incident_type_uuid,
                    is_test,
                    last_modified_by_uuid,
                    modified,
                    non_datadog_creator,
                    notification_handles,
                    public_id,
                    resolved,
                    severity,
                    state,
                    title,
                    visibility,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentImportResponseAttributesVisitor)
    }
}
