use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub d_grammar);

#[test]
fn d_gram_test(){
    assert!(
        d_grammar::SpecParser::new()
            .parse("
                    obj: balloon;
                    obj: pin;
                   ")
            .is_ok()
    );
}


#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
