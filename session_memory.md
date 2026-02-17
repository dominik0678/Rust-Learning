# Rust Learning Session Memory

This file is the persistent memory for our Rust coaching sessions.
It is stored in the repository so future sessions can resume quickly.

## Memory Meta
- Created: 2026-02-16
- Last updated: 2026-02-16 15:00
- Owner: Dominik
- Workspace: `/home/dominik/Dokumente/Work/Rust-Learning`
- Main goal: Reach IPA readiness and become independent from AI support.
- Deadline milestones (latest info from Grobplanung):
  - IPA start week: 06-04-2026 to 12-04-2026
  - PA execution window: 06-04-2026 to 31-05-2026
  - Submission deadline: 09-03-2026
  - Earlier note in chat: "IPA from 07-05-2026" (keep as provisional until detailed plan confirms)
- Baseline timing: 80 days remaining from 2026-02-16.

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

## Active Learning Track
- Selected: Ownership/Borrowing Challenge Path
- Start state: Challenge 1 completed; Challenge 2 pending for next session

## Next Session Start Checklist
1. Read `session_memory.md` first.
2. Confirm current day and update `Last updated` field.
3. Resume from "Current Session State" and "Active Learning Track".
4. Continue ownership/borrowing challenge sequence.
5. End session by updating log, wins, blockers, and next task.

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

## Notes for Future Assistant Runs
- Always read this file before coaching.
- Keep guidance practical and incremental.
- Prefer asking Dominik to explain code decisions.
- Prioritize independence, not just passing compilation.
