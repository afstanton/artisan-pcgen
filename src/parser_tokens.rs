use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
pub enum LineToken {
    #[token("|")]
    Pipe,

    #[regex(r"\\.", |lex| lex.slice()[1..].to_string())]
    #[regex(r"[^|\\]+", |lex| lex.slice().to_string())]
    #[token("\\", |_| "\\".to_string())]
    Piece(String),
}

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
pub enum ClauseToken {
    #[token(":")]
    Colon,

    #[regex(r"\\.", |lex| lex.slice()[1..].to_string())]
    #[regex(r"[^:\\]+", |lex| lex.slice().to_string())]
    #[token("\\", |_| "\\".to_string())]
    Piece(String),
}

pub fn line_tokens(input: &str) -> Vec<(usize, LineToken, usize)> {
    let mut out = Vec::new();
    for (result, span) in LineToken::lexer(input).spanned() {
        let token = match result {
            Ok(token) => token,
            Err(_) => continue,
        };
        out.push((span.start, token, span.end));
    }
    out
}

pub fn clause_tokens(input: &str) -> Vec<(usize, ClauseToken, usize)> {
    let mut out = Vec::new();
    for (result, span) in ClauseToken::lexer(input).spanned() {
        let token = match result {
            Ok(token) => token,
            Err(_) => continue,
        };
        out.push((span.start, token, span.end));
    }
    out
}
