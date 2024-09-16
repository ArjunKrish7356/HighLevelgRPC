use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BTCPaymentRequest, BTCPaymentResponse};

pub mod payments{
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService{
    async fn send_payment(
        &self,
        request: Request<BTCPaymentRequest>,
    ) -> Result<Response<BTCPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);
        
        let req = request.into_inner();

        let reply = payments::BTCPaymentResponse {
            successfull : true,
            message : format!("Payment of {} BTC to {} was successfull", req.amount, req.address),
        };

        Ok(Response::new(reply))
    }
}