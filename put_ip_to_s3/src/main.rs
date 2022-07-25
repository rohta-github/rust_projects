use aws_sdk_s3::types::ByteStream;
use aws_smithy_http::body::SdkBody;
use aws_sdk_s3::{Client, Error};

async fn upload_object(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;

    for bucket in resp.buckets().unwrap_or_default() {
        println!("bucket: {:?}", bucket.name().unwrap_or_default())
    }

    println!();

    let body = ByteStream::new(SdkBody::from(get_local_ip().unwrap()));

    let resp = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;

    println!("Upload success. Version: {:?}", resp.version_id);

    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    let data = resp.body.collect().await;
    println!("data: {:?}", data.unwrap().into_bytes());

    Ok(())
}

fn get_ip_list() -> Vec<String> {
    use pnet::datalink;
    let mut ips: Vec<String> = Vec::new();
    for interface in datalink::interfaces() {
        // 空ではなく、動作中である。
        if !interface.ips.is_empty() && interface.is_up() {
            for ip_net in interface.ips {
                // ループバックでなく、ipv4である
                if ip_net.is_ipv4() && !ip_net.ip().is_loopback() {
                    ips.push(ip_net.ip().to_string());
                }
            }
        }
    };
    ips
}

fn get_local_ip() -> Option<String> {
    let local_ip_header = "192";
    for ip in get_ip_list().into_iter().filter(|ip| ip.find(local_ip_header) == Some(0)) {
        if ip.find(local_ip_header) == Some(0) {
            return Some(ip)
        }
    }
    return None
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let bucket = "hoge-20220725";
    let key = "index.html";

    upload_object(&client, &bucket, &key).await
}