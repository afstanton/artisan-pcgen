# artisan-pcgen Backlog

This backlog is derived from `TOKEN_INVENTORY.txt`, the current schema/projection/emission
code, and spot checks against the PCGen and BahamutDragon external corpora.

Current inventory status (regenerate with `bash inv.sh`):

- `610` semantically interpreted token keys
- `610` fully structured canonical token keys (**all interpreted tokens are fully-structured**)
- `0` semantically interpreted but not fully structured
- `0` unhandled token keys
- `0` fallback-needed token keys
- `0` policy-supported-only token keys
- `610` fixture tokens = `610` observed tokens

The corpus is now scanned from three subdirectory types: `data`, `system`, and `characters`.

---

## Current Focus: Emission Fidelity

Token coverage is now complete at the inventory level. The next phase is proving
that structured entities emit the right PCGen surface syntax for the more awkward
record families, especially `.pcg` bracketed sub-records and selector/path-style
constructs.

- [x] Add explicit textual round-trip assertions for `.pcg` bracketed records:
      `WEAPONPROF`, `DEITYDOMAINS`, `DEITYFAVWEAP`, `FEATLIST`, `CUSTOMIZATION`,
      `CLASSBOUGHT`.
- [ ] Audit `.pcg` emitter output against sample files in `externals/PCGen/pcgen/characters`
      for record ordering and bracket preservation.
- [ ] Verify `CHANNEL` variable entities emit the same legal syntax seen in the corpus.
- [x] Verify lower-frequency structured tokens also have direct emitter assertions:
      `ALTCRITICAL`, `SKILLLIST`, `NONPP`, `HELP`.
- [ ] Decide whether selector/path-style forms beyond the current
      `EQUIPMENT.PART:...` head-record support need a deeper structured model for
      `LOCAL:EQUIPMENT.PART|...` and `MODIFYOTHER:EQUIPMENT.PART|...`.

## Notes

- `DESC.CLEAR` is fully structured and aligned with the PCGen docs.
- `EQUIPMENT.PART` is no longer policy-only in the inventory because the
  head-record form is schema-backed. The remaining open question is whether the
  deeper selector/path variants deserve a generalized structured model.
- The earlier backlog item about a standalone datacontrol `DYNAMIC` entity
  appears to have been based on a bad assumption. Spot checks of `*_dynamic.lst`
  files in the corpus currently show records like `MOVEMENT:` and `VISION:`,
  not standalone `DYNAMIC:` entity records.

---

## Done Criteria Per Token

- [x] `TokenDef` exists in the correct schema with `ArtisanMapping::Attribute`
- [x] `fields.rs` projects the token into a named entity attribute
- [x] `emit.rs` emits the token from structured data with no raw fallback
- [x] Round-trip fixture covers the token
- [x] `TOKEN_INVENTORY.txt` shows the token as fully-structured (not Unhandled)
