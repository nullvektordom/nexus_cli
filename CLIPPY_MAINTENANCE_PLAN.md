# Clippy Warnings Maintenance Plan

**Total Warnings: 248 (232 unique in bin, 16 duplicates)**

## Summary by Category

### Group 1: Idiomatic/Lint (Low Risk) - ~180 warnings
- **uninlined_format_args**: ~80 instances - Use inline format args `{var}` instead of `{}, var`
- **doc_markdown**: ~30 instances - Missing backticks in documentation
- **missing_errors_doc**: ~10 instances - Missing `# Errors` section in docs
- **must_use_candidate**: ~5 instances - Functions that should have `#[must_use]`
- **needless_raw_string_hashes**: ~25 instances - Unnecessary `#` in raw strings
- **empty_line_after_doc_comments**: ~5 instances - Empty lines after doc comments
- **similar_names**: ~10 instances - Variable names too similar (context/content)
- **implicit_clone**: ~15 instances - Use `.clone()` instead of `.to_string()`
- **struct_field_names**: ~3 instances - Field naming issues

### Group 2: Performance (Medium Risk) - ~40 warnings
- **format_push_string**: ~10 instances - Use `write!` instead of `format!` + `push_str`
- **assigning_clones**: ~15 instances - Use `clone_from()` instead of assignment
- **needless_pass_by_value**: ~5 instances - Pass by reference instead of value
- **map_unwrap_or**: ~5 instances - Use `map_or_else` or `is_some_and`
- **redundant_closure**: ~3 instances - Direct method reference instead of closure
- **explicit_iter_loop**: ~2 instances - Use `&mut vec` instead of `.iter_mut()`

### Group 3: Logic/Safety (Higher Risk) - ~25 warnings
- **cast_lossless**: ~8 instances - Use `From` trait for infallible casts (u32→i64)
- **cast_possible_truncation**: ~8 instances - Use `try_from` for potentially lossy casts
- **cast_sign_loss**: ~4 instances - Casting signed to unsigned
- **cast_precision_loss**: ~5 instances - u64→f64 precision loss
- **unnecessary_wraps**: ~8 instances - Functions returning `Result<T>` that never fail
- **unused_self**: ~1 instance - Method doesn't use `self`
- **unused_async**: ~1 instance - Async function with no await

### Group 4: Code Quality (Low-Medium Risk) - ~15 warnings
- **too_many_lines**: ~4 instances - Functions exceeding 100 lines
- **if_not_else**: ~4 instances - Inverted if/else logic
- **single_match_else**: ~1 instance - Use `if let` instead of match
- **match_same_arms**: ~1 instance - Duplicate match arms
- **needless_continue**: ~1 instance - Redundant continue statement
- **wildcard_imports**: ~1 instance - Use specific imports
- **unnested_or_patterns**: ~3 instances - Nested pattern matching

### Group 5: Dead Code/Unused (Safe to Address) - ~8 warnings
- **dead_code**: ~8 instances - Unused fields/methods (may be intentional for API)

### Group 6: Test/Example Code (Low Priority) - ~20 warnings
- Deprecated `Command::cargo_bin` usage in tests
- Test-specific formatting issues

---

## Execution Plan

### Phase 1: Idiomatic/Lint (Group 1)
**Risk: Low | Impact: High (code clarity)**

Files to modify:
- [`src/embeddings.rs`](src/embeddings.rs) - format args, doc comments
- [`src/llm.rs`](src/llm.rs) - format args, doc markdown
- [`src/planning.rs`](src/planning.rs) - format args, implicit clones
- [`src/brain.rs`](src/brain.rs) - doc markdown, format args
- [`src/catalyst/`](src/catalyst/) - raw strings, doc comments
- [`src/commands/`](src/commands/) - format args
- [`src/config.rs`](src/config.rs) - doc markdown
- [`src/context.rs`](src/context.rs) - format args, field names
- [`src/history.rs`](src/history.rs) - format args, doc markdown
- [`src/git_ops.rs`](src/git_ops.rs) - format args
- [`src/watcher.rs`](src/watcher.rs) - format args, doc markdown
- [`src/main.rs`](src/main.rs) - format args

### Phase 2: Performance (Group 2)
**Risk: Low-Medium | Impact: Medium (performance)**

Files to modify:
- [`src/planning.rs`](src/planning.rs) - `clone_from()` optimizations
- [`src/context.rs`](src/context.rs) - `write!` instead of format
- [`src/history.rs`](src/history.rs) - `write!` instead of format
- [`src/config.rs`](src/config.rs) - `map_or_else`
- [`src/commands/gate.rs`](src/commands/gate.rs) - `map_or_else`, redundant closures
- [`src/watcher.rs`](src/watcher.rs) - pass by reference, redundant closures

### Phase 3: Logic/Safety (Group 3)
**Risk: Medium-High | Impact: High (correctness)**

Files to modify:
- [`src/embeddings.rs`](src/embeddings.rs) - Safe casts with `From`/`try_from`
- [`src/brain.rs`](src/brain.rs) - Safe casts, precision handling
- [`src/watcher.rs`](src/watcher.rs) - Remove unnecessary `Result` wrappers
- [`src/catalyst/engine.rs`](src/catalyst/engine.rs) - Remove unnecessary `Result` wrappers
- [`src/commands/shell.rs`](src/commands/shell.rs) - Remove unnecessary `Result` wrappers

### Phase 4: Verification
1. Run `cargo check` after each phase
2. Run `cargo test` after all fixes
3. Run `cargo build --release` for final verification

---

## Constraints to Maintain

From [`04-Tech-Stack-Standard.md`](templates/project/04-Architecture.md):
- ✅ `#[forbid(unsafe_code)]` - No unsafe code allowed
- ✅ `anyhow` for all error propagation
- ✅ Maintain architecture skeleton from `03-Architecture-Logic.md`

---

## Auto-fixable vs Manual

**Auto-fixable (~140 warnings):**
- Most format args
- Raw string hashes
- Some cast improvements
- Redundant closures

**Manual review needed (~108 warnings):**
- `unnecessary_wraps` - May affect API
- `too_many_lines` - Requires refactoring
- `if_not_else` - Logic flow changes
- Dead code - May be intentional
- Cast truncation - Needs validation

---

## Estimated Impact

- **Build time**: No change
- **Runtime performance**: Minor improvement (clone_from, format optimizations)
- **Code clarity**: Significant improvement
- **Maintainability**: High improvement
- **Breaking changes**: None (internal only)
