use tonic::{Request, Response, Status};

use proto::v1::{
    CreateUserRequest, CreateUserResponse, GetUserRequest, GetUserResponse,
    user_service_server::UserServiceServer,
};

mod proto {
    pub mod v1 {
        tonic::include_proto!("user.v1");
    }
}

#[derive(Debug, Default)]
pub struct UserService {}

#[tonic::async_trait]
impl proto::v1::user_service_server::UserService for UserService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        Ok(Response::new(CreateUserResponse {
            id: "1".to_string(),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        Ok(Response::new(GetUserResponse {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "".to_string(),
        }))
    }
}

pub fn get_service() -> UserServiceServer<UserService> {
    UserServiceServer::new(UserService::default())
}
