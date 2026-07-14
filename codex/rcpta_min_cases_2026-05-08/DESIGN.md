# Design Notes

Planned semantic additions:
1. Container element summary pointer recognition (Vec/array-like class element holder).
2. Reference-return bridge operation:
   - semantic operation: `ElemRefBridge(container_elem_summary, caller_ref_local)`
   - apply for iterator-next and index-like reference returns.
3. Optional provenance map in AnalysisContext to connect iter/index temporaries back to container base.

Validation policy:
- First make both minimal cases pass.
- Then replay entry5.
