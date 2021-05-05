use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3};
use std::time::Duration;

pub async fn verify_cred(region: Region) -> bool {
    let mut provider = ChainProvider::new();
    let mut result: bool = false;

    provider.set_timeout(Duration::from_secs(200));
    let s3client = S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        provider,
        region,
    );
    println!("Attempting to verify credentials using S3 bucket listing");
    match s3client.list_buckets().await {
        Err(e) => println!("Error listing buckets: {}", e),
        Ok(output) => {
            result = true;
            match output.buckets {
                None => println!("Successful call but no buckets"),
                Some(bucket_list) => {
                    for bucket in bucket_list {
                        println!("{:?}\t{:?}", bucket.name.unwrap(), bucket.creation_date.unwrap());
                    }
                }
            }
        }
    };
    result
}
