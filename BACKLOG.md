# artisan-pcgen Backlog

This backlog is derived from `TOKEN_INVENTORY.txt`, the current schema/projection/emission
code, and spot checks against the PCGen and BahamutDragon external corpora.

Current inventory status (regenerate with `bash inv.sh`):

- `613` semantically interpreted token keys
- `613` fully structured canonical token keys (**all interpreted tokens are fully-structured**)
- `0` semantically interpreted but not fully structured
- `0` unhandled token keys
- `0` fallback-needed token keys
- `1` policy-supported-only target remains in the intended end-state backlog:
  `EQUIPMENT.PART`
- `614` fixture tokens = `614` observed tokens

The corpus is now scanned from three subdirectory types: `data`, `system`, and `characters`.

---

## End-State Gap: Policy-Supported Tokens

These are intentionally recognized as valid PCGen syntax today, but they are
not the intended end state for `artisan-pcgen`. The long-term goal is that
legal PCGen files should parse into structured state and emit back into valid
PCGen syntax without relying on token-policy exceptions.

### `DESC.CLEAR`

Current state:
- Documented in PCGen docs as valid `DESC:.CLEAR` / regex-clear syntax.
- Structured through `GlobalGroup::Desc`, projection, and emission.
- No longer intended to remain policy-only once inventory is regenerated with
  schema-aware classification precedence.

Target state:
- [x] Promote `DESC.CLEAR` to structured support in the `GlobalGroup::Desc`
      path.
- [x] Project clear-state into a dedicated attribute (or generic clear-token
      structure) in `fields.rs`.
- [x] Emit `DESC.CLEAR:` from structured state in `emit.rs`.
- [x] Add strict round-trip fixture coverage and remove it from policy-only
      inventory-only treatment.

### `EQUIPMENT.PART`

Current state:
- Accepted by `token_policy.rs` as valid selector/path syntax.
- Counted as supported in inventory.
- Not modeled as a first-class schema-backed selector/scope structure.

Target state:
- [ ] Design a structured selector/scope representation suitable for
      `EQUIPMENT.PART` and related path-style PCGen constructs.
- [ ] Thread that representation through parse, projection, and emission.
- [ ] Add fixture coverage from BahamutDragon parser-replacement examples.
- [ ] Remove the policy-only special case once selector support is real.

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
