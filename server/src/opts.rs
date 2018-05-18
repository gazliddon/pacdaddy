use opts;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opts {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8, 

    #[structopt(short = "h", long = "host")]
    host: Option<String>,

    #[structopt(short = "p", long = "port")]
    port: Option<u32>,

    // Refresh rate in millis
    #[structopt(short = "r", long = "rate")]
    rate: Option<u32>,


}

impl Opts {
    pub fn new() -> Self {
        use std::env;
        use env_logger;
        use structopt::StructOpt;
        let opts = opts::Opts::from_args();
        if opts.verbose > 0 {
            env::set_var("RUST_LOG", "info");
        }

        env_logger::init();
        opts
    }

    pub fn get_rate_millis(&self) -> u64 {
        self.rate.unwrap_or(17) as u64
    }

    pub fn get_host(&self) -> String {
        self.host.clone().unwrap_or("localhost".to_string())
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(6502)
    }
}
