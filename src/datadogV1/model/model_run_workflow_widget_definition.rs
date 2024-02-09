// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Run workflow is widget that allows you to run a workflow from a dashboard.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunWorkflowWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Array of workflow inputs to map to dashboard template variables.
    #[serde(rename = "inputs")]
    pub inputs: Option<Vec<crate::datadogV1::model::RunWorkflowWidgetInput>>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the run workflow widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::RunWorkflowWidgetDefinitionType,
    /// Workflow id.
    #[serde(rename = "workflow_id")]
    pub workflow_id: String,
}

impl RunWorkflowWidgetDefinition {
    pub fn new(
        type_: crate::datadogV1::model::RunWorkflowWidgetDefinitionType,
        workflow_id: String,
    ) -> RunWorkflowWidgetDefinition {
        RunWorkflowWidgetDefinition {
            custom_links: None,
            inputs: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            workflow_id,
        }
    }

    pub fn custom_links(
        &mut self,
        value: Vec<crate::datadogV1::model::WidgetCustomLink>,
    ) -> &mut Self {
        self.custom_links = Some(value);
        self
    }

    pub fn inputs(
        &mut self,
        value: Vec<crate::datadogV1::model::RunWorkflowWidgetInput>,
    ) -> &mut Self {
        self.inputs = Some(value);
        self
    }

    pub fn time(&mut self, value: crate::datadogV1::model::WidgetTime) -> &mut Self {
        self.time = Some(value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(&mut self, value: crate::datadogV1::model::WidgetTextAlign) -> &mut Self {
        self.title_align = Some(value);
        self
    }

    pub fn title_size(&mut self, value: String) -> &mut Self {
        self.title_size = Some(value);
        self
    }
}
