# Generated Artifacts

These files are checked-in observation artifacts for the current test program.

- `expanded.rs`: produced with `cargo expand --bin min_class_downcast`.
- `min_class_downcast.mir`: produced with `cargo rustc --bin min_class_downcast -- --emit=mir`.

Regenerate from this directory's parent with:

```sh
cargo expand --bin min_class_downcast > artifacts/expanded.rs
cargo rustc --bin min_class_downcast -- --emit=mir
```

The second command writes MIR using rustc's output naming; copy the generated `.mir` file into `artifacts/min_class_downcast.mir` when refreshing this snapshot.

