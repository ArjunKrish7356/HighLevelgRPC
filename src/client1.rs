use proto::sorting_client::SortingClient;
use std::error::Error;

pub mod proto {
    tonic::include_proto!("sorting");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = SortingClient::connect(url).await?;

    let numbers = vec![1, 7, 2, 8, 12, 5, 14, 28];

    let req = proto::SortRequest { numbers: numbers };
    let request = tonic::Request::new(req);

    let response = client.sort(request).await?;

    println!("Response: {:?}", response.get_ref().numbers);

    Ok(())
}