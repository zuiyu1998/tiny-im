use crate::{manager::Manager, Result};
use abi::{
    config::{Config, ServiceType},
    pb::message::{
        msg_service_server::{MsgService, MsgServiceServer},
        ChatMsg, SendMsgResponse,
    },
    tonic::{async_trait, transport::Server, Request, Response, Status},
    tracing,
    utils::register_service,
};

pub struct MsgRpcService {
    manager: Manager,
}

impl MsgRpcService {
    pub fn new(manager: Manager) -> Self {
        Self { manager }
    }

    pub async fn start(manager: Manager, config: &Config) -> Result<()> {
        register_service(config, ServiceType::Msg);
        tracing::info!("<connect> rpc service register to service register center");

        let service = Self::new(manager);
        let svc = MsgServiceServer::new(service);
        tracing::info!(
            "<connect> rpc service started at {}",
            config.rpc.msg.rpc_server_url()
        );

        Server::builder()
            .add_service(svc)
            .serve(config.rpc.msg.rpc_server_url().parse().unwrap())
            .await
            .unwrap();
        Ok(())
    }
}

#[async_trait]
impl MsgService for MsgRpcService {
    async fn send_message(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let msg = request.into_inner();

        self.manager.broadcast(msg).await;
        let response = Response::new(SendMsgResponse {});
        Ok(response)
    }

    async fn send_msg_to_user(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let msg = request.into_inner();

        self.manager.send_single_msg(&msg.sender_id, &msg).await;

        let response = Response::new(SendMsgResponse {});
        Ok(response)
    }

    async fn send_group_msg_to_user(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let _msg = request.into_inner();

        // self.manager.send_group(&msg.sender_id, &msg).await;

        todo!()
    }
}
