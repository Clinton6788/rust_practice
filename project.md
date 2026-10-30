# Market Operations Simulator
**Project Type:** Incremental systems programming capstone  
**Primary Language:** Rust  
**Secondary Languages:** C++, Go, Python  
**Objective:** Build a simulated trading system that processes market data, makes trading decisions, manages positions, and operates reliably under normal and failure conditions.

## 1. Project Overview

Develop a miniature automated trading system that monitors simulated market prices, identifies trading opportunities, executes simulated orders, and tracks account performance.

The program will initially operate as a simple command-line application. As development progresses, it will evolve into a modular system with persistent storage, asynchronous operations, multiple workers, and network communication.

The trading logic should remain simple. The primary objective is to learn how to design, implement, and maintain a software system.

### Core Rules

1. The simulator receives market prices and processes them in chronological order.
2. A strategy generates BUY, SELL, or HOLD signals based on configurable conditions.
3. Orders are validated against account funds, position limits, and other risk constraints.
4. Accepted orders generate simulated fills.
5. Fills update positions, cash, and realized profit or loss.
6. Every important operation generates an event that can be logged and recorded.
7. Components must handle invalid inputs, failures, and shutdown without corrupting application state.

### Initial Trading Rules

Keep the financial logic deliberately basic:

- Start with a configurable account balance.
- Track one or more simulated instruments.
- Generate a BUY signal when the price falls below a configured threshold.
- Generate a SELL signal when the price rises above a configured threshold.
- Otherwise, generate HOLD.
- Each order has a configurable quantity.
- Reject orders that violate risk limits.
- Execute accepted orders at the current simulated price.
- Calculate realized PnL when a position is closed.

For simplicity, assume no commissions, slippage, partial fills, or market impact in the initial version.

These can be added later if desired.

---

## 2. Development Phases

Complete each phase before moving to the next. Keep the existing program working as you add functionality.

### Phase 1 — Basic Simulator

**Goal:** Create a working command-line program that simulates a trading session.

Implement:
- Project initialization and dependency management.
- Variables, constants, and appropriate data types.
- Type conversions and input validation.
- Arithmetic and string manipulation.
- Arrays or vectors containing simulated market prices.
- Conditional logic and loops.
- Functions for calculating signals and PnL.
- Aggregations, filtering, searching, and sorting.
- User-configurable starting balance and trading thresholds.

**Deliverable:** Run a simulated trading session and print the trades, ending balance, and total PnL.

### Phase 2 — Structured Trading System

**Goal:** Organize the program into independent components.

Create components for:
- Market data.
- Strategy evaluation.
- Order management.
- Risk management.
- Account and position tracking.
- Logging and reporting.

Implement:
- Structs and enums.
- Constructors and methods.
- Encapsulation and controlled state mutation.
- Composition and shared behavioral interfaces.
- Functions with multiple parameters and return values.
- Module separation and reusable libraries.
- Configurable strategy implementations.
- Explicit state and state transitions.
- Initialization, activation, shutdown, and cleanup.

A strategy should not execute orders directly. It should generate a signal that the order-management component evaluates.

**Deliverable:** A modular simulator in which each component has a defined responsibility and public interface.

### Phase 3 — Memory and Resource Management

**Goal:** Understand how the language manages data and resource lifetimes.

Implement and investigate:
- References and indirect mutation.
- Ownership transfers.
- Borrowing and lifetimes.
- Shallow versus deep copies.
- Dynamic allocation and collection growth.
- Shared ownership.
- Resource cleanup.
- Memory usage and allocation behavior.

Use the same market-data processing operation to investigate how Rust and C++ handle ownership differently.

For C++, experiment with RAII, `std::unique_ptr`, `std::shared_ptr`, and non-owning references.

For Rust, use ownership, borrowing, `Box`, and—when necessary—`Arc`.

For C, implement a small dynamically allocated array to understand manual memory management.

**Deliverable:** A collection-processing component with documented ownership rules and a comparison of the languages' approaches.

### Phase 4 — Persistence and Recovery

**Goal:** Preserve trading activity and restore the simulator after a restart.

Implement:
- Reading market data from files.
- Exporting trades and reports.
- JSON configuration and serialization.
- Binary versus text record storage.
- SQLite storage for orders, fills, and account state.
- Database CRUD operations.
- Transactions and rollback.
- Configuration defaults and overrides.
- An append-only event journal.
- State reconstruction by replaying events.
- Recovery from the last valid state.

For example, if the simulator records three fills and then terminates unexpectedly, it should be possible to reconstruct the account state from the recorded events.

**Deliverable:** A persistent simulator that can restart and reconstruct its trading history without duplicating trades.

### Phase 5 — Reliability and Error Handling

**Goal:** Make failures explicit and recoverable.

Introduce controlled failures:
- Invalid market data.
- Missing configuration.
- Rejected orders.
- Database errors.
- Failed worker initialization.
- Timeouts.
- Temporary service unavailability.
- Interrupted operations.

Implement:
- Custom error types.
- Error propagation.
- Input-validation boundaries.
- Structured logging.
- Retry policies.
- Operation timeouts.
- Cleanup on failure.
- Fault isolation.
- Transactional state changes.
- Recovery from a known valid state.
- Unit tests for normal and exceptional behavior.

For example, a database failure must not leave an order recorded as filled when its corresponding state changes were never committed.

**Deliverable:** A simulator that reports failures clearly and preserves consistent account state when operations fail.

### Phase 6 — Concurrent Processing

**Goal:** Process market data and trading jobs using multiple workers.

Introduce separate workers for tasks such as:
- Market-data processing.
- Strategy evaluation.
- Order validation.
- Trade execution.
- Reporting and persistence.

Implement:
- Concurrent tasks.
- Asynchronous operations.
- Worker pools and queues.
- Bounded queues and backpressure.
- Shared-state synchronization.
- Cancellation.
- Graceful shutdown.
- Worker supervision.
- Dynamic worker creation and removal.
- Worker restart policies.

Create controlled examples of a race condition and a deadlock. Investigate the cause, then correct the implementation.

Ensure that shutting down the simulator stops new work, handles outstanding jobs according to policy, and releases resources.

**Deliverable:** A concurrent simulator that can process multiple instruments or independent trading jobs without corrupting shared state.

### Phase 7 — Networking and Distributed Workers

**Goal:** Separate the simulator into communicating processes.

Run the trading engine and worker manager as separate processes.

Implement:
- An HTTP endpoint for account status.
- An HTTP client for requesting information.
- TCP communication and message framing.
- Structured message serialization.
- Request/reply correlation.
- Publish/subscribe events.
- Worker heartbeats.
- Request identifiers and idempotency.
- Retry and duplicate-message handling.
- Worker status monitoring.
- Timeouts and restart requests.
- Remote worker shutdown.

The worker manager should track the difference between the state it wants a worker to be in and the state the worker reports.

For example, a worker may be intended to run but report that it has stopped. The manager must detect the discrepancy and apply its configured recovery policy.

**Deliverable:** A multi-process simulator that can monitor workers, exchange messages, and recover from selected failures.

### Phase 8 — Testing and Performance

**Goal:** Evaluate correctness, performance, and maintainability.

Implement:
- Unit tests.
- Integration tests.
- Property-based tests.
- Mock dependencies.
- Stress tests.
- Profiling.
- Runtime benchmarks.
- Memory-usage measurements.
- Reproducible builds.
- Formatting and linting.
- Static analysis.

Benchmark the same market-data processing algorithm in Rust and C++ using equivalent inputs and comparable optimization settings.

Compare:
- Total execution time.
- Startup time.
- Memory consumption.
- Performance as dataset size increases.
- Performance under concurrent workloads.
- Development complexity.
- Ease of testing and maintenance.

**Deliverable:** A benchmark report identifying actual bottlenecks and explaining which implementation is most appropriate for each workload.

---

## 3. Language-Specific Experiments

Some concepts are language-specific and should be explored through small experiments within the project rather than forced into the core architecture.

### Python
- Compare dynamic typing and runtime validation.
- Benchmark ordinary loops against NumPy vectorization.
- Call a compiled Rust or C++ component from Python.

### C
- Implement a dynamically sized array.
- Build a function-pointer dispatch table.
- Investigate manual allocation and cleanup.

### C++
- Implement resource-owning types using RAII.
- Write generic algorithms using templates.
- Compare unique ownership, shared ownership, and references.
- Implement the same worker interface using inheritance and composition.

### Rust
- Resolve ownership and borrowing constraints without unnecessary cloning.
- Implement explicit error propagation with `Result` and `Option`.
- Use traits and generics to support interchangeable workers.
- Compare shared ownership with message passing.

### Go
- Implement the worker pool using goroutines and channels.
- Propagate cancellation through nested operations using contexts.
- Implement interchangeable storage backends through interfaces.

### Cross-Language Comparison
Implement the same small worker lifecycle using:
1. Object-oriented design.
2. Procedural functions.
3. Event-driven design.

Compare clarity, complexity, testability, performance, and failure handling.

---

## 4. Final Acceptance Criteria

The project is complete when it can:

1. Process market data and generate trading signals.
2. Validate orders and enforce risk limits.
3. Track positions, fills, and account performance.
4. Persist its state and reconstruct it after restarting.
5. Handle expected failures without corrupting account state.
6. Run multiple workers concurrently.
7. Communicate with workers through a documented message interface.
8. Detect failed workers and apply recovery policies.
9. Shut down gracefully.
10. Run automated tests and repeatable performance benchmarks.

The final implementation should be understandable, modular, and testable. Avoid adding financial complexity unless it serves a specific programming objective.

**Guiding principle:** The trading simulator provides the motivation; the real assignment is to understand how each language handles data, state, memory, concurrency, failures, and system design.