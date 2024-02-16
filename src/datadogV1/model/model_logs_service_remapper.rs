// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Use this processor if you want to assign one or more attributes as the official service.
///
/// **Note:** If multiple service remapper processors can be applied to a given log,
/// only the first one (according to the pipeline order) is taken into account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsServiceRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Type of logs service remapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsServiceRemapperType,
}

impl LogsServiceRemapper {
    pub fn new(
        sources: Vec<String>,
        type_: crate::datadogV1::model::LogsServiceRemapperType,
    ) -> LogsServiceRemapper {
        LogsServiceRemapper {
            is_enabled: None,
            name: None,
            sources,
            type_,
        }
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}
