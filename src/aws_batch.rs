use rusoto_batch::{Batch, BatchClient, DescribeJobDefinitionsRequest};
use rusoto_core::Region;

pub async fn foo (region: Region) {
 let client = BatchClient::new(region);
    let request = DescribeJobDefinitionsRequest::default();
    let response = client.describe_job_definitions(request).await.unwrap();

    println!("Response: {:?}", response);
}