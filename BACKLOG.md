# artisan-pcgen Backlog

This backlog is derived from `TOKEN_INVENTORY.txt`, the current schema/projection/emission
code, and spot checks against the PCGen and BahamutDragon external corpora.

Current inventory status after the latest `.pcg` standalone-token pass:

- `540` semantically interpreted token keys
- `539` fully structured canonical token keys
- `1` semantically interpreted but not fully structured token key (`NEWCATEGORY`)
- `66` unhandled token keys
- `0` fallback-needed token keys

The immediate goal is narrow and concrete:

- [ ] Generate valid PCGen files from properly structured Artisan entities without relying on raw-clause fallback.
- [ ] Ensure every implemented token has a schema definition, structured projection, and round-trip fixture coverage.

---

## Priority 0: Finish Existing PCC Schema Backlog

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
- [x] `DYNAMIC` -> schema + structured attribute in PCC
- [x] `GLOBALMODIFIER` -> schema + structured attribute in PCC
- [x] Add a focused `.pcc` fixture covering the PCC backlog tokens above

---

## Priority 1: System and Game-Mode `.lst` Tokens

- [x] `SKILLCOST_CLASS`
- [x] `SKILLCOST_EXCLUSIVE`
- [x] `SPELLBASECONCENTRATION`
- [x] `XPAWARD`
- [x] `STATINPUT`
- [x] Confirm the right schema locations in `src/schema/system.rs`
- [x] Add round-trip fixtures for system-style lines

---

## Priority 2: Datacontrol and Function Tokens

- [x] `FUNCTION`
- [x] `DYNAMICSCOPE`
- [ ] `DYNAMIC` as a datacontrol entity (distinct from PCC DYNAMIC include-file
      directive — these are `*_dynamic.lst` entity entries; needs a dedicated schema
      in `src/schema/datacontrol.rs`)
- [x] `GLOBALMODIFIER` (as PCC include directive)
- [x] `VALUE` in datacontrol FUNCTION context

---

## Priority 3: LST Token Gaps

These are real `.lst` tokens belonging in existing schemas. All confirmed from
corpus inspection.

### 3a. Class schema additions

- [x] `HASSUBCLASS`
- [x] `PROHIBITED` — repeatable pipe-delimited list of prohibited spell schools.
      Appears in wizard specialist class lines both in `.lst` files and as a
      sub-token in `.pcg` CLASS lines. Add to `class.rs`.

### 3b. Equipment / equipmod additions

- [x] `COSTPRE`
- [x] `FORTIFICATION` — appears in some third-party equipmod files as an armor
      property token (e.g. `FORTIFICATION:25`). Add as `TokenDef::text` in
      `equipment.rs` globals or as a shared token group.
- [x] `HEALING` — same pattern as FORTIFICATION; appears in some equipmod files.
      Note: "Healing" as an entity NAME in `mic_equipmods.lst` is NOT this token —
      `HEALING:value` as a clause is what generates the unhandled count.

### 3c. PCC / inventory follow-up

- [ ] `ISMATURE` — schema support exists for PCC include lines, but inventory
      still reports it as semantically interpreted rather than fully structured.
      Investigate the remaining classification/emission mismatch.

### 3d. NEWCATEGORY — semantically interpreted but not fully-structured

`NEWCATEGORY` (320 corpus hits) already has a `TokenDef` and explicit projection
arm, but inventory still reports it as semantically interpreted rather than fully
structured. The likely remaining issue is schema selection for migration-style
`ABILITY:... NEWCATEGORY:...` lines.

- [ ] Fix schema selection so migration-style `ABILITY:` records use the ability
      entity schema rather than the PCC include schema when appropriate.

### 3e. Next highest-impact residual tokens

These are the largest remaining unhandled buckets after the latest inventory run.

- [ ] `NOTE` — mainly `.pcg` equipment-set / equipment-record notes
- [ ] `HP` — `.pcg` / equipmod residuals; likely needs separate handling from `HITPOINTS`
- [ ] `VARIABLE` — appears both in `.pcc` include lines and `.pcg` pool usage
- [ ] `WIDTH` — mostly paper/output/profile-style standalone records
- [ ] `CHANNEL` — low-count but real token family in spell/ability contexts

---

## Priority 4: `.pcg` Character-File Workstream

The `.pcg` format is line-oriented (same LALRPOP parser as `.lst`) but with an
entirely different token vocabulary. These tokens are confirmed from reading
`Sorcerer.pcg` and `Everything.pcg`.

**Design note:** `.pcg` files have two kinds of unhandled tokens:

1. **Standalone top-level key:value lines** — parsed as head+clauses with an
   empty clause list. Examples: `HEIGHT:51`, `WEIGHT:154`, `AGE:52`, `MONEY:0`.
   These need a dedicated PCG schema (or set of schemas) with `HeadFormat::TokenPrefixed`.

2. **Sub-tokens inside pipe-delimited lines** — appear as the VALUE field of a
   clause within a larger line. Examples: `STAT:STR|SCORE:18` (SCORE is a
   sub-token), `CLASS:Sorcerer|LEVEL:3|SKILLPOOL:0|SPELLBASE:CHA|CANCASTPERDAY:6,5`.
   These are currently parsed into the `clauses` attribute as raw key:value pairs
   but have no schema projection.

A dedicated `.pcg` schema milestone is required. The existing `.pcg` fixtures
are placeholders; real `.pcg` round-trip fixtures cannot be written until the
schema is designed.

### Configuration / session tokens (standalone)

- [x] `PCGVERSION` — version string, first line of every `.pcg` file
- [x] `POOLPOINTS` — integer, character pool points total
- [x] `POOLPOINTSAVAIL` — integer, pool points available (may be negative)
- [x] `TABLABEL` — integer, active tab index
- [x] `AUTOSPELLS` — Y/N flag
- [x] `USEHIGHERKNOWN` — Y/N, whether to use higher-level known spells
- [x] `USEHIGHERPREPPED` — Y/N, whether to use higher-level prepped spells
- [x] `LOADCOMPANIONS` — Y/N flag
- [x] `USETEMPMODS` — Y/N flag
- [x] `IGNORECOST` — Y/N flag
- [x] `ALLOWDEBT` — Y/N flag
- [x] `AUTORESIZEGEAR` — Y/N flag
- [x] `SKILLSOUTPUTORDER` — integer preference
- [x] `SKILLFILTER` — integer preference
- [x] `PURCHASEPOINTS` — text or N

### Character bio tokens (standalone)

- [x] `CHARACTERNAME` — display name of the character
- [ ] `TABNAME` — tab label override
- [x] `PLAYERNAME` — player's name
- [x] `HEIGHT` — integer (inches in Imperial, cm in Metric)
- [x] `WEIGHT` — integer (lbs or kg)
- [x] `AGE` — integer (years)
- [x] `HANDED` — text: Left, Right, Both
- [ ] `SKINCOLOR` — free text
- [ ] `EYECOLOR` — free text
- [ ] `HAIRCOLOR` — free text
- [ ] `HAIRSTYLE` — free text
- [ ] `LOCATION` — free text
- [ ] `CITY` — free text
- [ ] `BIRTHDAY` — free text
- [ ] `BIRTHPLACE` — free text
- [ ] `PERSONALITYTRAIT1` — free text
- [ ] `PERSONALITYTRAIT2` — free text
- [ ] `SPEECHPATTERN` — free text
- [ ] `PHOBIAS` — free text
- [ ] `INTERESTS` — free text
- [ ] `CATCHPHRASE` — free text
- [ ] `PORTRAIT` — file path or empty

### Character progression (standalone)

- [ ] `EXPERIENCE` — integer XP total
- [ ] `EXPERIENCETABLE` — text, name of XP table in use
- [ ] `MONEY` — numeric, currency total in base units

### Character description blocks (standalone, multiline-capable)

- [ ] `CHARACTERBIO`
- [ ] `CHARACTERDESC`
- [ ] `CHARACTERCOMP`
- [ ] `CHARACTERASSET`
- [ ] `CHARACTERMAGIC`
- [ ] `CHARACTERDMNOTES`

### Sub-tokens inside CLASS lines

- [ ] `LEVEL` — character level in that class
- [ ] `SKILLPOOL` — skill points available
- [ ] `SPELLBASE` — spellcasting ability stat or `None`
- [ ] `CANCASTPERDAY` — comma-delimited spells-per-day per level
- [ ] `PROHIBITED` — pipe-delimited prohibited spell schools (specialist wizard)

### Sub-tokens inside CLASSABILITIESLEVEL lines

- [ ] `CLASSABILITIESLEVEL` — entity head: `ClassName=level`
- [ ] `HITPOINTS` — HP gained at this level
- [ ] `SKILLSGAINED` — skill points gained
- [ ] `SKILLSREMAINING` — skill points remaining to spend

### Sub-tokens inside STAT lines

- [ ] `STAT` — entity head: `StatAbbreviation`
- [x] `SCORE` — base stat score value

### Sub-tokens inside EQUIPNAME lines

- [ ] `EQUIPNAME` — item name
- [ ] `OUTPUTORDER` — integer display order
- [ ] `QUANTITY` — numeric quantity
- [ ] `NOTE` — free text note on item (often empty)
- [ ] `CUSTOMIZATION` — bracket-delimited customization block

### Equipment set tokens (standalone)

- [ ] `EQUIPSET` — set name + ID sub-token
- [ ] `CALCEQUIPSET` — active set ID
- [ ] `ID` — sub-token inside EQUIPSET and EQUIPNAME
- [ ] `VALUE` — item reference inside EQUIPSET entries

### Pool usage tokens (standalone)

- [ ] `USERPOOL` — pool name + POOLPOINTS sub-token
- [ ] `VARIABLE` — variable name/value pair

### HP tracking (standalone in some versions)

- [ ] `HP` — current hit points (distinct from HITPOINTS which is per-level gain)

---

## Priority 5: Inventory Triage — Artifact / Noise Tokens

These should be classified as `Artifact` in `token_policy.rs` rather than left
as `Unhandled`. None require a schema or projection.

### Confirmed output-sheet layout tokens (not game data)

- [ ] `LEFTMARGIN` — HTML output sheet layout
- [ ] `RIGHTMARGIN` — HTML output sheet layout
- [ ] `TOPMARGIN` — HTML output sheet layout
- [ ] `BOTTOMMARGIN` — HTML output sheet layout

### Dice notation fragments parsed as token heads

- [ ] `D0`, `D1`, `D1.5` — dice shorthand appearing in some value contexts;
      the parser splits these as keys when they appear at the start of a clause

### Parser noise / malformed corpus entries

- [ ] `VIIBLE` — typo for `VISIBLE` in a data file; no action, classify as Artifact
- [ ] `SERVAAS` — proper name (deity) parsed as a token head in some data file
- [ ] `SERVEAS` — corpus variant of `SERVESAS`; if present, treat as Artifact
- [ ] `IV.PRECLASS` — prerequisite expression fragment split by the parser
- [ ] `R`, `F`, `IE` — single/two-letter fragments, parser noise

### Proper names and abbreviations in body text

- [ ] `SELUNE` — deity name appearing as a token key due to prose formatting
- [ ] `WWII` — abbreviation appearing in prose
- [ ] `STR` — stat abbreviation appearing as a clause key in some context
      (in normal use, STR appears as a VALUE: `STAT:STR|SCORE:18`)

---

## Done Criteria Per Token

- [ ] `TokenDef` exists in the correct schema
- [ ] `ArtisanMapping` is not `None`
- [ ] `fields.rs` projects the token into a named entity attribute
- [ ] `emit.rs` emits the token from structured data with no raw fallback
- [ ] Round-trip fixture covers the token
- [ ] `TOKEN_INVENTORY.txt` shows the token as fully-structured (not Unhandled)
