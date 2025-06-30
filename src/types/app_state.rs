use solana_client::nonblocking::rpc_client::RpcClient;


pub struct AppState {
    pub app_name: String,
    pub rpc_client: RpcClient,
}