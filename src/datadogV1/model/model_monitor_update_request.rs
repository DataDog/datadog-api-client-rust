// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a monitor update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorUpdateRequest {
    /// Timestamp of the monitor creation.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV1::model::Creator>,
    /// Whether or not the monitor is deleted. (Always `null`)
    #[serde(
        rename = "deleted",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Indicates whether the monitor is in a draft or published state.
    ///
    /// `draft`: The monitor appears as Draft and does not send notifications.
    /// `published`: The monitor is active and evaluates conditions and notify as configured.
    ///
    /// This field is in preview. The draft value is only available to customers with the feature enabled.
    #[serde(rename = "draft_status")]
    pub draft_status: Option<crate::datadogV1::model::MonitorDraftStatus>,
    /// ID of this monitor.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// A message to include with notifications for this monitor.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Last timestamp when the monitor was edited.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether or not the monitor is broken down on different groups.
    #[serde(rename = "multi")]
    pub multi: Option<bool>,
    /// The monitor name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of options associated with your monitor.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV1::model::MonitorOptions>,
    /// The different states your monitor can be in.
    #[serde(rename = "overall_state")]
    pub overall_state: Option<crate::datadogV1::model::MonitorOverallStates>,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    #[serde(
        rename = "priority",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<i64>>,
    /// The monitor query.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// A list of unique role identifiers to define which roles are allowed to edit the monitor. The unique identifiers for all roles can be pulled from the [Roles API](<https://docs.datadoghq.com/api/latest/roles/#list-roles>) and are located in the `data.id` field. Editing a monitor includes any updates to the monitor configuration, monitor deletion, and muting of the monitor for any amount of time. You can use the [Restriction Policies API](<https://docs.datadoghq.com/api/latest/restriction-policies/>) to manage write authorization for individual monitors by teams and users, in addition to roles.
    #[serde(
        rename = "restricted_roles",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub restricted_roles: Option<Option<Vec<String>>>,
    /// Wrapper object with the different monitor states.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV1::model::MonitorState>,
    /// Tags associated to your monitor.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of the monitor. For more information about `type`, see the [monitor options](<https://docs.datadoghq.com/monitors/guide/monitor_api_options/>) docs.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::MonitorType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorUpdateRequest {
    pub fn new() -> MonitorUpdateRequest {
        MonitorUpdateRequest {
            created: None,
            creator: None,
            deleted: None,
            draft_status: None,
            id: None,
            message: None,
            modified: None,
            multi: None,
            name: None,
            options: None,
            overall_state: None,
            priority: None,
            query: None,
            restricted_roles: None,
            state: None,
            tags: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn creator(mut self, value: crate::datadogV1::model::Creator) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn deleted(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn draft_status(mut self, value: crate::datadogV1::model::MonitorDraftStatus) -> Self {
        self.draft_status = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn multi(mut self, value: bool) -> Self {
        self.multi = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV1::model::MonitorOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn overall_state(mut self, value: crate::datadogV1::model::MonitorOverallStates) -> Self {
        self.overall_state = Some(value);
        self
    }

    pub fn priority(mut self, value: Option<i64>) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn restricted_roles(mut self, value: Option<Vec<String>>) -> Self {
        self.restricted_roles = Some(value);
        self
    }

    pub fn state(mut self, value: crate::datadogV1::model::MonitorState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::MonitorType) -> Self {
        self.type_ = Some(value);
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

impl Default for MonitorUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorUpdateRequestVisitor;
        impl<'a> Visitor<'a> for MonitorUpdateRequestVisitor {
            type Value = MonitorUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut creator: Option<crate::datadogV1::model::Creator> = None;
                let mut deleted: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut draft_status: Option<crate::datadogV1::model::MonitorDraftStatus> = None;
                let mut id: Option<i64> = None;
                let mut message: Option<String> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut multi: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV1::model::MonitorOptions> = None;
                let mut overall_state: Option<crate::datadogV1::model::MonitorOverallStates> = None;
                let mut priority: Option<Option<i64>> = None;
                let mut query: Option<String> = None;
                let mut restricted_roles: Option<Option<Vec<String>>> = None;
                let mut state: Option<crate::datadogV1::model::MonitorState> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV1::model::MonitorType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted" => {
                            deleted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "draft_status" => {
                            if v.is_null() {
                                continue;
                            }
                            draft_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _draft_status) = draft_status {
                                match _draft_status {
                                    crate::datadogV1::model::MonitorDraftStatus::UnparsedObject(
                                        _draft_status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multi" => {
                            if v.is_null() {
                                continue;
                            }
                            multi = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overall_state" => {
                            if v.is_null() {
                                continue;
                            }
                            overall_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _overall_state) = overall_state {
                                match _overall_state {
                                    crate::datadogV1::model::MonitorOverallStates::UnparsedObject(_overall_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "priority" => {
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restricted_roles" => {
                            restricted_roles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::MonitorType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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

                let content = MonitorUpdateRequest {
                    created,
                    creator,
                    deleted,
                    draft_status,
                    id,
                    message,
                    modified,
                    multi,
                    name,
                    options,
                    overall_state,
                    priority,
                    query,
                    restricted_roles,
                    state,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorUpdateRequestVisitor)
    }
}
