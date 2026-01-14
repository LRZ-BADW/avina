# Changelog
This is the combined changelog of all contained `avina` crates.

## [Unreleased]
...

## [avina-api-v1.0.1] - 2026-01-14

### Fixes
- correct default is_active setting in user_import

## Repository - 2025-12-31

### Documentation
- add cli installation section with link

### CI
- add build job to test workflow
- add --all --all-targets to clippy command in lint workflow
- use production mariadb server version in test and lint workflows
- use --locked for zizmor installation in workflow workflow
- add missing mold install in test and lint workflows
- use sqlx offline mode for clippy job in lint workflow
- bump actions/checkout from 4 to 6
- bump actions/labeler from 5 to 6
- bump actions/stale from 9 to 10
- bump taiki-e/install-action from 2.52.6 to 2.65.7

### Misc
- update spellcheck dictionary
- update sqlx offline query data
- improve cargo-husty pre-push hook

## [avina-ui-v0.0.2] - 2025-12-31

### Dependencies
- bump avina from 1.8.0 to 2.0.0
- bump dioxus from 0.6.3 to 0.7.2
- run cargo update

### Features
- update rust edition to 2024
- use lib to call user_me api endpoint

### Documentation
- describe how to run ui locally

### Misc
- add run_ui.sh script

## [avina-test-v0.7.0] - 2025-12-31

### Dependencies
- bump avina-api from 0.8.0 to 1.0.0
- bump avina-wire from 1.7.0 to 2.0.0
- bump avina from 1.8.0 to 2.0.0
- bump wiremock from 0.6.3 to 0.6.5
- bump serde_json from 1.0.140 to 1.0.148
- bump anyhow from 1.0.98 to 1.0.100
- bump rand from 0.9.1 to 0.9.2
- bump chrono from 0.4.41 to 0.4.42
- bump reqwest from 0.12.19 to 0.13.0
- bump uuid from 1.17.0 to 1.19.0
- bump tokio from 1.45.1 to 1.48.0
- run cargo update

### Features
- update rust edition to 2024
- add various new setup functions
- revise for async library calls

### Fixes
- use Uuid for instance_id
- use enum for user_class
- repair compilation on WASM

### Tests
- add tests for server cost endpoint
- add tests for project budget modify endpoint
- add tests for user budget modify endpoint
- add tests for flavor price list endpoint

## [avina-cli-v2.0.0] - 2025-12-31

### Dependencies
- bump avina-wire from 1.7.0 to 2.0.0
- bump avina from 1.8.0 to 2.0.0
- bump clap from 4.5.39 to 4.5.53
- bump serde_json from 1.0.140 to 1.0.148
- bump anyhow from 1.0.98 to 1.0.100
- bump chrono from 0.4.41 to 0.4.42
- bump uuid from 1.17.0 to 1.19.0
- bump tokio from 1.45.1 to 1.48.0
- run cargo update

### Features
- update rust edition to 2024
- re-route all calls to rust api
- revise for async library calls
- make --current an actual flag on flavor-price list

### Fixes
- set global=true for top level cli arguments
- use unique short options for global arguments
- use Uuid for instance_id
- use enum for user_class

### Tests
- add test check_cli_parser

## [avina-lib-v2.0.0] - 2025-12-31

### Dependencies
- bump avina-wire from 1.7.0 to 2.0.0
- bump serde_json from 1.0.140 to 1.0.148
- bump anyhow from 1.0.98 to 1.0.100
- bump chrono from 0.4.41 to 0.4.42
- bump reqwest from 0.12.19 to 0.13.0
- bump serde from 1.0.219 to 1.0.228
- bump uuid from 1.17.0 to 1.19.0
- bump thiserror from 2.0.12 to 2.0.17
- run cargo update

### Features
- update rust edition to 2024
- make all library calls async
- add Debug derives to Api and Token structs
- add missing common trait derives to various public structs
- add BudgetOverTreeParams, FlavorQuotaCheckParams, FlavorUsageParams
- don't set timeout in Api::new on wasm target
- add FlavorGroupUsageParams and FlavorQuotaCheckParams.openstackproject

### Fixes
- use Uuid for instance_id
- use enum for user_class
- correct url for user-import endpoint

### Refactors
- rename FlavorGroupUsage to FlavorGroupUsageSimple
- rename FlavorUsage to FlavorUsageSimple

## [avina-api-v1.0.0] - 2025-12-31

### Depencencies
- bump avina-wire from 1.6.0 to 2.0.0
- bump indexmap from 2.9.0 to 2.12.1
- bump actix-web from 4.11.0 to 4.12.1
- bump tracing-actix-web from 0.7.18 to 0.7.20
- bump wiremock from 0.6.3 to 0.6.5
- bump tracing from 0.1.41 to 0.1.44
- bump tracing-subscriber from 0.3.19 to 0.3.22
- bump strum from 0.27.1 to 0.27.2
- bump serde_json from 1.0.140 to 1.0.148
- bump config from 0.15.11 to 0.15.19
- bump anyhow from 1.0.98 to 1.0.100
- bump rand from 0.9.1 to 0.9.2
- bump chrono from 0.4.41 to 0.4.42
- bump reqwest from 0.12.19 to 0.13.0
- bump serde from 1.0.219 to 1.0.228
- bump uuid from 1.17.0 to 1.19.0
- bump thiserror from 2.0.12 to 2.0.17
- bump tokio from 1.45.1 to 1.48.0
- update yanked tower-http dependency
- run cargo update

### Features
- update rust edition to 2024
- implement flavor usage endpoint
- implement flavor-group usage endpoint
- implement budget-over-tree endpoint
- implement cloud-usage endpoint
- implement flavor-quota check endpoint
- implement proper user import endpoint with help of avina-ldap
- extend flavor_price_list
- add OpenStack functions get_domains, get_projects, and get_servers_of_project
- setup actix-cors for localhost ui
- add token, avina-ldap url, cloud-usage url to request extensions
- add lrz flavor and flavor-group select functions
- add select_flavor_quota_by_user_and_group_from_db
- add select_user_by_openstack_id_from_db
- add select_flavor_prices_for_userclass_and_period_from_db
- fetch first row with select server states func

### Fixes
- use Uuid for instance_id
- use enum for user_class
- use left join for FlavorGroup selection
- return new names from flavor_price_modify, server_state_modify, and flavor_quota_modify
- corrent server_consumption get endpoint permissions
- correct server_cost get endpoint permissions
- correct budget modify endpoint permissions
- filter out outdated prices in select_flavor_prices_for_*period_from_db functions
- correct FlavorDb.group_id type
- correct tracing name of select_maybe_flavor_detail_from_db and flavor_group_usage function
- allow both filters on flavor-price list

### Refactors
- outsource utils::start_of_the_year function
- simplify name retrieval in flavor_quota_modify
- simplify calculate_flavor_group_usage_*_simple functions
- remove unneeded project param from various endpoints

### Misc
- add run_api.sh script
- rename api container to avina in build_api_container.sh
- use rust:1 as image in build_api_container.sh
- update to debian trixie in build_api_container.sh
- use production mariadb server version in init_db.sh
- allow OpenSSL license in cargo deny config

### Documentation
- use run_api.sh script in README.md

## [avina-wire-v2.0.0] - 2025-12-31

### Dependencies
- bump clap from 4.5.39 to 4.5.53
- bump strum from 0.27.1 to 0.27.2
- bump rand from 0.9.1 to 0.9.2
- bump chrono from 0.4.41 to 0.4.42
- bump serde from 1.0.219 to 1.0.228
- bump uuid from 1.17.0 to 1.19.0
- bump thiserror from 2.0.12 to 2.0.17
- run cargo update

### Features
- update rust edition to 2024
- add BudgetOverTreeParams, FlavorQuotaCheckParams, FlavorUsageParams
- add FlavorGroupUsageParams and FlavorQuotaCheckParams.openstackproject
- add sqlx feature for optional sqlx dep
- add tabled feature for optional tabled dependency
- make BudgetOverTreeUser/Project.budget(_id) fields an Option
- implement From<UserDetailed> for User

### Fixes
- use Uuid for instance_id
- use enum for user_class
- make tabled derives dependent on feature
- use default skip_serializing_if
- use default skip_serializing_if
- add sqlx try_froms to FlavorQuota fields
- add sqlx try_from to FlavorMinimal.id
- correct id, project types in FromRow for FlavorGroup
- adjust FlarvorGroup FromRow for left join

### Refactors
- rename FlavorGroupUsage to FlavorGroupUsageSimple
- rename FlavorUsage to FlavorUsageSimple
- move UserClass to user/project submodule

## Repository - 2025-06-05

### Dependencies
- bump ring to v0.17.13
- bump openssl from 0.10.71 to 0.10.72

### CI
- add zizmor config to ignore unpinned-uses for now
- sort imports with rustfmt nightly
- increase stale days to 90 and close to 30
- bump sqlx from 0.8.3 to 0.8.6
- bump taiki-e/install-action from 2.48.1 to 2.52.4

### Documentation
- docs(readme): remove "partial", "rewrite" from descriptions

### Misc
- rename project to avina
- update sqlx offline query data
- update spellcheck dictionary, remove old cspell dictionary
- add deny ignore for unmaintained paste
- correct sqlx-cli install cmd in init_db.sh
- remove OpenSSL and add CDLA-Permissive-2.0 licenses in deny config
- add bsl-1.0 as allowed license in deny config

## [avina-ui-v0.0.1] - 2025-06-05

### Features
- add initial ui crate
- add scripts to build and run the dioxus app
- add Dockerfile for ui app
- add Django wrapper app

## [avina-test-v0.6.0] - 2025-06-05

### Dependencies
- bump anyhow from 1.0.95 to 1.0.98
- bump chrono from 0.4.39 to 0.4.41
- bump once_cell from 1.20.2 to 1.21.3
- bump rand from 0.9.0 to 0.9.1
- bump reqwest from 0.12.12 to 0.12.19
- bump serde_json from 1.0.138 to 1.0.140
- bump sqlx from 0.8.3 to 0.8.6
- bump tokio from 1.43.0 to 1.45.1
- bump uuid from 1.12.1 to 1.17.0
- bump wiremock from 0.6.2 to 0.6.3

### Tests
- add tests for project budget over endpoint
- add tests for user budget over endpoint
- update tests for server state list endpoint
- add tests for user budget access
- add tests for project budget access

## [avina-cli-v1.6.0] - 2025-06-05

### Fixes
- remove unused struct patterns

### Dependencies
- bump anyhow from 1.0.95 to 1.0.98
- bump chrono from 0.4.39 to 0.4.41
- bump clap from 4.5.28 to 4.5.39
- bump serde from 1.0.217 to 1.0.219
- bump serde_json from 1.0.138 to 1.0.140
- bump tabled from 0.17.0 to 0.20.0

## [avina-lib-v1.8.0] - 2025-06-05

### Features
- rename UserBudgetOver to UserBudgetOverSimple
- move UserBudgetOverParams to wire
- rename ProjectBudgetOver response structs
- move ProjectBudgetOverParams into wire

### Fixes
- correct typo in UserBudgetOver response types

### Dependencies
- bump anyhow from 1.0.95 to 1.0.98
- bump chrono from 0.4.39 to 0.4.41
- bump reqwest from 0.12.12 to 0.12.19
- bump serde from 1.0.217 to 1.0.219
- bump serde_json from 1.0.138 to 1.0.140
- bump thiserror from 2.0.11 to 2.0.12

## [avina-api-v0.8.0] - 2025-06-05

### Dependencies
- bump actix-web from 4.9.0 to 4.11.0
- bump anyhow from 1.0.95 to 1.0.98
- bump chrono from 0.4.39 to 0.4.41
- bump config from 0.15.7 to 0.15.11
- bump indexmap from 2.7.0 to 2.9.0
- bump once_cell from 1.20.2 to 1.21.3
- bump rand from 0.9.0 to 0.9.1
- bump reqwest from 0.12.12 to 0.12.19
- bump serde from 1.0.217 to 1.0.219
- bump serde-aux from 4.5.0 to 4.7.0
- bump serde_json from 1.0.138 to 1.0.140
- bump sqlx from 0.8.3 to 0.8.6
- bump strum from 0.26.3 to 0.27.1
- bump thiserror from 2.0.11 to 2.0.12
- bump tokio from 1.43.0 to 1.45.1
- bump tracing-actix-web from 0.7.15 to 0.7.18
- bump uuid from 1.12.1 to 1.17.0
- bump wiremock from 0.6.2 to 0.6.3

### Fixes
- add sqlx from to Row structs in server_state::import mod
- correct error message in insert_flavor_into_db
- correct join in select_all_flavors_from_db

### Features
- add server-state import endpoint
- add Openstack.get_servers function
- add flavor import endpoint
- add OpenStack.get_flavors
- add user-budget over endpoint
- add project-budget over endpoint
- add budget bulk create endpoint
- implement proper project and user budget permission checks

### Documentation
- add development requirements section to README.md
- update endpoint todo list in routes module

## [avina-wire-v1.7.0] - 2025-06-05

### Features
- rename UserBudgetOver to UserBudgetOverSimple
- move UserBudgetOverParams to wire
- rename ProjectBudgetOver response structs
- move ProjectBudgetOverParams into wire

### Fixes
- correct typo in UserBudgetOver response types
- add sqlx try_froms to ProjectBudget
- replace tabled display_with with display
- add sqlx try_froms to UserBudget
- make UserBudgetOverCombinedDetail.project_budget optional
- make UserBudgetOverCombinedDetail.project_budget_id optional
- make UserBudgetOverCombined.project_budget_id optional

### Dependencies
- bump chrono from 0.4.39 to 0.4.41
- bump serde from 1.0.217 to 1.0.219
- bump sqlx from 0.8.3 to 0.8.6
- bump tabled from 0.17.0 to 0.20.0

## Repository - 2025-02-05

### Documentation
- fix typo in crates list in readme

### CI
- bump taiki-e/install-action from 2.47.0 to 2.47.12
- bump sqlx version to 8.3
- configure empty permissions in workflows

### Miscellaneous
- add Zlib license allowance in deny configuration

## [avina-api-v0.7.0] - 2025-02-05

### Database
- add select_user_class_by_project_from_db
- add select_user_class_by_user_from_db
- add select_user_class_by_server_from_db
- add sqlx try_froms to FlavorPriceRow
- add select_flavor_prices_for_period_from_db
- add select_ordered_server_states_by_user_begin_and_end_from_db
- add select_ordered_server_states_by_server_begin_and_end_from_db

### Documentation
- add development section with local api run and call guide to readme

### Features
- add server-cost endpoint and implement for server, user, project, all, and normal, detail
- add server-consumption endpoint and implement for server, user, project and all

### Dependencies
- add indexmap dependency
- add strum dependency
- bump avina-wire from 1.5 to 1.6
- bump config from 0.15.4 to 0.15.7
- bump sqlx from 0.8.2 to 0.8.3
- bump reqwest from 0.12.10 to 0.12.12
- bump serde from 1.0.216 to 1.0.217
- bump serde_json from 1.0.134 to 1.0.138
- bump thiserror from 2.0.9 to 2.0.11
- bump tokio from 1.42.0 to 1.43.0

## [avina-cli-v1.5.2] - 2025-02-05

### Fixes
- correct typo in flavor-price list help message

### Dependencies
- bump avina-wire from 1.5 to 1.6
- bump avina-lib from 1.6 to 1.7
- bump clap from 4.5.23 to 4.5.28
- bump colored from 2.2.0 to 3.0.0
- bump serde from 1.0.216 to 1.0.217
- bump serde_json from 1.0.134 to 1.0.138

## [avina-lib-v1.7.0] - 2025-02-05
- use ServerCostParams in ServerCostRequest
- revise ServerConsumptionRequest to use ServerConsumptionParams
- revise for optional ServerConsumptionParams fields

### Fixes
- correct "Failed to envode..." typo in error messages

### Dependencies
- force reqwest to use rustls
- bump avina-wire from 1.5 to 1.6
- bump config from 0.15.4 to 0.15.7
- bump reqwest from 0.12.10 to 0.12.12
- bump serde from 1.0.216 to 1.0.217
- bump serde_json from 1.0.134 to 1.0.138
- bump thiserror from 2.0.9 to 2.0.11

## [avina-wire-v1.6.0] - 2025-02-05

### Features
- add ServerCostParams, ServerConsumptionParams
- derive Default for ServerConsumption structs
- make ServerConsumptionParams.all/detail Options

### Fixes
- manually implement FromRow for Flavor

### Dependencies
- bump sqlx from 0.8.2 to 0.8.3
- bump serde from 1.0.216 to 1.0.217

## [avina-test-v0.5.0] - 2025-02-05

### Features
- add rudimentary bench

### Fixes
- revise for new rand api

### Dependencies
- force reqwest to use rustls
- add bencher dependency
- bump avina-lib from 1.6 to 1.7
- bump avina-api from 0.6 to 0.7
- bump avina-wire from 1.5 to 1.6
- bump rand from 0.8.5 to 0.9.0
- bump sqlx from 0.8.2 to 0.8.3
- bump reqwest from 0.12.10 to 0.12.12
- bump serde_json from 1.0.134 to 1.0.138
- bump tokio from 1.42.0 to 1.43.0
- bump uuid from 1.11.0 to 1.12.1

## [avina-api-v0.6.1] - 2024-12-29

### Fixes
- dynamically retrieve current year in sync_user_budgets_in_db
- correct typo in error message in sync_user_budgets_in_db

## [avina-cli-v1.5.1] - 2024-12-29

### Fixes
- route user-budget sync and flavor/flavor-group modify to rust api

## [avina-cli-v1.5.0] - 2024-12-29

### Features
- add user-budget sync command to bugdgeting module

### Dependencies
- bump avina-wire from 1.4.0 to 1.5.0
- bump avina from 1.5.0 to 1.6.0
- bump anyhow from 1.0.94 to 1.0.95
- bump serde_json from 1.0.133 to 1.0.134
- bump serde from 1.0.215 to 1.0.216
- bump colored from 2.1.0 to 2.2.0

## [avina-lib-v1.6.0] - 2024-12-29

### Features
- add UserBudgetApi.sync function to budgeting module

### Fixes
- add user and end in BudgetOverTreeRequest.params in budgeting module

### Dependencies
- bump avina-wire from 1.4.0 to 1.5.0
- bump anyhow from 1.0.94 to 1.0.95
- bump thiserror from 2.0.6 to 2.0.9
- bump reqwest from 0.12.9 to 0.12.10
- bump serde_json from 1.0.133 to 1.0.134
- bump serde from 1.0.215 to 1.0.216

## [avina-api-v0.6.0] - 2024-12-29

### Features
- add user_budget_sync endpoint to budgeting module
- add sync_user_budgets_in_db function to database module

### Fixes
- correct query in select_maybe_project_minimal_from_db
- correct field types in select_maybe_flavor_group_from_db
- use left join in flavor select functions

### Dependencies
- bump avina-wire from 1.4.0 to 1.5.0
- bump anyhow from 1.0.94 to 1.0.95
- bump thiserror from 2.0.6 to 2.0.9
- bump reqwest from 0.12.9 to 0.12.10
- bump serde_json from 1.0.133 to 1.0.134
- bump serde from 1.0.215 to 1.0.216
- bump config from 0.14.1 to 0.15.4

## [avina-wire-v1.5.0] - 2024-12-29

### Features
- add UserBudgetSync struct to budgeting module

### Fixes
- add sqlx try_from and rename to Flavor fields in resource module

### Dependencies
- Bump serde from 1.0.215 to 1.0.216

## [avina-test-v0.4.0] - 2024-12-29

### Tests
- add flavor modify tests to resource module
- add flavor modify tests to resources module

### Fixes
- correct get call in e2e_lib_flavor_price_delete_works in pricing module

### Dependencies
- bump avina-wire from 1.4.0 to 1.5.0
- bump avina from 1.5.0 to 1.6.0
- bump avina-api from 0.5.0 to 0.6.0
- bump anyhow from 1.0.94 to 1.0.95
- bump reqwest from 0.12.9 to 0.12.10
- bump serde_json from 1.0.133 to 1.0.134

## Repository

### Docs
- add logo.png
- update readme with logo and badges
- remove version badge for test crate
- fix crate name in version badge link

### CI
- add workflow running zizmor to check workflows
- replace template injection by environment variable usage in test and lint workflows
- specify complete version on install action step in audit workflow
- replace install-action shorthand with full version in audit workflow
- set persist-credentials: false on all checkout steps
- change trigger from pull_request_target to pull_request in label workflow

### Dependencies
- bump taiki-e/install-action from 2.46.20 to 2.47.0
- bump taiki-e/install-action from 2.46.9 to 2.46.20
- bump taiki-e/install-action from 2.46.8 to 2.46.9

### Misc
- update sqlx offline query data
- increase mariadb connection limit in init_db.sh script

## [avina-test-v0.3.0] - 2024-12-11

### Dependencies
- add chrono dependency
- add anyhow dependency
- bump chrono from 0.4.38 to 0.4.39
- run cargo update
- bump avina-wire from 1.3 to 1.4
- bump avina-lib from 1.4 to 1.5
- bump avina-api from 0.4 to 0.5

### Tests
- add TestApp.setup_test_flavor
- add server state create tests
- reuse api::database::insert_flavor_into_db in test
- add TestApp.setup_test_server_state
- add server state delete tests
- add helper assert_equal_server_states
- use assert_equal_server_states in server state create tests
- add server_state_get tests
- fix some linting issues in server_state_get test
- adjust tests for new NotFoundErrors
- fix minor linting issues
- correct expected not found message in server_state_delete test
- use assert_equal_server_states in server_state_get tests
- revise expected not found error messages
- add equal_server_states and (assert_)contains_server_state
- add first few server_state_list tests
- add server_state_modify tests
- add TestApp.setup_test_server_state_with_server_id
- add e2e_lib_server_state_list_server_filter_works_across_projects_for_admin_user
- add e2e_lib_server_state_list_server_filter_stays_within_project_for_master_user
- e2e_lib_master_user_can_combine_server_state_list_filters
- e2e_lib_admin_user_can_combine_server_state_list_filters
- add flavor quota delete tests
- add flavor price delete tests
- add TestApp.setup_test_flavor_price function
- add flavor group delete tests
- fix typos in flavor delete test error messages
- add TestApp.setup_test_flavor_group
- add todo comment to project delete tests
- add flavor_delete tests
- add project_budget_delete tests
- add TestApp.setup_test_project_budget
- add user_budget_delete tests
- add TestApp.setup_test_user_budget

## [avina-cli-v1.4.0] - 2024-12-11

### Features
- route server-state crud and other delete commands to Rust API

### Dependencies
- run cargo update
- bump tabled from 0.16.0 to 0.17.0
- bump clap from 4.5.22 to 4.5.23
- bump chrono from 0.4.38 to 0.4.39
- bump avina-lib from 1.4 to 1.5
- bump avina-wire from 1.3 to 1.4

## [avina-lib-v1.5.0] - 2024-12-11

### Dependencies
- bump chrono from 0.4.38 to 0.4.39
- bump thiserror from 2.0.4 to 2.0.6
- run cargo update
- bump avina-wire from 1.3 to 1.4

### Fixes
- add missing trailing slash in server_state_modify url

## [avina-api-v0.5.0] - 2024-12-11

### Dependencies
- run cargo update
- bump tracing-subscriber from 0.3.18 to 0.3.19
- bump tracing from 0.1.40 to 0.1.41
- bump chrono from 0.4.38 to 0.4.39
- bump thiserror from 2.0.4 to 2.0.6
- bump avina-wire from 1.3 to 1.4

### Database
- move insert_flavor_into_db to database module
- move insert_server_state_into_db to database module
- add missing sqlx try_froms to ServerStateRow id fields
- adjust all getters for new NotFoundErrors
- add select_server_states_by_server_and_project_from_db
- add select_server_states_by_server_and_user_from_db
- move insert_flavor_quota_into_db into database module
- move insert_flavor_price_into_db to database module
- move insert_flavor_group_into_db to database module
- move insert_project_budget_into_db to database module
- move insert_user_budget_into_db to database module

### Error
- match messages for all NotFoundError variants
- add NotFoundOnlyError with impls

### Endpoints
- revise getters for new NotFoundErrors
- remove done todo comment
- use require_master_user_or_return_not_found in user_get
- correct authorization check in server_state_get
- homogenize errors of server_state_list
- complete server_state_list endpoint

### Authorization
- add require_*_or_return_not_found functions

## [avina-wire-v1.4.0] - 2024-12-11

### Dependencies
- bump chrono from 0.4.38 to 0.4.39
- run cargo update

## [avina-test-v0.2.1] - 2024-11-22

### Dependencies
- bump wire from 1.2 to 1.3
- bump api from 0.3 to 0.4
- bump lib from 1.3 to 1.4
- bump serde from 1.0.210 to 1.0.214
- bump serde_json from 1.0.128 to 1.0.133
- bump tokio from 1.40.0 to 1.41.1
- bump reqwest from 0.12.8 to 0.12.9

## [avina-cli-v1.3.1] - 2024-11-22

### Dependencies
- bump anyhow from 1.0.89 to 1.0.93
- bump serde from 1.0.210 to 1.0.214
- bump serde_json from 1.0.128 to 1.0.133

## [avina-lib-v1.4.0] - 2024-11-22

### Refactors
- move ServerStateListParams from lib to wire
- move FlavorGroupListParams from lib to wire
- move FlavorListParams from lib to wire
- move FlavorQuotaListParams from lib to wire
- move ProjectBudgetListParams from lib to wire
- move UserBudgetListParams from lib to wire

### Dependencies
- bump anyhow from 1.0.89 to 1.0.93
- bump config from 0.14.0 to 0.14.1
- bump reqwest from 0.12.8 to 0.12.9
- bump serde from 1.0.210 to 1.0.214
- bump serde_json from 1.0.128 to 1.0.133
- bump thiserror from 1.0.64 to 2.0.0

## [avina-api-v0.4.0] - 2024-11-22

### Features

#### General
- add migrations for remaining active tables
- add accounting, quota, pricing, budgeting modules
- implement ResponseError for NotFoundOrUnexpectedApiError

#### Database
- add database module for shared database functions
- add resources::select_flavor_group_from_db to database module
- add resources::select_flavor_from_db to database module
- add database::pricing::flavor_price submodule
- add database::budgeting::project/user_budget submodule with helpers
- add database::quota submodule with helper functions
- add database::accounting submodule with helper functions
- add select_project_minimal_from_db function
- add select_minimal_flavors_by_group_from_db function
- add select_flavor_detail_from_db function
- move all select functions from routes to database module
- add server state select functions to database module
- add project budget select functions to database module
- add user budget select functions to database module
- add flavor price select functions to database module
- add flavor quota select functions to database module
- add select_all_flavor_groups_from_db to database module
- add select_lrz_flavor_groups_from_db to database module
- add select_all_flavors_from_db to database module
- add select_lrz_flavors_from_db to database module
- add select_flavors_by_flavor_group_from_db to database module

#### Endpoints
- add accounting::server_state_delete endpoint
- add resources::flavor_group_delete endpoint
- add resources::flavor_delete endpoint
- add quota::flavor_quota_delete endpoint
- add pricing::flavor_price_delete endpoint
- add budgeting::project_budget_delete endpoint
- add budgeting::user_budget_delete endpoint
- add accounting::server_state_create endpoint
- add quota::flavor_quota_create endpoint
- add resources::flavor_group_create endpoint
- add resources::flavor_create endpoint
- add pricing::flavor_price_create endpoint
- add budgeting::project_budget_create endpoint
- add budgeting::user_budget_create endpoint
- add resources::flavor_group_modify endpoint
- add resources::flavor_modify endpoint
- add pricing::flavor_price_modify endpoint
- add simplified budgeting::project_budget_modify endpoint
- add simplified budgeting::user_budget_modify endpoint
- add simplified quota::flavor_quota_modify endpoint
- add simplified accounting::server_state_modify endpoint
- add simplified accounting::erver_state_get endpoint
- add simplified resources::flavor_group_get endpoint
- require admin user for accounting::server_state_get
- require admin user for resources::flavor_group_get
- add simplified resources::flavor_get endpoint
- add simplified quota::flavor_quota_get endpoint
- add simplified pricing::flavor_price_get endpoint
- add simplified budgeting::project_budget_get
- add simplified budgeting::user_budget_get endpoint
- add simplified accounting::server_state_list endpoint
- add budgeting::project_budget_list endpoint
- add budgeting::user_budget_list endpoint
- add pricing::flavor_price_list endpoint
- add quota::flavor_quota_list endpoint
- add resources::flavor_group_list endpoint
- add resources::flavor_list endpoint

### Fixes
- minor naming fixes in user and pricing modules
- fix typo in select_project_budget_by_user_from_db function name

### Dependencies
- bump anyhow from 1.0.89 to 1.0.93
- bump config from 0.14.0 to 0.14.1
- bump reqwest from 0.12.8 to 0.12.9
- bump serde from 1.0.210 to 1.0.214
- bump serde_json from 1.0.128 to 1.0.133
- bump thiserror from 1.0.64 to 2.0.0
- bump tokio from 1.40.0 to 1.41.1
- bump tracing-actix-web from 0.7.13 to 0.7.15

## [avina-wire-v1.3.0] - 2024-11-22

### Features
- derive Deserialize for FlavorPriceCreateData
- derive Deserialize for UserBudgetCreateData
- derive FromRow for Flavor and make group_name field public
- derive Deserialize for FlavorPriceModifyData
- derive FromRow for ProjectBudget and UserBudget
- derive Deserialize for ProjectBudgetModifyData and UserBudgetModifyData
- derive FromRow for FlavorQuota
- derive FromRow for FlavorMinimal
- move ServerStateListParams from lib to wire
- move FlavorGroupListParams from lib to wire
- move FlavorListParams from lib to wire
- move FlavorQuotaListParams from lib to wire
- move ProjectBudgetListParams from lib to wire
- move UserBudgetListParams from lib to wire

### Dependencies
- bump serde from 1.0.210 to 1.0.214
- bump uuid from 1.10.0 to 1.11.0

## [avina-cli-v1.3.0] - 2024-10-08

### Features
- point user commands except import to rust api as well

### Dependencies
- bump wire from 1.1 to 1.2
- bump lib from 1.2 to 1.3
- bump clap version from 4.5.18 to 4.5.20

## [avina-test-v0.2.0] - 2024-10-08

### Features
- add tests for all user endpoints but import
- add TestUser/Project and TestApp.setup_test_user/project
- simplify assertions by using PartialEq implementations
- add tests for master user authorization on user and project endpoints
- deactivate admin user insert in test setup

### Dependencies
- bump wire from 1.1 to 1.2
- bump api from 0.2 to 0.3
- bump lib from 1.2 to 1.3
- test: bump reqwest version from 0.12.7 to 0.12.8
- test: bump once_cell from 1.20.1 to 1.20.2

## [avina-lib-v1.3.0] - 2024-10-08

### Features
- use User instead of UserCreated for UserApi::create call
- revise UserCreateRequest for new UserCreateData
- revise UserListRequest to use UserListParams

### Dependencies
- bump wire from 1.1 to 1.2
- bump reqwest version from 0.12.7 to 0.12.8

## [avina-api-v0.3.0] - 2024-10-08

### Features
- add authorization module and move require_admin_user there
- add require_master_user to authorization module
- add user me, create, delete, get, list, and modify endpoints
- implement proper master user access to user get and list endpoint
- make user and project create submodules public
- add ApplicationSettings.insert_user_admin
- insert admin user and project on application start when set

### Dependencies
- bump wire from 1.1 to 1.2
- bump once_cell from 1.20.1 to 1.20.2

## [avina-wire-v1.2.0] - 2024-10-08

### Features
- remove UserCreated
- make UserCreateData.is_staff/is_active Options
- derive FromRow for User, UserDetailed, and ProjectMinimal
- add UserListParams
- impl PartialEq for all response structs
- implement inter-type PartialEqs for User and Project structs

## [avina-cli-v1.2.1] - 2024-09-30

### Features
- use Rust API for project commands by default

## [avina-cli-v1.2.0] - 2024-09-30

### Dependencies
- bump lib from 1.1 to 1.2
- bump wire from 1.0 to 1.1

## [avina-test-v0.1.0] - 2024-09-30

### Features
- add test crate for shared test helpers and cross-crate testing
- add test/tests with two library e2e tests for hello endpoint
- revise project list tests to use all parameter
- split project tests into submodules and add more tests
- move api/tests/helpers to test crate

## [avina-lib-v1.2.0] - 2024-09-30

### Features
- refactor to use Project for project create response
- use wire.project.ProjectListParams in project list with serde_urlencoded

## [avina-api-v0.2.0] - 2024-09-30

### Features
- move api/tests/helpers to test crate
- move endpoint scope creation to respective modules
- add not_found(_error) and default_service

#### Implement Project Endpoint
- add rudimentary project endpoint
- split project endpoint into submodules
- move require_admin_user from middleware to handlers
- add hierarchical api errors
- split of query functions from all project handlers
- add proper error handling to all project handlers
- implement all and user class filters for project list
- implement limited normal user access for project get
- fill users and flavor_groups field in project get handler
- refactor to use Project for project create response

### Dependencies
- bump secrecy from 0.10.1 to 0.10.2
- bump thiserror from 1.0.63 to 1.0.64
- bump once_cell from 1.20.0 to 1.20.1

## [avina-wire-v1.1.0] - 2024-09-30

### Features
- derive FromRow for Project, UserMinimal, FlavorGroupMinimal
- remove ProjectCreated
- add ProjectListParams

## [avina-cli-v1.1.2] - 2024-09-24

### Fixes
- revise to parse ProjectRetrieved in project get command

### Dependencies
- bump clap from 4.5.17 to 4.5.18

## [avina-lib-v1.1.1] - 2024-09-24

### Fixes
- revise to parse ProjectRetrieved during ProjectApi.get_project

### Dependencies
- bump thiserror from 1.0.63 to 1.0.64

### Testing
- add e2e tests for hello and project modules

## [avina-wire-v1.0.1] - 2024-09-24

### Fixes
- add project.ProjectRetrieved enum

## [avina-cli-v1.1.1] - 2024-09-15

### Fixes
- Add aliases `show` to all `get` commands to comply with former Python CLI client.

### Documentation
- Update name in `cargo install` command in README.

## [avina-api-v0.1.0] - 2024-09-15
Initial release of the `avina-api` crate containing a partial rewrite of the API
server with authentication, the hello endpoint, proper testing, and basic
docker deployment.

## [avina-cli-v1.1.0] - 2024-09-15
Initial release of the `avina-cli` crate containing just the CLI application.

## [avina-v1.1.0] - 2024-09-15
Initial release of the `avina` crate containing just the Rust bindings.

## [avina-wire-v1.0.0] - 2024-09-15
Initial release of the `avina-wire` crate containing just the shared data
structures used for API communication.

## [v1.0.0] - 2024-08-16
Initial release of the `avina` crate containing the new Rust bindings as well
as a full Rust rewrite of the CLI application.
