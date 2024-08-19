// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Facet data for incidents returned by a search query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn commander(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
    ) -> Self {
        self.commander = Some(value);
        self
    }

    pub fn created_by(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
    ) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponsePropertyFieldFacetData>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn impact(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    ) -> Self {
        self.impact = Some(value);
        self
    }

    pub fn last_modified_by(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
    ) -> Self {
        self.last_modified_by = Some(value);
        self
    }

    pub fn postmortem(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    ) -> Self {
        self.postmortem = Some(value);
        self
    }

    pub fn responder(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
    ) -> Self {
        self.responder = Some(value);
        self
    }

    pub fn severity(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    ) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn state(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    ) -> Self {
        self.state = Some(value);
        self
    }

    pub fn time_to_repair(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseNumericFacetData>,
    ) -> Self {
        self.time_to_repair = Some(value);
        self
    }

    pub fn time_to_resolve(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentSearchResponseNumericFacetData>,
    ) -> Self {
        self.time_to_resolve = Some(value);
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

impl Default for IncidentSearchResponseFacetsData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentSearchResponseFacetsData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentSearchResponseFacetsDataVisitor;
        impl<'a> Visitor<'a> for IncidentSearchResponseFacetsDataVisitor {
            type Value = IncidentSearchResponseFacetsData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commander: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
                > = None;
                let mut created_by: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
                > = None;
                let mut fields: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponsePropertyFieldFacetData>,
                > = None;
                let mut impact: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
                > = None;
                let mut last_modified_by: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
                > = None;
                let mut postmortem: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
                > = None;
                let mut responder: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseUserFacetData>,
                > = None;
                let mut severity: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
                > = None;
                let mut state: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
                > = None;
                let mut time_to_repair: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseNumericFacetData>,
                > = None;
                let mut time_to_resolve: Option<
                    Vec<crate::datadogV2::model::IncidentSearchResponseNumericFacetData>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "commander" => {
                            if v.is_null() {
                                continue;
                            }
                            commander = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact" => {
                            if v.is_null() {
                                continue;
                            }
                            impact = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "postmortem" => {
                            if v.is_null() {
                                continue;
                            }
                            postmortem = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responder" => {
                            if v.is_null() {
                                continue;
                            }
                            responder = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            if v.is_null() {
                                continue;
                            }
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentSearchResponseFacetsData {
                    commander,
                    created_by,
                    fields,
                    impact,
                    last_modified_by,
                    postmortem,
                    responder,
                    severity,
                    state,
                    time_to_repair,
                    time_to_resolve,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentSearchResponseFacetsDataVisitor)
    }
}
