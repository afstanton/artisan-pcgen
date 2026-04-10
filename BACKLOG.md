# artisan-pcgen Backlog

This backlog is derived from `TOKEN_INVENTORY.txt`, the current schema/projection/emission
code, and spot checks against the PCGen and BahamutDragon external corpora.

Current inventory status (regenerate with `bash inv.sh`):

- `591` semantically interpreted token keys
- `587` fully structured canonical token keys
- `4` semantically interpreted but not fully structured (NOTE, HELP, APPLIEDTO, NONPP)
- `0` unhandled token keys
- `0` fallback-needed token keys
- `2` policy-supported-only (EQUIPMENT.PART, DESC.CLEAR — correct by design)

The primary corpus-coverage goal — **zero unhandled tokens** — is now met.

---

## Priority 1: Round-Trip Fixture Coverage

Every token family should have a fixture that exercises the full parse → project →
emit cycle. Tokens newly added in the `.pcg` and LST-gap passes still lack fixtures.

### `.pcg` schemas needing fixtures

- [ ] `USERPOOL` + `POOLPOINTS` sub-token
- [ ] `EQUIPSET` + `ID`, `VALUE`, `QUANTITY`, `USETEMPMODS`
- [ ] `EQUIPNAME` + `OUTPUTORDER`, `COST`, `WT`, `QUANTITY`, `CUSTOMIZATION`, `NOTE`
- [ ] `CLASSABILITIESLEVEL` + `HITPOINTS`, `SKILLSGAINED`, `SKILLSREMAINING`
- [ ] Bio tokens: `TABNAME`, `SKINCOLOR`, `EYECOLOR`, `HAIRCOLOR`, `HAIRSTYLE`,
      `CITY`, `BIRTHDAY`, `BIRTHPLACE`, `PERSONALITYTRAIT1`, `PERSONALITYTRAIT2`,
      `SPEECHPATTERN`, `PHOBIAS`, `INTERESTS`, `CATCHPHRASE`, `PORTRAIT`
- [ ] Progression: `EXPERIENCE`, `EXPERIENCETABLE`, `MONEY`
- [ ] Description blocks: `CHARACTERBIO`, `CHARACTERDESC`, `CHARACTERCOMP`,
      `CHARACTERASSET`, `CHARACTERMAGIC`, `CHARACTERDMNOTES`
- [ ] Misc: `VERSION`, `FEATPOOL`, `CALCEQUIPSET`, `SUPPRESSBIOFIELDS`

### LST-gap schemas needing fixtures

- [ ] `STATMODSAVE` (system/codeControl)
- [ ] `ALTHP` (system/codeControl)
- [ ] `HIDDENEQUIPTYPES` (system/miscinfo)
- [ ] `HIDDENFEATTYPES` (system/miscinfo)
- [ ] `CHANNEL` (variable definition)
- [ ] `ALTCRITICAL` (equipment)
- [ ] `SKILLLIST` (class)
- [ ] `NONPP` (template)
- [ ] `HELP` (PCC)
- [ ] `EXPRESSION` (ROLLMETHOD sub-token)

---

## Priority 2: Semi-Structured Token Promotion

These 4 tokens are schema-defined and projected correctly, but no corpus entity
produces a non-empty attribute value for them — so they show as "semi-structured"
rather than "fully-structured" in the inventory.

- [ ] `NOTE` (10 occurrences) — appears in EQUIPNAME records, but the value is
      always empty in the test corpus. Add a fixture with a real note string.
- [ ] `HELP` (1 occurrence) — maps to `pcgen_help` in PCC. The corpus entry
      (`HELP:./help_spycraft.html`) should be fully-structured; investigate why
      the attribute isn't being stored (possibly a PCC parsing path issue).
- [ ] `APPLIEDTO` (1 occurrence) — maps to `pcgen_appliedto` in feat/ability
      records. Add a fixture with a non-empty APPLIEDTO value.
- [ ] `NONPP` (1 occurrence) — maps to `pcgen_nonpp` in template records. The
      corpus entry `NONPP:-8` is in a zen_test file; add a real fixture.

---

## Priority 3: DYNAMIC Datacontrol Entity Schema

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

## Priority 4: Emission Quality

The emitter (`emit.rs`) can output structured entities but several token families
lack emit coverage or have edge cases.

- [ ] Audit emit paths for all newly added `.pcg` schemas (USERPOOL, EQUIPSET,
      EQUIPNAME, CLASSABILITIESLEVEL, bio tokens, progression tokens).
- [ ] Verify CHANNEL variable entities round-trip through emit correctly.
- [ ] Check that `ALTCRITICAL`, `SKILLLIST`, `NONPP` emit correctly from their
      respective schemas.

---

## Done Criteria Per Token

- [x] `TokenDef` exists in the correct schema with `ArtisanMapping::Attribute`
- [x] `fields.rs` projects the token into a named entity attribute
- [ ] `emit.rs` emits the token from structured data with no raw fallback
- [ ] Round-trip fixture covers the token
- [x] `TOKEN_INVENTORY.txt` shows the token as fully-structured (not Unhandled)
