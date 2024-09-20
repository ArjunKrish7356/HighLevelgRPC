use proto::sorting_server::{Sorting , SortingServer};
use proto::{SortRequest , SortResponse};
use tonic::transport::Server;

mod proto{
    tonic::include_proto!("sorting");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("sorting_descriptor");
}

#[derive(Default,Debug)]
struct SortService {

}


#[tonic::async_trait]
impl Sorting for SortService {
    async fn sort(
        &self,
        request: tonic::Request<SortRequest>,
    ) -> Result< tonic::Response<SortResponse>,tonic::Status> {

        let mut numbers = request.get_ref().numbers.clone();
        linear_sort(&mut numbers);

        let response = SortResponse {
            numbers: numbers,
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let sor = SortService {} ;
    
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(SortingServer::new(sor))
        .serve(addr)
        .await?;

    Ok(())
}


fn linear_sort(arr: &mut Vec<i32>) {
    if arr.is_empty() { return; } // Handle empty arrays

    // Find the maximum value in the array
    let max_value = arr.iter().max().unwrap(); 

    // Create a count array to store the frequency of each element
    let mut count = vec![0; (*max_value + 1) as usize]; 
    for &num in arr.iter() {
        count[num as usize] += 1;
    }

    // Modify the count array to store the actual position of each element in the output
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    // Build the output array
    let mut output = vec![0; arr.len()];
    for &num in arr.iter().rev() { // Iterate in reverse to maintain stability
        output[count[num as usize] - 1] = num;
        count[num as usize] -= 1;
    }

    // Copy the sorted elements back to the original array
    arr.copy_from_slice(&output);
}