use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub d_grammar);

#[test]
fn d_gram_test(){
    assert!(
        d_grammar::SpecParser::new()
            .parse("
                    module time is

                        Clocks are {
                            hours   in Int is between 1..12.
                            minutes in Int is between 1..59.
                            seconds in Int is between 1..59.
                        };



                    end time;
                   ")
            .is_ok()
    );
}


#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
