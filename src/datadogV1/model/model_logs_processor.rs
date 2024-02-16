// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Definition of a logs processor.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}
