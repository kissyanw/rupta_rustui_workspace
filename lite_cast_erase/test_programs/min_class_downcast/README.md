# min_class_downcast

Minimal Lite DSL program for class-only downcast analysis.

The concrete allocation is `Dog::new()`, then it is viewed as `CRc<Animal>`.
Two downcast callsites are present:

- `animal.downcast_rc::<Dog>()`: expected must-success.
- `animal.downcast_rc::<Cat>()`: expected must-fail.

