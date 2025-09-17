# Requirements

From reading the scope document, the following is a break down of the requirements of the system.

## Functional Requirements

1. Read an input csv
2. Output a to std out
3. Must be ran like this `The input file is the first and only argument to the binary. Output should be written to std out` - therefore no use of clap or writing to csv necessary.
4. Handle incorrect scenarios
5. Tx ids are handled as a u32 (min 0, max ) and client id is a u16 (min 0, max).
6. If a client does not have sufficient available funds the withdrawal should fail and the total amount
of funds should not change

## Non functional

1. Try to use as much safe code as possible
2. Can we use an alternative approach to synchronous programming, can we stream data for example?

## Assumptions

1. Transactions are loaded in chronological order - therefore no need for ordering
2. Precision of 4 places.
3. When an account is locked no other tx's can be performed unless the next state is relevant.
4. If the tx specified by the dispute doesn't exist you can ignore it
and assume this is an error on our partners side.

## Out of scope

1. Handling out of order txs.

## How I Would Improve the Current System

### Turn event type into enum and deserialise

### Use the typestate pattern to create a state machine

Enforce valid state transitions (e.g., `Dispute -> Resolve or Chargeback`) using typestate.  
This leverages phantom data to ensure transitions are encoded at the type level, reducing the need for runtime checks in `match` statements.  

### Macro to reduce duplication

Introduce macros to eliminate repetitive logic in functions such as `dispute_and_hold_funds` and `resolve_dispute`.  
This would improve readability, maintainability, and reduce the likelihood of subtle bugs from duplicated code.  

### Undefined scenarios

Consider how the system should handle edge cases, such as:  

1. Missing `amount` values in transactions.
2. Withdrawn transactions that cannot create accounts but are later disputed.
These undefined flows need explicit handling to avoid inconsistent state.

### Better error definition

Adopt [`thiserror`](https://crates.io/crates/thiserror) for well-defined application error types.  
This provides clearer error semantics and makes debugging and reporting more precise.  

### Handling of errors

Currently, errors are wrapped using `anyhow`, which simplifies error propagation but obscures specifics.  
Improvements:  

1. Use `thiserror` for domain errors and reserve `anyhow` for generic application entry points.  
2. Introduce structured logging of errors without polluting the program output.  
3. Provide actionable error messages for both developers and operators.  

### Getters and setters

Introduce getters and setters where appropriate to ensure better encapsulation of account state and prevent accidental misuse.  

### Use of BigDecimal

The system currently uses `f64`, which can represent the required precision (4 decimal places), but floating-point operations may lead to subtle rounding issues.  
Adopt [BigDecimal](https://crates.io/crates/bigdecimal), an arbitrary-precision decimal type, to ensure accurate financial calculations.  

### 12-Factor App improvements

#### Logger

Add structured, configurable logging to improve debugging and enable monitoring in production.  
Support multiple log levels (debug, info, warn, error) with outputs tailored to different environments.  

#### Configuration

Introduce a configuration system (via environment variables, config files, or secrets management).  
This enables flexibility for different deployment environments, testing setups, and security-sensitive data.  

### Separate tasks with channel communication

Split responsibilities into two asynchronous tasks:  

1. One task for stream reading.
2. One task for updating the account cache.  
Coordinate via channels for improved scalability and cleaner concurrency handling.  

#### Handle disputes on withdrawals

Clarify the business logic for disputed withdrawals:

1. From a “held funds” perspective, disputes on withdrawals don’t make sense (you cannot hold withdrawn funds).
2. Could instead be treated as temporary credit, but this requires clear business rules.

Close collaboration with product stakeholders is needed to define a consistent approach.  

### More complex scenarios

Explore additional improvements for robustness and performance:  

1. **Batch processing & parallelisation**: Sort and group transactions by account/ID to process in parallel safely.  
2. **Overflow handling**: Ensure the system prevents or gracefully handles numerical overflows in balances.  
3. **Resilience to malformed input**: Validate input formats strictly, e.g., commas at the end of CSV rows.  

### Additional improvements to consider

#### Testing

1. Property-based tests to validate state transitions.  
2. Load tests to measure performance under heavy streams.
3. Golden-file tests to lock down expected outputs.

#### Observability

1. Metrics collection (e.g., Prometheus integration).  
2. Tracing for async flows (`tracing` crate).

#### Documentation & onboarding

1. Clearer docs for contributors.  
2. Architectural overview diagrams.  
3. Examples for typical transaction flows.
