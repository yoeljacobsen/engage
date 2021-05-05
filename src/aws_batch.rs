use rusoto_batch::{Batch, BatchClient, DescribeJobDefinitionsRequest};
use rusoto_core::Region;
use crate::args::Opts;

pub async fn foo (region: Region) {
 let client = BatchClient::new(region);
    let request = DescribeJobDefinitionsRequest::default();
    let response = client.describe_job_definitions(request).await.unwrap();

    println!("Response: {:?}", response);
}

async fn ensure_compatible_ce (opts: &Opts) {
    println!("Ensuring proper ce in region: {}", opts.region)
}

async fn ensure_compatible_queue (opts: &Opts) {
    println!("Ensuring proper queue in region: {}", opts.region)
}

async fn ensure_compatible_jd (opts: &Opts) {
    println!("Ensuring proper job definition in region: {}", opts.region)
}

pub async fn ensure_compatible_batch_elements (opts: &Opts) {
    println!("Ensuring proper batch elements in region: {}, using: ", opts.region);
    ensure_compatible_ce(&opts).await;
    ensure_compatible_queue(&opts).await;
    ensure_compatible_jd(&opts).await;
}
