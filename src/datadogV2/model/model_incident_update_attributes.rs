// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident's attributes for an update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }

    pub fn customer_impact_end(mut self, value: Option<String>) -> Self {
        self.customer_impact_end = Some(value);
        self
    }

    pub fn customer_impact_scope(mut self, value: String) -> Self {
        self.customer_impact_scope = Some(value);
        self
    }

    pub fn customer_impact_start(mut self, value: Option<String>) -> Self {
        self.customer_impact_start = Some(value);
        self
    }

    pub fn customer_impacted(mut self, value: bool) -> Self {
        self.customer_impacted = Some(value);
        self
    }

    pub fn detected(mut self, value: Option<String>) -> Self {
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

    pub fn notification_handles(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentNotificationHandle>,
    ) -> Self {
        self.notification_handles = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
}

impl Default for IncidentUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}
