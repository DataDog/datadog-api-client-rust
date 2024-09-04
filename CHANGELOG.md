# CHANGELOG

## 0.4.0 / 2024-09-04

### Fixed
* Add `is_totp` and `is_fido` to Synthetic global variables by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/197
### Added
* Add `api_key` and `name` to `CloudflareAccountResponseAttributes`. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/241
* Add `api_key` and `name` to `FastlyAccountUpdateRequestAttributes`. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/242
* Add `opsgenie_api_key` to `OpsgenieServiceResponseAttributes`. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/243
* Add `category` and `remote_config_read_enabled` to `APIKeyCreateAttributes`, add `LeakedKey`. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/244
* Allow 4 group-bys for pattern viz by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/250
* add url attribute to metrics assets v2 api by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/252
* Add editable field to suppression rule by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/161
* Handle nested oneOf models by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/259
* Add `num_flex_logs_retention_days` field to logs_indexes api spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/253
* Software catalog openapi spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/260
### Changed
* include additionalProperties by default and check when false by @amaskara-dd in https://github.com/DataDog/datadog-api-client-rust/pull/249
* allow variables in port by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/236
* Fix VFTs and extracted local variables enum types by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/218
* Changed Widget time schema to add support for new fixed_span and live_span object by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/239
### Deprecated
* mark groupby_simple_monitor as deprecated by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/269


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.3.0...v0.4.0

## 0.3.0 / 2024-08-12

### Added
* Documentation for new device tags endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/213
* Update documentation for Flex Logs Starter by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/227
### Changed
* add mfa_enabled field and change created_at type to datetime by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/228


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.2.0...v0.3.0

## 0.2.0 / 2024-08-01

### Fixed
* fix monitor enum by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/183
* dashboards add support for time-slice SLOs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/184
* Make modified by field nullable for get all API keys by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/189
* Add `409 Conflict` to `CreateGlobalVariable` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/201
### Added
* add cross org uuids to timeseries query by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/149
* Add network performance monitor type to API spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/162
* Document `force_delete_dependencies` for synthetics test deletion by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/153
* Support metric filtering in integration azure GET, PUT APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/167
* add enableProfiling and enableSecurityTesting options by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/176
* Add convert rule JSON to terraform to Datadog API Spec. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/151
* add changes for datadog partner program to estimated cost and billable usage APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/155
* Add type as a required field for the different basic auth types by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/169
* Adding Network Device Monitoring API Documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/164
* Security Monitoring - Support anomaly threshold detection method by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/181
* update hourly usage API docs for partner program by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/190
* Add resource_type query param to authn mapping spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/128
* Add rum stream to API definition by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/196
* Support `incident_analytics` enum in dashboard widget `FormulaAndFunctionEventsDataSource` data sources by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/208
* update usage summary API docs for partner program by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/205
* update historical_cost and projected_cost for partner program by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/194
* Add custom cost endpoints to public API documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/209
* Update documentation for Cloud SIEM Analyzed Logs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/211
* Update documentation for App Sec SCA by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/199
* Add trigger API documentation for workflow automation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/204
* Add PUT endpoint to scorecards APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/219
### Changed
* Rename examples with OperationIDs by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/192
### Deprecated
* Deprecate `ListAWSRelatedAccounts` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/200

## New Contributors
* @amaskara-dd made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/188

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.1.3...v0.2.0

## 0.1.3 / 2024-07-01

### Fixed
* Security Monitoring - Define specific payload for rule validation/testing by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/126
* Remove the maximum limitation for the synthetics renotify_interval monitor option by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/130
* Add bodyHash as a synthetics assertion type. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/131
* Add missing attributes envelope in ListAPIs response by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/140
### Added
* Allow the usage of the filters field when creating an agent rule by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/106
* Add tileDef sort attribute by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/108
* Add Security Monitoring rule test endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/122
* Add originalFileName field to the SyntheticsTestRequestBodyFile definition by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/127
* Add support for API management ListAPIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/134
* Add elementsOperator to json path assertion for synthetic HTTP tests by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/146
* Add /api/v2/org_configs specs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/145
* Support rustls with feature flag via reqwest by @skeggse in https://github.com/DataDog/datadog-api-client-rust/pull/158
* Update docs for RU Rollout New and Deprecated Keys planned for Oct 1st by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/160
* Add option for wait step in multistep api tests by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/159
### Changed
* Monitor priority can have custom ranges and be null by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/154

## New Contributors
* @jack-edmonds-dd made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/150
* @skeggse made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/158

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.0.3...v0.1.3

## 0.0.3 / 2024-05-22

### Fixed
* Fix incorrect propagation of nullable rendering within additionalProperties by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/96
### Added
* Add support variablesFromScript in Synthetics API test by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/83
* Add JSONSchema assertion support to API and multistep tests by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/52
* add 1 day logs to usage api docs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/90
* Update UserTeamIncluded to include teams by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/95
* Security Monitoring - Make Default Tags available in the response by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/105
* Add flex logs storage tier by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/107
### Changed
* Rename the Cloud Workload Security tag to CSM Threats by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/94
* Update license after code generation by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/110


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.0.2...v0.0.3

## 0.0.2 / 2024-04-16

### Fixed
* Build form data request body manually by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/63
* fix case search documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/80
### Added
* Add UA documentation for online_archive and incident_management by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/62
* Mark `unit` as nullable by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/65
* Add query_interval_seconds to time-slice SLO condition parameters by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/68
* Support providing files for the file upload feature when creating a Synthetic API test by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/67
* Documentation upgrades by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/66
* Adding SLO Reporting API Documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/50
* Security Monitoring Suppression - Add data_exclusion_query field by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/73
* aws api adding extended and deprecating old resource collection field by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/71
### Changed
* Add Team relationship to AuthNMappings by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/60
* Refactor imports by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/69
* Handle datetime format by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/41
* Add UUID support by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/47
* Replace duplicate error enums with a single variant per schema by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/77

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/commits/v0.0.2

## 0.0.1 / 2024-03-28

### Fixed
* Always serialize required nullable fields by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/16
* Fix edge case naming by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/18
* Update license check script and actions CI by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/32
* Custom de/serializing handling for enums by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/31
* Optimize example generation by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/40
### Added
* Add all APIs without oneOfs by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/5
* Add support for `unstableOperations` by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/12
* Add `unparsedObject` support by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/33
* Remove deprecated warnings by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/34
* Add proxy support by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/39
* Add validation endpoint for Security Monitoring Rules by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/58
* Add retry by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/42
### Changed
* Introduce API instance pattern by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/3
* Add cassette record/replay/passthrough modes in bdd runner by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/4
* Implement OneOfs and remove default on enums by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/6
* Bugfixes based on BDD testing by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/7
* Add builder-like pattern to API instances and models by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/9
* End user usability refactors by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/11
* Configurable server variables by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/15
* Remove Option wrapper in successful responses by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/20
* Remove `x-generate-alias-as-model` support by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/25
* Example generation by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/27
### Deprecated
* Remove deprecated /api/v1/usage/attribution by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/45
* Deprecate legacy hourly usage metering endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/53

## New Contributors
* @nkzou made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/1
* @skarimo made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/13
* @HantingZhang2 made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/17
* @antonio-ramadas-dd made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/37
* @api-clients-generation-pipeline made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/38

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/commits/2b9c33b
