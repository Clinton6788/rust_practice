# Cross-Language Naming Conventions

| Element | Python | Go | C | C++ | Rust |
|---|---|---|---|---|---|
| Variables | `snake_case` | `camelCase` | `snake_case` | `snake_case` | `snake_case` |
| Functions | `snake_case` | `PascalCase` for exported, `camelCase` for unexported | `snake_case` | `snake_case` | `snake_case` |
| Classes / Structs / Types | `PascalCase` | `PascalCase` | Usually `snake_case` for struct tags; typedefs vary | `PascalCase` commonly | `PascalCase` |
| Constants | `UPPER_SNAKE_CASE` commonly | `PascalCase` or `camelCase` depending on visibility | `UPPER_SNAKE_CASE` | `UPPER_SNAKE_CASE` commonly | `UPPER_SNAKE_CASE` |
| Enum members | `UPPER_SNAKE_CASE` commonly | Often `PascalCase` | `UPPER_SNAKE_CASE` commonly | `PascalCase` or `UPPER_SNAKE_CASE` | `PascalCase` |
| File names | `snake_case.py` | Usually `snake_case.go` | Usually `snake_case.c` or project-specific | Usually `snake_case.cpp` or project-specific | `snake_case.rs` |

# Cross-Language Programming Problem Set

## Getting Started - Basics
- **BSC-01 - Packages:** Understand the required packages/downloads
- **BSC-02 - File Structure:** Start a new project in the language's style
- **BSC-03 - Using Dependencies:** Import/use an add-on library

## 1. Fundamentals — Variables, Types, and Data Shapes
- **FND-01 — Variable Assignment:** Declare variables of different types and assign, reassign, and print their values.
- **FND-02 — Type Conversion:** Convert between integers, floating-point numbers, strings, and booleans, handling invalid conversions.
- **FND-03 — Constants:** Define constants, attempt to modify them, and compare compile-time and runtime restrictions where applicable.
- **FND-04 — Arithmetic:** Perform arithmetic operations, integer division, floating-point division, modulo, and exponentiation.
- **FND-05 — String Manipulation:** Concatenate, split, search, replace, and format strings.
- **FND-06 — Arrays and Lists:** Create a collection of numbers, access elements by index, modify values, and determine its length.
- **FND-07 — Multidimensional Data:** Create and traverse a two-dimensional grid, retrieving and modifying individual cells.
- **FND-08 — Key-Value Data:** Store, retrieve, update, and delete values using string keys.
- **FND-09 — Sets and Uniqueness:** Remove duplicate values from a collection while preserving or explicitly disregarding order.
- **FND-10 — Input and Validation:** Accept user input, validate its type and range, and reject invalid values.

## 2. Control Flow — Conditions and Loops
- **CTL-01 — Conditional Branching:** Classify a number as positive, negative, or zero, and as even or odd.
- **CTL-02 — Compound Conditions:** Evaluate multiple conditions using AND, OR, and NOT to determine whether an action is permitted.
- **CTL-03 — Counting Loop:** Print a sequence of integers between configurable start and end values.
- **CTL-04 — Conditional Loop:** Continue accepting numbers until a termination condition is met.
- **CTL-05 — Nested Loops:** Generate a multiplication table or traverse every cell in a two-dimensional grid.
- **CTL-06 — Loop Control:** Use continue, break, or equivalent mechanisms to skip or terminate iterations.
- **CTL-07 — Array Aggregation:** Calculate the sum, average, minimum, and maximum of a numeric collection.
- **CTL-08 — Filtering:** Create a new collection containing only elements that satisfy configurable conditions.
- **CTL-09 — Searching:** Find the first occurrence of a target value and return its index or an explicit not-found result.
- **CTL-10 — Sorting:** Sort a collection in ascending and descending order, using built-in and manual implementations where practical.

## 3. Functions and Modular Design
- **MOD-01 — Basic Functions:** Create functions that accept parameters, perform calculations, and return results.
- **MOD-02 — Multiple Parameters:** Write a function that accepts a collection, a threshold, and an operation and returns a calculated result.
- **MOD-03 — Scope:** Compare local variables, module-level variables, global variables, and persistent state.
- **MOD-04 — Mutable Arguments:** Pass a collection to a function, modify it, and determine whether the caller observes the changes.
- **MOD-05 — Recursion:** Implement factorial, Fibonacci, or recursive directory traversal and examine recursion limits.
- **MOD-06 — Function Composition:** Combine multiple functions into a pipeline that transforms input data into a final result.
- **MOD-07 — Higher-Order Functions:** Pass functions as arguments or return them from functions where the language supports it.
- **MOD-08 — Modularization:** Split a program into multiple source files or modules with explicit public and private interfaces.
- **MOD-09 — Dependency Injection:** Supply a function or service to a component instead of having the component construct its own dependency.
- **MOD-10 — Reusable Library:** Package a collection of functions and import or link it from a separate program.

## 4. Data Structures and Algorithms
- **DSA-01 — Stack:** Implement push, pop, and peek operations and detect underflow.
- **DSA-02 — Queue:** Implement enqueue and dequeue operations and compare array-based and linked implementations where practical.
- **DSA-03 — Linked List:** Build a singly linked list supporting insertion, deletion, traversal, and search.
- **DSA-04 — Hash Table:** Implement key-value storage and investigate collisions, hashing, and lookup performance.
- **DSA-05 — Binary Search:** Search a sorted collection using an iterative and recursive implementation.
- **DSA-06 — Merge Two Collections:** Combine two sorted collections into one sorted collection without sorting the result afterward.
- **DSA-07 — Deduplication:** Remove duplicate records using a selected field as the identity key.
- **DSA-08 — Sliding Window:** Calculate rolling sums and averages over a numeric time series.
- **DSA-09 — Event Queue:** Store incoming events and process them in order, supporting different event types.
- **DSA-10 — Complexity Comparison:** Implement two solutions to the same problem and compare their runtime and memory usage as input size increases.

## 5. State and Mutation
- **STA-01 — Persistent State:** Maintain a counter across repeated function calls without passing its current value directly each time.
- **STA-02 — Explicit State:** Represent a system's state in a data structure and pass it to functions that inspect or modify it.
- **STA-03 — State Transitions:** Implement a state machine with CREATED, RUNNING, STOPPING, STOPPED, and FAILED states.
- **STA-04 — Transition Validation:** Reject invalid state transitions and return an informative error describing the attempted operation.
- **STA-05 — Nested Mutation:** Modify a deeply nested data structure and verify that unrelated values remain unchanged.
- **STA-06 — Copy Versus Reference:** Copy a collection, modify the copy, and investigate whether the original changes.
- **STA-07 — Transactional Mutation:** Apply several changes to state atomically or roll them back if an operation fails.
- **STA-08 — Undo and Redo:** Record state changes and implement undo and redo operations.
- **STA-09 — Event-Driven State:** Transform a current state by processing a sequence of events and produce the final state.
- **STA-10 — State Persistence:** Serialize application state to a file, reload it, and verify that the restored state matches the original.

## 6. Errors, Validation, and Reliability
- **ERR-01 — Custom Errors:** Define a custom error type and raise, return, or propagate it through multiple functions.
- **ERR-02 — Error Categories:** Distinguish invalid input, missing resources, failed operations, and unrecoverable errors.
- **ERR-03 — Error Propagation:** Pass an error through several layers while preserving its original cause and useful context.
- **ERR-04 — Cleanup on Failure:** Ensure resources are released when an operation succeeds, fails, or is cancelled.
- **ERR-05 — Retry Logic:** Retry a failed operation a configurable number of times, with a delay and a final failure result.
- **ERR-06 — Timeouts:** Stop waiting for an operation after a deadline and handle the timeout without corrupting state.
- **ERR-07 — Validation Boundaries:** Validate incoming data once at a defined boundary before passing trusted data into internal logic.
- **ERR-08 — Logging:** Record structured errors with timestamps, component identifiers, operation names, and relevant context.
- **ERR-09 — Fault Isolation:** Run independent operations so that one failure does not unnecessarily terminate unrelated work.
- **ERR-10 — Recovery:** Simulate a failed component, restore its last valid state, and resume operation safely.

## 7. Object-Oriented Programming
- **OOP-01 — Basic Class:** Define a class with fields, a constructor, and methods that operate on its state.
- **OOP-02 — Encapsulation:** Restrict direct access to internal state and expose controlled methods for reading and modifying it.
- **OOP-03 — Object Identity:** Create two objects with equal values and determine whether they represent the same instance.
- **OOP-04 — Composition:** Build a system in which one object owns or coordinates several other objects.
- **OOP-05 — Inheritance:** Create a base class and specialized subclasses that override shared behavior.
- **OOP-06 — Polymorphism:** Process different object types through a common interface without branching on their concrete types.
- **OOP-07 — Interfaces and Traits:** Define a shared behavioral contract and implement it for multiple types using the language's native mechanism.
- **OOP-08 — Lifecycle:** Implement object initialization, activation, shutdown, and cleanup, including failures during initialization.
- **OOP-09 — Dependency Injection:** Construct an object with replaceable dependencies and substitute test doubles during testing.
- **OOP-10 — Design Comparison:** Implement the same small system using class inheritance, composition, and procedural functions, then compare the results.

## 8. Pointers, References, Ownership, and Memory
- **MEM-01 — Address and Value:** Store a value and inspect its memory address or reference identity where the language exposes that information.
- **MEM-02 — Indirect Mutation:** Modify a variable through a pointer, reference, or equivalent mechanism and observe the original value.
- **MEM-03 — Aliasing:** Create two references to the same mutable data and demonstrate how changes through one affect the other.
- **MEM-04 — Pointer Arithmetic:** Traverse a contiguous array using pointer arithmetic in a language that supports it.
- **MEM-05 — Dynamic Allocation:** Allocate and release memory manually where required, and compare with garbage-collected or ownership-managed approaches.
- **MEM-06 — Lifetime:** Return or retain references to data and determine which lifetimes are valid or rejected by the language.
- **MEM-07 — Shallow and Deep Copy:** Copy a nested structure and test whether its internal objects are shared or duplicated.
- **MEM-08 — Ownership Transfer:** Transfer responsibility for a resource between functions or objects using the language's ownership model.
- **MEM-09 — Shared Ownership:** Let multiple components access a shared resource and manage its lifetime safely.
- **MEM-10 — Memory Failure Investigation:** Examine leaks, dangling pointers, double frees, and use-after-free, using safe diagnostics or controlled demonstrations rather than relying on undefined behavior.

## 9. Files, Serialization, and Databases
- **IO-01 — File Read:** Read a text file line by line and count its lines, words, and characters.
- **IO-02 — File Write:** Generate a file from a collection of records and verify its contents.
- **IO-03 — Structured Data:** Parse and generate JSON, handling missing fields, invalid types, and malformed input.
- **IO-04 — Binary Data:** Write and read a binary record format and compare its size with a text representation.
- **IO-05 — Large Files:** Process a file larger than available working memory using streaming or chunked reads.
- **IO-06 — Configuration:** Load settings from a file, apply defaults, validate values, and support explicit overrides.
- **IO-07 — Database CRUD:** Create, read, update, and delete records in a SQLite database.
- **IO-08 — Transactions:** Execute multiple database changes in a transaction and roll them back when any step fails.
- **IO-09 — Concurrent Access:** Run multiple database readers and writers and handle locking, conflicts, and retries.
- **IO-10 — Event Journal:** Persist ordered state-changing events and reconstruct the current state by replaying them.

## 10. Concurrency and Asynchronous Programming
- **CON-01 — Concurrent Tasks:** Run multiple independent tasks and collect their results.
- **CON-02 — Async Waiting:** Await an asynchronous operation without unnecessarily blocking other tasks.
- **CON-03 — Cancellation:** Start a long-running task, request cancellation, and verify that it terminates and cleans up.
- **CON-04 — Worker Pool:** Maintain a configurable pool of workers that process jobs from a shared queue.
- **CON-05 — Bounded Queue:** Apply backpressure when producers generate work faster than consumers can process it.
- **CON-06 — Race Condition:** Reproduce a lost-update problem in shared state, then fix it using the language's synchronization mechanisms.
- **CON-07 — Mutual Exclusion:** Protect a shared resource with a mutex or equivalent synchronization primitive.
- **CON-08 — Deadlock:** Construct a controlled deadlock scenario, identify its cause, and redesign the locking strategy.
- **CON-09 — Graceful Shutdown:** Stop accepting new jobs, finish or cancel existing work according to policy, and release all resources.
- **CON-10 — Dynamic Workers:** Add, remove, restart, and monitor workers at runtime while preventing duplicate execution and resource leaks.
- **CON-11 — Task Supervision:** Detect failed child tasks, classify failures, and apply configurable restart or termination policies.
- **CON-12 — Async I/O vs. Parallel CPU Work:** Compare asynchronous I/O, threads, and processes for suitable workloads.

## 11. Networking and Distributed Systems
- **NET-01 — HTTP Client:** Send a request to an HTTP endpoint, parse the response, and handle status codes and timeouts.
- **NET-02 — HTTP Server:** Expose an endpoint that validates input and returns structured data.
- **NET-03 — TCP Client and Server:** Exchange framed messages over a TCP connection and handle disconnects.
- **NET-04 — Message Serialization:** Encode and decode typed messages using a documented schema.
- **NET-05 — Request and Reply:** Send a request with an identifier and correlate it with the correct response.
- **NET-06 — Publish and Subscribe:** Publish events to multiple consumers without requiring direct knowledge of each consumer.
- **NET-07 — Heartbeats:** Detect missed heartbeats and distinguish an unresponsive component from a confirmed failure.
- **NET-08 — Idempotency:** Process duplicate requests without repeating their intended side effects.
- **NET-09 — Retry and Delivery:** Simulate lost messages and retries, and prevent duplicate processing where required.
- **NET-10 — Distributed Worker Manager:** Coordinate worker intent, observed status, timeouts, restart requests, and shutdown across multiple processes.

## 12. Testing, Tooling, and Performance
- **TST-01 — Unit Tests:** Write tests for normal results, boundary values, invalid inputs, and expected failures.
- **TST-02 — Property-Based Tests:** Generate many inputs and verify general invariants rather than only fixed examples.
- **TST-03 — Mocking:** Replace an external dependency with a controlled test double and simulate success, delay, and failure.
- **TST-04 — Integration Tests:** Test interactions between multiple modules using a real database or local service.
- **TST-05 — Stress Testing:** Increase the number of inputs or concurrent workers until the system reaches a resource or performance limit.
- **TST-06 — Benchmarking:** Measure execution time and memory usage for equivalent implementations using consistent test data.
- **TST-07 — Profiling:** Identify the functions responsible for the largest CPU and memory costs.
- **TST-08 — Build and Dependencies:** Create a reproducible build using the language's package manager, compiler, or build system.
- **TST-09 — Static Analysis:** Configure a formatter, linter, type checker, compiler warnings, or equivalent analysis tools.
- **TST-10 — Cross-Language Interface:** Call a compiled component from a higher-level language through a stable, documented interface.

## 13. Language-Specific Experiments
- **SPC-01 — Python: Dynamic Typing:** Process a mixed collection of values and compare runtime validation with static type annotations and a type checker.
- **SPC-02 — Python: Vectorization:** Implement a numerical operation using Python loops and NumPy, then benchmark both versions.
- **SPC-03 — C: Manual Memory:** Implement a dynamically sized array with explicit allocation, resizing, and cleanup.
- **SPC-04 — C: Function Pointers:** Build a dispatch table that selects operations through function pointers rather than conditional branching.
- **SPC-05 — C++: RAII:** Implement a resource-owning type whose destructor releases its resource automatically.
- **SPC-06 — C++: Templates:** Write a generic algorithm that operates on multiple data types without duplicating its implementation.
- **SPC-07 — C++: Smart Pointers:** Compare unique ownership, shared ownership, and non-owning references using the appropriate smart-pointer types.
- **SPC-08 — Rust: Ownership and Borrowing:** Implement a collection-processing function and resolve ownership or borrowing restrictions without unnecessary cloning.
- **SPC-09 — Rust: Result and Option:** Build a multi-step data pipeline that explicitly handles missing values and operation failures.
- **SPC-10 — Rust: Traits and Generics:** Implement multiple worker types that satisfy a shared trait and process them through a common interface.
- **SPC-11 — Go: Goroutines and Channels:** Implement a worker pool that distributes jobs through channels and collects results.
- **SPC-12 — Go: Context Cancellation:** Propagate a cancellation deadline through several nested operations and verify that all relevant workers exit.
- **SPC-13 — Go: Interfaces:** Implement interchangeable storage backends behind a small interface and test them with the same operations.
- **SPC-14 — Compiled vs. Interpreted:** Implement the same CPU-bound algorithm in multiple languages and compare runtime, startup time, and memory use.
- **SPC-15 — Paradigm Comparison:** Implement the same worker lifecycle using OOP, procedural functions, and an event-driven design, comparing clarity, testability, and complexity.

## 14. Capstone Projects
- **CAP-01 — Data Analyzer:** Read a large time-series dataset, calculate statistics and rolling indicators, and export the results.
- **CAP-02 — Job Scheduler:** Schedule jobs, track their lifecycle, retry failures, and shut down gracefully.
- **CAP-03 — Worker Supervisor:** Manage dynamically created workers with explicit state transitions, health monitoring, cancellation, and restart policies.
- **CAP-04 — Event-Driven Simulator:** Simulate a system by processing timestamped events and updating state deterministically.
- **CAP-05 — Miniature Trading Engine:** Process market data, generate signals, track simulated positions, enforce risk limits, and record fills.
- **CAP-06 — Distributed Processing System:** Run workers in separate processes that communicate through messages and recover from selected failures.
- **CAP-07 — Cross-Language System:** Implement a performance-critical module in a compiled language and integrate it into a higher-level application.
- **CAP-08 — Benchmark Suite:** Compare multiple implementations of the same workload, including correctness, runtime, memory usage, and development complexity.