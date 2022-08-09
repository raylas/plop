use std::error::Error;
use std::path::Path;
use std::time::Duration;
use structopt::StructOpt;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Region};

/// Upload and share files via S3 presigned URIs
#[derive(Debug, StructOpt)]
struct Options {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,

    /// AWS Region to use
    #[structopt(short, long, env = "AWS_REGION")]
    region: Option<String>,

    /// Lifetime, in seconds, of presigned URI
    #[structopt(short, long, default_value = "1800")]
    expire: u64,

    /// Local file to upload
    file: String,

    /// Name of the bucket
    bucket: String,

    /// Prefix for uploaded objects
    #[structopt(short, long, default_value = "plop/")]
    prefix: String,
}

/// Put object (as `ByteStream`) in bucket, with given parameters.
/// 
/// Return presigned URI as `String`.
async fn plop_object(
    client: &Client,
    expire: u64,
    file: &str,
    bucket: &str,
    prefix: &str,
) -> Result<String, Box<dyn Error>> {
    let expires_in = Duration::from_secs(expire);

    let body = ByteStream::from_path(Path::new(file)).await;

    let path = format!("{}{}", prefix, file);

    client
        .put_object()
        .bucket(bucket)
        .key(&path)
        .body(body?)
        .send()
        .await?;

    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(&path)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    Ok(presigned_request.uri().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Options {
        verbose,
        region,
        expire,
        file,
        bucket,
        prefix,
    } = Options::from_args();

    
    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let presigned_uri = match plop_object(&client, expire, &file, &bucket, &prefix).await {
        Ok(uri) => uri,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    if verbose {
        println!("Region:        {}", shared_config.region().unwrap());
        println!("Bucket:        {}", &bucket);
        println!("Path:          {}{}", &prefix, &file);
        println!("Expires in:    {} seconds", expire);
    }
    
    println!("Presigned URI: {}", presigned_uri);

    Ok(())
}
