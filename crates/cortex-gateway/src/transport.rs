/// Transport layer abstraction: Streamable HTTP, SSE, gRPC, WebSocket.
pub enum Transport {
    Http,
    Sse,
    Grpc,
    WebSocket,
}
