# How I Solve Problems Such as These

## Stage 1: As simple as possible — branch `initial-build`

- Define non-functional and functional requirements to clarify expected behaviors.
- Start with the simplest working case (no disputes).
- Apply a TDD approach from the beginning.
- Define scenarios as BDD specifications (e.g., Gherkin) so that business stakeholders can easily understand and validate them.
- Focus first on input/output correctness.

For this case, the initial step was about:  

- Loading data into a stream.
- Deserialising transactions into an event-driven architecture.
- Matching events as they occur (e.g., deposits/withdrawals).
- Updating held funds per client correctly.

## Stage 2: Tidy up into a maintainable format — branch `stage-2/tidy-up`

- Refactor towards SOLID principles.  
- Structure code so that each folder maps to a domain in the system.  
- Ensure types and utils inherit the parent directory name, improving clarity.  
- Apply separation of concerns so that each module has a clear responsibility and is easy to extend.  
- Increase testability and enable dependency injection.  
  - Example: the generic `BoxStream` abstraction can be injected for testing with TCP, file streams, or websockets.  

## Stage 3: Increase complexity — branch `stage-3/complex scenarios`

- Introduce business complexity such as disputes and resolutions.  
- Define new BDD scenarios covering these cases.  
- Add test fixtures to simulate realistic flows.  
- Continue a TDD-first cycle: write tests → implement minimal logic → refactor.  

## Testing Approach

I aim for a holistic testing strategy that balances readability, stakeholder communication, and technical coverage.  

### BDD Scenarios

- Written in Gherkin format to keep tests human-readable.
- Ensures product, engineering, and business teams share the same understanding.
- Used to drive both acceptance tests and automated validation.

### Unit Tests

- Core logic is covered with unit tests.
- Written with `rstest` and Rust’s built-in test framework.
- Driven by the Gherkin specs to ensure scenarios map to low-level checks.
- Always TDD: write the failing test, implement the minimum solution, then refactor.

### Integration Tests

- Validate that modules work together (e.g., parsing transactions through the full pipeline).
- Include setup/teardown of streams and simulated environments (file, TCP, websocket).
- Focus on correctness of end-to-end flows.
