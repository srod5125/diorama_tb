use lalrpop_util::{lalrpop_mod,ParseError};

lalrpop_mod!(pub d_grammar);

//#[test]
fn d_gram_test(){
    let mut errors = Vec::new();

    let spec = d_grammar::SpecParser::new()
                    .parse( &mut errors,
                            r#"
                            module time is

                                Clocks are { hours, minutes, seconds }
                                    where: hours is in Int and is between 1..12.
                                    where: minutes is in Int and is between 1..59.
                                    where: seconds is in Int and is between 1..59.

                                members are { c in Clocks }

                                action Tick is
                                    when always:
                                    then:
                                        for c in members.

                                        c->seconds' := c->seconds + 1.
                                        

                                end action

                            end time 
                        
                            "#);
    match spec {
        Ok(spec) => println!("no errs: {}",spec),
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
    //println!("Line\n:{}\n",spec.clone().unwrap());
    //assert!( spec.is_ok() );
}



//#[cfg(not(test))]
fn main() {
    d_gram_test();
}
