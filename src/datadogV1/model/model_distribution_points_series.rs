// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A distribution points metric to submit to Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionPointsSeries {
    /// The name of the host that produced the distribution point metric.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// The name of the distribution points metric.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Points relating to the distribution point metric. All points must be tuples with timestamp and a list of values (cannot be a string). Timestamps should be in POSIX time in seconds.
    #[serde(rename = "points")]
    pub points: Vec<Vec<crate::datadogV1::model::DistributionPointItem>>,
    /// A list of tags associated with the distribution point metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of the distribution point.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::DistributionPointsType>,
}

impl DistributionPointsSeries {
    pub fn new(
        metric: String,
        points: Vec<Vec<crate::datadogV1::model::DistributionPointItem>>,
    ) -> DistributionPointsSeries {
        DistributionPointsSeries {
            host: None,
            metric,
            points,
            tags: None,
            type_: None,
        }
    }

    pub fn with_host(&mut self, value: String) -> &mut Self {
        self.host = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn with_type_(
        &mut self,
        value: crate::datadogV1::model::DistributionPointsType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
