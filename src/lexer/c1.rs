use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex

     
    // Schlüsselwörter
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("return")]
    KwReturn,

    #[token("void")]
    KwVoid,

    #[token("while")]
    KwWhile,


    // Operatoren
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("<")]
    Lss,

    #[token(">")]
    Grt,

    #[token("<=")]
    Leq,

    #[token(">=")]
    Geq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,


    // Sonstige Token
    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,


    // Pseudotoken
    // #[regex("[0-9]")]                 
    // Digit,
    
    // #[regex("([0-9]+)")]         
    // Integer,

    // #[regex("[0-9]*[.][0-9]+")]      
    // Float,

    // #[regex("[a-zA-Z]")]
    // Letter, 	


    // Termvariablen
    #[regex("[0-9]+")]
    ConstInt,
             
    #[regex("([0-9]*[.][0-9]+([eE]([-+])?([0-9]+))?)|(([0-9]+)[eE]([-+])?([0-9]+))")]   
    ConstFloat,

    #[regex("true|false")]
    ConstBoolean,

    #[regex("\"[^\n\"]*\"")]                                                             
    ConstString,

     #[regex("([a-zA-Z])+([0-9]|[a-zA-Z])*")]  
    Id,


    // Kommentare             
    // "/*" <comment> "*/", 
    #[regex("/[*]([^*]|([*][^/]))*[*]+/", logos::skip)]  
    Ccomment,      

    // "//" <comment> "\n"
    #[regex("//([^(\n)]*)\n", logos::skip)]               



    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    #[regex(r"[ \r\t\n]+", logos::skip)]
    Error,
}
