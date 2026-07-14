# RCPTA Min Cases Fix Track (2026-05-08)

Scope:
- Target two minimal test entries in `tests/rcpta_min_cases/main.rs`:
  - `test_rcpta_min_iter_next_unwrap_drive`
  - `test_rcpta_min_index_drive`
- Goal: make `interfacesDrivable::drive::param_1` non-empty in class_pts/type-info.

Method:
- Introduce/adjust rcpta abstraction concepts and semantic operations for container/ref return flow.
- Keep all steps reproducible and logged in this directory.

Files in this folder:
- `BASELINE.md`: baseline observations before new fixes.
- `DESIGN.md`: semantic model additions and rationale.
- `CHANGELOG.md`: chronological code changes and validations.
