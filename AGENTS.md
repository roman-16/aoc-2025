# Agent Guidelines

## General Principles
- **Strictness**: ALWAYS/NEVER = strict rules. Prefer/Avoid = strong defaults with exceptions allowed
- **Git Operations**: NEVER EVER do ANY git operation (`git add`, `git stage`, `git restore --staged`, `git commit`, `git push`, `git checkout`, `git branch`, `git merge`, `git rebase`, etc.) without EXPLICIT user permission. This is an absolute rule with ZERO exceptions. Only the user initiates git operations
- **Verify Before Implementing**: ALWAYS verify APIs, library features, and configurations using Context7 or official documentation before implementation. NEVER assume attributes, methods, or behavior exist without verification
- **Documentation**: Use `docs/README.md` as the main documentation file (rest of `docs/` folder available for additional docs)
- **Ask Questions**: ALWAYS ask if unclear. NEVER assume. STOP and ask before proceeding if ANY of:
  - Multiple valid approaches exist
  - User intent could be interpreted multiple ways
  - Requirements are vague or incomplete
  - Design decisions needed (architecture, patterns, data models, APIs)
  - Trade-offs exist between options
  - Scope is ambiguous (what's in/out, how deep to go)

## Feature Workflow
1. **Research**: Understand the codebase, requirements, and constraints before making changes
   - Check existing patterns and implementations for similar functionality
   - Review related tests to understand expected behavior
   - Identify dependencies and potential side effects
2. **Plan**: Create an initial plan breaking down the task into clear, actionable steps
   - Create a markdown feature file in `docs/features/` named `YYYY-MM-DD-HHMM_FEATURE_NAME.md`
   - Use `date +%Y-%m-%d-%H%M` to get the timestamp (e.g., `docs/features/2025-11-26-1530_AUTHENTICATION.md`)
3. **Present Summary**: Present a brief plan summary to the user
   - Display: "Type `y` to go to clarifying"
   - If user adds context/feedback: immediately update the feature file
   - Continue showing the prompt until user types `y`
   - Only proceed to step 4 (Clarify) after user confirmation
4. **Clarify**: Ask questions to ensure complete understanding. REQUIRED before implementation if ANY ambiguity exists
   - Ask ONE question at a time, wait for answer, then ask the next question
   - Use previous answers to inform subsequent questions
   - Format each question as:
     ```
     **Question:** [question]?
     (1) [option]
     (2) [option]
     ```
     Mention that text answers are welcome (pick number, add context, or free-form text). Don't include "text answer" as a numbered option
   - Update the feature file with each Q&A after answering
   - Continue until ALL ambiguities resolved - don't stop after pre-written questions. Proactively identify new ambiguities and ask follow-ups. Don't ask permission to continue
   - Know when to stop: architecture, file structure, user-facing changes, breaking changes, major patterns - NOT minor implementation details
   - After all questions: comprehensively update plan with all decisions
   - NEVER skip if uncertain - defaulting to assumption is unacceptable
5. **Confirm**: Present the final plan summary and ask "Type `y` to implement this plan"
   - If "y": proceed to implementation
   - If other feedback: adjust the plan and ask for confirmation again
6. **Implement**: Execute the plan incrementally, following code style and architecture guidelines
   - Write tests alongside implementation
   - Make incremental commits for major milestones if working on large features
7. **Validate**: Run all quality gates in order to ensure correctness (see Quality Gates section)
   - If any gate fails: fix issues and re-run all gates from the beginning
8. **Complete**: After all quality gates pass, summarize changes made and ask about committing (see Version Control section)

## Architecture
Advent of Code 2025 puzzle solutions in Rust:
- **Purpose**: Daily programming puzzles (December 1-25)
- **Structure**: Each day has two parts with increasing difficulty
- **Input**: Each puzzle has unique input data per user

## Project Structure
Key directories:
- `src/` - Main source code
- `src/bin/` - Individual day solutions (day01.rs, day02.rs, etc.)
- `inputs/` - Puzzle input files

## Code Style

### General Principles
- **Simplicity**: Straightforward solutions. No unnecessary intermediate variables—directly invoke/access if used once
- **Paradigm**: Functional preferred—iterators, combinators, immutability where practical
- **Duplicate Code**: Extract to reusable helpers in shared modules
- **Dependencies**: Minimal external crates. Prefer standard library

### Style & Formatting
- **Formatting**: rustfmt with default settings, empty line at end of files, whitespace between logical blocks
- **Property Ordering**: Alphabetical by default unless another ordering makes better sense

### Naming Conventions
- **PascalCase**: Types/Structs/Enums/Traits (`OrderEvent`, `UserConfig`)
- **snake_case**: Functions/Variables/Constants/Modules (`process_order`, `max_retries`)
- **SCREAMING_SNAKE_CASE**: Static constants (`MAX_SIZE`)
- **Descriptive Names**: Full names, not abbreviations. Exceptions: `i` (index), `e` (error in match), single-letter generics (`T`, `K`, `V`)

### Rust Practices
- **Types**: Leverage type system fully. Use newtypes for domain concepts
- **Error Handling**: Use `Result` and `?` operator. Prefer `anyhow` for applications, `thiserror` for libraries
- **Ownership**: Prefer borrowing over cloning. Use `Cow` for flexibility
- **Iterators**: Prefer iterator chains over explicit loops
- **Pattern Matching**: Use exhaustive matches. Avoid catch-all `_` when variants might be added

### Comments & Documentation
- **When**: Explain "why" not "what"—business logic, workarounds, non-obvious decisions
- **Avoid**: NEVER restate code. If self-explanatory, no comment needed
- **TODOs**: `// TODO:` with context (optional ticket ref)
- **Doc Comments**: Use `///` for public APIs

### Error Handling & Logging
- **Errors**: Use `Result<T, E>` consistently. Provide context with `.context()` or custom error types
- **Panics**: Avoid in library code. Use only for unrecoverable states or test assertions

## Quality Gates
Run in this order to fail fast:

1. Code must compile with no errors (`cargo build`)
2. Clippy lints must pass (`cargo clippy -- -D warnings`)
3. Format must be correct (`cargo fmt --check`)
4. All tests must pass (`cargo test`)

## Version Control

### CRITICAL: Explicit Permission Required
- **NEVER do ANY git operation without explicit user permission** - This includes: commit, push, stage, unstage, branch operations, merges, rebases, etc.
- **ALWAYS wait for user to type `y`, `c`, or `p`** before executing ANY git command
- **Even if quality gates pass, even if the user said "commit" earlier in the conversation, even if it seems obvious** - STOP and ask for confirmation with the exact options below
- **No exceptions. No shortcuts. No assuming intent.**

### Quality Gates & Timing
- **Quality Gates Required**: Run ALL quality gates before ANY git operation. If any gate fails, inform the user and stop
- **When to Ask About Committing**: Ask when you feel like it makes sense
  - Logical unit complete (feature/bugfix/refactor/task finished)
  - Quality gates pass (or minimally, changes validated)
  - Before significantly different task
  - **Key principle**: When in doubt, ask. Only skip if certain larger commit coming
- **Commit Workflow**: NEVER commit automatically. Only ask when logical
  - Ask: "Type `y` to start committing"
  - If "y": Run quality gates first. If any gate fails, inform the user and stop. Then proceed with commit workflow:
    - Check staged files (`git status`, `git diff --staged`)
    - Display: files to unstage (if any), additional files to stage (if any), proposed commit message (conventional format describing ALL changes), horizontal rule (`---`)
    - Display options based on staging needs:
      - If staging changes needed (files to unstage or additional files to stage): Type `s` to stage | `c` to stage and commit | `p` to stage, commit and push
      - If no staging changes needed: Type `c` to commit | `p` to commit and push
    - On `s`: unstage specified files, stage additional files, show staged changes, prompt with `c`/`p` options
    - On `c`/`p`: perform staging changes if needed, then commit (and push if `p`)
    - On other response: treat as instruction (modify message, change files, make more changes, etc.)
    - If file changes made relevant to current commit: restart entire workflow from beginning
  - On other response: treat as instruction (don't start commit workflow)
- **Commit Message Format**: `emoji type(scope): description`
  - Examples: `✨ feat(day01): solve part 1` | `🐛 fix(day05): handle edge case` | `✅ test(utils): add parser tests`
  - **Body**: Keep simple and concise. Skip body for obvious changes. Use bullet list only for meaningful details (key architectural decisions, breaking changes, important context). Avoid exhaustive change lists
- **Types with Emojis**:
  - `✨ feat` - New feature
  - `🐛 fix` - Bug fix
  - `♻️ refactor` - Code refactoring
  - `✅ test` - Adding or updating tests
  - `📚 docs` - Documentation changes
  - `🔧 chore` - Maintenance tasks
  - `⚡ perf` - Performance improvements
  - `🎨 style` - Code style/formatting changes
  - `🔒 security` - Security improvements
- **Scope**: day01-day25, utils, lib

## Commands
- **Build**: `cargo build` (debug) | `cargo build --release` (optimized)
- **Run**: `cargo run --bin dayXX` (replace XX with day number)
- **Check**: `cargo check` (fast compile check without codegen)
- **Lint**: `cargo clippy -- -D warnings`
- **Format**: `cargo fmt` (apply) | `cargo fmt --check` (verify)
- **Test**: `cargo test`
