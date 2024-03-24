use z3::*;

//pub mod ast{

    pub struct Spec {
        cfg: z3::Config,
        pub ctx: z3::Context,
    }
    
    impl Spec {
        pub fn new() -> Self {
            let t_cfg: z3::Config   = Config::new();
            let t_ctx: z3::Context  = Context::new(&t_cfg);
            
            let spec = Spec{
                cfg: t_cfg,
                ctx: t_ctx
            };
            spec
        }
    }

//}

