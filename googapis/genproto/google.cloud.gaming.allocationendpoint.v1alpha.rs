#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationRequest {
    /// The required realm name in the following form:
    /// `{location}/{realm}`.
    #[prost(string, tag = "1")]
    pub realm: ::prost::alloc::string::String,
    /// The default game server deployment name.
    /// This is used to increase the likelihood of a successful
    /// allocation.
    #[prost(string, tag = "2")]
    pub default_game_server_deployment: ::prost::alloc::string::String,
    /// The ordered list of game server labels to match for allocations.
    /// If the first game server selector is not matched, the selection attempts
    /// the second game server selector, and so on.
    #[prost(message, repeated, tag = "3")]
    pub game_server_selectors: ::prost::alloc::vec::Vec<GameServerSelector>,
    /// Metadata is optional custom metadata that is added to the game server at
    /// allocation. You can use this to tell the server necessary session data.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<MetaPatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationResponse {
    /// The name of the allocated game server.
    #[prost(string, tag = "1")]
    pub game_server_name: ::prost::alloc::string::String,
    /// The allocated game server's port information.
    #[prost(message, repeated, tag = "2")]
    pub ports: ::prost::alloc::vec::Vec<allocation_response::GameServerStatusPort>,
    /// The address of the allocated game server.
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    /// The node name of the allocated game server.
    #[prost(string, tag = "4")]
    pub node_name: ::prost::alloc::string::String,
    /// The game server cluster from which the game server was allocated.
    #[prost(string, tag = "5")]
    pub game_server_cluster_name: ::prost::alloc::string::String,
    /// The game server deployment from which the game server was allocated.
    #[prost(string, tag = "6")]
    pub deployment_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AllocationResponse`.
pub mod allocation_response {
    /// The game server port info that is allocated.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GameServerStatusPort {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub port: i32,
    }
}
/// MetaPatch is the metadata used to patch the Game Server metadata on
/// allocation. It behaves exactly as it does in OSS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaPatch {
    #[prost(map = "string, string", tag = "1")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(map = "string, string", tag = "2")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// GameServerSelector used for finding a GameServer with matching labels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerSelector {
    /// Labels to match.
    #[prost(map = "string, string", tag = "1")]
    pub match_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Generated client implementations.
pub mod allocation_endpoint_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AllocationEndpointServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AllocationEndpointServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AllocationEndpointServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AllocationEndpointServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Proxy allocation service to the Game Server Clusters.
        pub async fn allocate(
            &mut self,
            request: impl tonic::IntoRequest<super::AllocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllocationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.allocationendpoint.v1alpha.AllocationEndpointService/Allocate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.gaming.allocationendpoint.v1alpha.AllocationEndpointService",
                        "Allocate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
