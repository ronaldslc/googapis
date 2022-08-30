/// Request to get a vulnerability summary for some set of occurrences.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVulnerabilityOccurrencesSummaryRequest {
    /// The name of the project to get a vulnerability summary for in the form of
    /// `projects/\[PROJECT_ID\]`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter expression.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
}
/// A summary of how many vulnerability occurrences there are per resource and
/// severity type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityOccurrencesSummary {
    /// A listing by resource of the number of fixable and total vulnerabilities.
    #[prost(message, repeated, tag="1")]
    pub counts: ::prost::alloc::vec::Vec<vulnerability_occurrences_summary::FixableTotalByDigest>,
}
/// Nested message and enum types in `VulnerabilityOccurrencesSummary`.
pub mod vulnerability_occurrences_summary {
    /// Per resource and severity counts of fixable and total vulnerabilities.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixableTotalByDigest {
        /// The affected resource.
        #[prost(string, tag="1")]
        pub resource_uri: ::prost::alloc::string::String,
        /// The severity for this count. SEVERITY_UNSPECIFIED indicates total across
        /// all severities.
        #[prost(enumeration="super::super::super::super::super::grafeas::v1::Severity", tag="2")]
        pub severity: i32,
        /// The number of fixable vulnerabilities associated with this resource.
        #[prost(int64, tag="3")]
        pub fixable_count: i64,
        /// The total number of vulnerabilities associated with this resource.
        #[prost(int64, tag="4")]
        pub total_count: i64,
    }
}
/// Generated client implementations.
pub mod container_analysis_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Retrieves analysis results of Cloud components such as Docker container
    /// images. The Container Analysis API is an implementation of the
    /// [Grafeas](https://grafeas.io) API.
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
    pub struct ContainerAnalysisClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ContainerAnalysisClient<T>
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
        ) -> ContainerAnalysisClient<InterceptedService<T, F>>
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
            ContainerAnalysisClient::new(InterceptedService::new(inner, interceptor))
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
        ) -> Result<
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
                "/google.devtools.containeranalysis.v1.ContainerAnalysis/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
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
        ) -> Result<
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
                "/google.devtools.containeranalysis.v1.ContainerAnalysis/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
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
        ) -> Result<
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
                "/google.devtools.containeranalysis.v1.ContainerAnalysis/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a summary of the number and severity of occurrences.
        pub async fn get_vulnerability_occurrences_summary(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetVulnerabilityOccurrencesSummaryRequest,
            >,
        ) -> Result<
            tonic::Response<super::VulnerabilityOccurrencesSummary>,
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
                "/google.devtools.containeranalysis.v1.ContainerAnalysis/GetVulnerabilityOccurrencesSummary",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
