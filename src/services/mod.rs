pub mod server;
pub mod server_member;
pub mod user;

pub mod model {
    tonic::include_proto!("ycchat.model");
}

pub mod ycchat_user {
    tonic::include_proto!("ycchat.user");
}

pub mod ycchat_server {
    tonic::include_proto!("ycchat.server");

    pub mod member {
        tonic::include_proto!("ycchat.server.member");
    }
}
