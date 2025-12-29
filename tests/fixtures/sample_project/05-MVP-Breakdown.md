# MVP broken into sprints

## Sprint 0: Setup (day 1)
- [x] Create Cargo project
- [x] Add clap dependency
- [x] Basic CLI skeleton with subcommands
**Exit criteria:** Can run `todo --help` and see command list

## Sprint 1: Core data model (days 2-3)
- [x] Define Task struct with serde
- [x] Define TaskList struct
- [x] Implement JSON file read/write
- [x] Unit tests for serialization
**Exit criteria:** Can save and load tasks from JSON file

## Sprint 2: Add and list commands (days 4-6)
- [x] Implement `todo add <description>` command
- [x] Implement `todo list` command
- [x] Format output with colored text
**Exit criteria:** Can add tasks and see them listed

## Sprint 3: Done and delete commands (days 7-9)
- [x] Implement `todo done <id>` command
- [x] Implement `todo delete <id>` command
- [x] Update list to show completed tasks differently
**Exit criteria:** Full CRUD operations working

## Sprint 4: Polish and testing (days 10-14)
- [x] Error handling for all commands
- [x] Integration tests
- [x] User documentation in README
- [x] Installation instructions
**Exit criteria:** Can install with `cargo install` and use reliably

## Definition of Done (each sprint):
- [x] Builds without errors
- [x] Tested manually
- [ ] Committed to git
- [ ] Session log updated

---
✅ Done when: MVP divided into max 5 sprints + each has exit criteria
