use lalrpop_util::{lalrpop_mod,ParseError};

mod ast;

lalrpop_mod!(pub d_grammar);

//#[test]
fn d_gram_test(){
    let mut errors = Vec::new();

    let spec_obj = Box::new(ast::Spec::new());

    let spec = d_grammar::SpecParser::new()
                    .parse( &mut errors,
                            &spec_obj,
                            r#"
                                123
                        
                            "#);
    match spec {
        Ok(_) => println!("no errs: {}",123),
        Err(parse_error) => match parse_error {
            ParseError::InvalidToken { location } => 
                println!("Invalid token at location: {:?}", location),
            ParseError::UnrecognizedEof { location, expected } => 
                println!("Unrecognized EOF at location: {:?}, expected: {:?}", location, expected),
            ParseError::UnrecognizedToken { token, expected } => 
                println!("Unrecognized token {:?}, expected: {:?}", token, expected),
            ParseError::ExtraToken { token } => 
                println!("Extra token found: {:?}", token),
            ParseError::User { error } => 
                println!("User error: {:?}", error),
        },
    }
}



//#[cfg(not(test))]
fn main() {
    d_gram_test();
}
