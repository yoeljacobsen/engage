use engage::args;
use rusoto_batch::{Batch, BatchClient, DescribeJobDefinitionsRequest};
use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3};
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let opts: args::Opts = args::parse();
    println!("{:?}", opts);
    let region = Region::from_str(&opts.region).unwrap_or_default();

    if opts.verify_cred {
        let mut provider = ChainProvider::new();
        provider.set_timeout(Duration::from_secs(200));
        let s3client = S3Client::new_with(
            HttpClient::new().expect("failed to create request dispatcher"),
            provider,
            region,
        );
        println!("Attempting to verify credentials using S3 bucket listing");
        match s3client.list_buckets().await {
            Err(e) => println!("Error listing buckets: {}", e),
            Ok(output) => match output.buckets {
                None => println!("Successful call but no buckets"),
                Some(bucket_list) => {
                    for bucket in bucket_list {
                        println!("{:?}\t{:?}", bucket.name, bucket.creation_date);
                    }
                }
            },
        };
    }

    let client = BatchClient::new(Region::UsWest1);
    let request = DescribeJobDefinitionsRequest::default();
    let response = client.describe_job_definitions(request).await.unwrap();

    println!("Response: {:?}", response);
    println!("Depth: {}", args::count_params(&opts.command));
}
