use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub d_grammar);

//#[test]
fn d_gram_test(){
    let mut errors = Vec::new();

    let spec = d_grammar::SpecParser::new()
                    .parse( &mut errors,
                            "
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
                                        
                                end action

                            end time 
                        
                            ");
    println!("Line\n:{}\n",spec.clone().unwrap());
    //assert!( spec.is_ok() );
}


//#[cfg(not(test))]
fn main() {
    d_gram_test();
}
