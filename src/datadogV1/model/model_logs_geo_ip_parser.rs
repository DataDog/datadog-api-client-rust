// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The GeoIP parser takes an IP address attribute and extracts if available
/// the Continent, Country, Subdivision, and City information in the target attribute path.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsGeoIPParser {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Name of the parent attribute that contains all the extracted details from the `sources`.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of GeoIP parser.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsGeoIPParserType,
}

impl LogsGeoIPParser {
    pub fn new(
        sources: Vec<String>,
        target: String,
        type_: crate::datadogV1::model::LogsGeoIPParserType,
    ) -> LogsGeoIPParser {
        LogsGeoIPParser {
            is_enabled: None,
            name: None,
            sources,
            target,
            type_,
        }
    }
}
