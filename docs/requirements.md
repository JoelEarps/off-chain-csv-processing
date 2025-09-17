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
3. When an account is locked no other tx's can be performed unless the next state is relevant

## Out of scope

1. Handling out of order txs.

## How I would improve the current system and expand

1. Use typestate to create a state machine to only allow transitions of the states for Dispute -> Resolve -> Chargeback.
2. The TCP server improvements - stream -> channel.
3. Undefined scenarios - what about if no amount value present in the tx.
4. Use thiserror to create definite errors for the application, leading to a better and more specific error report
5. Getters and setters to provide better encapsulation.
6. Use of BigDecimal to stop floating point errors and therefore keep precision to 4 decimal places.
7. Add a logger to enable better debugging and configurable log level.
8. Create config for type of stream being created and any secrets/ config data that may be required.
9. Create two separate tasks running with a channel between them to handle asynchronous handling of stream reading and account cache updates.
