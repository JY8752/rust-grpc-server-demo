use std::future::Future;

use bytes::Bytes;
use prost::Message;
use proto_types::Status as GrpcStatus;
use protocheck::ProtoValidator;
use tonic::{Code, Request, Response, Status};

pub async fn execute<T, F, Fut, R>(req: Request<T>, handler: F) -> Result<Response<R>, Status>
where
    T: ProtoValidator,
    F: FnOnce(Request<T>) -> Fut,
    Fut: Future<Output = Result<Response<R>, Status>>,
{
    if let Err(violations) = req.get_ref().validate() {
        let status_inner: GrpcStatus = violations.into();

        let status = Status::with_details(
            Code::InvalidArgument,
            "Validation Error",
            Bytes::from(status_inner.encode_to_vec()),
        );

        return Err(status);
    }

    handler(req).await
}
