// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CI information associated with the test result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultCI {
    /// Details of the CI pipeline.
    #[serde(rename = "pipeline")]
    pub pipeline: Option<crate::datadogV2::model::SyntheticsTestResultCIPipeline>,
    /// Details of the CI provider.
    #[serde(rename = "provider")]
    pub provider: Option<crate::datadogV2::model::SyntheticsTestResultCIProvider>,
    /// Details of the CI stage.
    #[serde(rename = "stage")]
    pub stage: Option<crate::datadogV2::model::SyntheticsTestResultCIStage>,
    /// Path of the workspace that ran the CI job.
    #[serde(rename = "workspace_path")]
    pub workspace_path: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultCI {
    pub fn new() -> SyntheticsTestResultCI {
        SyntheticsTestResultCI {
            pipeline: None,
            provider: None,
            stage: None,
            workspace_path: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn pipeline(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultCIPipeline,
    ) -> Self {
        self.pipeline = Some(value);
        self
    }

    pub fn provider(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultCIProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn stage(mut self, value: crate::datadogV2::model::SyntheticsTestResultCIStage) -> Self {
        self.stage = Some(value);
        self
    }

    pub fn workspace_path(mut self, value: String) -> Self {
        self.workspace_path = Some(value);
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

impl Default for SyntheticsTestResultCI {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultCI {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultCIVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultCIVisitor {
            type Value = SyntheticsTestResultCI;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pipeline: Option<crate::datadogV2::model::SyntheticsTestResultCIPipeline> =
                    None;
                let mut provider: Option<crate::datadogV2::model::SyntheticsTestResultCIProvider> =
                    None;
                let mut stage: Option<crate::datadogV2::model::SyntheticsTestResultCIStage> = None;
                let mut workspace_path: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pipeline" => {
                            if v.is_null() {
                                continue;
                            }
                            pipeline = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            if v.is_null() {
                                continue;
                            }
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stage" => {
                            if v.is_null() {
                                continue;
                            }
                            stage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workspace_path" => {
                            if v.is_null() {
                                continue;
                            }
                            workspace_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultCI {
                    pipeline,
                    provider,
                    stage,
                    workspace_path,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultCIVisitor)
    }
}
