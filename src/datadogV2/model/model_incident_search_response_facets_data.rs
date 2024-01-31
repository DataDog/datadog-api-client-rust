// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Facet data for incidents returned by a search query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseFacetsData {
    /// Facet data for incident commander users.
    #[serde(rename = "commander")]
    pub commander: Option<Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>>,
    /// Facet data for incident creator users.
    #[serde(rename = "created_by")]
    pub created_by: Option<Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>>,
    /// Facet data for incident property fields.
    #[serde(rename = "fields")]
    pub fields: Option<Vec<crate::datadogV2::model::IncidentSearchResponsePropertyFieldFacetData>>,
    /// Facet data for incident impact attributes.
    #[serde(rename = "impact")]
    pub impact: Option<Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>>,
    /// Facet data for incident last modified by users.
    #[serde(rename = "last_modified_by")]
    pub last_modified_by: Option<Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>>,
    /// Facet data for incident postmortem existence.
    #[serde(rename = "postmortem")]
    pub postmortem: Option<Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>>,
    /// Facet data for incident responder users.
    #[serde(rename = "responder")]
    pub responder: Option<Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>>,
    /// Facet data for incident severity attributes.
    #[serde(rename = "severity")]
    pub severity: Option<Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>>,
    /// Facet data for incident state attributes.
    #[serde(rename = "state")]
    pub state: Option<Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>>,
    /// Facet data for incident time to repair metrics.
    #[serde(rename = "time_to_repair")]
    pub time_to_repair:
        Option<Vec<crate::datadogV2::model::IncidentSearchResponseNumericFacetData>>,
    /// Facet data for incident time to resolve metrics.
    #[serde(rename = "time_to_resolve")]
    pub time_to_resolve:
        Option<Vec<crate::datadogV2::model::IncidentSearchResponseNumericFacetData>>,
}

impl IncidentSearchResponseFacetsData {
    pub fn new() -> IncidentSearchResponseFacetsData {
        IncidentSearchResponseFacetsData {
            commander: None,
            created_by: None,
            fields: None,
            impact: None,
            last_modified_by: None,
            postmortem: None,
            responder: None,
            severity: None,
            state: None,
            time_to_repair: None,
            time_to_resolve: None,
        }
    }

    pub fn with_commander(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
    ) -> &mut Self {
        self.commander = Some(value);
        self
    }

    pub fn with_created_by(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
    ) -> &mut Self {
        self.created_by = Some(value);
        self
    }

    pub fn with_fields(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponsePropertyFieldFacetData>,
    ) -> &mut Self {
        self.fields = Some(value);
        self
    }

    pub fn with_impact(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    ) -> &mut Self {
        self.impact = Some(value);
        self
    }

    pub fn with_last_modified_by(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
    ) -> &mut Self {
        self.last_modified_by = Some(value);
        self
    }

    pub fn with_postmortem(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    ) -> &mut Self {
        self.postmortem = Some(value);
        self
    }

    pub fn with_responder(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
    ) -> &mut Self {
        self.responder = Some(value);
        self
    }

    pub fn with_severity(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    ) -> &mut Self {
        self.severity = Some(value);
        self
    }

    pub fn with_state(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    ) -> &mut Self {
        self.state = Some(value);
        self
    }

    pub fn with_time_to_repair(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseNumericFacetData>,
    ) -> &mut Self {
        self.time_to_repair = Some(value);
        self
    }

    pub fn with_time_to_resolve(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseNumericFacetData>,
    ) -> &mut Self {
        self.time_to_resolve = Some(value);
        self
    }
}
impl Default for IncidentSearchResponseFacetsData {
    fn default() -> Self {
        Self::new()
    }
}
