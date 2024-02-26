// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the pipeline event to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppCreatePipelineEventRequestAttributes {
    /// The Datadog environment.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Details of the CI pipeline event.
    #[serde(rename = "resource")]
    pub resource: crate::datadogV2::model::CIAppCreatePipelineEventRequestAttributesResource,
    /// If the CI provider is SaaS, use this to differentiate between instances.
    #[serde(rename = "service")]
    pub service: Option<String>,
}

impl CIAppCreatePipelineEventRequestAttributes {
    pub fn new(
        resource: crate::datadogV2::model::CIAppCreatePipelineEventRequestAttributesResource,
    ) -> CIAppCreatePipelineEventRequestAttributes {
        CIAppCreatePipelineEventRequestAttributes {
            env: None,
            resource,
            service: None,
        }
    }

    pub fn env(&mut self, value: String) -> &mut Self {
        self.env = Some(value);
        self
    }

    pub fn service(&mut self, value: String) -> &mut Self {
        self.service = Some(value);
        self
    }
}
