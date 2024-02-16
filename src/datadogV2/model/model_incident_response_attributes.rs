// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident's attributes from a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentResponseAttributes {
    /// Timestamp of when the incident was archived.
    #[serde(
        rename = "archived",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub archived: Option<Option<String>>,
    /// The incident case id.
    #[serde(
        rename = "case_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub case_id: Option<Option<i64>>,
    /// Timestamp when the incident was created.
    #[serde(rename = "created")]
    pub created: Option<String>,
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
    pub customer_impact_end: Option<Option<String>>,
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
    /// A condensed view of the user-defined fields attached to incidents.
    #[serde(rename = "fields")]
    pub fields: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    >,
    /// Timestamp when the incident was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
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
    pub resolved: Option<Option<String>>,
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
        }
    }

    pub fn archived(&mut self, value: Option<String>) -> &mut Self {
        self.archived = Some(value);
        self
    }

    pub fn case_id(&mut self, value: Option<i64>) -> &mut Self {
        self.case_id = Some(value);
        self
    }

    pub fn created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn customer_impact_duration(&mut self, value: i64) -> &mut Self {
        self.customer_impact_duration = Some(value);
        self
    }

    pub fn customer_impact_end(&mut self, value: Option<String>) -> &mut Self {
        self.customer_impact_end = Some(value);
        self
    }

    pub fn customer_impact_scope(&mut self, value: Option<String>) -> &mut Self {
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

    pub fn modified(&mut self, value: String) -> &mut Self {
        self.modified = Some(value);
        self
    }

    pub fn non_datadog_creator(
        &mut self,
        value: Option<crate::datadogV2::model::IncidentNonDatadogCreator>,
    ) -> &mut Self {
        self.non_datadog_creator = Some(value);
        self
    }

    pub fn notification_handles(
        &mut self,
        value: Option<Vec<crate::datadogV2::model::IncidentNotificationHandle>>,
    ) -> &mut Self {
        self.notification_handles = Some(value);
        self
    }

    pub fn public_id(&mut self, value: i64) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn resolved(&mut self, value: Option<String>) -> &mut Self {
        self.resolved = Some(value);
        self
    }

    pub fn severity(&mut self, value: crate::datadogV2::model::IncidentSeverity) -> &mut Self {
        self.severity = Some(value);
        self
    }

    pub fn state(&mut self, value: Option<String>) -> &mut Self {
        self.state = Some(value);
        self
    }

    pub fn time_to_detect(&mut self, value: i64) -> &mut Self {
        self.time_to_detect = Some(value);
        self
    }

    pub fn time_to_internal_response(&mut self, value: i64) -> &mut Self {
        self.time_to_internal_response = Some(value);
        self
    }

    pub fn time_to_repair(&mut self, value: i64) -> &mut Self {
        self.time_to_repair = Some(value);
        self
    }

    pub fn time_to_resolve(&mut self, value: i64) -> &mut Self {
        self.time_to_resolve = Some(value);
        self
    }

    pub fn visibility(&mut self, value: Option<String>) -> &mut Self {
        self.visibility = Some(value);
        self
    }
}
