// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single metric's ingested and indexed volume.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricIngestedIndexedVolume {
    /// Object containing the definition of a metric's ingested and indexed volume.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::MetricIngestedIndexedVolumeAttributes>,
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The metric ingested and indexed volume type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MetricIngestedIndexedVolumeType>,
}

impl MetricIngestedIndexedVolume {
    pub fn new() -> MetricIngestedIndexedVolume {
        MetricIngestedIndexedVolume {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::MetricIngestedIndexedVolumeAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::MetricIngestedIndexedVolumeType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for MetricIngestedIndexedVolume {
    fn default() -> Self {
        Self::new()
    }
}
