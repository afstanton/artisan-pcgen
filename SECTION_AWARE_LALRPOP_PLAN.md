# LALRPOP Section-Aware Grammar Refactor Checklist

## 1. Define a Custom Token Enum
- [ ] Ensure `LineToken` enum in `parser_tokens.rs` has variants for all needed tokens, e.g. `SectionHeader(String)`, `Pipe`, `Piece(String)`.

## 2. Use the Custom Token Type in LALRPOP
- [ ] At the top of the `.lalrpop` grammar, specify:
  ```
  grammar;
  use crate::parsing::parser_tokens::LineToken;
  ```

## 3. Update the Extern Block
- [ ] In the `extern` block, declare:
  ```
  extern {
      type Location = usize;
      type Error = String;
      type Token = LineToken;
  }
  ```

## 4. Match Value-Carrying Tokens in Grammar Rules
- [ ] In grammar rules, match tokens like `SectionHeader(h)` to bind the value.

## 5. Update the Parser Invocation
- [ ] When invoking the parser, pass a stream of `(Location, LineToken, Location)` tuples (not just strings).
- [ ] Use the lexer to produce this stream.

## 6. Adjust Tests and Integration
- [ ] Update all test and production code to use the correct token stream when calling the parser.

## 7. Validate and Iterate
- [ ] Build and run tests.
- [ ] Adjust grammar, lexer, or parser as needed to handle all section and line cases.

---

**Summary:**
This checklist enables robust section-aware parsing in LALRPOP by using a custom token type and value-carrying tokens.
