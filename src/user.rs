use tonic::{Request, Response, Status};
use uuid::Uuid;

use proto::v1::{
    CreateUserRequest, CreateUserResponse, GetUserRequest, GetUserResponse,
    user_service_server::UserServiceServer,
};

use crate::store::Store;
use crate::validate;

mod proto {
    pub mod v1 {
        tonic::include_proto!("user.v1");
    }
}

#[derive(Debug, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
}

type UserStore = Store<User>;

pub struct UserService {
    store: UserStore,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            store: Store::new(),
        }
    }
}

impl Default for UserService {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl proto::v1::user_service_server::UserService for UserService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        validate::execute(request, |req| async {
            let req = req.into_inner();
            let id = Uuid::new_v4().to_string();

            self.store
                .write(
                    id.clone(),
                    User {
                        id: id.clone(),
                        name: req.name,
                        email: req.email,
                    },
                )
                .await;

            Ok(Response::new(CreateUserResponse { id }))
        })
        .await
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();

        if let Some(user) = self.store.read(&req.id).await {
            Ok(Response::new(GetUserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
            }))
        } else {
            Err(Status::not_found(format!("User not found: {}", req.id)))
        }
    }
}

pub fn get_service() -> UserServiceServer<UserService> {
    UserServiceServer::new(UserService::new())
}
