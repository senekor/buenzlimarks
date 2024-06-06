#[tarpc::service]
pub trait Rpc {
    async fn hello(name: String) -> String;
}
