use elasticsearch::cat::CatIndicesParts;
use elasticsearch::Elasticsearch;
use elasticsearch::http::transport::Transport;
use serde_json::Value;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let transport = Transport::single_node("http://10.197.31.59:9200")?;
    let client = Elasticsearch::new(transport);

    let response = client
      .cat()
      .indices(CatIndicesParts::Index(&["*"]))
      .send()
      .await?;

    let response_body = response.json::<Value>().await?;

    for record  in response_body.as_array().unwrap() {
        println!("{}", record["index"].as_str().unwrap());
    }

    println!("Hello, world!");

    Ok(())
}
