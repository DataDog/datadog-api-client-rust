// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Definition of a logs processor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogsProcessor {
    LogsGrokParser(crate::datadogV1::model::LogsGrokParser),
    LogsDateRemapper(crate::datadogV1::model::LogsDateRemapper),
    LogsStatusRemapper(crate::datadogV1::model::LogsStatusRemapper),
    LogsServiceRemapper(crate::datadogV1::model::LogsServiceRemapper),
    LogsMessageRemapper(crate::datadogV1::model::LogsMessageRemapper),
    LogsAttributeRemapper(crate::datadogV1::model::LogsAttributeRemapper),
    LogsURLParser(crate::datadogV1::model::LogsURLParser),
    LogsUserAgentParser(crate::datadogV1::model::LogsUserAgentParser),
    LogsCategoryProcessor(crate::datadogV1::model::LogsCategoryProcessor),
    LogsArithmeticProcessor(crate::datadogV1::model::LogsArithmeticProcessor),
    LogsStringBuilderProcessor(crate::datadogV1::model::LogsStringBuilderProcessor),
    LogsPipelineProcessor(crate::datadogV1::model::LogsPipelineProcessor),
    LogsGeoIPParser(crate::datadogV1::model::LogsGeoIPParser),
    LogsLookupProcessor(crate::datadogV1::model::LogsLookupProcessor),
    ReferenceTableLogsLookupProcessor(crate::datadogV1::model::ReferenceTableLogsLookupProcessor),
    LogsTraceRemapper(crate::datadogV1::model::LogsTraceRemapper),
}
