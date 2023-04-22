/// The ReportPhishing request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportPhishingRequest {
    /// Required. The name of the project for which the report will be created,
    /// in the format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The URI that is being reported for phishing content to be analyzed.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
/// The ReportPhishing (empty) response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportPhishingResponse {}
/// Generated client implementations.
pub mod phishing_protection_service_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to report phishing URIs.
    #[derive(Debug, Clone)]
    pub struct PhishingProtectionServiceV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PhishingProtectionServiceV1Beta1Client<T>
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
        ) -> PhishingProtectionServiceV1Beta1Client<InterceptedService<T, F>>
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
            PhishingProtectionServiceV1Beta1Client::new(
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
        /// Reports a URI suspected of containing phishing content to be reviewed. Once
        /// the report review is complete, its result can be found in the Cloud
        /// Security Command Center findings dashboard for Phishing Protection. If the
        /// result verifies the existence of malicious phishing content, the site will
        /// be added the to [Google's Social Engineering
        /// lists](https://support.google.com/webmasters/answer/6350487/) in order to
        /// protect users that could get exposed to this threat in the future.
        pub async fn report_phishing(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportPhishingRequest>,
        ) -> Result<tonic::Response<super::ReportPhishingResponse>, tonic::Status> {
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
                "/google.cloud.phishingprotection.v1beta1.PhishingProtectionServiceV1Beta1/ReportPhishing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
