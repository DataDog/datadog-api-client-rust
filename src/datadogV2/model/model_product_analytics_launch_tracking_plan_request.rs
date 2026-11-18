// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Tracking plan of a launch, listing the events and properties to instrument.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchTrackingPlanRequest {
    /// Latest inspection of the repository for the tracking plan instrumentation. A new analysis replaces the stored
    /// one.
    /// Events and properties that it does not list are not verified.
    #[serde(rename = "code_analysis")]
    pub code_analysis: Option<crate::datadogV2::model::ProductAnalyticsLaunchCodeAnalysis>,
    /// ID of the code generation session that implemented the plan.
    #[serde(rename = "codegen_session_id")]
    pub codegen_session_id: Option<String>,
    /// Events to instrument.
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchEvent>>,
    /// ID of the Jira ticket that tracks the instrumentation work.
    #[serde(rename = "jira_ticket_id")]
    pub jira_ticket_id: Option<String>,
    /// Link to the pull request that implements the plan.
    #[serde(rename = "pr_link")]
    pub pr_link: Option<String>,
    /// Event properties of the tracking plan, keyed by property ID.
    #[serde(rename = "properties")]
    pub properties: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::ProductAnalyticsLaunchProperty>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchTrackingPlanRequest {
    pub fn new() -> ProductAnalyticsLaunchTrackingPlanRequest {
        ProductAnalyticsLaunchTrackingPlanRequest {
            code_analysis: None,
            codegen_session_id: None,
            events: None,
            jira_ticket_id: None,
            pr_link: None,
            properties: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn code_analysis(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsLaunchCodeAnalysis,
    ) -> Self {
        self.code_analysis = Some(value);
        self
    }

    pub fn codegen_session_id(mut self, value: String) -> Self {
        self.codegen_session_id = Some(value);
        self
    }

    pub fn events(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsLaunchEvent>,
    ) -> Self {
        self.events = Some(value);
        self
    }

    pub fn jira_ticket_id(mut self, value: String) -> Self {
        self.jira_ticket_id = Some(value);
        self
    }

    pub fn pr_link(mut self, value: String) -> Self {
        self.pr_link = Some(value);
        self
    }

    pub fn properties(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::ProductAnalyticsLaunchProperty,
        >,
    ) -> Self {
        self.properties = Some(value);
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

impl Default for ProductAnalyticsLaunchTrackingPlanRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchTrackingPlanRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchTrackingPlanRequestVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchTrackingPlanRequestVisitor {
            type Value = ProductAnalyticsLaunchTrackingPlanRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code_analysis: Option<
                    crate::datadogV2::model::ProductAnalyticsLaunchCodeAnalysis,
                > = None;
                let mut codegen_session_id: Option<String> = None;
                let mut events: Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchEvent>> =
                    None;
                let mut jira_ticket_id: Option<String> = None;
                let mut pr_link: Option<String> = None;
                let mut properties: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::ProductAnalyticsLaunchProperty,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code_analysis" => {
                            if v.is_null() {
                                continue;
                            }
                            code_analysis =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "codegen_session_id" => {
                            if v.is_null() {
                                continue;
                            }
                            codegen_session_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events" => {
                            if v.is_null() {
                                continue;
                            }
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jira_ticket_id" => {
                            if v.is_null() {
                                continue;
                            }
                            jira_ticket_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pr_link" => {
                            if v.is_null() {
                                continue;
                            }
                            pr_link = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsLaunchTrackingPlanRequest {
                    code_analysis,
                    codegen_session_id,
                    events,
                    jira_ticket_id,
                    pr_link,
                    properties,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsLaunchTrackingPlanRequestVisitor)
    }
}
