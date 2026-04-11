# artisan-pcgen Backlog

This backlog is derived from `TOKEN_INVENTORY.txt`, the current schema/projection/emission
code, and spot checks against the PCGen and BahamutDragon external corpora.

Current inventory status (regenerate with `bash inv.sh`):

- `612` semantically interpreted token keys
- `612` fully structured canonical token keys (**all interpreted tokens are fully-structured**)
- `0` semantically interpreted but not fully structured
- `0` unhandled token keys
- `0` fallback-needed token keys
- `2` policy-supported-only (EQUIPMENT.PART, DESC.CLEAR — correct by design)
- `617` fixture tokens ≥ `614` observed tokens (fixture set is a superset of corpus)

The corpus is now scanned from three subdirectory types: `data`, `system`, and `characters`.

---

## Priority 1: DYNAMIC Datacontrol Entity Schema

`DYNAMIC` has two roles in PCGen:
1. **PCC include directive** — `DYNAMIC:path/to/file.lst` inside a `.pcc` file.
   This is already fully-structured via `pcc::DYNAMIC_INCLUDE_SCHEMA`.
2. **Datacontrol entity** — entries in `*_dynamic.lst` files. These are not yet
   handled as a distinct entity type.

- [ ] Create `src/schema/datacontrol.rs` with a `DYNAMIC_SCHEMA` for datacontrol
      entity entries, analogous to how FUNCTION and DYNAMICSCOPE are handled.
- [ ] Add projection and emission support.
- [ ] Add round-trip fixture.

---

## Priority 2: Emission Quality

The emitter (`emit.rs`) can output structured entities but several token families
lack emit coverage or have edge cases.

- [ ] Audit emit paths for all `.pcg` schemas added in the characters pass:
      SPELLNAME, PCG_DEITY, PCG_DOMAIN, NOTE (standalone), WEAPONPROF (WEAPON
      inner bracket items), SKILL (CLASSBOUGHT/RANKS/CLASSSKILL).
- [ ] Verify CHANNEL variable entities round-trip through emit correctly.
- [ ] Verify ALTCRITICAL, SKILLLIST, NONPP, HELP emit correctly.
- [ ] The bracket notation in PCG files (e.g., `WEAPONPROF:[WEAPON:name|...]`)
      is parsed by splitting on every `|`, which breaks the bracket structure.
      Consider whether bracket grouping should be preserved in the round-trip emitter.

---

## Done Criteria Per Token

- [x] `TokenDef` exists in the correct schema with `ArtisanMapping::Attribute`
- [x] `fields.rs` projects the token into a named entity attribute
- [ ] `emit.rs` emits the token from structured data with no raw fallback
- [x] Round-trip fixture covers the token
- [x] `TOKEN_INVENTORY.txt` shows the token as fully-structured (not Unhandled)
