# artisan-pcgen Backlog

This backlog is derived from `TOKEN_INVENTORY.txt`, the current schema/projection/emission code, and spot checks against the `PCGen` and `BahamutDragon` external corpora.

The immediate goal is narrow and concrete:

- [ ] Generate valid PCGen files from properly structured Artisan entities without relying on raw-clause fallback.
- [ ] Ensure every implemented token has a schema definition, structured projection, and round-trip fixture coverage.

## Priority 0: Finish Existing PCC Schema Backlog

These tokens already exist in `src/schema/pcc.rs` but still use `ArtisanMapping::None`. They are the shortest path to improving valid `.pcc` generation.

- [x] `ABILITY` -> map to structured attribute and emit from schema
- [x] `ABILITYCATEGORY` -> map to structured attribute and emit from schema
- [x] `FEAT` -> map to structured attribute and emit from schema
- [x] `EQUIPMENT` -> map to structured attribute and emit from schema
- [x] `SPELL` -> map to structured attribute and emit from schema
- [x] `LICENSE` -> map to structured attribute and emit from schema
- [x] `INFOTEXT` -> map to structured attribute and emit from schema
- [x] `FORWARDREF` -> map to structured attribute and emit from schema
- [x] `HIDETYPE` -> map to structured attribute and emit from schema
- [x] `URL` -> map to structured attribute and emit from schema
- [x] Add a focused `.pcc` fixture covering the PCC backlog tokens above

## Priority 1: System and Game-Mode `.lst` Tokens

These appear repeatedly in official PCGen `system/gameModes/*` files and are important for generating valid ruleset/system files.

- [x] `SKILLCOST_CLASS`
- [x] `SKILLCOST_EXCLUSIVE`
- [x] `SPELLBASECONCENTRATION`
- [x] `XPAWARD`
- [x] `STATINPUT`

Likely homes:

- [x] Confirm the right schema locations in `src/schema/system.rs` and related system schemas
- [x] Add round-trip fixtures for `miscinfo.lst`/system-style lines using these tokens

## Priority 2: Datacontrol and Function Tokens

These show up in `*_datacontrols.lst` and similar files and look like real schema gaps rather than parser bugs.

- [x] `FUNCTION`
- [x] `DYNAMICSCOPE`
- [ ] Review `DYNAMIC`
- [ ] Review `GLOBALMODIFIER`
- [ ] Review `VALUE` in datacontrol contexts

Likely homes:

- [ ] `src/schema/datacontrol.rs`
- [ ] `src/analysis/fields.rs` structured projection helpers for formula-like values

## Priority 3: Entity-Specific `.lst` Gaps

These look like legitimate data-file tokens that belong in existing entity schemas.

Class and subclass related:

- [x] `HASSUBCLASS`

Equipment or equipmod related:

- [x] `COSTPRE`
- [ ] `FORTIFICATION`
- [ ] `HEALING`

Race, biosettings, startpack, or profile-adjacent:

- [x] `BASEAGEADD`
- [ ] `HEIGHT`
- [ ] `WIDTH`
- [ ] `WEIGHT`
- [ ] `AGE`
- [ ] `ISMATURE`

Spell or magic-system related:

- [ ] `SPELLBASE`
- [ ] `SPELLBASECONCENTRATION` if not completed under system work
- [ ] `USEHIGHERKNOWN`
- [ ] `USEHIGHERPREPPED`
- [ ] `CANCASTPERDAY`
- [ ] `PROHIBITED`

Pool/points style tokens:

- [ ] `POOLPOINTS`
- [ ] `POOLPOINTSAVAIL`
- [ ] `USERPOOL`
- [ ] `SKILLPOOL`
- [ ] `FEATPOOL`

## Priority 4: `.pcg` Character-File Workstream

These are clearly real PCGen character-file tokens, but they are a separate milestone from line-oriented game data generation.

Character metadata and biography:

- [ ] `PCGVERSION`
- [ ] `PLAYERNAME`
- [ ] `BIRTHPLACE`
- [ ] `SPEECHPATTERN`
- [ ] `CATCHPHRASE`
- [ ] `PORTRAIT`
- [ ] `CHARACTERBIO`
- [ ] `CHARACTERDMNOTES`
- [ ] `CHARACTERDESC`
- [ ] `CHARACTERMAGIC`
- [ ] `CHARACTERCOMP`
- [ ] `CHARACTERASSET`

Character progression and money:

- [ ] `EXPERIENCE`
- [ ] `EXPERIENCETABLE`
- [ ] `MONEY`
- [ ] `SKILLSGAINED`
- [ ] `SKILLSREMAINING`

Character equipment organization:

- [ ] `EQUIPSET`
- [ ] `CALCEQUIPSET`
- [ ] `NOTE`
- [ ] `VALUE`
- [ ] `ID`
- [ ] `USETEMPMODS`

Character personal traits:

- [ ] `SKINCOLOR`
- [ ] `EYECOLOR`
- [ ] `HAIRCOLOR`
- [ ] `HAIRSTYLE`
- [ ] `PERSONALITYTRAIT1`
- [ ] `PERSONALITYTRAIT2`
- [ ] `PHOBIAS`
- [ ] `INTERESTS`

## Priority 5: Inventory Triage and False-Positive Cleanup

These tokens look suspicious and should be classified before implementing anything.

- [ ] Review likely parser artifacts or malformed corpus entries:
  `D0`, `D1`, `D1.5`, `IV.PRECLASS`, `VIIBLE`, `SERVAAS`, `R`, `F`, `IE`
- [ ] Review context-sensitive or likely non-token head values:
  `SELUNE`, `WWII`, `STR`
- [ ] Decide for each whether to:
  ignore as artifact,
  harden token classification,
  or implement as a real token

## Done Criteria Per Token

- [ ] `TokenDef` exists in the correct schema
- [ ] `ArtisanMapping` is not `None`
- [ ] `fields.rs` projects the token into structured entity data
- [ ] `emit.rs` emits the token from structured data with no raw fallback
- [ ] Round-trip fixture covers the token
- [ ] `TOKEN_INVENTORY.txt` moves the token out of `Unhandled` or backlog categories
