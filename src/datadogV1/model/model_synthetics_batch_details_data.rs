// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Wrapper object that contains the details of a batch.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBatchDetailsData {
    /// Metadata for the Synthetic tests run.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SyntheticsCIBatchMetadata>,
    /// List of results for the batch.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::datadogV1::model::SyntheticsBatchResult>>,
    /// Determines whether or not the batch has passed, failed, or is in progress.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsStatus>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBatchDetailsData {
    pub fn new() -> SyntheticsBatchDetailsData {
        SyntheticsBatchDetailsData {
            metadata: None,
            results: None,
            status: None,
            _unparsed: false,
        }
    }

    pub fn metadata(
        &mut self,
        value: crate::datadogV1::model::SyntheticsCIBatchMetadata,
    ) -> &mut Self {
        self.metadata = Some(value);
        self
    }

    pub fn results(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsBatchResult>,
    ) -> &mut Self {
        self.results = Some(value);
        self
    }

    pub fn status(&mut self, value: crate::datadogV1::model::SyntheticsStatus) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for SyntheticsBatchDetailsData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsBatchDetailsData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBatchDetailsDataVisitor;
        impl<'a> Visitor<'a> for SyntheticsBatchDetailsDataVisitor {
            type Value = SyntheticsBatchDetailsData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metadata: Option<crate::datadogV1::model::SyntheticsCIBatchMetadata> = None;
                let mut results: Option<Vec<crate::datadogV1::model::SyntheticsBatchResult>> = None;
                let mut status: Option<crate::datadogV1::model::SyntheticsStatus> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results" => {
                            if v.is_null() {
                                continue;
                            }
                            results = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::SyntheticsStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsBatchDetailsData {
                    metadata,
                    results,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBatchDetailsDataVisitor)
    }
}
