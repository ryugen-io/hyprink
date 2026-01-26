# Task Completion Checklist

## Before EVERY Commit (MANDATORY)
```bash
just pre-commit
```
This runs format check, clippy, and tests. **DO NOT BYPASS** with `--no-verify`.

## Workflow
1. Make code changes
2. Stage changes: `git add <files>` (prefer specific files over `git add .`)
3. Run validation: `just pre-commit` (MUST PASS)
4. Commit: `git commit -m "..."`
5. Push only when explicitly requested

## If Pre-Commit Fails
1. Fix the reported issues
2. Re-stage if needed
3. Run `just pre-commit` again
4. Only commit after it passes

## Quick Checks
- Format only: `just fmt-check`
- Lint only: `just lint`
- Tests only: `just test`

## Notes
- Never push without explicit request
- No emojis in code, comments, commits, or docs
- Use text markers instead: [OK], [ERROR], [WARN], [INFO], [DEBUG], [TEST], [FAIL], [BUILD]
