    Checking nexus v0.1.0 (/home/nullvektor/repos/nexus_cli)
warning: docs for function returning `Result` missing `# Errors` section
  --> src/embeddings.rs:39:5
   |
39 |     pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#missing_errors_doc
   = note: `-W clippy::missing-errors-doc` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::missing_errors_doc)]`

warning: variables can be used directly in the `format!` string
  --> src/embeddings.rs:46:30
   |
46 |             .with_context(|| format!("Failed to load ONNX model from: {}", model_path))?;
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
   = note: `-W clippy::uninlined-format-args` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`
help: change this to
   |
46 -             .with_context(|| format!("Failed to load ONNX model from: {}", model_path))?;
46 +             .with_context(|| format!("Failed to load ONNX model from: {model_path}"))?;
   |

warning: variables can be used directly in the `format!` string
  --> src/embeddings.rs:50:26
   |
50 |             .map_err(|e| anyhow::anyhow!("Failed to load tokenizer from {}: {}", tokenizer_path, e))?;
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
50 -             .map_err(|e| anyhow::anyhow!("Failed to load tokenizer from {}: {}", tokenizer_path, e))?;
50 +             .map_err(|e| anyhow::anyhow!("Failed to load tokenizer from {tokenizer_path}: {e}"))?;
   |

warning: docs for function returning `Result` missing `# Errors` section
  --> src/embeddings.rs:65:5
   |
65 |     pub fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#missing_errors_doc

warning: variables can be used directly in the `format!` string
  --> src/embeddings.rs:70:26
   |
70 |             .map_err(|e| anyhow::anyhow!("Failed to tokenize text: {}", e))?;
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
70 -             .map_err(|e| anyhow::anyhow!("Failed to tokenize text: {}", e))?;
70 +             .map_err(|e| anyhow::anyhow!("Failed to tokenize text: {e}"))?;
   |

warning: casts from `u32` to `i64` can be expressed infallibly using `From`
  --> src/embeddings.rs:81:40
   |
81 |             input_ids.iter().map(|&id| id as i64).collect(),
   |                                        ^^^^^^^^^
   |
   = help: an `as` cast can become silently lossy if the types change in the future
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_lossless
   = note: `-W clippy::cast-lossless` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_lossless)]`
help: use `i64::from` instead
   |
81 -             input_ids.iter().map(|&id| id as i64).collect(),
81 +             input_ids.iter().map(|&id| i64::from(id)).collect(),
   |

warning: casts from `u32` to `i64` can be expressed infallibly using `From`
  --> src/embeddings.rs:87:47
   |
87 |             attention_mask.iter().map(|&mask| mask as i64).collect(),
   |                                               ^^^^^^^^^^^
   |
   = help: an `as` cast can become silently lossy if the types change in the future
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_lossless
help: use `i64::from` instead
   |
87 -             attention_mask.iter().map(|&mask| mask as i64).collect(),
87 +             attention_mask.iter().map(|&mask| i64::from(mask)).collect(),
   |

warning: casts from `u32` to `i64` can be expressed infallibly using `From`
  --> src/embeddings.rs:93:45
   |
93 |             token_type_ids.iter().map(|&id| id as i64).collect(),
   |                                             ^^^^^^^^^
   |
   = help: an `as` cast can become silently lossy if the types change in the future
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_lossless
help: use `i64::from` instead
   |
93 -             token_type_ids.iter().map(|&id| id as i64).collect(),
93 +             token_type_ids.iter().map(|&id| i64::from(id)).collect(),
   |

warning: casting `i64` to `usize` may truncate the value on targets with 32-bit wide pointers
   --> src/embeddings.rs:126:26
    |
126 |         let batch_size = shape_dims[0] as usize;
    |                          ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: if this is intentional allow the lint with `#[allow(clippy::cast_possible_truncation)]` ...
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_possible_truncation
    = note: `-W clippy::cast-possible-truncation` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_possible_truncation)]`
help: ... or use `try_from` and handle the error accordingly
    |
126 -         let batch_size = shape_dims[0] as usize;
126 +         let batch_size = usize::try_from(shape_dims[0]);
    |

warning: casting `i64` to `usize` may lose the sign of the value
   --> src/embeddings.rs:126:26
    |
126 |         let batch_size = shape_dims[0] as usize;
    |                          ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_sign_loss
    = note: `-W clippy::cast-sign-loss` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_sign_loss)]`

warning: casting `i64` to `usize` may truncate the value on targets with 32-bit wide pointers
   --> src/embeddings.rs:127:23
    |
127 |         let seq_len = shape_dims[1] as usize;
    |                       ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: if this is intentional allow the lint with `#[allow(clippy::cast_possible_truncation)]` ...
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_possible_truncation
help: ... or use `try_from` and handle the error accordingly
    |
127 -         let seq_len = shape_dims[1] as usize;
127 +         let seq_len = usize::try_from(shape_dims[1]);
    |

warning: casting `i64` to `usize` may lose the sign of the value
   --> src/embeddings.rs:127:23
    |
127 |         let seq_len = shape_dims[1] as usize;
    |                       ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_sign_loss

warning: casting `i64` to `usize` may truncate the value on targets with 32-bit wide pointers
   --> src/embeddings.rs:128:27
    |
128 |         let hidden_size = shape_dims[2] as usize;
    |                           ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: if this is intentional allow the lint with `#[allow(clippy::cast_possible_truncation)]` ...
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_possible_truncation
help: ... or use `try_from` and handle the error accordingly
    |
128 -         let hidden_size = shape_dims[2] as usize;
128 +         let hidden_size = usize::try_from(shape_dims[2]);
    |

warning: casting `i64` to `usize` may lose the sign of the value
   --> src/embeddings.rs:128:27
    |
128 |         let hidden_size = shape_dims[2] as usize;
    |                           ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_sign_loss

warning: unused `self` argument
   --> src/embeddings.rs:144:9
    |
144 |         &self,
    |         ^^^^^
    |
    = help: consider refactoring to an associated function
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unused_self
    = note: `-W clippy::unused-self` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::unused_self)]`

warning: variables can be used directly in the `format!` string
   --> src/embeddings.rs:152:13
    |
152 |             anyhow::bail!("Expected batch_size=1, got {}", batch_size);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
152 -             anyhow::bail!("Expected batch_size=1, got {}", batch_size);
152 +             anyhow::bail!("Expected batch_size=1, got {batch_size}");
    |

warning: the loop variable `seq_idx` is used to index `attention_mask`
   --> src/embeddings.rs:167:24
    |
167 |         for seq_idx in 0..seq_len {
    |                        ^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_range_loop
    = note: `-W clippy::needless-range-loop` implied by `-W clippy::all`
    = help: to override `-W clippy::all` add `#[allow(clippy::needless_range_loop)]`
help: consider using an iterator and enumerate()
    |
167 -         for seq_idx in 0..seq_len {
167 +         for (seq_idx, <item>) in attention_mask.iter().enumerate().take(seq_len) {
    |

warning: it is more concise to loop over references to containers instead of using explicit iteration methods
   --> src/embeddings.rs:181:26
    |
181 |             for value in pooled.iter_mut() {
    |                          ^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&mut pooled`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#explicit_iter_loop
    = note: `-W clippy::explicit-iter-loop` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::explicit_iter_loop)]`

warning: casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide)
   --> src/embeddings.rs:182:27
    |
182 |                 *value /= mask_sum as f32;
    |                           ^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_precision_loss
    = note: `-W clippy::cast-precision-loss` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_precision_loss)]`

warning: it is more concise to loop over references to containers instead of using explicit iteration methods
   --> src/embeddings.rs:189:26
    |
189 |             for value in pooled.iter_mut() {
    |                          ^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&mut pooled`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#explicit_iter_loop

warning: docs for function returning `Result` missing `# Errors` section
   --> src/embeddings.rs:198:5
    |
198 |     pub fn embed_batch(&mut self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#missing_errors_doc

warning: docs for function returning `Result` missing `# Errors` section
   --> src/embeddings.rs:214:1
    |
214 | pub fn initialize_embeddings(model_path: &str, tokenizer_path: &str) -> Result<()> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#missing_errors_doc

warning: docs for function returning `Result` missing `# Errors` section
   --> src/embeddings.rs:231:1
    |
231 | pub fn generate_embedding(text: &str) -> Result<Vec<f32>> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#missing_errors_doc

warning: this method could have a `#[must_use]` attribute
  --> src/history.rs:37:12
   |
37 |     pub fn new(project_id: String) -> Self {
   |            ^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#must_use_candidate
   = note: `-W clippy::must-use-candidate` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::must_use_candidate)]`
help: add the attribute
   |
37 ~     #[must_use] 
38 ~     pub fn new(project_id: String) -> Self {
   |

warning: item in documentation is missing backticks
  --> src/history.rs:46:48
   |
46 |     /// Automatically maintains the maximum of MAX_HISTORY_TURNS by removing oldest turns.
   |                                                ^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
   = note: `-W clippy::doc-markdown` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::doc_markdown)]`
help: try
   |
46 -     /// Automatically maintains the maximum of MAX_HISTORY_TURNS by removing oldest turns.
46 +     /// Automatically maintains the maximum of `MAX_HISTORY_TURNS` by removing oldest turns.
   |

warning: this method could have a `#[must_use]` attribute
  --> src/history.rs:66:12
   |
66 |     pub fn get_context_string(&self) -> String {
   |            ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#must_use_candidate
help: add the attribute
   |
66 ~     #[must_use] 
67 ~     pub fn get_context_string(&self) -> String {
   |

warning: `format!(..)` appended to existing `String`
  --> src/history.rs:74:13
   |
74 |             context.push_str(&format!("--- Turn {} ---\n", idx + 1));
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider using `write!` to avoid the extra allocation
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#format_push_string
   = note: `-W clippy::format-push-string` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::format_push_string)]`

warning: `format!(..)` appended to existing `String`
  --> src/history.rs:75:13
   |
75 |             context.push_str(&format!("User: {}\n", turn.user_input));
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider using `write!` to avoid the extra allocation
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#format_push_string

warning: `format!(..)` appended to existing `String`
  --> src/history.rs:84:17
   |
84 |                 context.push_str(&format!("Assistant: {}\n", truncated));
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider using `write!` to avoid the extra allocation
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#format_push_string

warning: variables can be used directly in the `format!` string
  --> src/history.rs:84:35
   |
84 |                 context.push_str(&format!("Assistant: {}\n", truncated));
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
84 -                 context.push_str(&format!("Assistant: {}\n", truncated));
84 +                 context.push_str(&format!("Assistant: {truncated}\n"));
   |

warning: docs for function returning `Result` missing `# Errors` section
   --> src/history.rs:103:5
    |
103 |     pub fn load(obsidian_root: &Path, project_id: &str) -> Result<Self> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#missing_errors_doc

warning: docs for function returning `Result` missing `# Errors` section
   --> src/history.rs:127:5
    |
127 |     pub fn save(&self, obsidian_root: &Path) -> Result<()> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#missing_errors_doc

warning: this method could have a `#[must_use]` attribute
   --> src/history.rs:158:12
    |
158 |     pub fn len(&self) -> usize {
    |            ^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#must_use_candidate
help: add the attribute
    |
158 ~     #[must_use] 
159 ~     pub fn len(&self) -> usize {
    |

warning: this method could have a `#[must_use]` attribute
   --> src/history.rs:163:12
    |
163 |     pub fn is_empty(&self) -> bool {
    |            ^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#must_use_candidate
help: add the attribute
    |
163 ~     #[must_use] 
164 ~     pub fn is_empty(&self) -> bool {
    |

warning: variables can be used directly in the `format!` string
   --> src/history.rs:189:30
    |
189 |             history.add_turn(format!("Query {}", i), Some(format!("Response {}", i)));
    |                              ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
189 -             history.add_turn(format!("Query {}", i), Some(format!("Response {}", i)));
189 +             history.add_turn(format!("Query {i}"), Some(format!("Response {}", i)));
    |

warning: variables can be used directly in the `format!` string
   --> src/history.rs:189:59
    |
189 |             history.add_turn(format!("Query {}", i), Some(format!("Response {}", i)));
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
189 -             history.add_turn(format!("Query {}", i), Some(format!("Response {}", i)));
189 +             history.add_turn(format!("Query {}", i), Some(format!("Response {i}")));
    |

warning: `nexus` (lib) generated 34 warnings (run `cargo clippy --fix --lib -p nexus` to apply 15 suggestions)
warning: `nexus` (lib test) generated 36 warnings (34 duplicates) (run `cargo clippy --fix --lib -p nexus --tests` to apply 2 suggestions)
warning: unnecessary hashes around raw string literal
  --> tests/gate_stress_tests.rs:77:19
   |
77 |       let content = r#"# Dashboard
   |  ___________________^
78 | | - [x] Task 1 completed
79 | | - [x] Task 2 completed
80 | | "#;
   | |__^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
   = note: `-W clippy::needless-raw-string-hashes` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::needless_raw_string_hashes)]`
help: remove all the hashes around the string literal
   |
77 ~     let content = r"# Dashboard
78 | - [x] Task 1 completed
79 | - [x] Task 2 completed
80 ~ ";
   |

warning: unnecessary hashes around raw string literal
   --> tests/unlock_integration.rs:149:9
    |
149 | /         r#"# What is the problem?
150 | |
151 | | ## My problem (personal):
152 | | I need a tool to manage my projects better and stay organized always.
...   |
168 | | - Not: A replacement for existing tools
169 | | "#,
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
    = note: `-W clippy::needless-raw-string-hashes` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::needless_raw_string_hashes)]`
help: remove all the hashes around the string literal
    |
149 ~         r"# What is the problem?
150 |
...
168 | - Not: A replacement for existing tools
169 ~ ",
    |

warning: unnecessary hashes around raw string literal
   --> tests/unlock_integration.rs:175:9
    |
175 | /         r#"# What am I building? (Scope)
176 | |
177 | | ## MVP (Minimum Viable Product):
178 | | Absolute minimum version that solves core problem:
...   |
190 | | - Platform: CLI only
191 | | "#,
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
175 ~         r"# What am I building? (Scope)
176 |
...
190 | - Platform: CLI only
191 ~ ",
    |

warning: unnecessary hashes around raw string literal
   --> tests/unlock_integration.rs:197:9
    |
197 | /         r#"# Technical choices
198 | |
199 | | ## Stack (force yourself to choose NOW):
200 | | - **Frontend:** CLI only
...   |
220 | | - Device: Desktop
221 | | "#,
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
197 ~         r"# Technical choices
198 |
...
220 | - Device: Desktop
221 ~ ",
    |

warning: unnecessary hashes around raw string literal
   --> tests/unlock_integration.rs:227:9
    |
227 | /         r#"# System design
228 | |
229 | | ## Folder structure:
230 | | ```
...   |
255 | | - Data persistence: Markdown files
256 | | "#,
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
227 ~         r"# System design
228 |
...
255 | - Data persistence: Markdown files
256 ~ ",
    |

warning: unnecessary hashes around raw string literal
   --> tests/unlock_integration.rs:262:9
    |
262 | /         r#"# MVP broken into sprints
263 | |
264 | | ## Sprint 0: Setup (day 1)
265 | | - [x] Create repo
...   |
291 | | - [x] Session log updated
292 | | "#,
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
262 ~         r"# MVP broken into sprints
263 |
...
291 | - [x] Session log updated
292 ~ ",
    |

warning: unnecessary hashes around raw string literal
  --> tests/gate_integration.rs:78:19
   |
78 |       let content = r#"# Problem
   |  ___________________^
79 | | This is a detailed problem statement with more than fifty words to ensure it passes the minimum word count validation requirement for our...
80 | |
81 | | # Vision
...  |
94 | | This is a comprehensive architecture overview with more than fifty words to ensure it passes the minimum word count validation requiremen...
95 | | "#;
   | |__^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
   = note: `-W clippy::needless-raw-string-hashes` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::needless_raw_string_hashes)]`
help: remove all the hashes around the string literal
   |
78 ~     let content = r"# Problem
79 | This is a detailed problem statement with more than fifty words to ensure it passes the minimum word count validation requirement for our comprehensive testing purposes today. The problem is clearly defined and articulated with sufficient context and detail to meet all validation criteria and requirements specified in the heuristics configuration file for proper gate validation functionality.
...
94 | This is a comprehensive architecture overview with more than fifty words to ensure it passes the minimum word count validation requirement for our comprehensive testing purposes today. The architecture is described clearly with adequate technical detail and context to meet all validation criteria and requirements specified in the heuristics configuration.
95 ~ ";
   |

warning: unnecessary hashes around raw string literal
   --> tests/gate_integration.rs:101:19
    |
101 |       let content = r#"# Problem
    |  ___________________^
102 | | TODO: Fill this in later
103 | |
104 | | # Vision
105 | | Not enough words here.
106 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
101 ~     let content = r"# Problem
102 | TODO: Fill this in later
...
105 | Not enough words here.
106 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> tests/gate_integration.rs:112:19
    |
112 |       let content = r#"# Dashboard
    |  ___________________^
113 | | - [x] Task 1 completed
114 | | - [x] Task 2 completed
115 | | - [x] Task 3 completed
116 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
112 ~     let content = r"# Dashboard
113 | - [x] Task 1 completed
114 | - [x] Task 2 completed
115 | - [x] Task 3 completed
116 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> tests/gate_integration.rs:122:19
    |
122 |       let content = r#"# Dashboard
    |  ___________________^
123 | | - [x] Task 1 completed
124 | | - [ ] Task 2 not done
125 | | - [x] Task 3 completed
126 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
122 ~     let content = r"# Dashboard
123 | - [x] Task 1 completed
124 | - [ ] Task 2 not done
125 | - [x] Task 3 completed
126 ~ ";
    |

warning: unnecessary hashes around raw string literal
  --> tests/sprint_integration.rs:57:23
   |
57 |       let mvp_content = r#"# MVP broken into sprints
   |  _______________________^
58 | |
59 | | ## Sprint 0: Setup (day 1)
60 | | - [x] Create nexus repo with Cargo.toml
...  |
91 | | **Exit criteria:** `nexus sprint X` creates branch and workspace
92 | | "#;
   | |__^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
   = note: `-W clippy::needless-raw-string-hashes` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::needless_raw_string_hashes)]`
help: remove all the hashes around the string literal
   |
57 ~     let mvp_content = r"# MVP broken into sprints
58 |
...
91 | **Exit criteria:** `nexus sprint X` creates branch and workspace
92 ~ ";
   |

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
 --> tests/init_integration.rs:8:14
  |
8 |     Command::cargo_bin("nexus").expect("Failed to find binary")
  |              ^^^^^^^^^
  |
  = note: `#[warn(deprecated)]` on by default

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_integration.rs:148:28
    |
148 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: use of deprecated associated function `assert_cmd::cargo::CommandCargoExt::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin!`
   --> tests/unlock_integration.rs:302:28
    |
302 |     let mut cmd = Command::cargo_bin("nexus")?;
    |                            ^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
  --> tests/gate_stress_tests.rs:98:28
   |
98 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
   |                            ^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_integration.rs:175:28
    |
175 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:119:28
    |
119 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::cargo::CommandCargoExt::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin!`
   --> tests/unlock_integration.rs:324:28
    |
324 |     let mut cmd = Command::cargo_bin("nexus")?;
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_integration.rs:202:28
    |
202 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:151:28
    |
151 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_integration.rs:229:28
    |
229 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: variables can be used directly in the `format!` string
  --> examples/test_embeddings.rs:33:9
   |
33 |         println!("  ⏱  Time: {:?}", elapsed);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
   = note: `-W clippy::uninlined-format-args` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`
help: change this to
   |
33 -         println!("  ⏱  Time: {:?}", elapsed);
33 +         println!("  ⏱  Time: {elapsed:?}");
   |

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_integration.rs:246:28
    |
246 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:179:28
    |
179 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: variables can be used directly in the `format!` string
  --> examples/test_embeddings.rs:37:9
   |
37 |         println!("  📊 L2 Norm: {:.6} (should be ~1.0)", norm);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
37 -         println!("  📊 L2 Norm: {:.6} (should be ~1.0)", norm);
37 +         println!("  📊 L2 Norm: {norm:.6} (should be ~1.0)");
   |

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/sprint_integration.rs:145:28
    |
145 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:213:28
    |
213 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: variables can be used directly in the `format!` string
  --> examples/test_embeddings.rs:40:68
   |
40 |         let sample: Vec<String> = embedding.iter().take(5).map(|x| format!("{:.4}", x)).collect();
   |                                                                    ^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
40 -         let sample: Vec<String> = embedding.iter().take(5).map(|x| format!("{:.4}", x)).collect();
40 +         let sample: Vec<String> = embedding.iter().take(5).map(|x| format!("{x:.4}")).collect();
   |

warning: use of deprecated associated function `assert_cmd::cargo::CommandCargoExt::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin!`
   --> tests/unlock_integration.rs:359:28
    |
359 |     let mut cmd = Command::cargo_bin("nexus")?;
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::cargo::CommandCargoExt::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin!`
   --> tests/unlock_integration.rs:364:29
    |
364 |     let mut cmd2 = Command::cargo_bin("nexus")?;
    |                             ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:245:28
    |
245 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: useless use of `vec!`
  --> examples/test_embeddings.rs:16:24
   |
16 |       let test_queries = vec![
   |  ________________________^
17 | |         "How do I implement user authentication?",
18 | |         "What is the architecture for the database layer?",
19 | |         "Write a function to parse sprint metadata",
20 | |         "Refactor the context injection system",
21 | |     ];
   | |_____^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#useless_vec
   = note: `-W clippy::useless-vec` implied by `-W clippy::all`
   = help: to override `-W clippy::all` add `#[allow(clippy::useless_vec)]`
help: you can use an array directly
   |
16 ~     let test_queries = ["How do I implement user authentication?",
17 +         "What is the architecture for the database layer?",
18 +         "Write a function to parse sprint metadata",
19 ~         "Refactor the context injection system"];
   |

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:289:28
    |
289 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/sprint_integration.rs:237:28
    |
237 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:321:28
    |
321 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/sprint_integration.rs:265:28
    |
265 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: variables can be used directly in the `format!` string
  --> tests/gate_integration.rs:12:26
   |
12 |       let config_content = format!(
   |  __________________________^
13 | |         r#"[project]
14 | | name = "test_project"
15 | | version = "0.1.0"
...  |
37 | |         vault_path_str
38 | |     );
   | |_____^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
   = note: `-W clippy::uninlined-format-args` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/sprint_integration.rs:295:28
    |
295 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/sprint_integration.rs:322:28
    |
322 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: item in documentation is missing backticks
 --> tests/init_integration.rs:6:45
  |
6 | /// Helper to create a command instance for nexus_cli
  |                                             ^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
  = note: `-W clippy::doc-markdown` implied by `-W clippy::pedantic`
  = help: to override `-W clippy::pedantic` add `#[allow(clippy::doc_markdown)]`
help: try
  |
6 - /// Helper to create a command instance for nexus_cli
6 + /// Helper to create a command instance for `nexus_cli`
  |

warning: `nexus` (example "test_embeddings") generated 4 warnings (run `cargo clippy --fix --example "test_embeddings" -p nexus` to apply 4 suggestions)
warning: variables can be used directly in the `format!` string
  --> tests/init_integration.rs:43:34
   |
43 |         config_content.contains(&format!("name = \"{}\"", project_name)),
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
   = note: `-W clippy::uninlined-format-args` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`
help: change this to
   |
43 -         config_content.contains(&format!("name = \"{}\"", project_name)),
43 +         config_content.contains(&format!("name = \"{project_name}\"")),
   |

warning: this function has too many lines (161/100)
  --> tests/unlock_integration.rs:92:1
   |
92 | fn setup_complete_project(project_dir: &Path) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#too_many_lines
   = note: `-W clippy::too-many-lines` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::too_many_lines)]`

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:367:28
    |
367 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: variables can be used directly in the `format!` string
  --> tests/init_integration.rs:80:9
   |
80 |         assert!(file_path.exists(), "Template file {} should exist", file);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
80 -         assert!(file_path.exists(), "Template file {} should exist", file);
80 +         assert!(file_path.exists(), "Template file {file} should exist");
   |

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:398:28
    |
398 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`: incompatible with a custom cargo build-dir, see instead `cargo::cargo_bin_cmd!`
   --> tests/gate_stress_tests.rs:433:28
    |
433 |     let mut cmd = Command::cargo_bin("nexus").unwrap();
    |                            ^^^^^^^^^

warning: variables can be used directly in the `format!` string
   --> tests/init_integration.rs:166:5
    |
166 | /     assert!(
167 | |         obsidian_path_line.contains('/'),
168 | |         "obsidian_path should be absolute: {}",
169 | |         obsidian_path_line
170 | |     );
    | |_____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args

warning: variables can be used directly in the `format!` string
  --> tests/sprint_integration.rs:17:9
   |
17 | /         format!(
18 | |             r#"
19 | | [state.active_sprint]
20 | | current = "{}"
...  |
23 | |             current, status
24 | |         )
   | |_________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
   = note: `-W clippy::uninlined-format-args` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`

warning: variables can be used directly in the `format!` string
  --> tests/sprint_integration.rs:29:26
   |
29 |       let config_content = format!(
   |  __________________________^
30 | |         r#"[project]
31 | | name = "test_project"
32 | | version = "0.1.0"
...  |
50 | |         vault_path_str, active_sprint_section
51 | |     );
   | |_____^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args

warning: `nexus` (test "gate_integration") generated 10 warnings (run `cargo clippy --fix --test "gate_integration" -p nexus` to apply 5 suggestions)
warning: `nexus` (test "unlock_integration") generated 10 warnings (run `cargo clippy --fix --test "unlock_integration" -p nexus` to apply 5 suggestions)
warning: `nexus` (test "init_integration") generated 5 warnings (run `cargo clippy --fix --test "init_integration" -p nexus` to apply 4 suggestions)
warning: variables can be used directly in the `format!` string
  --> tests/gate_stress_tests.rs:15:26
   |
15 |       let config_content = format!(
   |  __________________________^
16 | |         r#"[project]
17 | | name = "test_project"
18 | | version = "0.1.0"
...  |
40 | |         vault_path_str
41 | |     );
   | |_____^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
   = note: `-W clippy::uninlined-format-args` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:138:40
    |
138 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone
    = note: `-W clippy::implicit-clone` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::implicit_clone)]`

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:139:24
    |
139 |     create_heuristics(&vault_path.to_path_buf());
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:167:40
    |
167 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:168:24
    |
168 |     create_heuristics(&vault_path.to_path_buf());
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:198:40
    |
198 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:199:24
    |
199 |     create_heuristics(&vault_path.to_path_buf());
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:231:40
    |
231 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:232:24
    |
232 |     create_heuristics(&vault_path.to_path_buf());
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:271:40
    |
271 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:272:24
    |
272 |     create_heuristics(&vault_path.to_path_buf());
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: `nexus` (test "sprint_integration") generated 8 warnings (run `cargo clippy --fix --test "sprint_integration" -p nexus` to apply 3 suggestions)
warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:316:40
    |
316 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:341:40
    |
341 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:342:24
    |
342 |     create_heuristics(&vault_path.to_path_buf());
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: `format!(..)` appended to existing `String`
   --> tests/gate_stress_tests.rs:352:9
    |
352 | /         content.push_str(&format!(
353 | |             "This is line number {} with some padding text to make it longer. ",
354 | |             i
355 | |         ));
    | |__________^
    |
    = help: consider using `write!` to avoid the extra allocation
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#format_push_string
    = note: `-W clippy::format-push-string` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::format_push_string)]`

warning: variables can be used directly in the `format!` string
   --> tests/gate_stress_tests.rs:352:27
    |
352 |           content.push_str(&format!(
    |  ___________________________^
353 | |             "This is line number {} with some padding text to make it longer. ",
354 | |             i
355 | |         ));
    | |_________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:387:40
    |
387 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:417:40
    |
417 |     create_nexus_config(project_path, &vault_path.to_path_buf());
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: implicitly cloning a `PathBuf` by calling `to_path_buf` on its dereferenced type
   --> tests/gate_stress_tests.rs:418:24
    |
418 |     create_heuristics(&vault_path.to_path_buf());
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `vault_path.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone

warning: `nexus` (test "gate_stress_tests") generated 31 warnings (run `cargo clippy --fix --test "gate_stress_tests" -p nexus` to apply 19 suggestions)
warning: empty line after doc comment
 --> src/catalyst/mod.rs:4:1
  |
4 | / /// from a user's vision document (01) using DeepSeek R1 with sequential thinking.
5 | |
  | |_^
6 |   pub mod engine;
  |   -------------- the comment documents this module
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#empty_line_after_doc_comments
  = note: `-W clippy::empty-line-after-doc-comments` implied by `-W clippy::all`
  = help: to override `-W clippy::all` add `#[allow(clippy::empty_line_after_doc_comments)]`
  = help: if the empty line is unintentional, remove it
help: if the comment should document the parent module use an inner doc comment
  |
1 ~ //! Planning Catalyst - AI-powered planning document generation
2 ~ //! 
3 ~ //! This module provides intelligent generation of planning documents (02-05)
4 ~ //! from a user's vision document (01) using DeepSeek R1 with sequential thinking.
  |

warning: empty line after doc comment
 --> src/catalyst/engine.rs:1:1
  |
1 | / /// Core catalyst engine for document generation
2 | |
  | |_^
3 |   use anyhow::{Context, Result};
  |   - the comment documents this `use` import
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#empty_line_after_doc_comments
  = help: if the empty line is unintentional, remove it
help: if the comment should document the parent module use an inner doc comment
  |
1 - /// Core catalyst engine for document generation
1 + //! Core catalyst engine for document generation
  |

warning: binding's name is too similar to existing binding
   --> src/catalyst/engine.rs:216:13
    |
216 |         let content = self
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/catalyst/engine.rs:213:13
    |
213 |         let context = GenerationContext::new(vision);
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names
    = note: `-W clippy::similar-names` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::similar_names)]`

warning: binding's name is too similar to existing binding
   --> src/catalyst/engine.rs:276:13
    |
276 |         let content = self
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/catalyst/engine.rs:273:13
    |
273 |         let context = GenerationContext::new(vision).with_scope(scope);
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: binding's name is too similar to existing binding
   --> src/catalyst/engine.rs:338:13
    |
338 |         let content = self
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/catalyst/engine.rs:333:13
    |
333 |         let context = GenerationContext::new(vision)
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: binding's name is too similar to existing binding
   --> src/catalyst/engine.rs:402:13
    |
402 |         let content = self
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/catalyst/engine.rs:396:13
    |
396 |         let context = GenerationContext::new(vision)
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: unnecessary hashes around raw string literal
   --> src/catalyst/engine.rs:556:13
    |
556 | /             r#"{}
557 | |
558 | | REFINEMENT CONTEXT:
559 | | You previously generated this document:
...   |
568 | | Please regenerate the document incorporating the user's feedback while maintaining the required structure and format."#,
    | |_______________________________________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
    = note: `-W clippy::needless-raw-string-hashes` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::needless_raw_string_hashes)]`
help: remove all the hashes around the string literal
    |
556 ~             r"{}
557 |
...
567 |
568 ~ Please regenerate the document incorporating the user's feedback while maintaining the required structure and format.",
    |

warning: empty line after doc comment
 --> src/catalyst/generator.rs:1:1
  |
1 | / /// Document generation types and context management
2 | |
  | |_^
3 |   use serde::{Deserialize, Serialize};
  |   - the comment documents this `use` import
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#empty_line_after_doc_comments
  = help: if the empty line is unintentional, remove it
help: if the comment should document the parent module use an inner doc comment
  |
1 - /// Document generation types and context management
1 + //! Document generation types and context management
  |

warning: empty line after doc comment
 --> src/catalyst/prompts.rs:1:1
  |
1 | / /// Prompt templates for document generation
2 | |
  | |_^
3 |   use crate::catalyst::generator::GenerationContext;
  |   - the comment documents this `use` import
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#empty_line_after_doc_comments
  = help: if the empty line is unintentional, remove it
help: if the comment should document the parent module use an inner doc comment
  |
1 - /// Prompt templates for document generation
1 + //! Prompt templates for document generation
  |

warning: unnecessary hashes around raw string literal
  --> src/catalyst/prompts.rs:46:13
   |
46 | /             r#"Based on this vision, generate a comprehensive Scope and Boundaries document:
47 | |
48 | | # Vision
...  |
64 | | - Minimum 50 words per section
65 | | - Focus on making the MVP truly minimal"#,
   | |_________________________________________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
   |
46 ~             r"Based on this vision, generate a comprehensive Scope and Boundaries document:
47 |
...
64 | - Minimum 50 words per section
65 ~ - Focus on making the MVP truly minimal",
   |

warning: unnecessary hashes around raw string literal
  --> src/catalyst/prompts.rs:82:17
   |
82 | /                 r#"
83 | | # Scope Context
84 | |
85 | | **MVP Features**:
...  |
94 | | **Constraints**:
95 | | {}"#,
   | |____^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
   |
82 ~                 r"
83 | # Scope Context
...
94 | **Constraints**:
95 ~ {}",
   |

warning: unnecessary hashes around raw string literal
   --> src/catalyst/prompts.rs:152:13
    |
152 | /             r#"Based on this vision and scope, generate a comprehensive Tech Stack document:
153 | |
154 | | # Vision
...   |
168 | | - No placeholders or TODOs
169 | | - Justify based on MVP needs and constraints"#,
    | |______________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
152 ~             r"Based on this vision and scope, generate a comprehensive Tech Stack document:
153 |
...
168 | - No placeholders or TODOs
169 ~ - Justify based on MVP needs and constraints",
    |

warning: unnecessary hashes around raw string literal
   --> src/catalyst/prompts.rs:191:17
    |
191 | /                 r#"
192 | | # Scope Context
193 | |
194 | | **MVP Features**:
...   |
197 | | **Constraints**:
198 | | {}"#,
    | |____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
191 ~                 r"
192 | # Scope Context
...
197 | **Constraints**:
198 ~ {}",
    |

warning: unnecessary hashes around raw string literal
   --> src/catalyst/prompts.rs:213:17
    |
213 | /                 r#"
214 | | # Tech Stack Context
215 | |
216 | | **Language**: {}
217 | | **Framework**: {}
218 | | **Database**: {}
219 | | **Justification**: {}"#,
    | |_______________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
213 ~                 r"
214 | # Tech Stack Context
...
218 | **Database**: {}
219 ~ **Justification**: {}",
    |

warning: unnecessary hashes around raw string literal
   --> src/catalyst/prompts.rs:262:13
    |
262 | /             r#"Based on this vision, scope, and tech stack, generate a comprehensive Architecture document:
263 | |
264 | | # Vision
...   |
281 | | - List actual entities and fields
282 | | - Step-by-step user flow"#,
    | |__________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
262 ~             r"Based on this vision, scope, and tech stack, generate a comprehensive Architecture document:
263 |
...
281 | - List actual entities and fields
282 ~ - Step-by-step user flow",
    |

warning: unnecessary hashes around raw string literal
   --> src/catalyst/prompts.rs:300:17
    |
300 | /                 r#"
301 | | # Scope Context
302 | |
303 | | **MVP Features**:
304 | | {}"#,
    | |____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
300 ~                 r"
301 | # Scope Context
302 |
303 | **MVP Features**:
304 ~ {}",
    |

warning: unnecessary hashes around raw string literal
   --> src/catalyst/prompts.rs:318:17
    |
318 | /                 r#"
319 | | # Tech Stack
320 | |
321 | | **Language**: {}
322 | | **Framework**: {}"#,
    | |___________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
318 ~                 r"
319 | # Tech Stack
320 |
321 | **Language**: {}
322 ~ **Framework**: {}",
    |

warning: unnecessary hashes around raw string literal
   --> src/catalyst/prompts.rs:331:17
    |
331 | /                 r#"
332 | | # Architecture
333 | |
334 | | **Folder Structure**:
...   |
337 | | **Data Model**:
338 | | {}"#,
    | |____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
331 ~                 r"
332 | # Architecture
...
337 | **Data Model**:
338 ~ {}",
    |

warning: empty line after doc comment
 --> src/catalyst/validation.rs:1:1
  |
1 | / /// Document validation for generated planning documents
2 | |
  | |_^
3 |   use anyhow::{Context, Result};
  |   - the comment documents this `use` import
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#empty_line_after_doc_comments
  = help: if the empty line is unintentional, remove it
help: if the comment should document the parent module use an inner doc comment
  |
1 - /// Document validation for generated planning documents
1 + //! Document validation for generated planning documents
  |

warning: unnested or-patterns
   --> src/commands/shell.rs:566:13
    |
566 |             Some("arch") | Some("architecture") => engine.generate_architecture().await,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnested_or_patterns
    = note: `-W clippy::unnested-or-patterns` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::unnested_or_patterns)]`
help: nest the patterns
    |
566 -             Some("arch") | Some("architecture") => engine.generate_architecture().await,
566 +             Some("arch" | "architecture") => engine.generate_architecture().await,
    |

warning: unnested or-patterns
   --> src/commands/shell.rs:567:13
    |
567 |             Some("mvp") | Some("breakdown") => engine.generate_mvp_breakdown().await,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnested_or_patterns
help: nest the patterns
    |
567 -             Some("mvp") | Some("breakdown") => engine.generate_mvp_breakdown().await,
567 +             Some("mvp" | "breakdown") => engine.generate_mvp_breakdown().await,
    |

warning: unnested or-patterns
   --> src/commands/shell.rs:568:13
    |
568 |             Some("generate") | Some("all") => {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnested_or_patterns
help: nest the patterns
    |
568 -             Some("generate") | Some("all") => {
568 +             Some("generate" | "all") => {
    |

warning: unnecessary hashes around raw string literal
    --> src/catalyst/engine.rs:1053:23
     |
1053 |           let content = r#"
     |  _______________________^
1054 | | # My problem (personal):
1055 | |
1056 | | I need to manage my projects better.
...    |
1068 | | Not a full project management suite.
1069 | | "#;
     | |__^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
     |
1053 ~         let content = r"
1054 | # My problem (personal):
 ...
1068 | Not a full project management suite.
1069 ~ ";
     |

warning: unnecessary hashes around raw string literal
    --> src/catalyst/engine.rs:1080:20
     |
1080 |           let text = r#"
     |  ____________________^
1081 | | - Feature one
1082 | | - Feature two
1083 | | * Feature three
1084 | | 1. Feature four
1085 | | 2. Feature five
1086 | | "#;
     | |__^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
     |
1080 ~         let text = r"
1081 | - Feature one
 ...
1085 | 2. Feature five
1086 ~ ";
     |

warning: unnecessary hashes around raw string literal
   --> src/catalyst/validation.rs:148:23
    |
148 |           let content = r#"
    |  _______________________^
149 | | ## MVP (Minimum Viable Product):
150 | |
151 | | This is the MVP content.
...   |
156 | | This is version 2 content.
157 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
148 ~         let content = r"
149 | ## MVP (Minimum Viable Product):
...
156 | This is version 2 content.
157 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> src/context.rs:315:23
    |
315 |           let content = r#"
    |  _______________________^
316 | | # Sprint Tasks
317 | |
318 | | - [x] Completed task
...   |
322 | | * [ ] Unfinished task 3
323 | |         "#;
    | |__________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
315 ~         let content = r"
316 | # Sprint Tasks
...
322 | * [ ] Unfinished task 3
323 ~         ";
    |

warning: binding's name is too similar to existing binding
   --> src/planning.rs:411:25
    |
411 |                     let context = if current_item_text.is_empty() {
    |                         ^^^^^^^
    |
note: existing binding defined here
   --> src/planning.rs:368:9
    |
368 |     let content = fs::read_to_string(file_path)
    |         ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: binding's name is too similar to existing binding
   --> src/planning.rs:546:13
    |
546 |         let content = fs::read_to_string(&problem_vision_path)
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/planning.rs:541:13
    |
541 |     let mut context = PlanningContext::new(project_name);
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: binding's name is too similar to existing binding
   --> src/planning.rs:567:13
    |
567 |         let content =
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/planning.rs:541:13
    |
541 |     let mut context = PlanningContext::new(project_name);
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: binding's name is too similar to existing binding
   --> src/planning.rs:588:13
    |
588 |         let content =
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/planning.rs:541:13
    |
541 |     let mut context = PlanningContext::new(project_name);
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: binding's name is too similar to existing binding
   --> src/planning.rs:609:13
    |
609 |         let content =
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/planning.rs:541:13
    |
541 |     let mut context = PlanningContext::new(project_name);
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: binding's name is too similar to existing binding
   --> src/planning.rs:630:13
    |
630 |         let content =
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/planning.rs:541:13
    |
541 |     let mut context = PlanningContext::new(project_name);
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: unnecessary hashes around raw string literal
  --> src/scaffolding.rs:74:9
   |
74 | /         r#"# Sprint {} Tasks: {}
75 | |
76 | | ## Task List
77 | | {}
...  |
85 | | - (none yet)
86 | | "#,
   | |__^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
   |
74 ~         r"# Sprint {} Tasks: {}
75 |
...
85 | - (none yet)
86 ~ ",
   |

warning: unnecessary hashes around raw string literal
   --> src/scaffolding.rs:100:9
    |
100 | /         r#"# Sprint {} Context: {}
101 | |
102 | | ## Focus
103 | | {}
...   |
129 | | - Tech stack: `01-PLANNING/03-Tech-Stack.md`
130 | | "#,
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
100 ~         r"# Sprint {} Context: {}
101 |
...
129 | - Tech stack: `01-PLANNING/03-Tech-Stack.md`
130 ~ ",
    |

warning: unnecessary hashes around raw string literal
   --> src/planning.rs:812:23
    |
812 |           let content = r#"# Problem
    |  _______________________^
813 | | This is a test problem section with enough words to pass the minimum threshold requirement for validation purposes and testing needs for ...
814 | |
815 | | # Vision
...   |
828 | | Architecture overview containing enough words to meet minimum requirements for section validation in our testing framework and beyond. Th...
829 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
812 ~         let content = r"# Problem
813 | This is a test problem section with enough words to pass the minimum threshold requirement for validation purposes and testing needs for our comprehensive test suite validation. We need to ensure that all content here meets the minimum word count requirements so that this validation test can properly verify the streaming parser functionality without triggering any false positive errors during our automated testing procedures.
...
828 | Architecture overview containing enough words to meet minimum requirements for section validation in our testing framework and beyond. This section should provide detailed information about the system architecture while ensuring we meet all validation criteria including minimum word counts so that our comprehensive test suite can properly verify all aspects of the markdown streaming parser functionality.
829 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> src/planning.rs:846:23
    |
846 |           let content = r#"# Problem
    |  _______________________^
847 | | Just a few words here.
848 | |
849 | | # Vision
850 | | Not enough content.
851 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
846 ~         let content = r"# Problem
847 | Just a few words here.
...
850 | Not enough content.
851 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> src/planning.rs:871:23
    |
871 |           let content = r#"# Problem
    |  _______________________^
872 | | This section has a TODO placeholder that should be detected by our validation system for testing purposes here.
873 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
871 ~         let content = r"# Problem
872 | This section has a TODO placeholder that should be detected by our validation system for testing purposes here.
873 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> src/planning.rs:892:23
    |
892 |           let content = r#"# Problem
    |  _______________________^
893 | | This is a problem section with adequate word count for validation purposes and testing requirements today.
894 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
892 ~         let content = r"# Problem
893 | This is a problem section with adequate word count for validation purposes and testing requirements today.
894 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> src/planning.rs:914:23
    |
914 |           let content = r#"# Dashboard
    |  _______________________^
915 | | - [x] Task 1 completed
916 | | - [x] Task 2 completed
917 | | - [x] Task 3 completed
918 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
914 ~         let content = r"# Dashboard
915 | - [x] Task 1 completed
916 | - [x] Task 2 completed
917 | - [x] Task 3 completed
918 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> src/planning.rs:935:23
    |
935 |           let content = r#"# Dashboard
    |  _______________________^
936 | | - [x] Task 1 completed
937 | | - [ ] Task 2 not done
938 | | - [x] Task 3 completed
939 | | - [ ] Task 4 pending
940 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
935 ~         let content = r"# Dashboard
936 | - [x] Task 1 completed
...
939 | - [ ] Task 4 pending
940 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> src/planning.rs:966:23
    |
966 |           let content = r#"# Dashboard
    |  _______________________^
967 | | This is just regular text with no checkboxes.
968 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
966 ~         let content = r"# Dashboard
967 | This is just regular text with no checkboxes.
968 ~ ";
    |

warning: unnecessary hashes around raw string literal
   --> src/planning.rs:981:23
    |
981 |           let content = r#"# Dashboard
    |  _______________________^
982 | | Regular list:
983 | | - Item 1
984 | | - Item 2
...   |
991 | | - Item 3
992 | | "#;
    | |__^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
    |
981 ~         let content = r"# Dashboard
982 | Regular list:
...
991 | - Item 3
992 ~ ";
    |

warning: unnecessary hashes around raw string literal
    --> src/planning.rs:1018:23
     |
1018 |           let content = r#"# Dashboard
     |  _______________________^
1019 | | - [ ] This is a very long task description that should be truncated to 50 characters for context display
1020 | | "#;
     | |__^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
     |
1018 ~         let content = r"# Dashboard
1019 | - [ ] This is a very long task description that should be truncated to 50 characters for context display
1020 ~ ";
     |

warning: unnecessary hashes around raw string literal
    --> src/planning.rs:1087:23
     |
1087 |           let content = r#"# MVP broken into sprints
     |  _______________________^
1088 | |
1089 | | ## Sprint 0: Setup (day 1)
1090 | | - [x] Create nexus repo with Cargo.toml
...    |
1107 | | **Exit criteria:** `nexus sprint X` creates a clean branch
1108 | | "#;
     | |__^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
     |
1087 ~         let content = r"# MVP broken into sprints
1088 |
 ...
1107 | **Exit criteria:** `nexus sprint X` creates a clean branch
1108 ~ ";
     |

warning: unnecessary hashes around raw string literal
    --> src/planning.rs:1138:23
     |
1138 |           let content = r#"
     |  _______________________^
1139 | | _Focus: Test focus statement._
1140 | |
1141 | | - [x] Task one completed
...    |
1147 | | **Exit criteria:** All tests pass
1148 | | "#;
     | |__^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_raw_string_hashes
help: remove all the hashes around the string literal
     |
1138 ~         let content = r"
1139 | _Focus: Test focus statement._
 ...
1147 | **Exit criteria:** All tests pass
1148 ~ ";
     |

warning: binding's name is too similar to existing binding
  --> src/templating.rs:93:13
   |
93 |         let content = std::fs::read_to_string(&claude_path).unwrap();
   |             ^^^^^^^
   |
note: existing binding defined here
  --> src/templating.rs:82:17
   |
82 |         let mut context = PlanningContext::new("TestProject".to_string());
   |                 ^^^^^^^
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: binding's name is too similar to existing binding
   --> src/templating.rs:120:13
    |
120 |         let content = std::fs::read_to_string(&claude_path).unwrap();
    |             ^^^^^^^
    |
note: existing binding defined here
   --> src/templating.rs:114:13
    |
114 |         let context = PlanningContext::new("CustomProject".to_string());
    |             ^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#similar_names

warning: field `project_id` is never read
   --> src/catalyst/engine.rs:182:5
    |
180 | pub struct CatalystEngine {
    |            -------------- field in this struct
181 |     /// Project identifier
182 |     project_id: String,
    |     ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `display_name` is never used
  --> src/catalyst/generator.rs:30:12
   |
18 | impl DocumentType {
   | ----------------- method in this implementation
...
30 |     pub fn display_name(&self) -> &'static str {
   |            ^^^^^^^^^^^^

warning: associated function `empty` is never used
  --> src/catalyst/generator.rs:55:12
   |
53 | impl VisionData {
   | --------------- associated function in this implementation
54 |     /// Create an empty VisionData
55 |     pub fn empty() -> Self {
   |            ^^^^^

warning: method `render` is never used
   --> src/catalyst/prompts.rs:416:12
    |
 11 | impl PromptTemplate {
    | ------------------- method in this implementation
...
416 |     pub fn render(&self) -> String {
    |            ^^^^^^

warning: function `contains_placeholders` is never used
  --> src/catalyst/validation.rs:85:8
   |
85 | pub fn contains_placeholders(content: &str) -> bool {
   |        ^^^^^^^^^^^^^^^^^^^^^

warning: function `extract_section` is never used
   --> src/catalyst/validation.rs:107:8
    |
107 | pub fn extract_section(content: &str, header: &str) -> Option<String> {
    |        ^^^^^^^^^^^^^^^

warning: method `embed_batch` is never used
   --> src/embeddings.rs:198:12
    |
 20 | impl EmbeddingGenerator {
    | ----------------------- method in this implementation
...
198 |     pub fn embed_batch(&mut self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
    |            ^^^^^^^^^^^

warning: method `clear` is never used
   --> src/history.rs:153:12
    |
 35 | impl ConversationHistory {
    | ------------------------ method in this implementation
...
153 |     pub fn clear(&mut self) {
    |            ^^^^^

warning: item in documentation is missing backticks
 --> src/brain.rs:6:20
  |
6 | //! - Managing the nexus_brain collection
  |                    ^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
  = note: `-W clippy::doc-markdown` implied by `-W clippy::pedantic`
  = help: to override `-W clippy::pedantic` add `#[allow(clippy::doc_markdown)]`
help: try
  |
6 - //! - Managing the nexus_brain collection
6 + //! - Managing the `nexus_brain` collection
  |

warning: item in documentation is missing backticks
 --> src/brain.rs:7:43
  |
7 | //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, sprint_number)
  |                                           ^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
  |
7 - //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, sprint_number)
7 + //! - Storing vectors with rich metadata (`project_id`, file_path, layer, machine_id, sprint_number)
  |

warning: item in documentation is missing backticks
 --> src/brain.rs:7:55
  |
7 | //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, sprint_number)
  |                                                       ^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
  |
7 - //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, sprint_number)
7 + //! - Storing vectors with rich metadata (project_id, `file_path`, layer, machine_id, sprint_number)
  |

warning: item in documentation is missing backticks
 --> src/brain.rs:7:73
  |
7 | //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, sprint_number)
  |                                                                         ^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
  |
7 - //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, sprint_number)
7 + //! - Storing vectors with rich metadata (project_id, file_path, layer, `machine_id`, sprint_number)
  |

warning: item in documentation is missing backticks
 --> src/brain.rs:7:85
  |
7 | //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, sprint_number)
  |                                                                                     ^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
  |
7 - //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, sprint_number)
7 + //! - Storing vectors with rich metadata (project_id, file_path, layer, machine_id, `sprint_number`)
  |

warning: item in documentation is missing backticks
  --> src/brain.rs:23:34
   |
23 | /// Vector dimension size (using OpenAI ada-002 compatible size)
   |                                  ^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
   |
23 - /// Vector dimension size (using OpenAI ada-002 compatible size)
23 + /// Vector dimension size (using `OpenAI` ada-002 compatible size)
   |

warning: casts from `u32` to `i64` can be expressed infallibly using `From`
   --> src/brain.rs:127:57
    |
127 |             payload.insert("sprint_number".to_string(), (sprint_number as i64).into());
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: an `as` cast can become silently lossy if the types change in the future
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_lossless
    = note: `-W clippy::cast-lossless` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_lossless)]`
help: use `i64::from` instead
    |
127 -             payload.insert("sprint_number".to_string(), (sprint_number as i64).into());
127 +             payload.insert("sprint_number".to_string(), i64::from(sprint_number).into());
    |

warning: casts from `u32` to `i64` can be expressed infallibly using `From`
   --> src/brain.rs:134:55
    |
134 |             payload.insert("chunk_index".to_string(), (chunk_index as i64).into());
    |                                                       ^^^^^^^^^^^^^^^^^^^^
    |
    = help: an `as` cast can become silently lossy if the types change in the future
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_lossless
help: use `i64::from` instead
    |
134 -             payload.insert("chunk_index".to_string(), (chunk_index as i64).into());
134 +             payload.insert("chunk_index".to_string(), i64::from(chunk_index).into());
    |

warning: you should put bare URLs between `<`/`>` or make a proper Markdown link
   --> src/brain.rs:153:61
    |
153 |     /// * `url` - The gRPC URL of the Qdrant server (e.g., "http://100.x.x.x:6334")
    |                                                             ^^^^^^^^^^^^^^^^^^^^^ help: try: `<http://100.x.x.x:6334>`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown

warning: item in documentation is missing backticks
   --> src/brain.rs:172:20
    |
172 |     /// Ensure the nexus_brain collection exists with the correct schema
    |                    ^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
172 -     /// Ensure the nexus_brain collection exists with the correct schema
172 +     /// Ensure the `nexus_brain` collection exists with the correct schema
    |

warning: casting `u64` to `usize` may truncate the value on targets with 32-bit wide pointers
   --> src/brain.rs:285:28
    |
285 |         if vector.len() != VECTOR_SIZE as usize {
    |                            ^^^^^^^^^^^^^^^^^^^^
    |
    = help: if this is intentional allow the lint with `#[allow(clippy::cast_possible_truncation)]` ...
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_possible_truncation
    = note: `-W clippy::cast-possible-truncation` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_possible_truncation)]`
help: ... or use `try_from` and handle the error accordingly
    |
285 -         if vector.len() != VECTOR_SIZE as usize {
285 +         if vector.len() != usize::try_from(VECTOR_SIZE) {
    |

warning: item in documentation is missing backticks
   --> src/brain.rs:305:59
    |
305 |     /// By default, this restricts results to the current project_id.
    |                                                           ^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
305 -     /// By default, this restricts results to the current project_id.
305 +     /// By default, this restricts results to the current `project_id`.
    |

warning: item in documentation is missing backticks
   --> src/brain.rs:311:67
    |
311 |     /// * `project_id` - Project ID filter (required unless using global_search)
    |                                                                   ^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
311 -     /// * `project_id` - Project ID filter (required unless using global_search)
311 +     /// * `project_id` - Project ID filter (required unless using `global_search`)
    |

warning: item in documentation is missing backticks
   --> src/brain.rs:312:66
    |
312 |     /// * `layers` - Optional layer filter (e.g., Architecture + GlobalStandard)
    |                                                                  ^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
312 -     /// * `layers` - Optional layer filter (e.g., Architecture + GlobalStandard)
312 +     /// * `layers` - Optional layer filter (e.g., Architecture + `GlobalStandard`)
    |

warning: item in documentation is missing backticks
   --> src/brain.rs:364:51
    |
364 |     /// Global search across all projects (bypass project_id filter)
    |                                                   ^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
364 -     /// Global search across all projects (bypass project_id filter)
364 +     /// Global search across all projects (bypass `project_id` filter)
    |

warning: item in documentation is missing backticks
   --> src/brain.rs:413:20
    |
413 |     /// Filters by ProjectArchitecture and GlobalStandard layers.
    |                    ^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
413 -     /// Filters by ProjectArchitecture and GlobalStandard layers.
413 +     /// Filters by `ProjectArchitecture` and GlobalStandard layers.
    |

warning: item in documentation is missing backticks
   --> src/brain.rs:413:44
    |
413 |     /// Filters by ProjectArchitecture and GlobalStandard layers.
    |                                            ^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
413 -     /// Filters by ProjectArchitecture and GlobalStandard layers.
413 +     /// Filters by ProjectArchitecture and `GlobalStandard` layers.
    |

warning: item in documentation is missing backticks
   --> src/brain.rs:449:29
    |
449 |     /// Convert from Qdrant ScoredPoint
    |                             ^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
449 -     /// Convert from Qdrant ScoredPoint
449 +     /// Convert from Qdrant `ScoredPoint`
    |

warning: casting `i64` to `u32` may truncate the value
   --> src/brain.rs:472:79
    |
472 |             Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)) => Some(*i as u32),
    |                                                                               ^^^^^^^^^
    |
    = help: if this is intentional allow the lint with `#[allow(clippy::cast_possible_truncation)]` ...
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_possible_truncation
help: ... or use `try_from` and handle the error accordingly
    |
472 -             Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)) => Some(*i as u32),
472 +             Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)) => Some(u32::try_from(*i)),
    |

warning: casting `i64` to `u32` may lose the sign of the value
   --> src/brain.rs:472:79
    |
472 |             Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)) => Some(*i as u32),
    |                                                                               ^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_sign_loss
    = note: `-W clippy::cast-sign-loss` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_sign_loss)]`

warning: variables can be used directly in the `format!` string
   --> src/brain.rs:497:26
    |
497 |                 .map(|i| format!(" (chunk {})", i))
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
    = note: `-W clippy::uninlined-format-args` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`
help: change this to
    |
497 -                 .map(|i| format!(" (chunk {})", i))
497 +                 .map(|i| format!(" (chunk {i})"))
    |

warning: casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide)
   --> src/brain.rs:544:29
    |
544 |         format!("{:.2} GB", bytes as f64 / GB as f64)
    |                             ^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_precision_loss
    = note: `-W clippy::cast-precision-loss` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::cast_precision_loss)]`

warning: casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide)
   --> src/brain.rs:544:44
    |
544 |         format!("{:.2} GB", bytes as f64 / GB as f64)
    |                                            ^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_precision_loss

warning: casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide)
   --> src/brain.rs:546:29
    |
546 |         format!("{:.2} MB", bytes as f64 / MB as f64)
    |                             ^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_precision_loss

warning: casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide)
   --> src/brain.rs:546:44
    |
546 |         format!("{:.2} MB", bytes as f64 / MB as f64)
    |                                            ^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_precision_loss

warning: casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide)
   --> src/brain.rs:548:29
    |
548 |         format!("{:.2} KB", bytes as f64 / KB as f64)
    |                             ^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_precision_loss

warning: casting `u64` to `f64` causes a loss of precision (`u64` is 64 bits wide, but `f64`'s mantissa is only 52 bits wide)
   --> src/brain.rs:548:44
    |
548 |         format!("{:.2} KB", bytes as f64 / KB as f64)
    |                                            ^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_precision_loss

warning: variables can be used directly in the `format!` string
   --> src/brain.rs:550:9
    |
550 |         format!("{} bytes", bytes)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
550 -         format!("{} bytes", bytes)
550 +         format!("{bytes} bytes")
    |

warning: item in documentation is missing backticks
 --> src/catalyst/mod.rs:4:46
  |
4 | /// from a user's vision document (01) using DeepSeek R1 with sequential thinking.
  |                                              ^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
  |
4 - /// from a user's vision document (01) using DeepSeek R1 with sequential thinking.
4 + /// from a user's vision document (01) using `DeepSeek` R1 with sequential thinking.
  |

warning: this function's return value is unnecessarily wrapped by `Result`
   --> src/catalyst/engine.rs:191:5
    |
191 |     pub fn new(project_id: String, obsidian_path: PathBuf, llm_client: LlmClient) -> Result<Self> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnecessary_wraps
    = note: `-W clippy::unnecessary-wraps` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::unnecessary_wraps)]`
help: remove `Result` from the return type...
    |
191 -     pub fn new(project_id: String, obsidian_path: PathBuf, llm_client: LlmClient) -> Result<Self> {
191 +     pub fn new(project_id: String, obsidian_path: PathBuf, llm_client: LlmClient) -> catalyst::engine::CatalystEngine {
    |
help: ...and then remove the surrounding `Ok()` from returning expressions
    |
192 ~         Self {
193 +             project_id,
194 +             obsidian_path,
195 +             llm_client,
196 +         }
    |

warning: matching over `()` is more explicit
   --> src/catalyst/engine.rs:460:16
    |
460 |             Ok(_) => {
    |                ^ help: use `()` instead of `_`: `()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#ignored_unit_patterns
    = note: `-W clippy::ignored-unit-patterns` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::ignored_unit_patterns)]`

warning: variables can be used directly in the `format!` string
   --> src/catalyst/engine.rs:466:32
    |
466 |                 println!("{}", format!("✗ Failed: {}", e).red());
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
466 -                 println!("{}", format!("✗ Failed: {}", e).red());
466 +                 println!("{}", format!("✗ Failed: {e}").red());
    |

warning: matching over `()` is more explicit
   --> src/catalyst/engine.rs:474:16
    |
474 |             Ok(_) => {
    |                ^ help: use `()` instead of `_`: `()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#ignored_unit_patterns

warning: variables can be used directly in the `format!` string
   --> src/catalyst/engine.rs:480:32
    |
480 |                 println!("{}", format!("✗ Failed: {}", e).red());
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
480 -                 println!("{}", format!("✗ Failed: {}", e).red());
480 +                 println!("{}", format!("✗ Failed: {e}").red());
    |

warning: matching over `()` is more explicit
   --> src/catalyst/engine.rs:488:16
    |
488 |             Ok(_) => {
    |                ^ help: use `()` instead of `_`: `()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#ignored_unit_patterns

warning: variables can be used directly in the `format!` string
   --> src/catalyst/engine.rs:494:32
    |
494 |                 println!("{}", format!("✗ Failed: {}", e).red());
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
494 -                 println!("{}", format!("✗ Failed: {}", e).red());
494 +                 println!("{}", format!("✗ Failed: {e}").red());
    |

warning: matching over `()` is more explicit
   --> src/catalyst/engine.rs:502:16
    |
502 |             Ok(_) => {
    |                ^ help: use `()` instead of `_`: `()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#ignored_unit_patterns

warning: variables can be used directly in the `format!` string
   --> src/catalyst/engine.rs:508:32
    |
508 |                 println!("{}", format!("✗ Failed: {}", e).red());
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
508 -                 println!("{}", format!("✗ Failed: {}", e).red());
508 +                 println!("{}", format!("✗ Failed: {e}").red());
    |

warning: these match arms have identical bodies
   --> src/catalyst/engine.rs:657:25
    |
657 |                         Ok(false) => status.mark_needs_refinement(*doc_type),
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
658 |                         Err(_) => status.mark_needs_refinement(*doc_type),
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: if this is unintentional make the arms return different values
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#match_same_arms
    = note: `-W clippy::match-same-arms` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::match_same_arms)]`
help: otherwise merge the patterns into a single arm
    |
657 ~                         Ok(false) | Err(_) => status.mark_needs_refinement(*doc_type),
658 ~                         }
    |

warning: this function's return value is unnecessarily wrapped by `Result`
   --> src/catalyst/engine.rs:814:1
    |
814 | fn parse_vision_document(content: &str) -> Result<VisionData> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnecessary_wraps
help: remove `Result` from the return type...
    |
814 - fn parse_vision_document(content: &str) -> Result<VisionData> {
814 + fn parse_vision_document(content: &str) -> catalyst::generator::VisionData {
    |
help: ...and then remove the surrounding `Ok()` from returning expressions
    |
817 ~     VisionData {
818 +         problem: sections
819 +             .get("My problem (personal):")
820 +             .cloned()
821 +             .unwrap_or_default(),
822 +         solution: sections
823 +             .get("Solution in ONE SENTENCE:")
824 +             .cloned()
825 +             .unwrap_or_default(),
826 +         success_criteria: sections
827 +             .get("Success criteria (3 months):")
828 +             .cloned()
829 +             .unwrap_or_default(),
830 +         anti_vision: sections
831 +             .get("Anti-vision (what this project is NOT):")
832 +             .cloned()
833 +             .unwrap_or_default(),
834 +     }
    |

warning: this function's return value is unnecessarily wrapped by `Result`
   --> src/catalyst/engine.rs:838:1
    |
838 | fn parse_scope_document(content: &str) -> Result<crate::catalyst::generator::ScopeData> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnecessary_wraps
help: remove `Result` from the return type...
    |
838 - fn parse_scope_document(content: &str) -> Result<crate::catalyst::generator::ScopeData> {
838 + fn parse_scope_document(content: &str) -> catalyst::generator::ScopeData {
    |
help: ...and then remove the surrounding `Ok()` from returning expressions
    |
862 ~     crate::catalyst::generator::ScopeData {
863 +         mvp_features,
864 +         version2_features,
865 +         never_features,
866 +         constraints: sections
867 +             .get("Tech constraints:")
868 +             .cloned()
869 +             .unwrap_or_default(),
870 +     }
    |

warning: variables can be used directly in the `format!` string
   --> src/catalyst/engine.rs:904:42
    |
904 |                     current_content.push(format!("`{}`", code));
    |                                          ^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
904 -                     current_content.push(format!("`{}`", code));
904 +                     current_content.push(format!("`{code}`"));
    |

warning: redundant closure
   --> src/catalyst/engine.rs:937:58
    |
937 |             } else if trimmed.chars().next().is_some_and(|c| c.is_numeric()) {
    |                                                          ^^^^^^^^^^^^^^^^^^ help: replace the closure with the method itself: `char::is_numeric`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#redundant_closure_for_method_calls
    = note: `-W clippy::redundant-closure-for-method-calls` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::redundant_closure_for_method_calls)]`

warning: item in documentation is missing backticks
   --> src/catalyst/engine.rs:973:27
    |
973 | /// Reasoning models like DeepSeek R1 often wrap their thinking in <think> tags.
    |                           ^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
973 - /// Reasoning models like DeepSeek R1 often wrap their thinking in <think> tags.
973 + /// Reasoning models like `DeepSeek` R1 often wrap their thinking in <think> tags.
    |

warning: item in documentation is missing backticks
   --> src/catalyst/engine.rs:976:34
    |
976 | /// Returns: (Option<reasoning>, final_answer)
    |                                  ^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
976 - /// Returns: (Option<reasoning>, final_answer)
976 + /// Returns: (Option<reasoning>, `final_answer`)
    |

warning: this function's return value is unnecessarily wrapped by `Result`
    --> src/catalyst/engine.rs:1005:1
     |
1005 | fn parse_tech_stack_document(content: &str) -> Result<crate::catalyst::generator::TechStackData> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
1005 - fn parse_tech_stack_document(content: &str) -> Result<crate::catalyst::generator::TechStackData> {
1005 + fn parse_tech_stack_document(content: &str) -> catalyst::generator::TechStackData {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
1008 ~     crate::catalyst::generator::TechStackData {
1009 +         language: sections
1010 +             .get("Language:")
1011 +             .cloned()
1012 +             .unwrap_or_default(),
1013 +         framework: sections
1014 +             .get("Framework/Library:")
1015 +             .cloned()
1016 +             .unwrap_or_default(),
1017 +         database: sections.get("Database (if needed):").cloned(),
1018 +         justification: sections
1019 +             .get("Justification:")
1020 +             .cloned()
1021 +             .unwrap_or_default(),
1022 +     }
     |

warning: this function's return value is unnecessarily wrapped by `Result`
    --> src/catalyst/engine.rs:1026:1
     |
1026 | / fn parse_architecture_document(
1027 | |     content: &str,
1028 | | ) -> Result<crate::catalyst::generator::ArchitectureData> {
     | |_________________________________________________________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnecessary_wraps
help: remove `Result` from the return type...
     |
1028 - ) -> Result<crate::catalyst::generator::ArchitectureData> {
1028 + ) -> catalyst::generator::ArchitectureData {
     |
help: ...and then remove the surrounding `Ok()` from returning expressions
     |
1031 ~     crate::catalyst::generator::ArchitectureData {
1032 +         folder_structure: sections
1033 +             .get("Folder structure:")
1034 +             .cloned()
1035 +             .unwrap_or_default(),
1036 +         data_model: sections
1037 +             .get("Data model (main entities):")
1038 +             .cloned()
1039 +             .unwrap_or_default(),
1040 +         user_flow: sections
1041 +             .get("Flow (user journey):")
1042 +             .cloned()
1043 +             .unwrap_or_default(),
1044 +     }
     |

warning: this argument (1 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
  --> src/catalyst/generator.rs:20:21
   |
20 |     pub fn filename(&self) -> &'static str {
   |                     ^^^^^ help: consider passing by value instead: `self`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#trivially_copy_pass_by_ref
   = note: `-W clippy::trivially-copy-pass-by-ref` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::trivially_copy_pass_by_ref)]`

warning: this argument (1 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
  --> src/catalyst/generator.rs:30:25
   |
30 |     pub fn display_name(&self) -> &'static str {
   |                         ^^^^^ help: consider passing by value instead: `self`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#trivially_copy_pass_by_ref

warning: item in documentation is missing backticks
  --> src/catalyst/generator.rs:54:25
   |
54 |     /// Create an empty VisionData
   |                         ^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
   |
54 -     /// Create an empty VisionData
54 +     /// Create an empty `VisionData`
   |

warning: variables can be used directly in the `format!` string
  --> src/catalyst/prompts.rs:99:30
   |
99 |                     .map(|f| format!("- {}", f))
   |                              ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
99 -                     .map(|f| format!("- {}", f))
99 +                     .map(|f| format!("- {f}"))
   |

warning: variables can be used directly in the `format!` string
   --> src/catalyst/prompts.rs:105:30
    |
105 |                     .map(|f| format!("- {}", f))
    |                              ^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
105 -                     .map(|f| format!("- {}", f))
105 +                     .map(|f| format!("- {f}"))
    |

warning: variables can be used directly in the `format!` string
   --> src/catalyst/prompts.rs:111:30
    |
111 |                     .map(|f| format!("- {}", f))
    |                              ^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
111 -                     .map(|f| format!("- {}", f))
111 +                     .map(|f| format!("- {f}"))
    |

warning: variables can be used directly in the `format!` string
   --> src/catalyst/prompts.rs:202:30
    |
202 |                     .map(|f| format!("- {}", f))
    |                              ^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
202 -                     .map(|f| format!("- {}", f))
202 +                     .map(|f| format!("- {f}"))
    |

warning: variables can be used directly in the `format!` string
   --> src/catalyst/prompts.rs:308:30
    |
308 |                     .map(|f| format!("- {}", f))
    |                              ^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
308 -                     .map(|f| format!("- {}", f))
308 +                     .map(|f| format!("- {f}"))
    |

warning: variables can be used directly in the `format!` string
  --> src/catalyst/validation.rs:36:9
   |
36 |         eprintln!("Validation failed for {:?}:", doc_type);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
36 -         eprintln!("Validation failed for {:?}:", doc_type);
36 +         eprintln!("Validation failed for {doc_type:?}:");
   |

warning: variables can be used directly in the `format!` string
  --> src/catalyst/validation.rs:38:13
   |
38 |             eprintln!("  - {:?}", issue);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
38 -             eprintln!("  - {:?}", issue);
38 +             eprintln!("  - {issue:?}");
   |

warning: called `map(<f>).unwrap_or(false)` on an `Option` value
  --> src/commands/gate.rs:86:23
   |
86 |       let is_unlocked = config
   |  _______________________^
87 | |         .state
88 | |         .as_ref()
89 | |         .map(|s| s.is_unlocked)
90 | |         .unwrap_or(false);
   | |_________________________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#map_unwrap_or
   = note: `-W clippy::map-unwrap-or` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::map_unwrap_or)]`
help: use `is_some_and(<f>)` instead
   |
89 -         .map(|s| s.is_unlocked)
89 +         .is_some_and(|s| s.is_unlocked);
   |

warning: this function has too many lines (259/100)
   --> src/commands/gate.rs:131:1
    |
131 | / fn validate_planning_phase(
132 | |     vault_path: &Path,
133 | |     config: &NexusConfig,
134 | |     heuristics: &crate::heuristics::GateHeuristics,
135 | | ) -> Result<bool> {
    | |_________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#too_many_lines
    = note: `-W clippy::too-many-lines` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::too_many_lines)]`

warning: unnecessary boolean `not` operation
   --> src/commands/gate.rs:145:5
    |
145 | /     if !dashboard_path.exists() {
146 | |         all_passed = false;
147 | |         println!(
148 | |             "  {} Dashboard file not found: {}",
...   |
197 | |     }
    | |_____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#if_not_else
    = note: `-W clippy::if-not-else` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::if_not_else)]`
help: try
    |
145 ~     if dashboard_path.exists() {
146 +         // DEFENSIVE: Check dashboard file permissions
147 +         match dashboard_path.metadata() {
148 +             Ok(_) => match validate_dashboard_checkboxes(&dashboard_path) {
149 +                 Ok(result) => {
150 +                     if result.passed {
151 +                         println!(
152 +                             "  {} Dashboard clean - all tasks completed",
153 ~                             "✓".green().bold()
154 +                         );
155 +                     } else {
156 +                         all_passed = false;
157 ~                         println!("  {} Dashboard has unchecked items:", "✗".red().bold());
158 +                         print_validation_issues(&result.issues, &dashboard_path);
159 +                     }
160 +                 }
161 +                 Err(e) => {
162 +                     all_passed = false;
163 +                     let error_msg = if e.to_string().contains("invalid utf-8")
164 +                         || e.to_string().contains("stream did not contain valid UTF-8")
165 +                     {
166 +                         "File contains invalid UTF-8 or binary data".to_string()
167 +                     } else {
168 +                         e.to_string()
169 +                     };
170 +                     println!(
171 +                         "  {} Failed to read dashboard: {}",
172 ~                         "✗".red().bold(),
173 +                         error_msg
174 +                     );
175 +                 }
176 +             },
177 +             Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
178 +                 all_passed = false;
179 ~                 println!("  {} Dashboard file permission denied", "✗".red().bold());
180 +             }
181 +             Err(e) => {
182 +                 all_passed = false;
183 ~                 println!("  {} Cannot access dashboard: {}", "✗".red().bold(), e);
184 +             }
185 +         }
186 +     } else {
187 +         all_passed = false;
188 +         println!(
189 +             "  {} Dashboard file not found: {}",
190 ~             "✗".red().bold(),
191 +             dashboard_path.display()
192 +         );
193 +         println!(
194 +             "     Expected file: 00-START-HERE.md in {}",
195 +             config.structure.management_dir
196 +         );
197 +     }
    |

warning: unnecessary boolean `not` operation
   --> src/commands/gate.rs:205:5
    |
205 | /     if !planning_dir.exists() {
206 | |         all_passed = false;
207 | |         println!(
208 | |             "  {} Planning directory not found: {}",
...   |
425 | |     }
    | |_____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#if_not_else
help: try
    |
205 ~     if planning_dir.exists() {
206 +         // Use heuristics for validation parameters
207 +         let min_word_count = heuristics.min_section_length as usize;
208 +         let illegal_strings: Vec<String> = heuristics.illegal_strings.clone();
209 + 
210 +         // File-specific validation rules for structured projects
211 +         // Only use specific file rules if heuristics has required headers
212 +         let use_specific_file_rules = !heuristics.required_headers.is_empty();
213 + 
214 +         let file_rules: Vec<(&str, Vec<String>)> = if use_specific_file_rules {
215 +             vec![
216 +                 (
217 +                     "01-Problem-and-Vision.md",
218 +                     vec!["Problem".to_string(), "Vision".to_string()],
219 +                 ),
220 +                 (
221 +                     "02-Scope-and-Boundaries.md",
222 +                     vec!["Scope".to_string(), "Boundaries".to_string()],
223 +                 ),
224 +                 (
225 +                     "03-Tech-Stack.md",
226 +                     vec!["Tech Stack".to_string()],
227 +                 ),
228 +                 (
229 +                     "04-Architecture.md",
230 +                     vec!["Architecture".to_string()],
231 +                 ),
232 +             ]
233 +         } else {
234 +             vec![] // No specific file rules if heuristics doesn't require headers
235 +         };
236 + 
237 +         // Check if structured files exist - if any exist, require all
238 +         let structured_files_exist = use_specific_file_rules && file_rules.iter().any(|(name, _)| {
239 +             planning_dir.join(name).exists()
240 +         });
241 + 
242 +         if structured_files_exist {
243 +             // Validate structured planning files
244 +             for (file_name, required_headers) in file_rules {
245 +                 let file_path = planning_dir.join(file_name);
246 + 
247 +                 if !file_path.exists() {
248 +                     all_passed = false;
249 +                     println!(
250 +                         "  {} {} - File not found",
251 ~                         "✗".red().bold(),
252 +                         file_name
253 +                     );
254 +                     continue;
255 +                 }
256 + 
257 +                 // DEFENSIVE: Check for symlink loops and permissions
258 +                 match file_path.metadata() {
259 +                     Ok(metadata) => {
260 +                         // Check file size to warn about very large files
261 +                         if metadata.len() > 100_000_000 {
262 +                             // 100MB
263 +                             println!(
264 +                                 "  {} {} - File very large ({}MB), may take time to process",
265 ~                                 "⚠".yellow().bold(),
266 +                                 file_name,
267 +                                 metadata.len() / 1_000_000
268 +                             );
269 +                         }
270 +                     }
271 +                     Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
272 +                         all_passed = false;
273 ~                         println!("  {} {} - Permission denied", "✗".red().bold(), file_name);
274 +                         continue;
275 +                     }
276 +                     Err(e) => {
277 +                         all_passed = false;
278 +                         println!(
279 +                             "  {} {} - Cannot access file: {}",
280 ~                             "✗".red().bold(),
281 +                             file_name,
282 +                             e
283 +                         );
284 +                         continue;
285 +                     }
286 +                 }
287 + 
288 +                 // Validate with specific headers for this file
289 +                 match validate_planning_document_with_headers(
290 +                     &file_path,
291 +                     &required_headers,
292 +                     min_word_count,
293 +                     &illegal_strings,
294 +                 ) {
295 +                     Ok(result) => {
296 +                         if result.passed {
297 ~                             println!("  {} {}", "✓".green().bold(), file_name);
298 +                         } else {
299 +                             all_passed = false;
300 ~                             println!("  {} {}", "✗".red().bold(), file_name);
301 +                             print_validation_issues(&result.issues, &file_path);
302 +                         }
303 +                     }
304 +                     Err(e) => {
305 +                         all_passed = false;
306 +                         let error_msg = if e.to_string().contains("invalid utf-8")
307 +                             || e.to_string().contains("stream did not contain valid UTF-8")
308 +                         {
309 +                             "File contains invalid UTF-8 or binary data".to_string()
310 +                         } else if e.to_string().contains("Permission denied") {
311 +                             "Permission denied".to_string()
312 +                         } else {
313 +                             e.to_string()
314 +                         };
315 ~                         println!("  {} {} - {}", "✗".red().bold(), file_name, error_msg);
316 +                     }
317 +                 }
318 +             }
319 +         } else {
320 +             // Fallback: validate any .md files found (backward compatibility)
321 +             let planning_files = std::fs::read_dir(&planning_dir)
322 +                 .with_context(|| {
323 +                     format!(
324 +                         "Failed to read planning directory: {}",
325 +                         planning_dir.display()
326 +                     )
327 +                 })?
328 +                 .filter_map(|entry| entry.ok())
329 +                 .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("md"))
330 +                 .collect::<Vec<_>>();
331 + 
332 +             if planning_files.is_empty() {
333 +                 all_passed = false;
334 +                 println!(
335 +                     "  {} No planning documents found in {}",
336 ~                     "✗".red().bold(),
337 +                     planning_dir.display()
338 +                 );
339 +             } else {
340 +                 // Use generic validation with all required headers from heuristics
341 +                 for entry in planning_files {
342 +                     let file_path = entry.path();
343 +                     let file_name = match file_path.file_name() {
344 +                         Some(name) => name.to_string_lossy().to_string(),
345 +                         None => {
346 +                             all_passed = false;
347 +                             println!(
348 +                                 "  {} Skipping file with invalid path: {}",
349 ~                                 "⚠".yellow().bold(),
350 +                                 file_path.display()
351 +                             );
352 +                             continue;
353 +                         }
354 +                     };
355 + 
356 +                     // DEFENSIVE: Check for symlink loops before reading
357 +                     match file_path.metadata() {
358 +                         Ok(metadata) => {
359 +                             if metadata.len() > 100_000_000 {
360 +                                 println!(
361 +                                     "  {} {} - File very large ({}MB), may take time to process",
362 ~                                     "⚠".yellow().bold(),
363 +                                     file_name,
364 +                                     metadata.len() / 1_000_000
365 +                                 );
366 +                             }
367 +                         }
368 +                         Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
369 +                             all_passed = false;
370 ~                             println!("  {} {} - Permission denied", "✗".red().bold(), file_name);
371 +                             continue;
372 +                         }
373 +                         Err(e) => {
374 +                             all_passed = false;
375 +                             println!(
376 +                                 "  {} {} - Cannot access file: {}",
377 ~                                 "✗".red().bold(),
378 +                                 file_name,
379 +                                 e
380 +                             );
381 +                             continue;
382 +                         }
383 +                     }
384 + 
385 +                     // Use generic validation with heuristics
386 +                     match validate_planning_document_with_headers(
387 +                         &file_path,
388 +                         &heuristics.required_headers,
389 +                         min_word_count,
390 +                         &illegal_strings,
391 +                     ) {
392 +                         Ok(result) => {
393 +                             if result.passed {
394 ~                                 println!("  {} {}", "✓".green().bold(), file_name);
395 +                             } else {
396 +                                 all_passed = false;
397 ~                                 println!("  {} {}", "✗".red().bold(), file_name);
398 +                                 print_validation_issues(&result.issues, &file_path);
399 +                             }
400 +                         }
401 +                         Err(e) => {
402 +                             all_passed = false;
403 +                             let error_msg = if e.to_string().contains("invalid utf-8")
404 +                                 || e.to_string().contains("stream did not contain valid UTF-8")
405 +                             {
406 +                                 "File contains invalid UTF-8 or binary data".to_string()
407 +                             } else if e.to_string().contains("Permission denied") {
408 +                                 "Permission denied".to_string()
409 +                             } else {
410 +                                 e.to_string()
411 +                             };
412 ~                             println!("  {} {} - {}", "✗".red().bold(), file_name, error_msg);
413 +                         }
414 +                     }
415 +                 }
416 +             }
417 +         }
418 +     } else {
419 +         all_passed = false;
420 +         println!(
421 +             "  {} Planning directory not found: {}",
422 ~             "✗".red().bold(),
423 +             planning_dir.display()
424 +         );
425 +     }
    |

warning: redundant closure
   --> src/commands/gate.rs:335:29
    |
335 |                 .filter_map(|entry| entry.ok())
    |                             ^^^^^^^^^^^^^^^^^^ help: replace the closure with the method itself: `std::result::Result::ok`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#redundant_closure_for_method_calls

warning: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
   --> src/commands/gate.rs:350:37
    |
350 |                       let file_name = match file_path.file_name() {
    |  _____________________________________^
351 | |                         Some(name) => name.to_string_lossy().to_string(),
352 | |                         None => {
353 | |                             all_passed = false;
...   |
361 | |                     };
    | |_____________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#single_match_else
    = note: `-W clippy::single-match-else` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::single_match_else)]`
help: try
    |
350 ~                     let file_name = if let Some(name) = file_path.file_name() { name.to_string_lossy().to_string() } else {
351 +                         all_passed = false;
352 +                         println!(
353 +                             "  {} Skipping file with invalid path: {}",
354 ~                             "⚠".yellow().bold(),
355 +                             file_path.display()
356 +                         );
357 +                         continue;
358 ~                     };
    |

warning: unnecessary boolean `not` operation
   --> src/commands/gate.rs:464:5
    |
464 | /     if !tasks_path.exists() {
465 | |         all_passed = false;
466 | |         println!(
467 | |             "  {} Tasks.md not found in sprint folder",
...   |
496 | |     }
    | |_____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#if_not_else
help: try
    |
464 ~     if tasks_path.exists() {
465 +         match validate_dashboard_checkboxes(&tasks_path) {
466 +             Ok(result) => {
467 +                 if result.passed {
468 +                     println!(
469 +                         "  {} Tasks.md - All tasks completed",
470 ~                         "✓".green().bold()
471 +                     );
472 +                 } else {
473 +                     all_passed = false;
474 +                     println!(
475 +                         "  {} Tasks.md - Has unchecked items:",
476 ~                         "✗".red().bold()
477 +                     );
478 +                     print_validation_issues(&result.issues, &tasks_path);
479 +                 }
480 +             }
481 +             Err(e) => {
482 +                 all_passed = false;
483 +                 println!(
484 +                     "  {} Failed to read Tasks.md: {}",
485 ~                     "✗".red().bold(),
486 +                     e
487 +                 );
488 +             }
489 +         }
490 +     } else {
491 +         all_passed = false;
492 +         println!(
493 +             "  {} Tasks.md not found in sprint folder",
494 ~             "✗".red().bold()
495 +         );
496 +     }
    |

warning: unnecessary boolean `not` operation
   --> src/commands/gate.rs:500:5
    |
500 | /     if !context_path.exists() {
501 | |         all_passed = false;
502 | |         println!(
503 | |             "  {} Sprint-Context.md not found in sprint folder",
...   |
533 | |     }
    | |_____^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#if_not_else
help: try
    |
500 ~     if context_path.exists() {
501 +         match std::fs::read_to_string(&context_path) {
502 +             Ok(content) => {
503 +                 let trimmed = content.trim();
504 +                 if trimmed.is_empty() {
505 +                     all_passed = false;
506 +                     println!(
507 +                         "  {} Sprint-Context.md is empty",
508 ~                         "✗".red().bold()
509 +                     );
510 +                 } else {
511 +                     println!(
512 +                         "  {} Sprint-Context.md - Contains content ({} bytes)",
513 ~                         "✓".green().bold(),
514 +                         trimmed.len()
515 +                     );
516 +                 }
517 +             }
518 +             Err(e) => {
519 +                 all_passed = false;
520 +                 println!(
521 +                     "  {} Failed to read Sprint-Context.md: {}",
522 ~                     "✗".red().bold(),
523 +                     e
524 +                 );
525 +             }
526 +         }
527 +     } else {
528 +         all_passed = false;
529 +         println!(
530 +             "  {} Sprint-Context.md not found in sprint folder",
531 ~             "✗".red().bold()
532 +         );
533 +     }
    |

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:15:24
   |
15 |         .ok_or_else(|| format!("Invalid project path: {}", project_name))?
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
15 -         .ok_or_else(|| format!("Invalid project path: {}", project_name))?
15 +         .ok_or_else(|| format!("Invalid project path: {project_name}"))?
   |

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:20:20
   |
20 |           return Err(format!(
   |  ____________________^
21 | |             "Error: Project folder '{}' already exists. Please choose a different name or remove the existing folder.",
22 | |             project_name
23 | |         ));
   | |_________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:28:22
   |
28 |         .map_err(|e| format!("Failed to create project folder '{}': {}", project_name, e))?;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
28 -         .map_err(|e| format!("Failed to create project folder '{}': {}", project_name, e))?;
28 +         .map_err(|e| format!("Failed to create project folder '{project_name}': {e}"))?;
   |

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:30:5
   |
30 |     println!("✓ Created project folder: {}", folder_name);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
30 -     println!("✓ Created project folder: {}", folder_name);
30 +     println!("✓ Created project folder: {folder_name}");
   |

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:46:9
   |
46 |         format!("Failed to copy template files: {}", e)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
46 -         format!("Failed to copy template files: {}", e)
46 +         format!("Failed to copy template files: {e}")
   |

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:54:22
   |
54 |         .map_err(|e| format!("Failed to resolve absolute path: {}", e))?;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
54 -         .map_err(|e| format!("Failed to resolve absolute path: {}", e))?;
54 +         .map_err(|e| format!("Failed to resolve absolute path: {e}"))?;
   |

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:63:22
   |
63 |         .map_err(|e| format!("Failed to serialize config: {}", e))?;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
63 -         .map_err(|e| format!("Failed to serialize config: {}", e))?;
63 +         .map_err(|e| format!("Failed to serialize config: {e}"))?;
   |

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:67:22
   |
67 |         .map_err(|e| format!("Failed to write nexus.toml: {}", e))?;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
67 -         .map_err(|e| format!("Failed to write nexus.toml: {}", e))?;
67 +         .map_err(|e| format!("Failed to write nexus.toml: {e}"))?;
   |

warning: variables can be used directly in the `format!` string
  --> src/commands/init.rs:70:5
   |
70 |     println!("\n✅ Project '{}' initialized successfully!", folder_name);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
70 -     println!("\n✅ Project '{}' initialized successfully!", folder_name);
70 +     println!("\n✅ Project '{folder_name}' initialized successfully!");
   |

warning: variables can be used directly in the `format!` string
  --> src/commands/shell.rs:32:9
   |
32 |         eprintln!("  {}", e);
   |         ^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
32 -         eprintln!("  {}", e);
32 +         eprintln!("  {e}");
   |

warning: this `continue` expression is redundant
   --> src/commands/shell.rs:118:17
    |
118 |                 continue;
    |                 ^^^^^^^^
    |
    = help: consider dropping the `continue` expression
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_continue
    = note: `-W clippy::needless-continue` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::needless_continue)]`

warning: variables can be used directly in the `format!` string
  --> src/commands/shell.rs:72:17
   |
72 |                 format!("nexus:{}{}{}❯", project, watch_indicator, context_indicator)
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
72 -                 format!("nexus:{}{}{}❯", project, watch_indicator, context_indicator)
72 +                 format!("nexus:{project}{watch_indicator}{context_indicator}❯")
   |

warning: this function's return value is unnecessary
   --> src/commands/shell.rs:140:1
    |
140 | fn print_banner(state: &NexusState) -> Result<()> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnecessary_wraps
help: remove the return type...
    |
140 - fn print_banner(state: &NexusState) -> Result<()> {
140 + fn print_banner(state: &NexusState) -> () {
    |
help: ...and then remove returned values
    |
189 -     Ok(())
    |

warning: variables can be used directly in the `format!` string
   --> src/commands/shell.rs:364:9
    |
364 |         format!("Selecting project '{}'...", project_id).dimmed()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
364 -         format!("Selecting project '{}'...", project_id).dimmed()
364 +         format!("Selecting project '{project_id}'...").dimmed()
    |

warning: matching over `()` is more explicit
   --> src/commands/shell.rs:406:12
    |
406 |         Ok(_) => {
    |            ^ help: use `()` instead of `_`: `()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#ignored_unit_patterns

warning: variables can be used directly in the `format!` string
   --> src/commands/shell.rs:445:9
    |
445 |         format!("Activating sprint {}...", sprint_number).dimmed()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
445 -         format!("Activating sprint {}...", sprint_number).dimmed()
445 +         format!("Activating sprint {sprint_number}...").dimmed()
    |

warning: this function has too many lines (125/100)
   --> src/commands/shell.rs:451:1
    |
451 | fn execute_catalyst(state: &NexusState, args: &[&str]) -> Result<()> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#too_many_lines

warning: variables can be used directly in the `format!` string
   --> src/commands/shell.rs:592:17
    |
592 |                 anyhow::bail!("Unknown catalyst command: '{}'. Use 'catalyst help' for usage.", cmd)
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
592 -                 anyhow::bail!("Unknown catalyst command: '{}'. Use 'catalyst help' for usage.", cmd)
592 +                 anyhow::bail!("Unknown catalyst command: '{cmd}'. Use 'catalyst help' for usage.")
    |

warning: variables can be used directly in the `format!` string
   --> src/commands/shell.rs:616:14
    |
616 |           _ => anyhow::bail!(
    |  ______________^
617 | |             "Unknown document type: '{}'. Valid types: scope, stack, arch, mvp",
618 | |             name
619 | |         ),
    | |_________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args

warning: item in documentation is missing backticks
   --> src/commands/shell.rs:964:46
    |
964 | /// - `--arch`: Only search Architecture and GlobalStandard layers
    |                                              ^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
964 - /// - `--arch`: Only search Architecture and GlobalStandard layers
964 + /// - `--arch`: Only search Architecture and `GlobalStandard` layers
    |

warning: this function has too many lines (126/100)
    --> src/commands/shell.rs:1135:1
     |
1135 | fn execute_llm_query(input: &str, state: &NexusState) -> Result<()> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#too_many_lines

warning: unnecessary boolean `not` operation
    --> src/commands/shell.rs:1244:9
     |
1244 | /         if !context.architecture.snippets.is_empty() {
1245 | |             println!(
1246 | |                 "  {} Retrieved {} architecture snippets (score ≥ {})",
1247 | |                 "✓".green(),
...    |
1256 | |             );
1257 | |         }
     | |_________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#if_not_else
help: try
     |
1244 ~         if context.architecture.snippets.is_empty() {
1245 +             println!(
1246 +                 "  {} No architecture snippets above threshold ({})",
1247 ~                 "⚠".yellow(),
1248 +                 RELEVANCE_THRESHOLD
1249 +             );
1250 +         } else {
1251 +             println!(
1252 ~                 "  {} Retrieved {} architecture snippets (score ≥ {})",
1253 ~                 "✓".green(),
1254 +                 context.architecture.snippets.len(),
1255 +                 RELEVANCE_THRESHOLD
1256 +             );
1257 +         }
     |

warning: variables can be used directly in the `format!` string
    --> src/commands/shell.rs:1270:22
     |
1270 |             prompt = format!("{}\n{}", history_context, prompt);
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
     |
1270 -             prompt = format!("{}\n{}", history_context, prompt);
1270 +             prompt = format!("{history_context}\n{prompt}");
     |

warning: variables can be used directly in the `format!` string
    --> src/commands/shell.rs:1291:5
     |
1291 |     println!("{}", result);
     |     ^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
     |
1291 -     println!("{}", result);
1291 +     println!("{result}");
     |

warning: variables can be used directly in the `format!` string
    --> src/commands/shell.rs:1297:9
     |
1297 |         eprintln!("Warning: Failed to save conversation history: {}", e);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
     |
1297 -         eprintln!("Warning: Failed to save conversation history: {}", e);
1297 +         eprintln!("Warning: Failed to save conversation history: {e}");
     |

warning: usage of wildcard import
  --> src/commands/sprint.rs:10:5
   |
10 | use colored::*;
   |     ^^^^^^^^^^ help: try: `colored::Colorize`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#wildcard_imports
   = note: `-W clippy::wildcard-imports` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::wildcard_imports)]`

warning: this function has too many lines (155/100)
  --> src/commands/sprint.rs:23:1
   |
23 | pub fn execute(project_path: &Path, sprint_number: u32) -> Result<()> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#too_many_lines

warning: variables can be used directly in the `format!` string
  --> src/commands/sprint.rs:75:9
   |
75 | /         bail!(
76 | |             "Cannot start Sprint {} until previous sprint is approved. Please complete the current sprint first.",
77 | |             sprint_number
78 | |         );
   | |_________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args

warning: item in documentation is missing backticks
  --> src/commands/unlock.rs:19:25
   |
19 | /// 1. Load config from project_path
   |                         ^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
   |
19 - /// 1. Load config from project_path
19 + /// 1. Load config from `project_path`
   |

warning: matching over `()` is more explicit
  --> src/commands/unlock.rs:49:12
   |
49 |         Ok(_) => {
   |            ^ help: use `()` instead of `_`: `()`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#ignored_unit_patterns

warning: all fields have the same postfix: `dir`
  --> src/config.rs:34:1
   |
34 | / pub struct StructureConfig {
35 | |     pub planning_dir: String,
36 | |     pub management_dir: String,
37 | |     pub sprint_dir: String,
38 | | }
   | |_^
   |
   = help: remove the postfixes
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#struct_field_names
   = note: `-W clippy::struct-field-names` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::struct_field_names)]`

warning: item in documentation is missing backticks
  --> src/config.rs:49:35
   |
49 |     /// If not set, falls back to obsidian_path from [project]
   |                                   ^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
   |
49 -     /// If not set, falls back to obsidian_path from [project]
49 +     /// If not set, falls back to `obsidian_path` from [project]
   |

warning: item in documentation is missing backticks
  --> src/config.rs:64:25
   |
64 |     /// Sprint status: "in_progress" or "approved"
   |                         ^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
   |
64 -     /// Sprint status: "in_progress" or "approved"
64 +     /// Sprint status: "`in_progress`" or "approved"
   |

warning: you should put bare URLs between `<`/`>` or make a proper Markdown link
  --> src/config.rs:75:33
   |
75 |     /// Qdrant gRPC URL (e.g., "http://100.64.0.1:6334" for Tailscale)
   |                                 ^^^^^^^^^^^^^^^^^^^^^^ help: try: `<http://100.64.0.1:6334>`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown

warning: item in documentation is missing backticks
   --> src/config.rs:119:60
    |
119 |     /// Whether to show reasoning process from models like DeepSeek R1
    |                                                            ^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
119 -     /// Whether to show reasoning process from models like DeepSeek R1
119 +     /// Whether to show reasoning process from models like `DeepSeek` R1
    |

warning: item in documentation is missing backticks
   --> src/config.rs:136:22
    |
136 |     /// Create a new NexusConfig with the given project name and obsidian path
    |                      ^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
136 -     /// Create a new NexusConfig with the given project name and obsidian path
136 +     /// Create a new `NexusConfig` with the given project name and obsidian path
    |

warning: this argument is passed by value, but not consumed in the function body
   --> src/config.rs:137:53
    |
137 |     pub fn new(project_name: String, obsidian_path: String) -> Self {
    |                                                     ^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_pass_by_value
    = note: `-W clippy::needless-pass-by-value` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::needless_pass_by_value)]`
help: consider changing the type to
    |
137 -     pub fn new(project_name: String, obsidian_path: String) -> Self {
137 +     pub fn new(project_name: String, obsidian_path: &str) -> Self {
    |
help: change `obsidian_path.clone()` to
    |
142 -                 obsidian_path: obsidian_path.clone(),
142 +                 obsidian_path: obsidian_path.to_string(),
    |

warning: item in documentation is missing backticks
   --> src/config.rs:170:48
    |
170 |     /// Get the planning path, falling back to obsidian_path if not set
    |                                                ^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
170 -     /// Get the planning path, falling back to obsidian_path if not set
170 +     /// Get the planning path, falling back to `obsidian_path` if not set
    |

warning: called `map(<f>).unwrap_or_else(<g>)` on an `Option` value
   --> src/config.rs:172:9
    |
172 | /         self.obsidian
173 | |             .as_ref()
174 | |             .map(|o| o.planning_path.clone())
175 | |             .unwrap_or_else(|| PathBuf::from(&self.project.obsidian_path))
    | |__________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#map_unwrap_or
help: try
    |
172 ~         self.obsidian
173 +             .as_ref().map_or_else(|| PathBuf::from(&self.project.obsidian_path), |o| o.planning_path.clone())
    |

warning: item in documentation is missing backticks
   --> src/config.rs:178:34
    |
178 |     /// Get the vault/repo path (obsidian_path)
    |                                  ^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
178 -     /// Get the vault/repo path (obsidian_path)
178 +     /// Get the vault/repo path (`obsidian_path`)
    |

warning: field name starts with the struct's name
  --> src/context.rs:24:5
   |
24 |     pub sprint_context: String,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#struct_field_names

warning: `format!(..)` appended to existing `String`
  --> src/context.rs:88:17
   |
88 |                 output.push_str(&format!("\n--- Architecture Reference {} ---\n", idx + 1));
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider using `write!` to avoid the extra allocation
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#format_push_string
   = note: `-W clippy::format-push-string` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::format_push_string)]`

warning: `format!(..)` appended to existing `String`
  --> src/context.rs:98:13
   |
98 |             output.push_str(&format!("Sprint: {}\n\n", sprint_id));
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider using `write!` to avoid the extra allocation
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#format_push_string

warning: variables can be used directly in the `format!` string
  --> src/context.rs:98:30
   |
98 |             output.push_str(&format!("Sprint: {}\n\n", sprint_id));
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
98 -             output.push_str(&format!("Sprint: {}\n\n", sprint_id));
98 +             output.push_str(&format!("Sprint: {sprint_id}\n\n"));
   |

warning: item in documentation is missing backticks
   --> src/context.rs:135:5
    |
135 | /// ActiveContext containing architecture and sprint information
    |     ^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
135 - /// ActiveContext containing architecture and sprint information
135 + /// `ActiveContext` containing architecture and sprint information
    |

warning: variables can be used directly in the `format!` string
   --> src/context.rs:169:13
    |
169 |             eprintln!("Warning: Failed to retrieve architecture context: {}", e);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
169 -             eprintln!("Warning: Failed to retrieve architecture context: {}", e);
169 +             eprintln!("Warning: Failed to retrieve architecture context: {e}");
    |

warning: variables can be used directly in the `format!` string
  --> src/embeddings.rs:46:30
   |
46 |             .with_context(|| format!("Failed to load ONNX model from: {}", model_path))?;
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
46 -             .with_context(|| format!("Failed to load ONNX model from: {}", model_path))?;
46 +             .with_context(|| format!("Failed to load ONNX model from: {model_path}"))?;
   |

warning: casts from `u32` to `i64` can be expressed infallibly using `From`
  --> src/embeddings.rs:81:40
   |
81 |             input_ids.iter().map(|&id| id as i64).collect(),
   |                                        ^^^^^^^^^
   |
   = help: an `as` cast can become silently lossy if the types change in the future
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_lossless
help: use `i64::from` instead
   |
81 -             input_ids.iter().map(|&id| id as i64).collect(),
81 +             input_ids.iter().map(|&id| i64::from(id)).collect(),
   |

warning: casting `i64` to `usize` may truncate the value on targets with 32-bit wide pointers
   --> src/embeddings.rs:126:26
    |
126 |         let batch_size = shape_dims[0] as usize;
    |                          ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: if this is intentional allow the lint with `#[allow(clippy::cast_possible_truncation)]` ...
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_possible_truncation
help: ... or use `try_from` and handle the error accordingly
    |
126 -         let batch_size = shape_dims[0] as usize;
126 +         let batch_size = usize::try_from(shape_dims[0]);
    |

warning: casting `i64` to `usize` may lose the sign of the value
   --> src/embeddings.rs:126:26
    |
126 |         let batch_size = shape_dims[0] as usize;
    |                          ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_sign_loss

warning: casting `u32` to `f32` causes a loss of precision (`u32` is 32 bits wide, but `f32`'s mantissa is only 23 bits wide)
   --> src/embeddings.rs:182:27
    |
182 |                 *value /= mask_sum as f32;
    |                           ^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_precision_loss

warning: variables can be used directly in the `format!` string
  --> src/git_ops.rs:34:23
   |
34 |     let branch_name = format!("sprint-{}-{}", sprint_number, sprint_name);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
34 -     let branch_name = format!("sprint-{}-{}", sprint_number, sprint_name);
34 +     let branch_name = format!("sprint-{sprint_number}-{sprint_name}");
   |

warning: variables can be used directly in the `format!` string
  --> src/git_ops.rs:38:9
   |
38 | /         bail!(
39 | |             "Branch '{}' already exists. Please delete it first or use a different sprint.",
40 | |             branch_name
41 | |         );
   | |_________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args

warning: variables can be used directly in the `format!` string
  --> src/git_ops.rs:52:26
   |
52 |         .with_context(|| format!("Failed to create branch '{}'", branch_name))?;
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
52 -         .with_context(|| format!("Failed to create branch '{}'", branch_name))?;
52 +         .with_context(|| format!("Failed to create branch '{branch_name}'"))?;
   |

warning: variables can be used directly in the `format!` string
  --> src/git_ops.rs:97:26
   |
97 |         .with_context(|| format!("Failed to find branch '{}'", branch_name))?;
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
97 -         .with_context(|| format!("Failed to find branch '{}'", branch_name))?;
97 +         .with_context(|| format!("Failed to find branch '{branch_name}'"))?;
   |

warning: variables can be used directly in the `format!` string
   --> src/git_ops.rs:107:26
    |
107 |         .with_context(|| format!("Failed to set HEAD to '{}'", branch_name))?;
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
107 -         .with_context(|| format!("Failed to set HEAD to '{}'", branch_name))?;
107 +         .with_context(|| format!("Failed to set HEAD to '{branch_name}'"))?;
    |

warning: item in documentation is missing backticks
  --> src/history.rs:46:48
   |
46 |     /// Automatically maintains the maximum of MAX_HISTORY_TURNS by removing oldest turns.
   |                                                ^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
   |
46 -     /// Automatically maintains the maximum of MAX_HISTORY_TURNS by removing oldest turns.
46 +     /// Automatically maintains the maximum of `MAX_HISTORY_TURNS` by removing oldest turns.
   |

warning: `format!(..)` appended to existing `String`
  --> src/history.rs:74:13
   |
74 |             context.push_str(&format!("--- Turn {} ---\n", idx + 1));
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider using `write!` to avoid the extra allocation
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#format_push_string

warning: variables can be used directly in the `format!` string
  --> src/llm.rs:91:32
   |
91 |                 let combined = format!("{}\n\n{}", system_prompt, user_prompt);
   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
91 -                 let combined = format!("{}\n\n{}", system_prompt, user_prompt);
91 +                 let combined = format!("{system_prompt}\n\n{user_prompt}");
   |

warning: item in documentation is missing backticks
  --> src/llm.rs:97:46
   |
97 |     /// Send a prompt with system message to OpenRouter API
   |                                              ^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
   |
97 -     /// Send a prompt with system message to OpenRouter API
97 +     /// Send a prompt with system message to `OpenRouter` API
   |

warning: variables can be used directly in the `format!` string
   --> src/llm.rs:164:13
    |
164 |             anyhow::bail!("OpenRouter API error ({}): {}", status, error_text);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
164 -             anyhow::bail!("OpenRouter API error ({}): {}", status, error_text);
164 +             anyhow::bail!("OpenRouter API error ({status}): {error_text}");
    |

warning: item in documentation is missing backticks
   --> src/llm.rs:179:26
    |
179 |     /// Send a prompt to OpenRouter API (OpenAI-compatible format)
    |                          ^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
179 -     /// Send a prompt to OpenRouter API (OpenAI-compatible format)
179 +     /// Send a prompt to `OpenRouter` API (OpenAI-compatible format)
    |

warning: variables can be used directly in the `format!` string
   --> src/llm.rs:236:13
    |
236 |             anyhow::bail!("OpenRouter API error ({}): {}", status, error_text);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
236 -             anyhow::bail!("OpenRouter API error ({}): {}", status, error_text);
236 +             anyhow::bail!("OpenRouter API error ({status}): {error_text}");
    |

warning: variables can be used directly in the `format!` string
   --> src/llm.rs:308:13
    |
308 |             anyhow::bail!("Claude API error ({}): {}", status, error_text);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
308 -             anyhow::bail!("Claude API error ({}): {}", status, error_text);
308 +             anyhow::bail!("Claude API error ({status}): {error_text}");
    |

warning: variables can be used directly in the `format!` string
   --> src/llm.rs:374:13
    |
374 |             anyhow::bail!("Claude API error ({}): {}", status, error_text);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
374 -             anyhow::bail!("Claude API error ({}): {}", status, error_text);
374 +             anyhow::bail!("Claude API error ({status}): {error_text}");
    |

warning: variables can be used directly in the `format!` string
   --> src/llm.rs:454:13
    |
454 |             anyhow::bail!("Gemini API error ({}): {}", status, error_text);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
454 -             anyhow::bail!("Gemini API error ({}): {}", status, error_text);
454 +             anyhow::bail!("Gemini API error ({status}): {error_text}");
    |

warning: implicitly cloning a `String` by calling `to_string` on its dereferenced type
   --> src/planning.rs:330:9
    |
330 |         placeholder_upper.to_string(),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `placeholder_upper.clone()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#implicit_clone
    = note: `-W clippy::implicit-clone` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::implicit_clone)]`

warning: variables can be used directly in the `format!` string
   --> src/planning.rs:331:9
    |
331 |         format!("{}:", placeholder_upper),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
331 -         format!("{}:", placeholder_upper),
331 +         format!("{placeholder_upper}:"),
    |

warning: variables can be used directly in the `format!` string
   --> src/planning.rs:332:9
    |
332 |         format!("{} -", placeholder_upper),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
332 -         format!("{} -", placeholder_upper),
332 +         format!("{placeholder_upper} -"),
    |

warning: variables can be used directly in the `format!` string
   --> src/planning.rs:333:9
    |
333 |         format!("[{}]", placeholder_upper),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
333 -         format!("[{}]", placeholder_upper),
333 +         format!("[{placeholder_upper}]"),
    |

warning: variables can be used directly in the `format!` string
   --> src/planning.rs:334:9
    |
334 |         format!("({})", placeholder_upper),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
334 -         format!("({})", placeholder_upper),
334 +         format!("({placeholder_upper})"),
    |

warning: variables can be used directly in the `format!` string
   --> src/planning.rs:335:9
    |
335 |         format!("...{}", placeholder_upper),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
335 -         format!("...{}", placeholder_upper),
335 +         format!("...{placeholder_upper}"),
    |

warning: variables can be used directly in the `format!` string
   --> src/planning.rs:336:9
    |
336 |         format!("{}...", placeholder_upper),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
336 -         format!("{}...", placeholder_upper),
336 +         format!("{placeholder_upper}..."),
    |

warning: item in documentation is missing backticks
   --> src/planning.rs:366:60
    |
366 | /// It detects unchecked checkboxes using pulldown-cmark's TaskListMarker events.
    |                                                            ^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
366 - /// It detects unchecked checkboxes using pulldown-cmark's TaskListMarker events.
366 + /// It detects unchecked checkboxes using pulldown-cmark's `TaskListMarker` events.
    |

warning: item in documentation is missing backticks
   --> src/planning.rs:480:48
    |
480 | /// Extract sections from a markdown file as a HashMap
    |                                                ^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
480 - /// Extract sections from a markdown file as a HashMap
480 + /// Extract sections from a markdown file as a `HashMap`
    |

warning: variables can be used directly in the `format!` string
   --> src/planning.rs:511:42
    |
511 |                     current_content.push(format!("`{}`", code));
    |                                          ^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
511 -                     current_content.push(format!("`{}`", code));
511 +                     current_content.push(format!("`{code}`"));
    |

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:551:13
    |
551 |             context.problem_statement = solution.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.problem_statement.clone_from(solution)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones
    = note: `-W clippy::assigning-clones` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::assigning_clones)]`

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:554:13
    |
554 |             context.vision = success.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.vision.clone_from(success)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:557:13
    |
557 |             context.problem_details = problem.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.problem_details.clone_from(problem)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:560:13
    |
560 |             context.anti_scope = anti.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.anti_scope.clone_from(anti)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:572:13
    |
572 |             context.mvp_scope = mvp.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.mvp_scope.clone_from(mvp)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:581:13
    |
581 |             context.tech_constraints = constraints.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.tech_constraints.clone_from(constraints)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:593:13
    |
593 |             context.tech_stack = stack.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.tech_stack.clone_from(stack)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:596:13
    |
596 |             context.stack_justification = why.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.stack_justification.clone_from(why)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:599:13
    |
599 |             context.tech_exclusions = not_use.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.tech_exclusions.clone_from(not_use)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:602:13
    |
602 |             context.dependencies = deps.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.dependencies.clone_from(deps)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:614:13
    |
614 |             context.folder_structure = folder.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.folder_structure.clone_from(folder)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:617:13
    |
617 |             context.data_model = data.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.data_model.clone_from(data)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:620:13
    |
620 |             context.user_flow = flow.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.user_flow.clone_from(flow)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/planning.rs:623:13
    |
623 |             context.technical_decisions = decisions.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `context.technical_decisions.clone_from(decisions)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: item in documentation is missing backticks
   --> src/state.rs:101:20
    |
101 |     /// Update the last_updated timestamp
    |                    ^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
101 -     /// Update the last_updated timestamp
101 +     /// Update the `last_updated` timestamp
    |

warning: this function's return value is unnecessarily wrapped by `Result`
  --> src/watcher.rs:54:5
   |
54 |     pub fn new(brain_url: String) -> Result<Self> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnecessary_wraps
help: remove `Result` from the return type...
   |
54 -     pub fn new(brain_url: String) -> Result<Self> {
54 +     pub fn new(brain_url: String) -> watcher::SentinelWatcher {
   |
help: ...and then remove the surrounding `Ok()` from returning expressions
   |
63 ~         Self {
64 +             sender: tx,
65 +             thread_handle: Some(thread_handle),
66 +         }
   |

warning: variables can be used directly in the `format!` string
  --> src/watcher.rs:59:17
   |
59 |                 eprintln!("Watcher thread error: {}", e);
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
59 -                 eprintln!("Watcher thread error: {}", e);
59 +                 eprintln!("Watcher thread error: {e}");
   |

warning: this argument is passed by value, but not consumed in the function body
   --> src/watcher.rs:109:25
    |
109 | fn run_watcher_loop(rx: Receiver<WatcherMessage>, brain_url: String) -> Result<()> {
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_pass_by_value
help: consider taking a reference instead
    |
109 | fn run_watcher_loop(rx: &Receiver<WatcherMessage>, brain_url: String) -> Result<()> {
    |                         +

warning: this argument is passed by value, but not consumed in the function body
   --> src/watcher.rs:109:62
    |
109 | fn run_watcher_loop(rx: Receiver<WatcherMessage>, brain_url: String) -> Result<()> {
    |                                                              ^^^^^^ help: consider changing the type to: `&str`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#needless_pass_by_value

warning: variables can be used directly in the `format!` string
   --> src/watcher.rs:126:21
    |
126 |                     println!("🔍 Sentinel: Starting watch for project '{}'", project_id);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
126 -                     println!("🔍 Sentinel: Starting watch for project '{}'", project_id);
126 +                     println!("🔍 Sentinel: Starting watch for project '{project_id}'");
    |

warning: variables can be used directly in the `format!` string
   --> src/watcher.rs:181:13
    |
181 |             eprintln!("Error handling file event: {}", e);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
    |
181 -             eprintln!("Error handling file event: {}", e);
181 +             eprintln!("Error handling file event: {e}");
    |

warning: this function's return value is unnecessary
   --> src/watcher.rs:192:1
    |
192 | / fn handle_file_event(
193 | |     event: &Event,
194 | |     project_id: &str,
195 | |     brain_url: &str,
196 | |     watched_paths: &Arc<Mutex<HashSet<PathBuf>>>,
197 | | ) -> Result<()> {
    | |_______________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unnecessary_wraps
help: remove the return type...
    |
197 - ) -> Result<()> {
197 + ) -> () {
    |
help: ...and then remove returned values
    |
201 ~         _ => return ,
202 |     }
...
257 |
258 ~     
    |

warning: called `map(<f>).unwrap_or(false)` on an `Option` value
   --> src/watcher.rs:220:31
    |
220 |           let is_architecture = path
    |  _______________________________^
221 | |             .file_name()
222 | |             .and_then(|n| n.to_str())
223 | |             .map(|n| n == "04-Architecture.md")
224 | |             .unwrap_or(false);
    | |_____________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#map_unwrap_or
help: use `is_some_and(<f>)` instead
    |
223 -             .map(|n| n == "04-Architecture.md")
223 +             .is_some_and(|n| n == "04-Architecture.md");
    |

warning: variables can be used directly in the `format!` string
   --> src/watcher.rs:227:13
    |
227 | /             println!(
228 | |                 "🧠 Architecture.md changed - triggering full re-index for {}",
229 | |                 project_id
230 | |             );
    | |_____________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args

warning: item in documentation is missing backticks
   --> src/watcher.rs:272:33
    |
272 | /// - Files in `01-PLANNING/` → ProjectArchitecture
    |                                 ^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
272 - /// - Files in `01-PLANNING/` → ProjectArchitecture
272 + /// - Files in `01-PLANNING/` → `ProjectArchitecture`
    |

warning: item in documentation is missing backticks
   --> src/watcher.rs:273:43
    |
273 | /// - Files in `00-MANAGEMENT/sprints/` → SprintMemory
    |                                           ^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
273 - /// - Files in `00-MANAGEMENT/sprints/` → SprintMemory
273 + /// - Files in `00-MANAGEMENT/sprints/` → `SprintMemory`
    |

warning: item in documentation is missing backticks
   --> src/watcher.rs:274:41
    |
274 | /// - Files in `src/`, `tests/`, etc. → SourceCode
    |                                         ^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
274 - /// - Files in `src/`, `tests/`, etc. → SourceCode
274 + /// - Files in `src/`, `tests/`, etc. → `SourceCode`
    |

warning: item in documentation is missing backticks
   --> src/watcher.rs:275:47
    |
275 | /// - Files in a global standards directory → GlobalStandard
    |                                               ^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#doc_markdown
help: try
    |
275 - /// - Files in a global standards directory → GlobalStandard
275 + /// - Files in a global standards directory → `GlobalStandard`
    |

warning: redundant closure
   --> src/watcher.rs:321:14
    |
321 |         .map(|s| s.to_string());
    |              ^^^^^^^^^^^^^^^^^ help: replace the closure with the method itself: `std::string::ToString::to_string`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#redundant_closure_for_method_calls

warning: assigning the result of `Clone::clone()` may be inefficient
   --> src/watcher.rs:357:13
    |
357 |             metadata.file_type = file_type.clone();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `metadata.file_type.clone_from(&file_type)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#assigning_clones

warning: casting `usize` to `u32` may truncate the value on targets with 64-bit wide pointers
   --> src/watcher.rs:358:41
    |
358 |             metadata.chunk_index = Some(idx as u32);
    |                                         ^^^^^^^^^^
    |
    = help: if this is intentional allow the lint with `#[allow(clippy::cast_possible_truncation)]` ...
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#cast_possible_truncation
help: ... or use `try_from` and handle the error accordingly
    |
358 -             metadata.chunk_index = Some(idx as u32);
358 +             metadata.chunk_index = Some(u32::try_from(idx));
    |

warning: variables can be used directly in the `format!` string
  --> src/main.rs:62:17
   |
62 |                 eprintln!("{}", e);
   |                 ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
62 -                 eprintln!("{}", e);
62 +                 eprintln!("{e}");
   |

warning: variables can be used directly in the `format!` string
  --> src/main.rs:68:17
   |
68 |                 eprintln!("{}", e);
   |                 ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
68 -                 eprintln!("{}", e);
68 +                 eprintln!("{e}");
   |

warning: variables can be used directly in the `format!` string
  --> src/main.rs:74:17
   |
74 |                 eprintln!("{}", e);
   |                 ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
74 -                 eprintln!("{}", e);
74 +                 eprintln!("{e}");
   |

warning: variables can be used directly in the `format!` string
  --> src/main.rs:83:17
   |
83 |                 eprintln!("{}", e);
   |                 ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
83 -                 eprintln!("{}", e);
83 +                 eprintln!("{e}");
   |

warning: variables can be used directly in the `format!` string
  --> src/main.rs:89:17
   |
89 |                 eprintln!("{}", e);
   |                 ^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#uninlined_format_args
help: change this to
   |
89 -                 eprintln!("{}", e);
89 +                 eprintln!("{e}");
   |

warning: unused `async` for function with no await statements
   --> src/brain.rs:164:5
    |
164 | /     pub async fn connect(url: &str) -> Result<Self> {
165 | |         let client = Qdrant::from_url(url)
166 | |             .build()
167 | |             .context("Failed to create Qdrant client")?;
168 | |
169 | |         Ok(Self { client })
170 | |     }
    | |_____^
    |
    = help: consider removing the `async` from this function
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#unused_async
    = note: `-W clippy::unused-async` implied by `-W clippy::pedantic`
    = help: to override `-W clippy::pedantic` add `#[allow(clippy::unused_async)]`

warning: `nexus` (bin "nexus") generated 232 warnings (16 duplicates) (run `cargo clippy --fix --bin "nexus" -p nexus` to apply 140 suggestions)
warning: `nexus` (bin "nexus" test) generated 248 warnings (231 duplicates) (run `cargo clippy --fix --bin "nexus" -p nexus --tests` to apply 15 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.36s
