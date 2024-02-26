// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Use the Lookup Processor to define a mapping between a log attribute
/// and a human readable value saved in the processors mapping table.
/// For example, you can use the Lookup Processor to map an internal service ID
/// into a human readable service name. Alternatively, you could also use it to check
/// if the MAC address that just attempted to connect to the production
/// environment belongs to your list of stolen machines.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsLookupProcessor {
    /// Value to set the target attribute if the source value is not found in the list.
    #[serde(rename = "default_lookup")]
    pub default_lookup: Option<String>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Mapping table of values for the source attribute and their associated target attribute values,
    /// formatted as `["source_key1,target_value1", "source_key2,target_value2"]`
    #[serde(rename = "lookup_table")]
    pub lookup_table: Vec<String>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Source attribute used to perform the lookup.
    #[serde(rename = "source")]
    pub source: String,
    /// Name of the attribute that contains the corresponding value in the mapping list
    /// or the `default_lookup` if not found in the mapping list.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs lookup processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsLookupProcessorType,
}

impl LogsLookupProcessor {
    pub fn new(
        lookup_table: Vec<String>,
        source: String,
        target: String,
        type_: crate::datadogV1::model::LogsLookupProcessorType,
    ) -> LogsLookupProcessor {
        LogsLookupProcessor {
            default_lookup: None,
            is_enabled: None,
            lookup_table,
            name: None,
            source,
            target,
            type_,
        }
    }

    pub fn default_lookup(&mut self, value: String) -> &mut Self {
        self.default_lookup = Some(value);
        self
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
