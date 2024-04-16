# CHANGELOG

## 0.0.2 / 2024-04-16

### Fixed
* Always serialize required nullable fields by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/16
* Fix edge case naming by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/18
* Update license check script and actions CI by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/32
* Custom de/serializing handling for enums by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/31
* Optimize example generation by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/40
* Build form data request body manually by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/63
* fix case search documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/80
### Added
* Add all APIs without oneOfs by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/5
* Add support for `unstableOperations` by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/12
* Add `unparsedObject` support by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/33
* Remove deprecated warnings by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/34
* Add proxy support by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/39
* Add validation endpoint for Security Monitoring Rules by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/58
* Add retry by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/42
* Add UA documentation for online_archive and incident_management by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/62
* Mark `unit` as nullable by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/65
* Add query_interval_seconds to time-slice SLO condition parameters by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/68
* Support providing files for the file upload feature when creating a Synthetic API test by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/67
* Documentation upgrades by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/66
* Adding SLO Reporting API Documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/50
* Security Monitoring Suppression - Add data_exclusion_query field by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/73
* aws api adding extended and deprecating old resource collection field by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/71
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
* Add Team relationship to AuthNMappings by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/60
* Refactor imports by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/69
* Handle datetime format by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/41
* Add UUID support by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/47
* Replace duplicate error enums with a single variant per schema by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/77
### Deprecated
* Remove deprecated /api/v1/usage/attribution by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/45
* Deprecate legacy hourly usage metering endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/53

## New Contributors
* @nkzou made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/1
* @skarimo made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/13
* @HantingZhang2 made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/17
* @antonio-ramadas-dd made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/37
* @api-clients-generation-pipeline made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/38

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/commits/v0.0.2

## 0.0.1 / 2024-03-28

Initial crate release.