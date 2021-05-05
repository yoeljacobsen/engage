use clap::{ArgGroup, Clap};
use config::File;
//use std::sync::RwLock;
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Clap, Debug, Serialize, Deserialize)]
#[clap(author, about, version)]
#[clap(group = ArgGroup::new("CONTAINER").required(false))]
pub struct Opts {
    // Test flags
    #[clap(long = "verify_cred", about = "Just verify the credentials work")]
    pub verify_cred: bool,

    #[clap(long = "debug", about = "Print debugging informagtion")]
    pub debug: bool,

    // Computational Flags
    #[clap(
        short = 'c',
        long = "cores",
        about = "Number of CPU cores",
        default_value = "4"
    )]
    pub cores: i32,

    #[clap(
        short = 'm',
        long = "mem",
        about = "RAM size in GB",
        default_value = "8"
    )]
    pub mem: i32,

    #[clap(short = 'g', long = "gpus", about = "Number of GPU devices", requires_all = &["gpu_type", "framework"])]
    pub gpus: Option<i32>,

    #[clap(name = "gpu_type", long = "gpu_type", about = "GPU Type", arg_enum)]
    pub gpu_type: Option<GPUType>,

    // Computational Runtime
    #[clap(short = 'f', long = "framework", arg_enum, group = "CONTAINER")]
    pub framework: Option<Framework>,

    #[clap(long = "container", group = "CONTAINER")]
    pub container: Option<String>,

    // Environment
    #[clap(short = 'u', long = "user", default_value = "PI")]
    pub user: String,

    #[clap(long = "working_directory", env = "PWD", required = false)]
    pub wd: String,

    #[clap(short = 'w', long = "wait", about = "Block until job is finished?")]
    pub wait: bool,

    // AWS Resources
    #[clap(
        short = 's',
        long = "spot",
        about = "Use SPOT (true) or On-Demand (false)"
    )]
    pub spot: bool,

    #[clap(long = "region", default_value = "us-west-1")]
    pub region: String,

    // Job meta-data
    #[clap(short = 'n', long = "name", default_value = "Unnamed job")]
    pub name: String,

    #[clap(long = "monitor", about = "Should the job be monitored", requires_all = &["monitor_interval", "emails"])]
    pub monitor: bool,

    #[clap(
        name = "monitor_interval",
        long = "monitor_interval",
        about = "Intervals for status mails",
        default_value_if("monitor", None, "1")
    )]
    monitor_interval: Option<i32>,

    #[clap(
        long = "emails",
        multiple = true,
        about = "e-mail addresses for status notifications"
    )]
    pub emails: Option<Vec<String>>,

    // General
    #[clap(short = 'v', long = "verbose", parse(from_occurrences))]
    pub verbose: i32,

    // The command
    pub command: String,
}

#[derive(Clap, Debug, PartialEq, Serialize, Deserialize)]
pub enum Framework {
    Pytorch12,
    Pytorch14,
    Tensorflow30,
}

#[derive(Clap, Debug, PartialEq, Serialize, Deserialize)]
pub enum GPUType {
    V100,
    K80,
    T4,
}

fn update_with_config(opts: Opts) -> Opts {
    let mut settings = Config::default();
    // The order of the next two lines determines which source overrides which
    settings.merge(Config::try_from(&opts).unwrap()).unwrap();

    settings.merge(File::with_name("Settings.toml")).unwrap();
    //println!("{:?}", settings);
    let as_opts: Opts = settings.try_into().unwrap();
    as_opts
}

pub fn parse() -> Opts {
    update_with_config(Opts::parse())
}

pub fn count_params(s: &String) -> i32 {
    let mut counter: i32 = 0;
    let mut current: i32 = 0;

    for c in s.chars() {
        if c == '(' {
            current += 1;
            if current > counter {
                counter = current
            }
        } else if c == ')' {
            current -= 1
        }
    }
    println!("Balance: {}", current);
    counter
}
