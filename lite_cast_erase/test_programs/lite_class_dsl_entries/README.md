# Lite Class DSL RCPTA Entry Audit

Generated from the current `lite_class_dsl` source tree. This directory records candidate rcpta `--entry-func` roots that can be copied or mirrored into focused `lite_cast_erase` test programs.

## Scope

- Source roots scanned: `lite_class_dsl/oop_rs/tests`, `lite_class_dsl/manual_impl/tests`.
- Candidate rule: zero-argument Rust functions annotated with `#[test]`.
- Excluded from the primary list: helper methods, constructors, trait methods, and compile-only modules without `#[test]` functions.
- `qualified_guess` is a source-level path guess for disambiguation; rcpta may still need the plain function name or MIR symbol spelling after compilation.

## Summary

- Total candidate entries: `320`
- `manual_impl` entries: `2`
- `oop_rs` entries: `318`
- `high` priority: `27`
- `medium` priority: `48`
- `low` priority: `245`

Category counts:

- `constructor_or_method`: `104`
- `downcast`: `27`
- `field_access`: `188`
- `option_flow`: `58`
- `polymorphic_collection`: `7`
- `upcast_or_typed_view`: `70`

## High Priority Downcast Entries

These are the first entries to mirror into focused cast-erasure tests because their bodies contain downcast calls.

| crate | test crate | entry func | source | tags |
|---|---|---|---|---|
| `oop_rs` | `animal_hierarchy` | `test_shark_full_conversion_chain` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/fish/mod.rs:752` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, assert` |
| `oop_rs` | `animal_hierarchy` | `test_downcast_animal_to_dog_success` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:144` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, assert` |
| `oop_rs` | `animal_hierarchy` | `test_downcast_animal_to_eagle_through_bird` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:206` | `downcast_rc, downcast, explicit_as_crc, clone, field_get, assert` |
| `oop_rs` | `animal_hierarchy` | `test_downcast_animal_to_shark_through_fish` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:295` | `downcast_rc, downcast, explicit_as_crc, clone, field_get, assert` |
| `oop_rs` | `animal_hierarchy` | `test_downcast_animal_dog_to_cat_failure` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:391` | `downcast_rc, downcast, explicit_as_crc, typed_crc, field_get, option, assert` |
| `oop_rs` | `animal_hierarchy` | `test_downcast_animal_eagle_to_penguin_failure` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:423` | `downcast_rc, downcast, explicit_as_crc, clone, field_get, option, assert` |
| `oop_rs` | `animal_hierarchy` | `test_downcast_bird_eagle_to_duck_failure` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:451` | `downcast_rc, downcast, explicit_as_crc, clone, field_get, option, assert` |
| `oop_rs` | `animal_hierarchy` | `test_downcast_fish_shark_to_salmon_failure` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:478` | `downcast_rc, downcast, explicit_as_crc, clone, field_get, option, assert` |
| `oop_rs` | `animal_hierarchy` | `test_downcast_does_not_panic` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:512` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, option, assert` |
| `oop_rs` | `animal_hierarchy` | `prop_downcast_type_safety` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:578` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, option, assert` |
| `oop_rs` | `animal_hierarchy` | `test_mixin_reference_back_conversion` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:772` | `downcast_rc, downcast, typed_crc, clone, field_get, assert` |
| `oop_rs` | `animal_hierarchy` | `prop_mixin_bidirectional_conversion` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:1092` | `downcast_rc, downcast, typed_crc, clone, field_get, assert` |
| `oop_rs` | `animal_hierarchy` | `test_conversion_chain_object_identity` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:2186` | `downcast_rc, downcast, explicit_as_crc, clone, field_get, assert` |
| `oop_rs` | `run` | `multiple_classes_casting` | `lite_class_dsl/oop_rs/tests/run/inheritance/multiple_classes_casting.rs:46` | `downcast_ref, downcast, typed_crc, assert` |
| `oop_rs` | `run` | `two_classes_casting` | `lite_class_dsl/oop_rs/tests/run/inheritance/two_classes_casting.rs:35` | `downcast_ref, downcast, typed_crc, assert` |
| `oop_rs` | `run` | `multiple_interfaces_casting` | `lite_class_dsl/oop_rs/tests/run/interface/multiple_interfaces_casting.rs:39` | `downcast_ref, downcast, assert` |
| `oop_rs` | `run` | `single_interface_casting` | `lite_class_dsl/oop_rs/tests/run/interface/single_interface_casting.rs:30` | `downcast_ref, downcast, typed_crc, assert` |
| `oop_rs` | `run` | `test_downcast` | `lite_class_dsl/oop_rs/tests/run/mixin/downcast.rs:49` | `downcast_ref, downcast, assert` |
| `oop_rs` | `shape_hierarchy` | `test_downcast_shape_to_circle_success` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:194` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, option, assert` |
| `oop_rs` | `shape_hierarchy` | `test_downcast_shape_to_rectangle_success` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:215` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, assert` |
| `oop_rs` | `shape_hierarchy` | `test_downcast_shape_to_triangle_success` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:240` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, assert` |
| `oop_rs` | `shape_hierarchy` | `test_downcast_shape_to_wrong_type_failure` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:267` | `downcast_rc, downcast, explicit_as_crc, typed_crc` |
| `oop_rs` | `shape_hierarchy` | `test_multilevel_conversion_colored_circle` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:286` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, option, assert` |
| `oop_rs` | `shape_hierarchy` | `test_multilevel_conversion_square` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:327` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, assert` |
| `oop_rs` | `shape_hierarchy` | `test_multilevel_conversion_ellipse` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:370` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, assert` |
| `oop_rs` | `shape_hierarchy` | `prop_multilevel_type_conversion_round_trip` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/multilevel.rs:275` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, assert` |
| `oop_rs` | `shape_hierarchy` | `prop_type_conversion_round_trip_consistency` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/polymorphism.rs:121` | `downcast_rc, downcast, explicit_as_crc, typed_crc, clone, field_get, assert` |

## Medium Priority Upcast / Polymorphism Entries

These exercise upcasts, typed `CRc<T>` views, clones, interface/mixin references, collections, and field flows. They are useful after the direct downcast roots are stable.

| crate | test crate | entry func | source | categories |
|---|---|---|---|---|
| `oop_rs` | `animal_hierarchy` | `test_cat_upcast_to_animal` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/cat.rs:123` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_dog_upcast_to_animal` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/dog.rs:118` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_shark_multilevel_upcast` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/fish/mod.rs:488` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_salmon_multilevel_upcast` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/fish/mod.rs:530` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_flying_fish_multilevel_upcast` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/fish/mod.rs:571` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_shark_to_swimmable_mixin` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/fish/mod.rs:613` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_flying_fish_to_multiple_mixins` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/fish/mod.rs:666` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `prop_multilevel_upcast_preserves_identity` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:25` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `prop_mixin_reference_access_integrity` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:984` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `prop_multiple_mixin_independent_conversion` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:1267` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_polymorphic_collection` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:1400` | `upcast_or_typed_view, polymorphic_collection, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_polymorphic_collection_by_category` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:1521` | `polymorphic_collection, field_access` |
| `oop_rs` | `animal_hierarchy` | `test_polymorphic_method_calls` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:1612` | `upcast_or_typed_view` |
| `oop_rs` | `animal_hierarchy` | `prop_polymorphic_method_call_correctness` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:1778` | `upcast_or_typed_view` |
| `oop_rs` | `animal_hierarchy` | `prop_type_distinctiveness` | `lite_class_dsl/oop_rs/tests/animal_hierarchy/main.rs:2051` | `polymorphic_collection` |
| `oop_rs` | `run` | `ty_dispatches_to_subclass_via_base_ref` | `lite_class_dsl/oop_rs/tests/run/downcast_ty.rs:48` | `upcast_or_typed_view` |
| `oop_rs` | `run` | `ty_usable_as_hashmap_key` | `lite_class_dsl/oop_rs/tests/run/downcast_ty.rs:69` | `upcast_or_typed_view, option_flow` |
| `oop_rs` | `run` | `test_eq_and_hash` | `lite_class_dsl/oop_rs/tests/run/eq_hash.rs:178` | `upcast_or_typed_view` |
| `oop_rs` | `run` | `test_format` | `lite_class_dsl/oop_rs/tests/run/format.rs:79` | `upcast_or_typed_view` |
| `oop_rs` | `run` | `diamond_inheritance` | `lite_class_dsl/oop_rs/tests/run/inheritance/diamond_inheritance.rs:109` | `upcast_or_typed_view` |
| `oop_rs` | `shape_hierarchy` | `test_polymorphism_with_collection` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:68` | `upcast_or_typed_view, polymorphic_collection` |
| `oop_rs` | `shape_hierarchy` | `test_upcast_circle_to_shape` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:128` | `upcast_or_typed_view` |
| `oop_rs` | `shape_hierarchy` | `test_upcast_rectangle_to_shape` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:150` | `upcast_or_typed_view` |
| `oop_rs` | `shape_hierarchy` | `test_upcast_triangle_to_shape` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/main.rs:172` | `upcast_or_typed_view` |
| `oop_rs` | `shape_hierarchy` | `prop_multilevel_polymorphic_consistency` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/multilevel.rs:8` | `upcast_or_typed_view` |
| `oop_rs` | `shape_hierarchy` | `prop_inherited_property_transitivity` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/multilevel.rs:569` | `upcast_or_typed_view, field_access, option_flow` |
| `oop_rs` | `shape_hierarchy` | `prop_polymorphic_call_consistency` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/polymorphism.rs:8` | `upcast_or_typed_view` |
| `oop_rs` | `shape_hierarchy` | `prop_color_property_inheritance` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/polymorphism.rs:291` | `upcast_or_typed_view, field_access, option_flow` |
| `oop_rs` | `shape_hierarchy` | `prop_square_area_correctness` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/rectangle/square.rs:31` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `shape_hierarchy` | `prop_square_perimeter_correctness` | `lite_class_dsl/oop_rs/tests/shape_hierarchy/rectangle/square.rs:80` | `upcast_or_typed_view` |
| `oop_rs` | `vehicle_hierarchy` | `test_bicycle_upcast_to_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/bicycle.rs:273` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_car_upcast_to_motor_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/car.rs:371` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_car_upcast_to_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/car.rs:413` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_electric_car_upcast_to_car` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/electric_car.rs:353` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_electric_car_upcast_to_motor_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/electric_car.rs:382` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_electric_car_upcast_to_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/electric_car.rs:408` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_drivable_interface_polymorphism` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/main.rs:34` | `upcast_or_typed_view, polymorphic_collection` |
| `oop_rs` | `vehicle_hierarchy` | `test_maintainable_interface_polymorphism` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/main.rs:169` | `upcast_or_typed_view, polymorphic_collection` |
| `oop_rs` | `vehicle_hierarchy` | `test_multiple_interface_implementation` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/main.rs:450` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_autonomous_mixin_different_levels` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/mixins.rs:201` | `polymorphic_collection, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_motor_vehicle_upcast_to_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/motor_vehicle.rs:201` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_motor_vehicle_describe_overrides_vehicle_describe` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/motor_vehicle.rs:244` | `upcast_or_typed_view` |
| `oop_rs` | `vehicle_hierarchy` | `test_motorcycle_upcast_to_motor_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/motorcycle.rs:395` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_motorcycle_upcast_to_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/motorcycle.rs:436` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_sports_car_upcast_to_car` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/sports_car.rs:368` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_truck_upcast_to_motor_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/truck.rs:393` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_truck_upcast_to_vehicle` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/truck.rs:435` | `upcast_or_typed_view, field_access` |
| `oop_rs` | `vehicle_hierarchy` | `test_vehicle_upcast` | `lite_class_dsl/oop_rs/tests/vehicle_hierarchy/vehicle.rs:132` | `upcast_or_typed_view, field_access` |

## Full Machine-Readable Lists

- `entries.json`: full manifest with source path, crate, test crate, entry function, category tags, and priority.
- `entries.csv`: spreadsheet-friendly version.

## Suggested RCPTA Import Strategy

1. Start with high-priority entries and mirror the smallest self-contained source cases into `lite_cast_erase/test_programs`.
2. Keep each mirrored program focused on one cast behavior: must-safe downcast, must-unsafe sibling downcast, may-unsafe branch/collection, interface downcast, mixin downcast, field store/load downcast.
3. Use the original `source_path:line` as provenance, but avoid importing entire large hierarchy tests until smaller derived programs expose the same behavior.
4. For overloaded duplicate test names in different modules, use `qualified_guess` or create unique wrapper entry functions in the mirrored test crate.
