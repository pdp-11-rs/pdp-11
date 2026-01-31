## Issue Tracking

This project uses **bd (beads)** for issue tracking.
Run `bd prime` for workflow context.

**Quick reference:**
- `bd ready` - Find unblocked work
- `bd create "Title" --type task --priority 2` - Create issue
- `bd close <id>` - Complete work
- `bd sync` - Sync with git (run at session end)


## Rust development guidelines
1. Follow idiomatic Rust practices
2. Write clear, maintainable code
3. Use `cargo fmt` for formatting (run before every commit)
4. Use `cargo clippy --all-targets` for linting (ensure zero warnings before commit)
5. Write unit tests for new functionality
6. Document public APIs with comments
7. Ensure compatibility with the latest stable Rust version
8. Test with `cargo nextest run` or `cargo test` if nextest is not available

**Pre-commit checklist:**
- `cargo fmt` - Format code
- `cargo clippy --all-targets` - Must show zero warnings
- `cargo test` - All tests must pass


## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
