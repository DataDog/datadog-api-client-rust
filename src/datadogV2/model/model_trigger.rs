// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// One of the triggers that can start the execution of a workflow.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Trigger {
    APITriggerWrapper(Box<crate::datadogV2::model::APITriggerWrapper>),
    AppTriggerWrapper(Box<crate::datadogV2::model::AppTriggerWrapper>),
    CaseTriggerWrapper(Box<crate::datadogV2::model::CaseTriggerWrapper>),
    ChangeEventTriggerWrapper(Box<crate::datadogV2::model::ChangeEventTriggerWrapper>),
    DatabaseMonitoringTriggerWrapper(
        Box<crate::datadogV2::model::DatabaseMonitoringTriggerWrapper>,
    ),
    DashboardTriggerWrapper(Box<crate::datadogV2::model::DashboardTriggerWrapper>),
    GithubWebhookTriggerWrapper(Box<crate::datadogV2::model::GithubWebhookTriggerWrapper>),
    IncidentTriggerWrapper(Box<crate::datadogV2::model::IncidentTriggerWrapper>),
    MonitorTriggerWrapper(Box<crate::datadogV2::model::MonitorTriggerWrapper>),
    NotebookTriggerWrapper(Box<crate::datadogV2::model::NotebookTriggerWrapper>),
    ScheduleTriggerWrapper(Box<crate::datadogV2::model::ScheduleTriggerWrapper>),
    SecurityTriggerWrapper(Box<crate::datadogV2::model::SecurityTriggerWrapper>),
    SelfServiceTriggerWrapper(Box<crate::datadogV2::model::SelfServiceTriggerWrapper>),
    SlackTriggerWrapper(Box<crate::datadogV2::model::SlackTriggerWrapper>),
    SoftwareCatalogTriggerWrapper(Box<crate::datadogV2::model::SoftwareCatalogTriggerWrapper>),
    WorkflowTriggerWrapper(Box<crate::datadogV2::model::WorkflowTriggerWrapper>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for Trigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::APITriggerWrapper>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(Trigger::APITriggerWrapper(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::AppTriggerWrapper>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(Trigger::AppTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::CaseTriggerWrapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(Trigger::CaseTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ChangeEventTriggerWrapper>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(Trigger::ChangeEventTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DatabaseMonitoringTriggerWrapper>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(Trigger::DatabaseMonitoringTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DashboardTriggerWrapper>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(Trigger::DashboardTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GithubWebhookTriggerWrapper>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(Trigger::GithubWebhookTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::IncidentTriggerWrapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(Trigger::IncidentTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MonitorTriggerWrapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(Trigger::MonitorTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::NotebookTriggerWrapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(Trigger::NotebookTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ScheduleTriggerWrapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(Trigger::ScheduleTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityTriggerWrapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(Trigger::SecurityTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SelfServiceTriggerWrapper>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(Trigger::SelfServiceTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SlackTriggerWrapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(Trigger::SlackTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SoftwareCatalogTriggerWrapper>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(Trigger::SoftwareCatalogTriggerWrapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::WorkflowTriggerWrapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(Trigger::WorkflowTriggerWrapper(_v));
            }
        }

        return Ok(Trigger::UnparsedObject(crate::datadog::UnparsedObject {
            value,
        }));
    }
}
