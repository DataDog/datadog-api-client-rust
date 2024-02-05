// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Use the Category Processor to add a new attribute (without spaces or special characters in the new attribute name)
/// to a log matching a provided search query. Use categories to create groups for an analytical view.
/// For example, URL groups, machine groups, environments, and response time buckets.
///
/// **Notes**:
///
/// - The syntax of the query is the one of Logs Explorer search bar.
///   The query can be done on any log attribute or tag, whether it is a facet or not.
///   Wildcards can also be used inside your query.
/// - Once the log has matched one of the Processor queries, it stops.
///   Make sure they are properly ordered in case a log could match several queries.
/// - The names of the categories must be unique.
/// - Once defined in the Category Processor, you can map categories to log status using the Log Status Remapper.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsCategoryProcessor {
    /// Array of filters to match or not a log and their
    /// corresponding `name` to assign a custom value to the log.
    #[serde(rename = "categories")]
    pub categories: Vec<crate::datadogV1::model::LogsCategoryProcessorCategory>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Name of the target attribute which value is defined by the matching category.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs category processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsCategoryProcessorType,
}

impl LogsCategoryProcessor {
    pub fn new(
        categories: Vec<crate::datadogV1::model::LogsCategoryProcessorCategory>,
        target: String,
        type_: crate::datadogV1::model::LogsCategoryProcessorType,
    ) -> LogsCategoryProcessor {
        LogsCategoryProcessor {
            categories,
            is_enabled: None,
            name: None,
            target,
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
