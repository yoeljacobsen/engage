use clap::{Clap, ArgGroup};

#[derive(Clap, Debug)]
#[clap(author, about, version)]
#[clap(group = ArgGroup::new("CONTAINER").required(false))]
pub struct Opts {
    #[clap(short = 'c', long = "cores", about = "Number of CPU cores", default_value = "4")]
    pub cores: i32,
    #[clap(short = 'm', long = "mem", about = "RAM size in GB", default_value = "8")]
    pub mem: i32,
    #[clap(short = 'g', long = "gpus", about = "Number of GPU devices", requires_all = &["gpu_type", "framework"])]
    pub gpus: Option<i32>,
    #[clap(name = "gpu_type", long = "gpu_type", about = "GPU Type", arg_enum)]
    pub gpu_type: Option<GPUType>,
    #[clap(short = 'n', long = "name", default_value = "Unnamed job")]
    pub name: String,
    #[clap(short = 'v', long = "verbose", parse(from_occurrences))]
    pub verbose: i32,
    #[clap(short = 'f', long = "framework", arg_enum, group = "CONTAINER")]
    pub framework: Option<Framework>,
    #[clap(long = "container", group = "CONTAINER")]
    pub container: Option<String>,
    #[clap(short = 'u', long = "user", default_value = "PI")]
    pub user: String,
    #[clap(long = "working_directory", env = "PWD", required = false)]
    pub wd: String,
    #[clap(short = 'w', long = "wait", about = "Block until job is finished?")]
    pub wait: bool,
    #[clap(short = 's', long = "spot", about = "Use SPOT (true) or On-Demand (false)")]
    pub spot: bool,
    #[clap(long = "monitor", about = "Should the job be monitored", requires_all = &["monitor_interval", "emails"])]
    pub monitor: bool,
    #[clap(name = "monitor_interval",
           long = "monitor_interval",
           about = "Intervals for status mails",
           default_value_if("monitor", None, "1"))]
    monitor_interval: Option<i32>,
    #[clap(long = "emails", multiple = true, about = "e-mail addresses for status notifications")]
    pub emails: Option<Vec<String>>,
    pub command: String,
}

#[derive(Clap, Debug, PartialEq)]
pub enum Framework {
    Pytorch12,
    Pytorch14,
    Tensorflow30
}

#[derive(Clap, Debug, PartialEq)]
pub enum GPUType {
    V100,
    K80,
    T4
}

pub fn parse() -> Opts {
    Opts::parse()
}
pub fn count_params(s: &String) -> i32 {
    let mut counter: i32 = 0;
    let mut current: i32 = 0;

    for c in s.chars() {
        if c == '(' {
            current += 1;
            if current > counter { counter = current }
        } else if c == ')' {
            current -= 1
        }
    }
    println!("Balance: {}", current);
    counter
}

