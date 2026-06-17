# Contributing to ZenYT

We love your input! We want to make contributing to this project as easy and transparent as possible.

## Pull Requests
1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

Please read [AGENTS.md](AGENTS.md) and [docs/UI_UX_GUIDELINES.md](docs/UI_UX_GUIDELINES.md) before contributing to ensure consistency.

## Git Workflow
1. **Branching Strategy**: We use standard feature branching.
   - `master`: Stable production-ready code.
   - `feature/<feature-name>`: For new features.
   - `fix/<bug-name>`: For bug fixes.
2. **Commit Messages**: Follow conventional commits (e.g., `feat: add download button`, `fix: resolve crash on invalid URL`).
3. **Merging**: Always rebase onto the latest `master` before creating a PR to maintain a linear history.