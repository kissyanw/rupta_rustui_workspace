# Lite Class DSL Learning Context Index

This directory stores study notes for `lite_class_dsl`.

Purpose:

1. Serve as persistent context for future agents working on:
   - adapting RCPTA to the new DSL
   - writing the DSL paper
2. Separate DSL-learning context from RCPTA progress logs.

## Current Files

- `SURFACE_SYNTAX_MAP.md`
  - Surface syntax summary for `class / interface / mixin`
  - Includes fields, ctors, methods, modifiers, and attribute forms

## Current Learning Status

The following topics have already been covered at a high level:

1. Surface syntax
   - Learned from `oop_rs_macros/src/syntax/tests.rs`
   - Understood the main declaration forms:
     - `class<...>`
     - `interface<...>`
     - `mixin<...>`
     - `extends(...)`
     - `implements(...)`
     - `with(...)`
     - `on(...)`
   - Understood field-level and method-level modifiers such as:
     - `#[late]`
     - `#[vis(...)]`
     - `#[method(final)]`
     - `#[method(override(...))]`

2. Why `syntax/tests.rs` exists
   - It acts as executable surface-syntax documentation.
   - It mainly validates parser/printer round-trip stability.
   - It does not validate expansion correctness or runtime semantics.

3. `class.rs`
   - Understood the main AST structures:
     - `Class`
     - `ClassKind`
     - `ClassAttrs`
     - `ClassItems`
     - `ClassRef`
     - `ClassRefs`
   - Understood that `impl Parse for ...` encodes DSL parsing rules manually.
   - Understood that `ClassItems::parse` classifies body items into:
     - fields
     - ctors
     - methods
     - functions
     - consts
   - Understood that `ClassItemKind::try_from` is the key classifier for function-like items.

4. `method.rs`
   - Understood the main role of `TryFrom` in the DSL frontend:
     - convert general `syn` AST nodes into DSL-specific AST nodes
     - classify
     - validate
     - structure data for later expansion
   - Understood the major syntax-layer structures:
     - `Ctor`
     - `CtorBody`
     - `CtorSelf`
     - `CtorSelfExpr`
     - `CtorCallSuper`
     - `CtorSelfStmt`
     - `Method`
     - `Methods`
     - `Override`
     - `MethodModifiers`
     - `FnBody`
     - `ImplItemFn`

## Key Understanding So Far

The frontend pipeline currently understood is:

```text
surface syntax
  -> token stream
  -> syn-based parsing
  -> DSL syntax AST
  -> checking
  -> expansion
  -> generated Rust code
```

Important distinction:

- `Parse` handles syntax recognition.
- `TryFrom` performs syntax-layer classification and validation.
- `check` handles semantic well-formedness constraints.
- `expand` generates ordinary Rust code.

## Recommended Next Topics

1. `oop_rs_macros/src/syntax/field.rs`
   - finish the syntax-layer parser study

2. `oop_rs_macros/src/syntax/to_tokens.rs`
   - understand how parsed syntax is printed back into tokens
   - useful for understanding round-trip tests

3. `oop_rs_macros/src/check/*`
   - understand which constraints are syntax-adjacent vs semantic

4. `oop_rs_macros/src/expand/mod.rs`
5. `oop_rs_macros/src/expand/class.rs`
6. `oop_rs_macros/src/expand/trait_.rs`
7. `oop_rs_macros/src/expand/mixin.rs`
   - understand how syntax AST becomes generated Rust code

8. `oop_rs/src/*`
   - runtime support layer
   - needed for paper-level explanation and RCPTA adaptation

## Suggested Future Notes

Add future notes in this directory with separate files, for example:

- `SYNTAX_CLASS_RS_NOTES.md`
- `SYNTAX_METHOD_RS_NOTES.md`
- `CHECKING_RULES_MAP.md`
- `EXPANSION_PIPELINE_MAP.md`
- `RUNTIME_MODEL_MAP.md`
- `PAPER_CONTRIBUTION_NOTES.md`
- `RCPTA_HOOKS_FOR_NEW_DSL.md`

## Usage Guideline For Future Agent Context

When resuming work:

1. Read this file first.
2. Read `SURFACE_SYNTAX_MAP.md`.
3. Resume from the next topic in `Recommended Next Topics`.
4. Keep DSL-learning notes separate from RCPTA execution logs.
