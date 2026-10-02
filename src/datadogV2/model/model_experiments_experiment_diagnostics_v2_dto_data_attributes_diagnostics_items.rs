// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Result of one diagnostic check for an experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems {
    /// Explanation of the diagnostic check result.
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option")]
    pub message: Option<Option<String>>,
    /// Identifier of the metric associated with this check.
    #[serde(rename = "metric_id", default, with = "::serde_with::rust::double_option")]
    pub metric_id: Option<Option<String>>,
    /// Reason the diagnostic check could not be evaluated.
    #[serde(rename = "skipped_reason", default, with = "::serde_with::rust::double_option")]
    pub skipped_reason: Option<Option<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsSkippedReason>>,
    /// Outcome of an individual diagnostic check.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsStatus,
    /// Short title of the diagnostic check.
    #[serde(rename = "title")]
    pub title: String,
    /// Kind of diagnostic check performed.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems {
    pub fn new(
        status: crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsStatus,
        title: String,
        type_: crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsType,
    ) -> ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems {
        ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems {
            message: None,
            metric_id: None,
            skipped_reason: None,
            status,
            title,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn message(mut self, value: Option<String>) -> Self {
        self.message = Some(value);
        self
    }

    pub fn metric_id(mut self, value: Option<String>) -> Self {
        self.metric_id = Some(value);
        self
    }

    pub fn skipped_reason(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsSkippedReason>,
    ) -> Self {
        self.skipped_reason = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsVisitor
        {
            type Value = ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut message: Option<Option<String>> = None;
                let mut metric_id: Option<Option<String>> = None;
                let mut skipped_reason: Option<Option<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsSkippedReason>> = None;
                let mut status: Option<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsStatus> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_id" => {
                            metric_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "skipped_reason" => {
                            skipped_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _skipped_reason) = skipped_reason {
                                match _skipped_reason {
                                    Some(crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsSkippedReason::UnparsedObject(_skipped_reason)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems {
                    message,
                    metric_id,
                    skipped_reason,
                    status,
                    title,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsVisitor,
        )
    }
}
