/// Request object for ByteStream.Read.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    /// The name of the resource to read.
    #[prost(string, tag="1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The offset for the first byte to return in the read, relative to the start
    /// of the resource.
    ///
    /// A `read_offset` that is negative or greater than the size of the resource
    /// will cause an `OUT_OF_RANGE` error.
    #[prost(int64, tag="2")]
    pub read_offset: i64,
    /// The maximum number of `data` bytes the server is allowed to return in the
    /// sum of all `ReadResponse` messages. A `read_limit` of zero indicates that
    /// there is no limit, and a negative `read_limit` will cause an error.
    ///
    /// If the stream returns fewer bytes than allowed by the `read_limit` and no
    /// error occurred, the stream includes all data from the `read_offset` to the
    /// end of the resource.
    #[prost(int64, tag="3")]
    pub read_limit: i64,
}
/// Response object for ByteStream.Read.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    /// A portion of the data for the resource. The service **may** leave `data`
    /// empty for any given `ReadResponse`. This enables the service to inform the
    /// client that the request is still live while it is running an operation to
    /// generate more data.
    #[prost(bytes="vec", tag="10")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Request object for ByteStream.Write.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    /// The name of the resource to write. This **must** be set on the first
    /// `WriteRequest` of each `Write()` action. If it is set on subsequent calls,
    /// it **must** match the value of the first request.
    #[prost(string, tag="1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The offset from the beginning of the resource at which the data should be
    /// written. It is required on all `WriteRequest`s.
    ///
    /// In the first `WriteRequest` of a `Write()` action, it indicates
    /// the initial offset for the `Write()` call. The value **must** be equal to
    /// the `committed_size` that a call to `QueryWriteStatus()` would return.
    ///
    /// On subsequent calls, this value **must** be set and **must** be equal to
    /// the sum of the first `write_offset` and the sizes of all `data` bundles
    /// sent previously on this stream.
    ///
    /// An incorrect value will cause an error.
    #[prost(int64, tag="2")]
    pub write_offset: i64,
    /// If `true`, this indicates that the write is complete. Sending any
    /// `WriteRequest`s subsequent to one in which `finish_write` is `true` will
    /// cause an error.
    #[prost(bool, tag="3")]
    pub finish_write: bool,
    /// A portion of the data for the resource. The client **may** leave `data`
    /// empty for any given `WriteRequest`. This enables the client to inform the
    /// service that the request is still live while it is running an operation to
    /// generate more data.
    #[prost(bytes="vec", tag="10")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Response object for ByteStream.Write.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
    /// The number of bytes that have been processed for the given resource.
    #[prost(int64, tag="1")]
    pub committed_size: i64,
}
/// Request object for ByteStream.QueryWriteStatus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWriteStatusRequest {
    /// The name of the resource whose write status is being requested.
    #[prost(string, tag="1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Response object for ByteStream.QueryWriteStatus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWriteStatusResponse {
    /// The number of bytes that have been processed for the given resource.
    #[prost(int64, tag="1")]
    pub committed_size: i64,
    /// `complete` is `true` only if the client has sent a `WriteRequest` with
    /// `finish_write` set to true, and the server has processed that request.
    #[prost(bool, tag="2")]
    pub complete: bool,
}
/// Generated client implementations.
pub mod byte_stream_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// #### Introduction
    ///
    /// The Byte Stream API enables a client to read and write a stream of bytes to
    /// and from a resource. Resources have names, and these names are supplied in
    /// the API calls below to identify the resource that is being read from or
    /// written to.
    ///
    /// All implementations of the Byte Stream API export the interface defined here:
    ///
    /// * `Read()`: Reads the contents of a resource.
    ///
    /// * `Write()`: Writes the contents of a resource. The client can call `Write()`
    ///   multiple times with the same resource and can check the status of the write
    ///   by calling `QueryWriteStatus()`.
    ///
    /// #### Service parameters and metadata
    ///
    /// The ByteStream API provides no direct way to access/modify any metadata
    /// associated with the resource.
    ///
    /// #### Errors
    ///
    /// The errors returned by the service are in the Google canonical error space.
    #[derive(Debug, Clone)]
    pub struct ByteStreamClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ByteStreamClient<T>
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
        ) -> ByteStreamClient<InterceptedService<T, F>>
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
            ByteStreamClient::new(InterceptedService::new(inner, interceptor))
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
        /// `Read()` is used to retrieve the contents of a resource as a sequence
        /// of bytes. The bytes are returned in a sequence of responses, and the
        /// responses are delivered as the results of a server-side streaming RPC.
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReadResponse>>,
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
                "/google.bytestream.ByteStream/Read",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// `Write()` is used to send the contents of a resource as a sequence of
        /// bytes. The bytes are sent in a sequence of request protos of a client-side
        /// streaming RPC.
        ///
        /// A `Write()` action is resumable. If there is an error or the connection is
        /// broken during the `Write()`, the client should check the status of the
        /// `Write()` by calling `QueryWriteStatus()` and continue writing from the
        /// returned `committed_size`. This may be less than the amount of data the
        /// client previously sent.
        ///
        /// Calling `Write()` on a resource name that was previously written and
        /// finalized could cause an error, depending on whether the underlying service
        /// allows over-writing of previously written resources.
        ///
        /// When the client closes the request channel, the service will respond with
        /// a `WriteResponse`. The service will not view the resource as `complete`
        /// until the client has sent a `WriteRequest` with `finish_write` set to
        /// `true`. Sending any requests on a stream after sending a request with
        /// `finish_write` set to `true` will cause an error. The client **should**
        /// check the `WriteResponse` it receives to determine how much data the
        /// service was able to commit and whether the service views the resource as
        /// `complete` or not.
        pub async fn write(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::WriteRequest>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status> {
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
                "/google.bytestream.ByteStream/Write",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// `QueryWriteStatus()` is used to find the `committed_size` for a resource
        /// that is being written, which can then be used as the `write_offset` for
        /// the next `Write()` call.
        ///
        /// If the resource does not exist (i.e., the resource has been deleted, or the
        /// first `Write()` has not yet reached the service), this method returns the
        /// error `NOT_FOUND`.
        ///
        /// The client **may** call `QueryWriteStatus()` at any time to determine how
        /// much data has been processed for this resource. This is useful if the
        /// client is buffering data and needs to know which data can be safely
        /// evicted. For any sequence of `QueryWriteStatus()` calls for a given
        /// resource name, the sequence of returned `committed_size` values will be
        /// non-decreasing.
        pub async fn query_write_status(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWriteStatusRequest>,
        ) -> Result<tonic::Response<super::QueryWriteStatusResponse>, tonic::Status> {
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
                "/google.bytestream.ByteStream/QueryWriteStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
