/// A running instance of a
/// \[Workflow\](/workflows/docs/reference/rest/v1/projects.locations.workflows).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Execution {
    /// Output only. The resource name of the execution.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Marks the beginning of execution.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Marks the end of execution, successful or not.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Current state of the execution.
    #[prost(enumeration = "execution::State", tag = "4")]
    pub state: i32,
    /// Input parameters of the execution represented as a JSON string.
    /// The size limit is 32KB.
    ///
    /// *Note*: If you are using the REST API directly to run your workflow, you
    /// must escape any JSON string value of `argument`. Example:
    /// `'{"argument":"{\"firstName\":\"FIRST\",\"lastName\":\"LAST\"}"}'`
    #[prost(string, tag = "5")]
    pub argument: ::prost::alloc::string::String,
    /// Output only. Output of the execution represented as a JSON string. The
    /// value can only be present if the execution's state is `SUCCEEDED`.
    #[prost(string, tag = "6")]
    pub result: ::prost::alloc::string::String,
    /// Output only. The error which caused the execution to finish prematurely.
    /// The value is only present if the execution's state is `FAILED`
    /// or `CANCELLED`.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<execution::Error>,
    /// Output only. Revision of the workflow this execution is using.
    #[prost(string, tag = "8")]
    pub workflow_revision_id: ::prost::alloc::string::String,
    /// The call logging level associated to this execution.
    #[prost(enumeration = "execution::CallLogLevel", tag = "9")]
    pub call_log_level: i32,
}
/// Nested message and enum types in `Execution`.
pub mod execution {
    /// A single stack element (frame) where an error occurred.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StackTraceElement {
        /// The step the error occurred at.
        #[prost(string, tag = "1")]
        pub step: ::prost::alloc::string::String,
        /// The routine where the error occurred.
        #[prost(string, tag = "2")]
        pub routine: ::prost::alloc::string::String,
        /// The source position information of the stack trace element.
        #[prost(message, optional, tag = "3")]
        pub position: ::core::option::Option<stack_trace_element::Position>,
    }
    /// Nested message and enum types in `StackTraceElement`.
    pub mod stack_trace_element {
        /// Position contains source position information about the stack trace
        /// element such as line number, column number and length of the code block
        /// in bytes.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Position {
            /// The source code line number the current instruction was generated from.
            #[prost(int64, tag = "1")]
            pub line: i64,
            /// The source code column position (of the line) the current instruction
            /// was generated from.
            #[prost(int64, tag = "2")]
            pub column: i64,
            /// The number of bytes of source code making up this stack trace element.
            #[prost(int64, tag = "3")]
            pub length: i64,
        }
    }
    /// A collection of stack elements (frames) where an error occurred.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StackTrace {
        /// An array of stack elements.
        #[prost(message, repeated, tag = "1")]
        pub elements: ::prost::alloc::vec::Vec<StackTraceElement>,
    }
    /// Error describes why the execution was abnormally terminated.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        /// Error message and data returned represented as a JSON string.
        #[prost(string, tag = "1")]
        pub payload: ::prost::alloc::string::String,
        /// Human-readable stack trace string.
        #[prost(string, tag = "2")]
        pub context: ::prost::alloc::string::String,
        /// Stack trace with detailed information of where error was generated.
        #[prost(message, optional, tag = "3")]
        pub stack_trace: ::core::option::Option<StackTrace>,
    }
    /// Describes the current state of the execution. More states might be added
    /// in the future.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Invalid state.
        Unspecified = 0,
        /// The execution is in progress.
        Active = 1,
        /// The execution finished successfully.
        Succeeded = 2,
        /// The execution failed with an error.
        Failed = 3,
        /// The execution was stopped intentionally.
        Cancelled = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
    /// Describes the level of platform logging to apply to calls and call
    /// responses during workflow executions.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CallLogLevel {
        /// No call logging specified.
        Unspecified = 0,
        /// Log all call steps within workflows, all call returns, and all exceptions
        /// raised.
        LogAllCalls = 1,
        /// Log only exceptions that are raised from call steps within workflows.
        LogErrorsOnly = 2,
    }
    impl CallLogLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallLogLevel::Unspecified => "CALL_LOG_LEVEL_UNSPECIFIED",
                CallLogLevel::LogAllCalls => "LOG_ALL_CALLS",
                CallLogLevel::LogErrorsOnly => "LOG_ERRORS_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CALL_LOG_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                "LOG_ALL_CALLS" => Some(Self::LogAllCalls),
                "LOG_ERRORS_ONLY" => Some(Self::LogErrorsOnly),
                _ => None,
            }
        }
    }
}
/// Request for the
/// \[ListExecutions][\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsRequest {
    /// Required. Name of the workflow for which the executions should be listed.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of executions to return per call.
    /// Max supported value depends on the selected Execution view: it's 10000 for
    /// BASIC and 100 for FULL. The default value used if the field is not
    /// specified is 100, regardless of the selected view. Values greater than
    /// the max value will be coerced down to it.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListExecutions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListExecutions` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A view defining which fields should be filled in the returned executions.
    /// The API will default to the BASIC view.
    #[prost(enumeration = "ExecutionView", tag = "4")]
    pub view: i32,
}
/// Response for the
/// \[ListExecutions][google.cloud.workflows.executions.v1.Executions.ListExecutions\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsResponse {
    /// The executions which match the request.
    #[prost(message, repeated, tag = "1")]
    pub executions: ::prost::alloc::vec::Vec<Execution>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the
/// \[CreateExecution][google.cloud.workflows.executions.v1.Executions.CreateExecution\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExecutionRequest {
    /// Required. Name of the workflow for which an execution should be created.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    /// The latest revision of the workflow will be used.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Execution to be created.
    #[prost(message, optional, tag = "2")]
    pub execution: ::core::option::Option<Execution>,
}
/// Request for the
/// \[GetExecution][google.cloud.workflows.executions.v1.Executions.GetExecution\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionRequest {
    /// Required. Name of the execution to be retrieved.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A view defining which fields should be filled in the returned execution.
    /// The API will default to the FULL view.
    #[prost(enumeration = "ExecutionView", tag = "2")]
    pub view: i32,
}
/// Request for the
/// \[CancelExecution][google.cloud.workflows.executions.v1.Executions.CancelExecution\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelExecutionRequest {
    /// Required. Name of the execution to be cancelled.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Defines possible views for execution resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionView {
    /// The default / unset value.
    Unspecified = 0,
    /// Includes only basic metadata about the execution.
    /// Following fields are returned: name, start_time, end_time, state
    /// and workflow_revision_id.
    Basic = 1,
    /// Includes all data.
    Full = 2,
}
impl ExecutionView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionView::Unspecified => "EXECUTION_VIEW_UNSPECIFIED",
            ExecutionView::Basic => "BASIC",
            ExecutionView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod executions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Executions is used to start and manage running instances of
    /// [Workflows][google.cloud.workflows.v1.Workflow] called executions.
    #[derive(Debug, Clone)]
    pub struct ExecutionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExecutionsClient<T>
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
        ) -> ExecutionsClient<InterceptedService<T, F>>
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
            ExecutionsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns a list of executions which belong to the workflow with
        /// the given name. The method returns executions of all workflow
        /// revisions. Returned executions are ordered by their start time (newest
        /// first).
        pub async fn list_executions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionsResponse>,
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
                "/google.cloud.workflows.executions.v1.Executions/ListExecutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workflows.executions.v1.Executions",
                        "ListExecutions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new execution using the latest revision of the given workflow.
        pub async fn create_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExecutionRequest>,
        ) -> std::result::Result<tonic::Response<super::Execution>, tonic::Status> {
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
                "/google.cloud.workflows.executions.v1.Executions/CreateExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workflows.executions.v1.Executions",
                        "CreateExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns an execution of the given name.
        pub async fn get_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExecutionRequest>,
        ) -> std::result::Result<tonic::Response<super::Execution>, tonic::Status> {
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
                "/google.cloud.workflows.executions.v1.Executions/GetExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workflows.executions.v1.Executions",
                        "GetExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels an execution of the given name.
        pub async fn cancel_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelExecutionRequest>,
        ) -> std::result::Result<tonic::Response<super::Execution>, tonic::Status> {
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
                "/google.cloud.workflows.executions.v1.Executions/CancelExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workflows.executions.v1.Executions",
                        "CancelExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
