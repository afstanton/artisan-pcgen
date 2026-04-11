# artisan-pcgen: High-Level Grammar and Semantic Import Planning Checklist

## 1. Grammar Expansion
- [x] Design top-level LALRPOP grammar for `.lst` section headers and key-value lines (incremental)
- [ ] Design/expand grammars for `.pcg` and `.pcc` file structures
- [x] Define grammar rules for `.lst` section headers and constructs (e.g., `ABILITY:`)
- [ ] Define grammar rules for `.pcg`/`.pcc` constructs
- [x] Add rules for section bodies, attributes, and nested entities for `.lst` (incremental)
- [ ] Add rules for `.pcg`/`.pcc` as needed
- [x] Integrate existing line/clause grammars as building blocks (for `.lst`)

## 2. Tokenization & Lexing
- [x] Update Logos token definitions for `.lst` section headers and key-value lines
- [ ] Update for `.pcg`/`.pcc` as needed
- [x] Ensure lexer can distinguish between section headers, keys, values, delimiters for `.lst`
- [ ] Ensure for `.pcg`/`.pcc`

## 3. Semantic Model: Direct Mapping to artisan-core
- [ ] Parse directly to `artisan-core` `EntityType` and `Entity` data structures (no persistent pcgen-specific AST)
- [ ] If any intermediate AST is used, it must be immediately and fully transformed to the normalized model
- [ ] Map all section/entity types and format-specific constructs to internal representations
- [ ] Preserve all relationships and cross-references

## 4. Fixture-Driven Testing
- [x] Add minimal `.lst` fixture for section header and key-value line
- [ ] Collect/curate more `.lst`, `.pcg`, `.pcc` fixtures
- [ ] Write parser tests for each fixture (success, error, edge cases)
- [ ] Implement round-trip tests: parse → `EntityType`/`Entity` → emit → compare (where possible)
- [ ] Ensure that emission (unparsing) is always from the normalized model, not from any pcgen-specific AST
- [ ] Track fixture provenance and update manifest as needed
---
**Note:** Incremental progress is being made; checklist items will be checked off as each format/feature is implemented and tested.

## 5. Error Handling & Diagnostics
- [ ] Integrate LALRPOP error reporting for malformed input in all supported formats
- [ ] Add custom diagnostics for ambiguous or lossy constructs, format-specific errors
- [ ] Ensure test coverage for error scenarios in `.pcg`, `.pcc`, and `.lst`

## 6. Cross-Format Reconciliation
- [ ] Map parsed `.pcg`, `.pcc`, and `.lst` entities to normalized internal model
- [ ] Implement reconciliation logic for cross-format comparison (e.g., HeroLab, TOML, PCGen formats)
- [ ] Add regression tests for reconciliation outputs

## 7. Documentation
- [ ] Document the grammar and mapping rules for all supported formats
- [ ] Record known lossy/ambiguous cases and rationale for each format
- [ ] Update developer docs as the model evolves

## 8. Performance & Scalability
- [ ] Profile parser on large `.pcg`, `.pcc`, and `.lst` files
- [ ] Optimize grammar and model for speed/memory as needed
- [ ] Add performance tests for key scenarios in all formats

---

## Analysis of Checklist Completeness

- **Coverage:** The checklist now explicitly covers `.pcg`, `.pcc`, and `.lst` formats for grammar, lexing, AST/model, testing, error handling, reconciliation, documentation, and performance.
- **Dependencies:** Steps are ordered logically; grammar and lexing come before AST/model, which precedes testing and reconciliation.
- **Testing:** Emphasizes fixture-driven and round-trip tests, including error and regression cases, for all supported formats.
- **Documentation:** Ensures both technical and rationale documentation are included, with format-specific notes.
- **Performance:** Includes profiling and optimization for all formats, which is often overlooked.

**Conclusion:**
This checklist is comprehensive for evolving `artisan-pcgen` into a robust, grammar-driven, semantically rich importer for `.pcg`, `.pcc`, and `.lst` files. It enforces direct mapping to the `artisan-core` normalized model for both parsing and emission, ensuring no reliance on intermediate, format-specific structures for roundtripping. This provides a clear, format-inclusive, and future-proof roadmap for implementation and review.
