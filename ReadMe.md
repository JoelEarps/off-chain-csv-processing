# Off Chain CSV Processing

The following repo shows a coding challenge completed when aiming to process off chain

## Project Structure

The project is set up as a cargo crate, standard set up

Docs folder

## Run

`cargo run -- transactions.csv > accounts.csv`

## Build

`cargo build`

## Test

`cargo test`

### Code coverage

```bash

|| src/main.rs: 5-9
|| src/stream_handler/file_stream.rs: 19-29
|| src/transaction_handler/cache_handler.rs: 20-21
|| src/user_accounts_cache/cache_handler.rs: 33, 38, 54-55, 81-82, 103, 106-107, 111, 125, 128-129, 133, 147, 150-151, 155, 158, 164-168, 170, 173
|| src/user_accounts_cache/user_accounts.rs: 54, 65, 74, 85, 97, 99
|| Tested/Total Lines:
|| src/application_component_manager.rs: 0/23 +0.00%
|| src/main.rs: 0/5 +0.00%
|| src/stream_handler/file_stream.rs: 0/11 +0.00%
|| src/transaction_handler/cache_handler.rs: 22/24 +0.00%
|| src/transaction_handler/types.rs: 6/6 +0.00%
|| src/user_accounts_cache/cache_handler.rs: 47/73 +11.75%
|| src/user_accounts_cache/user_accounts.rs: 44/50 +0.50%
|| 
61.98% coverage, 119/192 lines covered, +7.55% change in coverage

```

## Branch documentation

Please see `docs/how-i-handle-problems.md` to view how I handled each problem and what each branch means
Please see `docs/software-flow.md` to view how the software works via a mermaid diagram.
Please see `docs/bdd-scenarios` for all scenarios covered by the application, written in Gherkin Syntax.
Please see `docs/decision-docs` for a list of decision e.g. arch, generics and crates.
