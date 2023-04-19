use std::net::SocketAddr;

use hyper::Server;
use tokio::runtime::Runtime;

fn main() -> Result<(), Error> {
    run()
}

#[derive(Debug, thiserror::Error)]
enum Error {
}

fn run() -> Result<(), Error> {
    let tokio_runtime = Runtime::new().expect("Failed to build tokio runtime");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let mut servers = Vec::new();
    servers.push(Server::bind(&addr));
        

    //tokio_runtime.block_on(join_all(servers.into_iter().map(|server| {
    //    server.serve(make_service_fn(|_addr_stream| {
    //        ready(Result::<>::Ok(AqaService::new()))
    //    })
    //}))?;

    Ok(())
}
