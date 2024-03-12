// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident's attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCreateAttributes {
    /// Required if `customer_impacted:"true"`. A summary of the impact customers experienced during the incident.
    #[serde(rename = "customer_impact_scope")]
    pub customer_impact_scope: Option<String>,
    /// A flag indicating whether the incident caused customer impact.
    #[serde(rename = "customer_impacted")]
    pub customer_impacted: bool,
    /// A condensed view of the user-defined fields for which to create initial selections.
    #[serde(rename = "fields")]
    pub fields: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    >,
    /// An array of initial timeline cells to be placed at the beginning of the incident timeline.
    #[serde(rename = "initial_cells")]
    pub initial_cells: Option<Vec<crate::datadogV2::model::IncidentTimelineCellCreateAttributes>>,
    /// Notification handles that will be notified of the incident at creation.
    #[serde(rename = "notification_handles")]
    pub notification_handles: Option<Vec<crate::datadogV2::model::IncidentNotificationHandle>>,
    /// The title of the incident, which summarizes what happened.
    #[serde(rename = "title")]
    pub title: String,
}

impl IncidentCreateAttributes {
    pub fn new(customer_impacted: bool, title: String) -> IncidentCreateAttributes {
        IncidentCreateAttributes {
            customer_impact_scope: None,
            customer_impacted,
            fields: None,
            initial_cells: None,
            notification_handles: None,
            title,
        }
    }

    pub fn customer_impact_scope(mut self, value: String) -> Self {
        self.customer_impact_scope = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn initial_cells(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentTimelineCellCreateAttributes>,
    ) -> Self {
        self.initial_cells = Some(value);
        self
    }

    pub fn notification_handles(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentNotificationHandle>,
    ) -> Self {
        self.notification_handles = Some(value);
        self
    }
}
