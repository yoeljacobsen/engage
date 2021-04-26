use engage::args;
use rusoto_batch::{Batch, BatchClient, DescribeJobDefinitionsRequest};
//use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use rusoto_core::Region;

#[tokio::main]
async fn main() {
    let opts: args::Opts = args::parse();
    println!("{:?}", opts);

    let client = BatchClient::new(Region::UsWest1);
    let request = DescribeJobDefinitionsRequest::default();
    let response = client.describe_job_definitions(request).await.unwrap();

    println!("Resonse: {:?}", response);
    println!("Depth: {}", args::count_params(&opts.command));
}
