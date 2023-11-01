use logos::{Logos, Lexer};

#[derive(Debug, Logos, PartialEq, Eq, Clone, Hash)]
enum Token {
    #[error]
    #[regex(r"[ \t\r]", logos::skip)]
    #[regex(r";[^\n]*", logos::skip)]
    Error,

    #[token("\n")]
    LineBreak,
}

fn main() {
    let lexer = Token::lexer("some text\nwith line breaks\nand ;comments");
    for token in lexer {
        println!("{:?}", token);
    }
}
