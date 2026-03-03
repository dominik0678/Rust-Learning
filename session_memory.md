# Rust Learning Session Memory

This file is the persistent memory for our Rust coaching sessions.
It is stored in the repository so future sessions can resume quickly.

## Memory Meta
- Created: 2026-02-16
- Last updated: 2026-03-02
- Owner: Dominik
- Workspace: `/home/dominik/Dokumente/Work/Rust-Learning`
- Main goal: Reach IPA readiness and become independent from AI support.
- Deadline milestones (latest info from Grobplanung):
  - IPA start week: 06-04-2026 to 12-04-2026
  - PA execution window: 06-04-2026 to 31-05-2026
  - Submission deadline: 09-03-2026
  - Earlier note in chat: "IPA from 07-05-2026" (keep as provisional until detailed plan confirms)
- Baseline timing: 79 days remaining from 2026-02-17 (derived from initial 80-day baseline).

## Tooling and Constraints
- IDE: VSCode
- New project workflow: `cargo new "<name>"`
- Run workflow: `cargo run` or VSCode CodeRunner extension
- Integrated AI in IDE: disabled (intentional)
- GitHub policy: repositories are private due to company policy; admin approval is required for many GitHub actions
- Coaching implication: local-first learning flow, avoid depending on remote GitHub operations

## IPA Grobplanung (User-provided)
- Prozessschritt: Startblock
- Startwoche 07: 06.04.2026 to 12.04.2026
- PA-Durchfuehrung: 06.04.2026 to 31.05.2026
- Einreichung bis: 09.03.2026

### IPA Theme and Scope
- Aufgabe: Signalprozessor-Software in Rust programmieren mit Einbindung von C-Code.
- Context: ELESTA GmbH develops safety sensors; signal processing is currently implemented in C.
- Ziel: Use Rust for a part of the implementation and integrate existing C signal-processing modules.
- Pflichtbestandteil: Setup of development environment and toolchains for both Rust and C.
- Test setup: UART interface to hardware exists for triggering tests and evaluating results.
- Out of scope: Existing UART test-control/evaluation software (already available).
- Company/location context (sensitive, keep private): ELESTA GmbH, Heuteilstrasse 18, 7310 Bad Ragaz.

## Learning Priorities Derived From IPA Scope
- Rust ownership/borrowing mastery specifically at C boundaries.
- Rust/C FFI fundamentals (`extern "C"`, ABI, raw pointers, safety boundaries).
- Build and linkage workflow (Cargo + C compilation/linking basics).
- Robust error handling and logging for hardware/UART-driven flows.
- Small integration tests around Rust/C module interaction.

## What "Success" Means
- You can implement medium-sized Rust tasks without AI writing code for you.
- You can explain ownership, borrowing, lifetimes (basic practical use), and error handling clearly.
- You can structure a Rust project with modules and clean responsibilities.
- You can debug compiler errors independently and fix them with confidence.
- You can deliver an IPA-style project with tests, documentation, and clear reasoning.

## Coaching Contract (Agreed Working Style)
- You solve first, AI helps second.
- AI gives hints before full solutions.
- Support is gradually reduced as confidence increases.
- Focus is independence over speed.
- Every session ends with a small reflection and next action.

## Repository Snapshot (High-level)
- Root contains mixed learning and project directories.
- Rust-focused folders:
  - `w3schools/` (topic-based practice)
  - `Todo-App/todo-app/` (GUI + CSV project)
  - `Nucleo-G474RE/` (embedded STM32 practice)
  - `RustWithC/` (cross-language/embedded style work)

## Baseline Skill Assessment (from code snapshot)

### Strengths already visible
- Fundamentals are present: variables, loops, functions, structs, enums, collections.
- Ownership and borrowing were already practiced in dedicated files.
- You built a real GUI app (`egui`) with persistent CSV storage.
- You worked with embedded Rust on STM32 (`stm32g4xx-hal`).
- You can read and write Rust code across different domains.

### Gaps to close for IPA confidence
- Deep ownership/borrowing fluency under pressure (borrow-checker thinking).
- Error handling quality (`Result`, `?`, custom error patterns).
- Code organization and scaling (`mod`, file separation, cleaner architecture).
- Testing habit (`#[test]`, edge cases, behavior-driven checks).
- Debug workflow and self-reliant problem decomposition.

## Evidence Files Reviewed
- `README.md`
- `w3schools/ownership/src/main.rs`
- `w3schools/borrowingAndReference/src/main.rs`
- `w3schools/hashMap/src/main.rs`
- `Todo-App/todo-app/src/main.rs`
- `Nucleo-G474RE/blink_LED/src/main.rs`
- `learning_with_opencode/challenge_2/src/main.rs`
- `learning_with_opencode/challenge_3/src/main.rs`
- `learning_with_opencode/challenge_4/src/main.rs`

## IPA Preparation Roadmap (80-day structure)

### Phase 1: Core Rust Fluency (Weeks 1-2)
- Ownership and borrowing drills
- `Option`/`Result` and pattern matching
- Struct/enum usage in realistic mini-exercises
- Compiler-error interpretation practice

### Phase 2: Project-grade Rust (Weeks 3-5)
- File I/O and robust error propagation
- Traits and reusable design
- Refactoring and module boundaries
- Testing and small quality checks

### Phase 3: IPA-style Build Practice (Weeks 6-8)
- Build one complete mini-project from scratch
- Track requirements and implementation decisions
- Validate behavior with tests and manual checks
- Document architecture and tradeoffs

### Phase 4: Independence + Speed (Weeks 9-10)
- Timed exercises without AI first
- Mock IPA runs end-to-end
- Post-run retrospectives (what slowed you down)
- Final polish on weak points

## Current Session State

### Session 001
- Date: 2026-02-16
- User intent: Learn Rust to IPA readiness and long-term independence from AI.
- User selected learning track: `1` (Ownership/Borrowing challenge).
- User request before starting track: Create persistent `session_memory` and keep it updated.
- User provided workflow details (VSCode, cargo commands, CodeRunner) and company GitHub constraints.
- User provided IPA Grobplanung details (dates, Rust+C integration scope, UART testing context).
- Challenge 1 assigned: Move vs borrow function drill (attempt-first, hint-only support).
- User started Challenge 1 and shared first code attempt.
- Observed blockers: missing function bodies, incorrect `print!` usage, and missing borrow in `length` call.
- Coaching action: provide hint-level clarification on function roles and `main` call order.
- User submitted revised attempt with correct call order and signatures but empty TODO function bodies.
- Observed blocker: uncertainty about implementing read-only borrow, mutable borrow, and ownership-move function behavior.
- Coaching action: give focused explanation of each function contract with minimal working guidance.
- User completed function bodies and confirmed successful run (`cargo run` works).
- Exercise outcome: expected ownership flow implemented successfully (`&String`, `&mut String`, moved `String` with return).
- Session close note: user ended for the day around 15:00.
- Status: Challenge 1 completed. Track remains active, next step is Challenge 2.

### Session 002
- Date: 2026-02-17
- Session open action: assistant re-read `session_memory.md` on request before continuing.
- User requested conceptual review of completed Challenge 1 before moving on.
- Concept clarification delivered:
  - `return` keyword is allowed in Rust, but optional for final expression.
  - Tail expression behavior explained (`s` returns value, `s;` does not).
  - Distinction between functions returning concrete values (`usize`, `String`) and functions returning unit `()`.
- Challenge 2 resumed and user provided a compiling attempt with wrong behavior.
- Observed blockers in user code:
  - Used `tasks.len()` (vector length) instead of summing per-string length.
  - Used `clone()` inside ownership exercise (`consume_last`) and discarded popped value.
  - Returned placeholder tuple instead of `(remaining_tasks, removed_task)`.
- Coaching action:
  - Explained semantic difference between vector length and string length.
  - Provided fix pattern for ownership-safe `pop()` without cloning.
  - Adapted explanation to beginner syntax level after user noted `iter()/map()` had not yet been learned.
- User feedback signal captured: "iterator syntax felt like a trap" because it was unknown at this stage.
- Teaching adjustment applied immediately:
  - Loop-first style preferred (`for`, mutable accumulator) before iterator chains.
  - Goal reaffirmed: syntax confidence before abstraction-heavy patterns.
- Additional drill progression:
  - Challenge 3 completed by user.
  - Correct parts: total-length loop and ownership-based pop tuple.
  - Needed correction: second task was hardcoded (`"code [done]"`) instead of appending to existing value.
- Challenge 4 completed cleanly by user and validated with `cargo run`.
- Concept deepening completed: `push` (single `char`) vs `push_str` (`&str`) with practical examples.
- Challenge 5 was prepared and assigned (safe second-element mutation via `get_mut(1)`) but not started.
- User announced immediate context switch: start integrating C into Rust now (higher urgency).
- Session close status:
  - Ownership/Borrowing fundamentals improved through consecutive successful drills.
  - Transition from pure syntax drills to Rust/C integration begins next session.

### Session 003
- Date: 2026-03-02
- Session open action: assistant re-read and analyzed `session_memory.md` on request.
- User direction change: did not want to continue in the previous session style; requested an understandable C library for Rust integration practice.
- Implementation work completed:
  - Created `RustWithC/ffi_c_library/` as a fully working Rust/C FFI reference project.
  - User feedback captured: full solution felt too "silver plate" for learning goals.
  - Created `RustWithC/ffi_learning_starter/` as a practice-first starter.
  - Kept Rust learning tasks open in `src/student_ffi.rs` with TODO wrappers (`sum_i16`, `offset_i16`, `max_i16`).
  - Completed C-side expectation for IPA-like setup by adding standalone C app files:
    - `RustWithC/ffi_learning_starter/c_lib/main.c`
    - `RustWithC/ffi_learning_starter/c_lib/Makefile`
- Verification completed:
  - `make -C c_lib run` works (standalone C library + C main demo).
  - `cargo test` and `cargo run` work in `RustWithC/ffi_learning_starter`.
- Session close status:
  - C library side is complete and independently runnable.
  - Next learning focus is Rust wrapper implementation by the user.

## Active Learning Track
- Primary: Rust/C FFI integration with learner-owned Rust wrapper implementation
- Secondary: Ownership/Borrowing reinforcement when needed
- Ownership drill status:
  - Challenge 1: completed
  - Challenge 2: completed
  - Challenge 3: completed (one correction discussed)
  - Challenge 4: completed
  - Challenge 5: assigned, paused due context switch

- FFI project status:
  - Reference project (fully solved): `RustWithC/ffi_c_library`
  - Practice project (current focus): `RustWithC/ffi_learning_starter`
  - C library state in practice project: complete (`simple_signal.c/.h` + `main.c` + `Makefile`)
  - Rust wrapper state in practice project: intentionally partial (student TODOs in `src/student_ffi.rs`)

## Immediate FFI Practice Plan (next session priority)
1. Implement `sum_i16` wrapper in `RustWithC/ffi_learning_starter/src/student_ffi.rs`.
2. Implement `offset_i16` wrapper in `RustWithC/ffi_learning_starter/src/student_ffi.rs`.
3. Implement `max_i16` wrapper in `RustWithC/ffi_learning_starter/src/student_ffi.rs`.
4. Add unit tests for wrapper success and at least one error path.
5. Update `RustWithC/ffi_learning_starter/src/main.rs` to call all implemented wrappers.
6. Keep `unsafe` only inside wrapper functions and map C status codes to Rust errors.

## Next Session Start Checklist
1. Read `session_memory.md` first.
2. Confirm current day and update `Last updated` field.
3. Resume from "Current Session State" and "Active Learning Track".
4. Open `RustWithC/ffi_learning_starter/src/student_ffi.rs` and start with `sum_i16` TODO.
5. Keep solution beginner-readable first; only introduce advanced patterns after base version works.
6. Validate with `cargo test` after each wrapper implementation.
7. End session by updating log, wins, blockers, and next concrete FFI task.

## Session Update Protocol (must be done each session)
At the end of each session, append:
- Date and session number
- Topics covered
- Exercises attempted
- What was solved without help
- Where hints were needed
- Common mistakes observed
- Confidence rating (1-10)
- Next concrete task

Also update:
- `Last updated`
- `Current Session State`
- `Active Learning Track` (if changed)

## Session Log
### Session 001
- Date: 2026-02-16
- Focus: Ownership and borrowing fundamentals (move vs borrow vs mutable borrow)
- Tasks: Implement `length`, `append_rust`, `consume_and_return` and use them correctly in `main`
- Independent wins: User fixed `main` flow and completed all function implementations
- Needed hints for: Function contracts, format macro usage, and borrow syntax in call sites
- Key mistakes/patterns: Missing function bodies, initial confusion between function signature and implementation
- Confidence (1-10): Not self-rated yet
- Homework/next action: Start Challenge 2 next session (ownership-focused, slightly harder)

### Session 002
- Date: 2026-02-17
- Focus: Consolidate ownership/borrowing understanding and close syntax gaps before FFI transition
- Tasks:
  - Revisit Challenge 1 concepts (`return` vs tail expression)
  - Complete Challenge 2 (`total_len`, mutable append, consume/pop ownership flow)
  - Complete Challenge 3 beginner drill
  - Complete Challenge 4 (`push` vs `push_str` practical use)
  - Prepare Challenge 5 but pause due context switch
- Independent wins:
  - User recognized conceptual confusion early and asked targeted questions
  - User completed Challenge 2 after clarifying ownership and length semantics
  - User completed Challenge 3 and Challenge 4 with strong ownership handling
  - User validated outputs and kept iterative debugging workflow
- Needed hints for:
  - Difference between vector length and string length
  - Why cloning in ownership drills is usually the wrong move
  - How to return popped value correctly from an owned `Vec<String>`
  - Idiomatic but optional syntax (`iter()/map()/sum()`) vs loop-first alternatives
- Key mistakes/patterns:
  - Hardcoded mutation target value instead of mutating existing data in place
  - Confusion when exposed to syntax not yet learned (`iter`, `map`)
  - Temporary assumption that `return` keyword is mandatory in all non-unit functions
- Confidence (1-10): Not self-rated in chat
- Homework/next action:
  - Begin Rust/C integration mini-project immediately
  - First objective: compile and call one C function from Rust via `extern "C"`
  - Keep `unsafe` surface minimal and document assumptions in comments/notes

### Session 003
- Date: 2026-03-02
- Focus: Start FFI work with a complete C-side baseline and a learner-owned Rust wrapper path
- Tasks:
  - Re-read session memory and align on changed learning preference
  - Create full reference project: `RustWithC/ffi_c_library`
  - Create practice project: `RustWithC/ffi_learning_starter`
  - Complete standalone C library + C main in practice project
  - Verify C-only and Cargo-based runs
- Independent wins:
  - User gave clear feedback to preserve learning ownership
  - User clarified IPA expectation that C library should be fully complete and runnable on its own
- Needed hints for:
  - No code-level hint cycle yet in this session (focus was environment/scaffold setup)
- Key mistakes/patterns:
  - Initial assistant approach oversolved Rust side relative to user learning preference
  - Quickly corrected by splitting into reference project + practice-first project
- Confidence (1-10): Not self-rated in chat
- Homework/next action:
  - Implement `sum_i16`, `offset_i16`, and `max_i16` wrappers in `RustWithC/ffi_learning_starter/src/student_ffi.rs`
  - Add tests for wrappers and run `cargo test`
  - Expand `src/main.rs` demo after wrappers compile and pass tests

## Session Log Template (copy for future entries)
### Session XXX
- Date:
- Focus:
- Tasks:
- Independent wins:
- Needed hints for:
- Key mistakes/patterns:
- Confidence (1-10):
- Homework/next action:

## Risk Register (for IPA readiness)
- Risk: Over-relying on AI suggestions.
  - Mitigation: Attempt-first rule, hint-only mode by default.
- Risk: Borrow-checker confusion in bigger code.
  - Mitigation: Frequent small drills with ownership diagrams.
- Risk: Project panic under time pressure.
  - Mitigation: Timed mock sessions and checklist-based workflow.
- Risk: Weak testing habits.
  - Mitigation: Add tests to every mini-project increment.
- Risk: Rust/C boundary bugs (ABI or pointer misuse).
  - Mitigation: Isolate FFI layer, document invariants, and keep unsafe blocks minimal and reviewed.
- Risk: Toolchain friction late in project.
  - Mitigation: Build reproducible setup early and rehearse clean-machine setup steps.
- Risk: Context-switch overload during transition from syntax drills to FFI.
  - Mitigation: Use one-concept-at-a-time exercises (call C function first, pointers second, buffers third).

## Notes for Future Assistant Runs
- Always read this file before coaching.
- Keep guidance practical and incremental.
- Prefer asking Dominik to explain code decisions.
- Prioritize independence, not just passing compilation.
