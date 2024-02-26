// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing a monitor update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorUpdateRequest {
    /// Timestamp of the monitor creation.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV1::model::Creator>,
    /// Whether or not the monitor is deleted. (Always `null`)
    #[serde(
        rename = "deleted",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted: Option<Option<String>>,
    /// ID of this monitor.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// A message to include with notifications for this monitor.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Last timestamp when the monitor was edited.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
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
    #[serde(rename = "priority")]
    pub priority: Option<i64>,
    /// The monitor query.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// A list of unique role identifiers to define which roles are allowed to edit the monitor. The unique identifiers for all roles can be pulled from the [Roles API](<https://docs.datadoghq.com/api/latest/roles/#list-roles>) and are located in the `data.id` field. Editing a monitor includes any updates to the monitor configuration, monitor deletion, and muting of the monitor for any amount of time. `restricted_roles` is the successor of `locked`. For more information about `locked` and `restricted_roles`, see the [monitor options docs](<https://docs.datadoghq.com/monitors/guide/monitor_api_options/#permissions-options>).
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
}

impl MonitorUpdateRequest {
    pub fn new() -> MonitorUpdateRequest {
        MonitorUpdateRequest {
            created: None,
            creator: None,
            deleted: None,
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
        }
    }

    pub fn created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn creator(&mut self, value: crate::datadogV1::model::Creator) -> &mut Self {
        self.creator = Some(value);
        self
    }

    pub fn deleted(&mut self, value: Option<String>) -> &mut Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn modified(&mut self, value: String) -> &mut Self {
        self.modified = Some(value);
        self
    }

    pub fn multi(&mut self, value: bool) -> &mut Self {
        self.multi = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn options(&mut self, value: crate::datadogV1::model::MonitorOptions) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn overall_state(
        &mut self,
        value: crate::datadogV1::model::MonitorOverallStates,
    ) -> &mut Self {
        self.overall_state = Some(value);
        self
    }

    pub fn priority(&mut self, value: i64) -> &mut Self {
        self.priority = Some(value);
        self
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn restricted_roles(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.restricted_roles = Some(value);
        self
    }

    pub fn state(&mut self, value: crate::datadogV1::model::MonitorState) -> &mut Self {
        self.state = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV1::model::MonitorType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for MonitorUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
