/// A scan configuration specifies whether Cloud components in a project have a
/// particular type of analysis being run. For example, it can configure whether
/// vulnerability scanning is being done on Docker images or not.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanConfig {
    /// Output only. The name of the scan configuration in the form of
    /// `projects/\[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. A human-readable description of what the scan configuration
    /// does.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Whether the scan is enabled.
    #[prost(bool, tag = "3")]
    pub enabled: bool,
    /// Output only. The time this scan config was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this scan config was last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request to get a scan configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScanConfigRequest {
    /// Required. The name of the scan configuration in the form of
    /// `projects/\[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list scan configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScanConfigsRequest {
    /// Required. The name of the project to list scan configurations for in the form of
    /// `projects/\[PROJECT_ID\]`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The filter expression.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The number of scan configs to return in the list.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing scan configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScanConfigsResponse {
    /// The scan configurations requested.
    #[prost(message, repeated, tag = "1")]
    pub scan_configs: ::prost::alloc::vec::Vec<ScanConfig>,
    /// The next pagination token in the list response. It should be used as
    /// `page_token` for the following request. An empty value means no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to update a scan configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateScanConfigRequest {
    /// Required. The name of the scan configuration in the form of
    /// `projects/\[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The updated scan configuration.
    #[prost(message, optional, tag = "2")]
    pub scan_config: ::core::option::Option<ScanConfig>,
}
/// Generated client implementations.
pub mod container_analysis_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Retrieves analysis results of Cloud components such as Docker container
    /// images. The Container Analysis API is an implementation of the
    /// [Grafeas](grafeas.io) API.
    ///
    /// Analysis results are stored as a series of occurrences. An `Occurrence`
    /// contains information about a specific analysis instance on a resource. An
    /// occurrence refers to a `Note`. A note contains details describing the
    /// analysis and is generally stored in a separate project, called a `Provider`.
    /// Multiple occurrences can refer to the same note.
    ///
    /// For example, an SSL vulnerability could affect multiple images. In this case,
    /// there would be one note for the vulnerability and an occurrence for each
    /// image with the vulnerability referring to that note.
    #[derive(Debug, Clone)]
    pub struct ContainerAnalysisV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ContainerAnalysisV1Beta1Client<T>
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
        ) -> ContainerAnalysisV1Beta1Client<InterceptedService<T, F>>
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
            ContainerAnalysisV1Beta1Client::new(
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
        /// Sets the access control policy on the specified note or occurrence.
        /// Requires `containeranalysis.notes.setIamPolicy` or
        /// `containeranalysis.occurrences.setIamPolicy` permission if the resource is
        /// a note or an occurrence, respectively.
        ///
        /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
        /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
        /// occurrences.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a note or an occurrence resource.
        /// Requires `containeranalysis.notes.setIamPolicy` or
        /// `containeranalysis.occurrences.setIamPolicy` permission if the resource is
        /// a note or occurrence, respectively.
        ///
        /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
        /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
        /// occurrences.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the permissions that a caller has on the specified note or
        /// occurrence. Requires list permission on the project (for example,
        /// `containeranalysis.notes.list`).
        ///
        /// The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for
        /// notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for
        /// occurrences.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the specified scan configuration.
        pub async fn get_scan_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScanConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ScanConfig>, tonic::Status> {
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GetScanConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "GetScanConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists scan configurations for the specified project.
        pub async fn list_scan_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListScanConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListScanConfigsResponse>,
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/ListScanConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "ListScanConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified scan configuration.
        pub async fn update_scan_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateScanConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ScanConfig>, tonic::Status> {
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/UpdateScanConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1",
                        "UpdateScanConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
