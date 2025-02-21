// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Definition of a logs processor.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogsProcessor {
    LogsGrokParser(Box<crate::datadogV1::model::LogsGrokParser>),
    LogsDateRemapper(Box<crate::datadogV1::model::LogsDateRemapper>),
    LogsStatusRemapper(Box<crate::datadogV1::model::LogsStatusRemapper>),
    LogsServiceRemapper(Box<crate::datadogV1::model::LogsServiceRemapper>),
    LogsMessageRemapper(Box<crate::datadogV1::model::LogsMessageRemapper>),
    LogsAttributeRemapper(Box<crate::datadogV1::model::LogsAttributeRemapper>),
    LogsURLParser(Box<crate::datadogV1::model::LogsURLParser>),
    LogsUserAgentParser(Box<crate::datadogV1::model::LogsUserAgentParser>),
    LogsCategoryProcessor(Box<crate::datadogV1::model::LogsCategoryProcessor>),
    LogsArithmeticProcessor(Box<crate::datadogV1::model::LogsArithmeticProcessor>),
    LogsStringBuilderProcessor(Box<crate::datadogV1::model::LogsStringBuilderProcessor>),
    LogsPipelineProcessor(Box<crate::datadogV1::model::LogsPipelineProcessor>),
    LogsGeoIPParser(Box<crate::datadogV1::model::LogsGeoIPParser>),
    LogsLookupProcessor(Box<crate::datadogV1::model::LogsLookupProcessor>),
    ReferenceTableLogsLookupProcessor(
        Box<crate::datadogV1::model::ReferenceTableLogsLookupProcessor>,
    ),
    LogsTraceRemapper(Box<crate::datadogV1::model::LogsTraceRemapper>),
    LogsSpanRemapper(Box<crate::datadogV1::model::LogsSpanRemapper>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LogsProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::LogsGrokParser>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsGrokParser(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::LogsDateRemapper>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsDateRemapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::LogsStatusRemapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsStatusRemapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::LogsServiceRemapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsServiceRemapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::LogsMessageRemapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsMessageRemapper(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::LogsAttributeRemapper>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsAttributeRemapper(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::LogsURLParser>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsURLParser(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::LogsUserAgentParser>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsUserAgentParser(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::LogsCategoryProcessor>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsCategoryProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsArithmeticProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsArithmeticProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogsStringBuilderProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsStringBuilderProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::LogsPipelineProcessor>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsPipelineProcessor(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::LogsGeoIPParser>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsGeoIPParser(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::LogsLookupProcessor>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsLookupProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ReferenceTableLogsLookupProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::ReferenceTableLogsLookupProcessor(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::LogsTraceRemapper>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsTraceRemapper(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::LogsSpanRemapper>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsProcessor::LogsSpanRemapper(_v));
            }
        }

        return Ok(LogsProcessor::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
