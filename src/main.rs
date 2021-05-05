use engage::args;
use engage::verify_cred::verify_cred;
use engage::aws_batch::foo;
use rusoto_core::Region;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let opts: args::Opts = args::parse();
    println!("{:?}", opts);
    let region = Region::from_str(&opts.region).unwrap_or_default();

    if opts.verify_cred {
        match verify_cred(region.clone()).await {
            true => println!("Credentials seems to work"),
            false => println!("Problems with credentials"),
        }
    }

   foo(region.clone()).await;
}
