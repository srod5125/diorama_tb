use z3::{*,Sort};
use z3::ast::Ast;


pub struct Spec {
    //cfg: z3::Config,
    pub ctx: z3::Context,
}

impl Spec {
    pub fn new() -> Self {
        let cfg: z3::Config   = Config::new();
        let ctx: z3::Context  = Context::new(&cfg);
        
        let spec = Spec{
            //cfg: cfg,
            ctx: ctx
        };
        spec
    }
}


// l = [, or ]
// c = (, or )
pub enum bound_kind {
    ll,
    lc,
    cl,
    cc
}
pub struct Range<'ctx> {
    pub lb : z3::ast::Int<'ctx>,
    pub ub : z3::ast::Int<'ctx>,
    pub interval : bound_kind
}


pub fn str_To_Sort<'input,'spec:'input>(
        sort_string : &str, 
        ctx: &'spec z3::Context 
    ) 
    -> Sort<'spec> {
    match sort_string {
        "int" => {
            Sort::int(&ctx)
        },
        "bool" => {
            Sort::bool(&ctx)
        },
        _ => Sort::int(&ctx)
    }
}

/*
pub fn seed_sort<'a>( seed: &'a z3::ast::Dynamic, ctx: &'a z3::Context ) 
    -> Sort{

    match seed.sort_kind() {

        z3::SortKind::Bool => {
            z3::Sort::bool(&ctx)
        },
        z3::SortKind::Int => {
            z3::Sort::int(&ctx)
        },
        z3::SortKind::Array => {
            let dom = seed.get_sort()
                          .array_domain()
                          .unwrap();
            let rng = seed.get_sort()
                          .array_range()
                          .unwrap();
            z3::Sort::array(
                &ctx,
                &dom,
                &rng
            )
        },
        z3::SortKind::Datatype => {
            //TODO: traverse datatype and return sort
            z3::Sort::bool(&ctx)
        },
        z3::SortKind::FloatingPoint => {
            z3::Sort::float32(&ctx)
        },
        _ | z3::SortKind::Unknown => {
            z3::Sort::uninterpreted(&ctx,
                                    z3::Symbol::String("u".to_string()))
        }
    }
}
*/

//TODO: switch to optimization instead of context
//TODO: add more paramters to config, timout, ect