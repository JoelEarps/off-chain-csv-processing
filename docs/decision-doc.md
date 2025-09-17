# Decision Document

## Use of error report dumps rather than printing at every error

In low-level systems, I have worked with system dumps, system logs, and error dumps, which proved invaluable when decoding issues. They helped isolate problems quickly, even in production environments, and by sending error dumps to a central location and tying them into alerts, we were able to respond to incidents faster and with better context.

For this application, given I wanted a specific output format, I had no logger and I did not want anything influencing output, I chose logging for the following reasons (with some additional benefits I foresaw in the future)

1. Error dumps allow you to track the number and types of errors over time, making it easier to build dashboards, identify recurring patterns, and monitor system health.
2. They make it possible to set thresholds for acceptable error rates, which in turn enables automated alerts when these thresholds are breached.
3. Dumps focus on actionable error data rather than clutter, improving the signal-to-noise ratio and reducing the risk of missing critical issues.
4. They also reduce the cost of logging by avoiding verbose outputs, capturing only the most relevant information and sending it to monitoring systems when needed.

## External Crate usage

| Crate        | Description                                                                                                                               | Maintenance status                                                                 |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| anyhow       | Provides `anyhow::Error`, a general-purpose error type + utilities for easy error handling in Rust applications. ([Crates][1])            | **Active** — recently updated; used broadly.                                       |
| async-trait  | Attribute macro to allow `async fn` in traits (i.e.\ trait methods) in stable Rust by boxing the returned future. ([Crates][2])           | **Active** — recent release; kept up-to-date.                                      |
| csv-async    | Asynchronous CSV reader & writer; aims to mimic the synchronous `csv` crate API, for async environments (with tokio etc.). ([Docs.rs][3]) | **Active** — version 1.3.1 was released relatively recently; looks maintained.     |
| futures      | Core abstractions for async programming: Futures, Streams, Sinks etc.; utilities around async I/O and compositions. ([Docs.rs][4])        | **Active / Mature** — stable with regular updates; widely used.                    |
| rstest       | Test framework for Rust: fixtures, parameterised tests, table - based tests. ([Crates][5])                                                | **Active** — recent versions; seems well maintained.                               |
| serde        | Framework for serializing / deserializing data; derive macros, etc. ([Docs.rs][3])                                                        | **Active / Core** — one of the essential crates; very well maintained.             |
| tokio        | Asynchronous runtime: non-blocking I/O, scheduling, tasks, timers, I/O drivers etc. ([Docs.rs][6])                                        | **Active / Very Active** — central to async Rust; frequent updates.                |
| tokio-stream | Utilities for working with streams in the Tokio ecosystem. (Stream trait etc.)                                                            | **Active** — maintained as part of the Tokio siblings; aligns with Tokio releases. |

[1]: https://crates.io/crates/anyhow?utm_source=chatgpt.com "anyhow - crates.io: Rust Package Registry"
[2]: https://crates.io/crates/async-trait?utm_source=chatgpt.com "async-trait - crates.io: Rust Package Registry"
[3]: https://docs.rs/csv-async?utm_source=chatgpt.com "csv_async - Rust"
[4]: https://docs.rs/futures/?utm_source=chatgpt.com "futures - Rust"
[5]: https://crates.io/crates/rstest?utm_source=chatgpt.com "rstest - Rust Package Registry"
[6]: https://docs.rs/tokio?utm_source=chatgpt.com "tokio - Rust"
