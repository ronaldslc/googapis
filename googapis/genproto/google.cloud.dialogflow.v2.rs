/// Represents a single validation error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationError {
    /// The severity of the error.
    #[prost(enumeration = "validation_error::Severity", tag = "1")]
    pub severity: i32,
    /// The names of the entries that the error is associated with.
    /// Format:
    ///
    /// - "projects/<Project ID>/agent", if the error is associated with the entire
    /// agent.
    /// - "projects/<Project ID>/agent/intents/<Intent ID>", if the error is
    /// associated with certain intents.
    /// - "projects/<Project
    /// ID>/agent/intents/<Intent Id>/trainingPhrases/<Training Phrase ID>", if the
    /// error is associated with certain intent training phrases.
    /// - "projects/<Project ID>/agent/intents/<Intent Id>/parameters/<Parameter
    /// ID>", if the error is associated with certain intent parameters.
    /// - "projects/<Project ID>/agent/entities/<Entity ID>", if the error is
    /// associated with certain entities.
    #[prost(string, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The detailed error message.
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ValidationError`.
pub mod validation_error {
    /// Represents a level of severity.
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
    pub enum Severity {
        /// Not specified. This value should never be used.
        Unspecified = 0,
        /// The agent doesn't follow Dialogflow best practices.
        Info = 1,
        /// The agent may not behave as expected.
        Warning = 2,
        /// The agent may experience partial failures.
        Error = 3,
        /// The agent may completely fail.
        Critical = 4,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Info => "INFO",
                Severity::Warning => "WARNING",
                Severity::Error => "ERROR",
                Severity::Critical => "CRITICAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "INFO" => Some(Self::Info),
                "WARNING" => Some(Self::Warning),
                "ERROR" => Some(Self::Error),
                "CRITICAL" => Some(Self::Critical),
                _ => None,
            }
        }
    }
}
/// Represents the output of agent validation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationResult {
    /// Contains all validation errors.
    #[prost(message, repeated, tag = "1")]
    pub validation_errors: ::prost::alloc::vec::Vec<ValidationError>,
}
/// A Dialogflow agent is a virtual agent that handles conversations with your
/// end-users. It is a natural language understanding module that understands the
/// nuances of human language. Dialogflow translates end-user text or audio
/// during a conversation to structured data that your apps and services can
/// understand. You design and build a Dialogflow agent to handle the types of
/// conversations required for your system.
///
/// For more information about agents, see the
/// [Agent guide](<https://cloud.google.com/dialogflow/docs/agents-overview>).
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Agent {
    /// Required. The project of this agent.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The name of this agent.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The default language of the agent as a language tag. See
    /// [Language
    /// Support](<https://cloud.google.com/dialogflow/docs/reference/language>)
    /// for a list of the currently supported language codes. This field cannot be
    /// set by the `Update` method.
    #[prost(string, tag = "3")]
    pub default_language_code: ::prost::alloc::string::String,
    /// Optional. The list of all languages supported by this agent (except for the
    /// `default_language_code`).
    #[prost(string, repeated, tag = "4")]
    pub supported_language_codes: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Required. The time zone of this agent from the
    /// [time zone database](<https://www.iana.org/time-zones>), e.g.,
    /// America/New_York, Europe/Paris.
    #[prost(string, tag = "5")]
    pub time_zone: ::prost::alloc::string::String,
    /// Optional. The description of this agent.
    /// The maximum length is 500 characters. If exceeded, the request is rejected.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Optional. The URI of the agent's avatar.
    /// Avatars are used throughout the Dialogflow console and in the self-hosted
    /// [Web
    /// Demo](<https://cloud.google.com/dialogflow/docs/integrations/web-demo>)
    /// integration.
    #[prost(string, tag = "7")]
    pub avatar_uri: ::prost::alloc::string::String,
    /// Optional. Determines whether this agent should log conversation queries.
    #[prost(bool, tag = "8")]
    pub enable_logging: bool,
    /// Optional. Determines how intents are detected from user queries.
    #[deprecated]
    #[prost(enumeration = "agent::MatchMode", tag = "9")]
    pub match_mode: i32,
    /// Optional. To filter out false positive results and still get variety in
    /// matched natural language inputs for your agent, you can tune the machine
    /// learning classification threshold. If the returned score value is less than
    /// the threshold value, then a fallback intent will be triggered or, if there
    /// are no fallback intents defined, no intent will be triggered. The score
    /// values range from 0.0 (completely uncertain) to 1.0 (completely certain).
    /// If set to 0.0, the default of 0.3 is used.
    #[prost(float, tag = "10")]
    pub classification_threshold: f32,
    /// Optional. API version displayed in Dialogflow console. If not specified,
    /// V2 API is assumed. Clients are free to query different service endpoints
    /// for different API versions. However, bots connectors and webhook calls will
    /// follow the specified API version.
    #[prost(enumeration = "agent::ApiVersion", tag = "14")]
    pub api_version: i32,
    /// Optional. The agent tier. If not specified, TIER_STANDARD is assumed.
    #[prost(enumeration = "agent::Tier", tag = "15")]
    pub tier: i32,
}
/// Nested message and enum types in `Agent`.
pub mod agent {
    /// Match mode determines how intents are detected from user queries.
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
    pub enum MatchMode {
        /// Not specified.
        Unspecified = 0,
        /// Best for agents with a small number of examples in intents and/or wide
        /// use of templates syntax and composite entities.
        Hybrid = 1,
        /// Can be used for agents with a large number of examples in intents,
        /// especially the ones using @sys.any or very large custom entities.
        MlOnly = 2,
    }
    impl MatchMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MatchMode::Unspecified => "MATCH_MODE_UNSPECIFIED",
                MatchMode::Hybrid => "MATCH_MODE_HYBRID",
                MatchMode::MlOnly => "MATCH_MODE_ML_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MATCH_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "MATCH_MODE_HYBRID" => Some(Self::Hybrid),
                "MATCH_MODE_ML_ONLY" => Some(Self::MlOnly),
                _ => None,
            }
        }
    }
    /// API version for the agent.
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
    pub enum ApiVersion {
        /// Not specified.
        Unspecified = 0,
        /// Legacy V1 API.
        V1 = 1,
        /// V2 API.
        V2 = 2,
        /// V2beta1 API.
        V2Beta1 = 3,
    }
    impl ApiVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApiVersion::Unspecified => "API_VERSION_UNSPECIFIED",
                ApiVersion::V1 => "API_VERSION_V1",
                ApiVersion::V2 => "API_VERSION_V2",
                ApiVersion::V2Beta1 => "API_VERSION_V2_BETA_1",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "API_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                "API_VERSION_V1" => Some(Self::V1),
                "API_VERSION_V2" => Some(Self::V2),
                "API_VERSION_V2_BETA_1" => Some(Self::V2Beta1),
                _ => None,
            }
        }
    }
    /// Represents the agent tier.
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
    pub enum Tier {
        /// Not specified. This value should never be used.
        Unspecified = 0,
        /// Trial Edition, previously known as Standard Edition.
        Standard = 1,
        /// Essentials Edition, previously known as Enterprise Essential Edition.
        Enterprise = 2,
        /// Essentials Edition (same as TIER_ENTERPRISE), previously known as
        /// Enterprise Plus Edition.
        EnterprisePlus = 3,
    }
    impl Tier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Tier::Unspecified => "TIER_UNSPECIFIED",
                Tier::Standard => "TIER_STANDARD",
                Tier::Enterprise => "TIER_ENTERPRISE",
                Tier::EnterprisePlus => "TIER_ENTERPRISE_PLUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIER_UNSPECIFIED" => Some(Self::Unspecified),
                "TIER_STANDARD" => Some(Self::Standard),
                "TIER_ENTERPRISE" => Some(Self::Enterprise),
                "TIER_ENTERPRISE_PLUS" => Some(Self::EnterprisePlus),
                _ => None,
            }
        }
    }
}
/// The request message for \[Agents.GetAgent][google.cloud.dialogflow.v2.Agents.GetAgent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgentRequest {
    /// Required. The project that the agent to fetch is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// The request message for \[Agents.SetAgent][google.cloud.dialogflow.v2.Agents.SetAgent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAgentRequest {
    /// Required. The agent to update.
    #[prost(message, optional, tag = "1")]
    pub agent: ::core::option::Option<Agent>,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for \[Agents.DeleteAgent][google.cloud.dialogflow.v2.Agents.DeleteAgent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAgentRequest {
    /// Required. The project that the agent to delete is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// The request message for \[Agents.SearchAgents][google.cloud.dialogflow.v2.Agents.SearchAgents\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAgentsRequest {
    /// Required. The project to list agents from.
    /// Format: `projects/<Project ID or '-'>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Agents.SearchAgents][google.cloud.dialogflow.v2.Agents.SearchAgents\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAgentsResponse {
    /// The list of agents. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub agents: ::prost::alloc::vec::Vec<Agent>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[Agents.TrainAgent][google.cloud.dialogflow.v2.Agents.TrainAgent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainAgentRequest {
    /// Required. The project that the agent to train is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// The request message for \[Agents.ExportAgent][google.cloud.dialogflow.v2.Agents.ExportAgent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAgentRequest {
    /// Required. The project that the agent to export is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Google Cloud Storage](<https://cloud.google.com/storage/docs/>)
    /// URI to export the agent to.
    /// The format of this URI must be `gs://<bucket-name>/<object-name>`.
    /// If left unspecified, the serialized agent is returned inline.
    #[prost(string, tag = "2")]
    pub agent_uri: ::prost::alloc::string::String,
}
/// The response message for \[Agents.ExportAgent][google.cloud.dialogflow.v2.Agents.ExportAgent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAgentResponse {
    /// The exported agent.
    #[prost(oneof = "export_agent_response::Agent", tags = "1, 2")]
    pub agent: ::core::option::Option<export_agent_response::Agent>,
}
/// Nested message and enum types in `ExportAgentResponse`.
pub mod export_agent_response {
    /// The exported agent.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Agent {
        /// The URI to a file containing the exported agent. This field is populated
        /// only if `agent_uri` is specified in `ExportAgentRequest`.
        #[prost(string, tag = "1")]
        AgentUri(::prost::alloc::string::String),
        /// Zip compressed raw byte content for agent.
        #[prost(bytes, tag = "2")]
        AgentContent(::prost::alloc::vec::Vec<u8>),
    }
}
/// The request message for \[Agents.ImportAgent][google.cloud.dialogflow.v2.Agents.ImportAgent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAgentRequest {
    /// Required. The project that the agent to import is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The agent to import.
    #[prost(oneof = "import_agent_request::Agent", tags = "2, 3")]
    pub agent: ::core::option::Option<import_agent_request::Agent>,
}
/// Nested message and enum types in `ImportAgentRequest`.
pub mod import_agent_request {
    /// Required. The agent to import.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Agent {
        /// The URI to a Google Cloud Storage file containing the agent to import.
        /// Note: The URI must start with "gs://".
        #[prost(string, tag = "2")]
        AgentUri(::prost::alloc::string::String),
        /// Zip compressed raw byte content for agent.
        #[prost(bytes, tag = "3")]
        AgentContent(::prost::alloc::vec::Vec<u8>),
    }
}
/// The request message for \[Agents.RestoreAgent][google.cloud.dialogflow.v2.Agents.RestoreAgent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreAgentRequest {
    /// Required. The project that the agent to restore is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The agent to restore.
    #[prost(oneof = "restore_agent_request::Agent", tags = "2, 3")]
    pub agent: ::core::option::Option<restore_agent_request::Agent>,
}
/// Nested message and enum types in `RestoreAgentRequest`.
pub mod restore_agent_request {
    /// Required. The agent to restore.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Agent {
        /// The URI to a Google Cloud Storage file containing the agent to restore.
        /// Note: The URI must start with "gs://".
        #[prost(string, tag = "2")]
        AgentUri(::prost::alloc::string::String),
        /// Zip compressed raw byte content for agent.
        #[prost(bytes, tag = "3")]
        AgentContent(::prost::alloc::vec::Vec<u8>),
    }
}
/// The request message for \[Agents.GetValidationResult][google.cloud.dialogflow.v2.Agents.GetValidationResult\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidationResultRequest {
    /// Required. The project that the agent is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The language for which you want a validation result. If not
    /// specified, the agent's default language is used. [Many
    /// languages](<https://cloud.google.com/dialogflow/docs/reference/language>)
    /// are supported. Note: languages must be enabled in the agent before they can
    /// be used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod agents_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Agents][google.cloud.dialogflow.v2.Agent].
    #[derive(Debug, Clone)]
    pub struct AgentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AgentsClient<T>
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
        ) -> AgentsClient<InterceptedService<T, F>>
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
            AgentsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves the specified agent.
        pub async fn get_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAgentRequest>,
        ) -> std::result::Result<tonic::Response<super::Agent>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Agents/GetAgent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Agents", "GetAgent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates/updates the specified agent.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn set_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAgentRequest>,
        ) -> std::result::Result<tonic::Response<super::Agent>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Agents/SetAgent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Agents", "SetAgent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified agent.
        pub async fn delete_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAgentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Agents/DeleteAgent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Agents", "DeleteAgent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the list of agents.
        ///
        /// Since there is at most one conversational agent per project, this method is
        /// useful primarily for listing all agents across projects the caller has
        /// access to. One can achieve that with a wildcard project collection id "-".
        /// Refer to [List
        /// Sub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections).
        pub async fn search_agents(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAgentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchAgentsResponse>,
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
                "/google.cloud.dialogflow.v2.Agents/SearchAgents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Agents", "SearchAgents"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Trains the specified agent.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn train_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::TrainAgentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Agents/TrainAgent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Agents", "TrainAgent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports the specified agent to a ZIP file.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: [ExportAgentResponse][google.cloud.dialogflow.v2.ExportAgentResponse]
        pub async fn export_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAgentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Agents/ExportAgent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Agents", "ExportAgent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports the specified agent from a ZIP file.
        ///
        /// Uploads new intents and entity types without deleting the existing ones.
        /// Intents and entity types with the same name are replaced with the new
        /// versions from [ImportAgentRequest][google.cloud.dialogflow.v2.ImportAgentRequest]. After the import, the imported draft
        /// agent will be trained automatically (unless disabled in agent settings).
        /// However, once the import is done, training may not be completed yet. Please
        /// call [TrainAgent][google.cloud.dialogflow.v2.Agents.TrainAgent] and wait for the operation it returns in order to train
        /// explicitly.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        ///
        /// The operation only tracks when importing is complete, not when it is done
        /// training.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn import_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportAgentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Agents/ImportAgent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Agents", "ImportAgent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restores the specified agent from a ZIP file.
        ///
        /// Replaces the current agent version with a new one. All the intents and
        /// entity types in the older version are deleted. After the restore, the
        /// restored draft agent will be trained automatically (unless disabled in
        /// agent settings). However, once the restore is done, training may not be
        /// completed yet. Please call [TrainAgent][google.cloud.dialogflow.v2.Agents.TrainAgent] and wait for the operation it
        /// returns in order to train explicitly.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        ///
        /// The operation only tracks when restoring is complete, not when it is done
        /// training.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn restore_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreAgentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Agents/RestoreAgent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Agents", "RestoreAgent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets agent validation result. Agent validation is performed during
        /// training time and is updated automatically when training is completed.
        pub async fn get_validation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidationResultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidationResult>,
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
                "/google.cloud.dialogflow.v2.Agents/GetValidationResult",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Agents",
                        "GetValidationResult",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Hints for the speech recognizer to help with recognition in a specific
/// conversation state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechContext {
    /// Optional. A list of strings containing words and phrases that the speech
    /// recognizer should recognize with higher likelihood.
    ///
    /// This list can be used to:
    ///
    /// * improve accuracy for words and phrases you expect the user to say,
    ///    e.g. typical commands for your Dialogflow agent
    /// * add additional words to the speech recognizer vocabulary
    /// * ...
    ///
    /// See the [Cloud Speech
    /// documentation](<https://cloud.google.com/speech-to-text/quotas>) for usage
    /// limits.
    #[prost(string, repeated, tag = "1")]
    pub phrases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Boost for this context compared to other contexts:
    ///
    /// * If the boost is positive, Dialogflow will increase the probability that
    ///    the phrases in this context are recognized over similar sounding phrases.
    /// * If the boost is unspecified or non-positive, Dialogflow will not apply
    ///    any boost.
    ///
    /// Dialogflow recommends that you use boosts in the range (0, 20] and that you
    /// find a value that fits your use case with binary search.
    #[prost(float, tag = "2")]
    pub boost: f32,
}
/// Information for a word recognized by the speech recognizer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechWordInfo {
    /// The word this info is for.
    #[prost(string, tag = "3")]
    pub word: ::prost::alloc::string::String,
    /// Time offset relative to the beginning of the audio that corresponds to the
    /// start of the spoken word. This is an experimental feature and the accuracy
    /// of the time offset can vary.
    #[prost(message, optional, tag = "1")]
    pub start_offset: ::core::option::Option<::prost_types::Duration>,
    /// Time offset relative to the beginning of the audio that corresponds to the
    /// end of the spoken word. This is an experimental feature and the accuracy of
    /// the time offset can vary.
    #[prost(message, optional, tag = "2")]
    pub end_offset: ::core::option::Option<::prost_types::Duration>,
    /// The Speech confidence between 0.0 and 1.0 for this word. A higher number
    /// indicates an estimated greater likelihood that the recognized word is
    /// correct. The default of 0.0 is a sentinel value indicating that confidence
    /// was not set.
    ///
    /// This field is not guaranteed to be fully stable over time for the same
    /// audio input. Users should also not rely on it to always be provided.
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// Instructs the speech recognizer how to process the audio content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAudioConfig {
    /// Required. Audio encoding of the audio content to process.
    #[prost(enumeration = "AudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// Required. Sample rate (in Hertz) of the audio content sent in the query.
    /// Refer to
    /// [Cloud Speech API
    /// documentation](<https://cloud.google.com/speech-to-text/docs/basics>) for
    /// more details.
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// Required. The language of the supplied audio. Dialogflow does not do
    /// translations. See [Language
    /// Support](<https://cloud.google.com/dialogflow/docs/reference/language>)
    /// for a list of the currently supported language codes. Note that queries in
    /// the same session do not necessarily need to specify the same language.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// If `true`, Dialogflow returns \[SpeechWordInfo][google.cloud.dialogflow.v2.SpeechWordInfo\] in
    /// \[StreamingRecognitionResult][google.cloud.dialogflow.v2.StreamingRecognitionResult\] with information about the recognized speech
    /// words, e.g. start and end time offsets. If false or unspecified, Speech
    /// doesn't return any word-level information.
    #[prost(bool, tag = "13")]
    pub enable_word_info: bool,
    /// A list of strings containing words and phrases that the speech
    /// recognizer should recognize with higher likelihood.
    ///
    /// See [the Cloud Speech
    /// documentation](<https://cloud.google.com/speech-to-text/docs/basics#phrase-hints>)
    /// for more details.
    ///
    /// This field is deprecated. Please use \[speech_contexts\]() instead. If you
    /// specify both \[phrase_hints\]() and \[speech_contexts\](), Dialogflow will
    /// treat the \[phrase_hints\]() as a single additional \[SpeechContext\]().
    #[deprecated]
    #[prost(string, repeated, tag = "4")]
    pub phrase_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Context information to assist speech recognition.
    ///
    /// See [the Cloud Speech
    /// documentation](<https://cloud.google.com/speech-to-text/docs/basics#phrase-hints>)
    /// for more details.
    #[prost(message, repeated, tag = "11")]
    pub speech_contexts: ::prost::alloc::vec::Vec<SpeechContext>,
    /// Which Speech model to select for the given request. Select the
    /// model best suited to your domain to get best results. If a model is not
    /// explicitly specified, then we auto-select a model based on the parameters
    /// in the InputAudioConfig.
    /// If enhanced speech model is enabled for the agent and an enhanced
    /// version of the specified model for the language does not exist, then the
    /// speech is recognized using the standard version of the specified model.
    /// Refer to
    /// [Cloud Speech API
    /// documentation](<https://cloud.google.com/speech-to-text/docs/basics#select-model>)
    /// for more details.
    #[prost(string, tag = "7")]
    pub model: ::prost::alloc::string::String,
    /// Which variant of the [Speech model]\[google.cloud.dialogflow.v2.InputAudioConfig.model\] to use.
    #[prost(enumeration = "SpeechModelVariant", tag = "10")]
    pub model_variant: i32,
    /// If `false` (default), recognition does not cease until the
    /// client closes the stream.
    /// If `true`, the recognizer will detect a single spoken utterance in input
    /// audio. Recognition ceases when it detects the audio's voice has
    /// stopped or paused. In this case, once a detected intent is received, the
    /// client should close the stream and start a new request with a new stream as
    /// needed.
    /// Note: This setting is relevant only for streaming methods.
    /// Note: When specified, InputAudioConfig.single_utterance takes precedence
    /// over StreamingDetectIntentRequest.single_utterance.
    #[prost(bool, tag = "8")]
    pub single_utterance: bool,
    /// Only used in \[Participants.AnalyzeContent][google.cloud.dialogflow.v2.Participants.AnalyzeContent\] and
    /// \[Participants.StreamingAnalyzeContent][google.cloud.dialogflow.v2.Participants.StreamingAnalyzeContent\].
    /// If `false` and recognition doesn't return any result, trigger
    /// `NO_SPEECH_RECOGNIZED` event to Dialogflow agent.
    #[prost(bool, tag = "14")]
    pub disable_no_speech_recognized_event: bool,
}
/// Description of which voice to use for speech synthesis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceSelectionParams {
    /// Optional. The name of the voice. If not set, the service will choose a
    /// voice based on the other parameters such as language_code and
    /// \[ssml_gender][google.cloud.dialogflow.v2.VoiceSelectionParams.ssml_gender\].
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The preferred gender of the voice. If not set, the service will
    /// choose a voice based on the other parameters such as language_code and
    /// \[name][google.cloud.dialogflow.v2.VoiceSelectionParams.name\]. Note that this is only a preference, not requirement. If a
    /// voice of the appropriate gender is not available, the synthesizer should
    /// substitute a voice with a different gender rather than failing the request.
    #[prost(enumeration = "SsmlVoiceGender", tag = "2")]
    pub ssml_gender: i32,
}
/// Configuration of how speech should be synthesized.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechConfig {
    /// Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal
    /// native speed supported by the specific voice. 2.0 is twice as fast, and
    /// 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any
    /// other values < 0.25 or > 4.0 will return an error.
    #[prost(double, tag = "1")]
    pub speaking_rate: f64,
    /// Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20
    /// semitones from the original pitch. -20 means decrease 20 semitones from the
    /// original pitch.
    #[prost(double, tag = "2")]
    pub pitch: f64,
    /// Optional. Volume gain (in dB) of the normal native volume supported by the
    /// specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of
    /// 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)
    /// will play at approximately half the amplitude of the normal native signal
    /// amplitude. A value of +6.0 (dB) will play at approximately twice the
    /// amplitude of the normal native signal amplitude. We strongly recommend not
    /// to exceed +10 (dB) as there's usually no effective increase in loudness for
    /// any value greater than that.
    #[prost(double, tag = "3")]
    pub volume_gain_db: f64,
    /// Optional. An identifier which selects 'audio effects' profiles that are
    /// applied on (post synthesized) text to speech. Effects are applied on top of
    /// each other in the order they are given.
    #[prost(string, repeated, tag = "5")]
    pub effects_profile_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The desired voice of the synthesized audio.
    #[prost(message, optional, tag = "4")]
    pub voice: ::core::option::Option<VoiceSelectionParams>,
}
/// Instructs the speech synthesizer on how to generate the output audio content.
/// If this audio config is supplied in a request, it overrides all existing
/// text-to-speech settings applied to the agent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputAudioConfig {
    /// Required. Audio encoding of the synthesized audio content.
    #[prost(enumeration = "OutputAudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// The synthesis sample rate (in hertz) for this audio. If not
    /// provided, then the synthesizer will use the default sample rate based on
    /// the audio encoding. If this is different from the voice's natural sample
    /// rate, then the synthesizer will honor this request by converting to the
    /// desired sample rate (which might result in worse audio quality).
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// Configuration of how speech should be synthesized.
    #[prost(message, optional, tag = "3")]
    pub synthesize_speech_config: ::core::option::Option<SynthesizeSpeechConfig>,
}
/// Configures speech transcription for \[ConversationProfile][google.cloud.dialogflow.v2.ConversationProfile\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechToTextConfig {
    /// Optional. The speech model used in speech to text.
    /// `SPEECH_MODEL_VARIANT_UNSPECIFIED`, `USE_BEST_AVAILABLE` will be treated as
    /// `USE_ENHANCED`. It can be overridden in \[AnalyzeContentRequest][google.cloud.dialogflow.v2.AnalyzeContentRequest\] and
    /// \[StreamingAnalyzeContentRequest][google.cloud.dialogflow.v2.StreamingAnalyzeContentRequest\] request.
    #[prost(enumeration = "SpeechModelVariant", tag = "1")]
    pub speech_model_variant: i32,
}
/// Audio encoding of the audio content sent in the conversational query request.
/// Refer to the
/// [Cloud Speech API
/// documentation](<https://cloud.google.com/speech-to-text/docs/basics>) for more
/// details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioEncoding {
    /// Not specified.
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    Linear16 = 1,
    /// \[`FLAC`\](<https://xiph.org/flac/documentation.html>) (Free Lossless Audio
    /// Codec) is the recommended encoding because it is lossless (therefore
    /// recognition is not compromised) and requires only about half the
    /// bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and
    /// 24-bit samples, however, not all fields in `STREAMINFO` are supported.
    Flac = 2,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    Mulaw = 3,
    /// Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000.
    Amr = 4,
    /// Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000.
    AmrWb = 5,
    /// Opus encoded audio frames in Ogg container
    /// (\[OggOpus\](<https://wiki.xiph.org/OggOpus>)).
    /// `sample_rate_hertz` must be 16000.
    OggOpus = 6,
    /// Although the use of lossy encodings is not recommended, if a very low
    /// bitrate encoding is required, `OGG_OPUS` is highly preferred over
    /// Speex encoding. The \[Speex\](<https://speex.org/>) encoding supported by
    /// Dialogflow API has a header byte in each block, as in MIME type
    /// `audio/x-speex-with-header-byte`.
    /// It is a variant of the RTP Speex encoding defined in
    /// [RFC 5574](<https://tools.ietf.org/html/rfc5574>).
    /// The stream is a sequence of blocks, one block per RTP packet. Each block
    /// starts with a byte containing the length of the block, in bytes, followed
    /// by one or more frames of Speex data, padded to an integral number of
    /// bytes (octets) as specified in RFC 5574. In other words, each RTP header
    /// is replaced with a single byte containing the block length. Only Speex
    /// wideband is supported. `sample_rate_hertz` must be 16000.
    SpeexWithHeaderByte = 7,
}
impl AudioEncoding {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AudioEncoding::Unspecified => "AUDIO_ENCODING_UNSPECIFIED",
            AudioEncoding::Linear16 => "AUDIO_ENCODING_LINEAR_16",
            AudioEncoding::Flac => "AUDIO_ENCODING_FLAC",
            AudioEncoding::Mulaw => "AUDIO_ENCODING_MULAW",
            AudioEncoding::Amr => "AUDIO_ENCODING_AMR",
            AudioEncoding::AmrWb => "AUDIO_ENCODING_AMR_WB",
            AudioEncoding::OggOpus => "AUDIO_ENCODING_OGG_OPUS",
            AudioEncoding::SpeexWithHeaderByte => "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUDIO_ENCODING_UNSPECIFIED" => Some(Self::Unspecified),
            "AUDIO_ENCODING_LINEAR_16" => Some(Self::Linear16),
            "AUDIO_ENCODING_FLAC" => Some(Self::Flac),
            "AUDIO_ENCODING_MULAW" => Some(Self::Mulaw),
            "AUDIO_ENCODING_AMR" => Some(Self::Amr),
            "AUDIO_ENCODING_AMR_WB" => Some(Self::AmrWb),
            "AUDIO_ENCODING_OGG_OPUS" => Some(Self::OggOpus),
            "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE" => Some(Self::SpeexWithHeaderByte),
            _ => None,
        }
    }
}
/// Variant of the specified [Speech model]\[google.cloud.dialogflow.v2.InputAudioConfig.model\] to use.
///
/// See the [Cloud Speech
/// documentation](<https://cloud.google.com/speech-to-text/docs/enhanced-models>)
/// for which models have different variants. For example, the "phone_call" model
/// has both a standard and an enhanced variant. When you use an enhanced model,
/// you will generally receive higher quality results than for a standard model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpeechModelVariant {
    /// No model variant specified. In this case Dialogflow defaults to
    /// USE_BEST_AVAILABLE.
    Unspecified = 0,
    /// Use the best available variant of the [Speech
    /// model]\[InputAudioConfig.model\] that the caller is eligible for.
    ///
    /// Please see the [Dialogflow
    /// docs](<https://cloud.google.com/dialogflow/docs/data-logging>) for
    /// how to make your project eligible for enhanced models.
    UseBestAvailable = 1,
    /// Use standard model variant even if an enhanced model is available.  See the
    /// [Cloud Speech
    /// documentation](<https://cloud.google.com/speech-to-text/docs/enhanced-models>)
    /// for details about enhanced models.
    UseStandard = 2,
    /// Use an enhanced model variant:
    ///
    /// * If an enhanced variant does not exist for the given
    ///    \[model][google.cloud.dialogflow.v2.InputAudioConfig.model\] and request language, Dialogflow falls
    ///    back to the standard variant.
    ///
    ///    The [Cloud Speech
    ///    documentation](<https://cloud.google.com/speech-to-text/docs/enhanced-models>)
    ///    describes which models have enhanced variants.
    ///
    /// * If the API caller isn't eligible for enhanced models, Dialogflow returns
    ///    an error. Please see the [Dialogflow
    ///    docs](<https://cloud.google.com/dialogflow/docs/data-logging>)
    ///    for how to make your project eligible.
    UseEnhanced = 3,
}
impl SpeechModelVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SpeechModelVariant::Unspecified => "SPEECH_MODEL_VARIANT_UNSPECIFIED",
            SpeechModelVariant::UseBestAvailable => "USE_BEST_AVAILABLE",
            SpeechModelVariant::UseStandard => "USE_STANDARD",
            SpeechModelVariant::UseEnhanced => "USE_ENHANCED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SPEECH_MODEL_VARIANT_UNSPECIFIED" => Some(Self::Unspecified),
            "USE_BEST_AVAILABLE" => Some(Self::UseBestAvailable),
            "USE_STANDARD" => Some(Self::UseStandard),
            "USE_ENHANCED" => Some(Self::UseEnhanced),
            _ => None,
        }
    }
}
/// Gender of the voice as described in
/// [SSML voice element](<https://www.w3.org/TR/speech-synthesis11/#edef_voice>).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SsmlVoiceGender {
    /// An unspecified gender, which means that the client doesn't care which
    /// gender the selected voice will have.
    Unspecified = 0,
    /// A male voice.
    Male = 1,
    /// A female voice.
    Female = 2,
    /// A gender-neutral voice.
    Neutral = 3,
}
impl SsmlVoiceGender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SsmlVoiceGender::Unspecified => "SSML_VOICE_GENDER_UNSPECIFIED",
            SsmlVoiceGender::Male => "SSML_VOICE_GENDER_MALE",
            SsmlVoiceGender::Female => "SSML_VOICE_GENDER_FEMALE",
            SsmlVoiceGender::Neutral => "SSML_VOICE_GENDER_NEUTRAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SSML_VOICE_GENDER_UNSPECIFIED" => Some(Self::Unspecified),
            "SSML_VOICE_GENDER_MALE" => Some(Self::Male),
            "SSML_VOICE_GENDER_FEMALE" => Some(Self::Female),
            "SSML_VOICE_GENDER_NEUTRAL" => Some(Self::Neutral),
            _ => None,
        }
    }
}
/// Audio encoding of the output audio format in Text-To-Speech.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutputAudioEncoding {
    /// Not specified.
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    /// Audio content returned as LINEAR16 also contains a WAV header.
    Linear16 = 1,
    /// MP3 audio at 32kbps.
    Mp3 = 2,
    /// MP3 audio at 64kbps.
    Mp364Kbps = 4,
    /// Opus encoded audio wrapped in an ogg container. The result will be a
    /// file which can be played natively on Android, and in browsers (at least
    /// Chrome and Firefox). The quality of the encoding is considerably higher
    /// than MP3 while using approximately the same bitrate.
    OggOpus = 3,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    Mulaw = 5,
}
impl OutputAudioEncoding {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OutputAudioEncoding::Unspecified => "OUTPUT_AUDIO_ENCODING_UNSPECIFIED",
            OutputAudioEncoding::Linear16 => "OUTPUT_AUDIO_ENCODING_LINEAR_16",
            OutputAudioEncoding::Mp3 => "OUTPUT_AUDIO_ENCODING_MP3",
            OutputAudioEncoding::Mp364Kbps => "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS",
            OutputAudioEncoding::OggOpus => "OUTPUT_AUDIO_ENCODING_OGG_OPUS",
            OutputAudioEncoding::Mulaw => "OUTPUT_AUDIO_ENCODING_MULAW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OUTPUT_AUDIO_ENCODING_UNSPECIFIED" => Some(Self::Unspecified),
            "OUTPUT_AUDIO_ENCODING_LINEAR_16" => Some(Self::Linear16),
            "OUTPUT_AUDIO_ENCODING_MP3" => Some(Self::Mp3),
            "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS" => Some(Self::Mp364Kbps),
            "OUTPUT_AUDIO_ENCODING_OGG_OPUS" => Some(Self::OggOpus),
            "OUTPUT_AUDIO_ENCODING_MULAW" => Some(Self::Mulaw),
            _ => None,
        }
    }
}
/// Dialogflow contexts are similar to natural language context. If a person says
/// to you "they are orange", you need context in order to understand what "they"
/// is referring to. Similarly, for Dialogflow to handle an end-user expression
/// like that, it needs to be provided with context in order to correctly match
/// an intent.
///
/// Using contexts, you can control the flow of a conversation. You can configure
/// contexts for an intent by setting input and output contexts, which are
/// identified by string names. When an intent is matched, any configured output
/// contexts for that intent become active. While any contexts are active,
/// Dialogflow is more likely to match intents that are configured with input
/// contexts that correspond to the currently active contexts.
///
/// For more information about context, see the
/// [Contexts guide](<https://cloud.google.com/dialogflow/docs/contexts-overview>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    /// Required. The unique identifier of the context. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/contexts/<Context ID>`,
    /// or `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>/contexts/<Context ID>`.
    ///
    /// The `Context ID` is always converted to lowercase, may only contain
    /// characters in a-zA-Z0-9_-% and may be at most 250 bytes long.
    ///
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    ///
    /// The following context names are reserved for internal use by Dialogflow.
    /// You should not use these contexts or create contexts with these names:
    ///
    /// * `__system_counters__`
    /// * `*_id_dialog_context`
    /// * `*_dialog_params_size`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The number of conversational query requests after which the
    /// context expires. The default is `0`. If set to `0`, the context expires
    /// immediately. Contexts expire automatically after 20 minutes if there
    /// are no matching queries.
    #[prost(int32, tag = "2")]
    pub lifespan_count: i32,
    /// Optional. The collection of parameters associated with this context.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///      -   If parameter's entity type is a composite entity: map
    ///      -   Else: depending on parameter value type, could be one of string,
    ///          number, boolean, null, list or map
    /// -   MapValue value:
    ///      -   If parameter's entity type is a composite entity:
    ///          map from composite entity property names to property values
    ///      -   Else: parameter value
    #[prost(message, optional, tag = "3")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
}
/// The request message for \[Contexts.ListContexts][google.cloud.dialogflow.v2.Contexts.ListContexts\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContextsRequest {
    /// Required. The session to list all contexts from.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>` or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Contexts.ListContexts][google.cloud.dialogflow.v2.Contexts.ListContexts\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContextsResponse {
    /// The list of contexts. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub contexts: ::prost::alloc::vec::Vec<Context>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[Contexts.GetContext][google.cloud.dialogflow.v2.Contexts.GetContext\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContextRequest {
    /// Required. The name of the context. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/contexts/<Context ID>`
    /// or `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>/contexts/<Context ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[Contexts.CreateContext][google.cloud.dialogflow.v2.Contexts.CreateContext\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContextRequest {
    /// Required. The session to create a context for.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>` or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The context to create.
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<Context>,
}
/// The request message for \[Contexts.UpdateContext][google.cloud.dialogflow.v2.Contexts.UpdateContext\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContextRequest {
    /// Required. The context to update.
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for \[Contexts.DeleteContext][google.cloud.dialogflow.v2.Contexts.DeleteContext\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteContextRequest {
    /// Required. The name of the context to delete. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/contexts/<Context ID>`
    /// or `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>/contexts/<Context ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[Contexts.DeleteAllContexts][google.cloud.dialogflow.v2.Contexts.DeleteAllContexts\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAllContextsRequest {
    /// Required. The name of the session to delete all contexts from. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>` or `projects/<Project
    /// ID>/agent/environments/<Environment ID>/users/<User ID>/sessions/<Session
    /// ID>`.
    /// If `Environment ID` is not specified we assume default 'draft' environment.
    /// If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod contexts_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Contexts][google.cloud.dialogflow.v2.Context].
    #[derive(Debug, Clone)]
    pub struct ContextsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ContextsClient<T>
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
        ) -> ContextsClient<InterceptedService<T, F>>
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
            ContextsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all contexts in the specified session.
        pub async fn list_contexts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContextsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListContextsResponse>,
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
                "/google.cloud.dialogflow.v2.Contexts/ListContexts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Contexts",
                        "ListContexts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified context.
        pub async fn get_context(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContextRequest>,
        ) -> std::result::Result<tonic::Response<super::Context>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Contexts/GetContext",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Contexts", "GetContext"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a context.
        ///
        /// If the specified context already exists, overrides the context.
        pub async fn create_context(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateContextRequest>,
        ) -> std::result::Result<tonic::Response<super::Context>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Contexts/CreateContext",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Contexts",
                        "CreateContext",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified context.
        pub async fn update_context(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContextRequest>,
        ) -> std::result::Result<tonic::Response<super::Context>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Contexts/UpdateContext",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Contexts",
                        "UpdateContext",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified context.
        pub async fn delete_context(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContextRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Contexts/DeleteContext",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Contexts",
                        "DeleteContext",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes all active contexts in the specified session.
        pub async fn delete_all_contexts(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAllContextsRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Contexts/DeleteAllContexts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Contexts",
                        "DeleteAllContexts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// An intent categorizes an end-user's intention for one conversation turn. For
/// each agent, you define many intents, where your combined intents can handle a
/// complete conversation. When an end-user writes or says something, referred to
/// as an end-user expression or end-user input, Dialogflow matches the end-user
/// input to the best intent in your agent. Matching an intent is also known as
/// intent classification.
///
/// For more information, see the [intent
/// guide](<https://cloud.google.com/dialogflow/docs/intents-overview>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    /// Optional. The unique identifier of this intent.
    /// Required for \[Intents.UpdateIntent][google.cloud.dialogflow.v2.Intents.UpdateIntent\] and \[Intents.BatchUpdateIntents][google.cloud.dialogflow.v2.Intents.BatchUpdateIntents\]
    /// methods.
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of this intent.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Indicates whether webhooks are enabled for the intent.
    #[prost(enumeration = "intent::WebhookState", tag = "6")]
    pub webhook_state: i32,
    /// Optional. The priority of this intent. Higher numbers represent higher
    /// priorities.
    ///
    /// - If the supplied value is unspecified or 0, the service
    ///    translates the value to 500,000, which corresponds to the
    ///    `Normal` priority in the console.
    /// - If the supplied value is negative, the intent is ignored
    ///    in runtime detect intent requests.
    #[prost(int32, tag = "3")]
    pub priority: i32,
    /// Optional. Indicates whether this is a fallback intent.
    #[prost(bool, tag = "4")]
    pub is_fallback: bool,
    /// Optional. Indicates whether Machine Learning is disabled for the intent.
    /// Note: If `ml_disabled` setting is set to true, then this intent is not
    /// taken into account during inference in `ML ONLY` match mode. Also,
    /// auto-markup in the UI is turned off.
    #[prost(bool, tag = "19")]
    pub ml_disabled: bool,
    /// Optional. Indicates that a live agent should be brought in to handle the
    /// interaction with the user. In most cases, when you set this flag to true,
    /// you would also want to set end_interaction to true as well. Default is
    /// false.
    #[prost(bool, tag = "20")]
    pub live_agent_handoff: bool,
    /// Optional. Indicates that this intent ends an interaction. Some integrations
    /// (e.g., Actions on Google or Dialogflow phone gateway) use this information
    /// to close interaction with an end user. Default is false.
    #[prost(bool, tag = "21")]
    pub end_interaction: bool,
    /// Optional. The list of context names required for this intent to be
    /// triggered.
    /// Format: `projects/<Project ID>/agent/sessions/-/contexts/<Context ID>`.
    #[prost(string, repeated, tag = "7")]
    pub input_context_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The collection of event names that trigger the intent.
    /// If the collection of input contexts is not empty, all of the contexts must
    /// be present in the active user session for an event to trigger this intent.
    /// Event names are limited to 150 characters.
    #[prost(string, repeated, tag = "8")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The collection of examples that the agent is
    /// trained on.
    #[prost(message, repeated, tag = "9")]
    pub training_phrases: ::prost::alloc::vec::Vec<intent::TrainingPhrase>,
    /// Optional. The name of the action associated with the intent.
    /// Note: The action name must not contain whitespaces.
    #[prost(string, tag = "10")]
    pub action: ::prost::alloc::string::String,
    /// Optional. The collection of contexts that are activated when the intent
    /// is matched. Context messages in this collection should not set the
    /// parameters field. Setting the `lifespan_count` to 0 will reset the context
    /// when the intent is matched.
    /// Format: `projects/<Project ID>/agent/sessions/-/contexts/<Context ID>`.
    #[prost(message, repeated, tag = "11")]
    pub output_contexts: ::prost::alloc::vec::Vec<Context>,
    /// Optional. Indicates whether to delete all contexts in the current
    /// session when this intent is matched.
    #[prost(bool, tag = "12")]
    pub reset_contexts: bool,
    /// Optional. The collection of parameters associated with the intent.
    #[prost(message, repeated, tag = "13")]
    pub parameters: ::prost::alloc::vec::Vec<intent::Parameter>,
    /// Optional. The collection of rich messages corresponding to the
    /// `Response` field in the Dialogflow console.
    #[prost(message, repeated, tag = "14")]
    pub messages: ::prost::alloc::vec::Vec<intent::Message>,
    /// Optional. The list of platforms for which the first responses will be
    /// copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform).
    #[prost(
        enumeration = "intent::message::Platform",
        repeated,
        packed = "false",
        tag = "15"
    )]
    pub default_response_platforms: ::prost::alloc::vec::Vec<i32>,
    /// Output only.
    /// Read-only. The unique identifier of the root intent in the chain of
    /// followup intents. It identifies the correct followup intents chain for
    /// this intent. We populate this field only in the output.
    ///
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "16")]
    pub root_followup_intent_name: ::prost::alloc::string::String,
    /// Read-only after creation. The unique identifier of the parent intent in the
    /// chain of followup intents. You can set this field when creating an intent,
    /// for example with \[CreateIntent][google.cloud.dialogflow.v2.Intents.CreateIntent\] or
    /// \[BatchUpdateIntents][google.cloud.dialogflow.v2.Intents.BatchUpdateIntents\], in order to make this
    /// intent a followup intent.
    ///
    /// It identifies the parent followup intent.
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "17")]
    pub parent_followup_intent_name: ::prost::alloc::string::String,
    /// Output only. Read-only. Information about all followup intents that have this intent as
    /// a direct or indirect parent. We populate this field only in the output.
    #[prost(message, repeated, tag = "18")]
    pub followup_intent_info: ::prost::alloc::vec::Vec<intent::FollowupIntentInfo>,
}
/// Nested message and enum types in `Intent`.
pub mod intent {
    /// Represents an example that the agent is trained on.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrainingPhrase {
        /// Output only. The unique identifier of this training phrase.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Required. The type of the training phrase.
        #[prost(enumeration = "training_phrase::Type", tag = "2")]
        pub r#type: i32,
        /// Required. The ordered list of training phrase parts.
        /// The parts are concatenated in order to form the training phrase.
        ///
        /// Note: The API does not automatically annotate training phrases like the
        /// Dialogflow Console does.
        ///
        /// Note: Do not forget to include whitespace at part boundaries,
        /// so the training phrase is well formatted when the parts are concatenated.
        ///
        /// If the training phrase does not need to be annotated with parameters,
        /// you just need a single part with only the \[Part.text][google.cloud.dialogflow.v2.Intent.TrainingPhrase.Part.text\] field set.
        ///
        /// If you want to annotate the training phrase, you must create multiple
        /// parts, where the fields of each part are populated in one of two ways:
        ///
        /// -   `Part.text` is set to a part of the phrase that has no parameters.
        /// -   `Part.text` is set to a part of the phrase that you want to annotate,
        ///      and the `entity_type`, `alias`, and `user_defined` fields are all
        ///      set.
        #[prost(message, repeated, tag = "3")]
        pub parts: ::prost::alloc::vec::Vec<training_phrase::Part>,
        /// Optional. Indicates how many times this example was added to
        /// the intent. Each time a developer adds an existing sample by editing an
        /// intent or training, this counter is increased.
        #[prost(int32, tag = "4")]
        pub times_added_count: i32,
    }
    /// Nested message and enum types in `TrainingPhrase`.
    pub mod training_phrase {
        /// Represents a part of a training phrase.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Part {
            /// Required. The text for this part.
            #[prost(string, tag = "1")]
            pub text: ::prost::alloc::string::String,
            /// Optional. The entity type name prefixed with `@`.
            /// This field is required for annotated parts of the training phrase.
            #[prost(string, tag = "2")]
            pub entity_type: ::prost::alloc::string::String,
            /// Optional. The parameter name for the value extracted from the
            /// annotated part of the example.
            /// This field is required for annotated parts of the training phrase.
            #[prost(string, tag = "3")]
            pub alias: ::prost::alloc::string::String,
            /// Optional. Indicates whether the text was manually annotated.
            /// This field is set to true when the Dialogflow Console is used to
            /// manually annotate the part. When creating an annotated part with the
            /// API, you must set this to true.
            #[prost(bool, tag = "4")]
            pub user_defined: bool,
        }
        /// Represents different types of training phrases.
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
        pub enum Type {
            /// Not specified. This value should never be used.
            Unspecified = 0,
            /// Examples do not contain @-prefixed entity type names, but example parts
            /// can be annotated with entity types.
            Example = 1,
            /// Templates are not annotated with entity types, but they can contain
            /// @-prefixed entity type names as substrings.
            /// Template mode has been deprecated. Example mode is the only supported
            /// way to create new training phrases. If you have existing training
            /// phrases that you've created in template mode, those will continue to
            /// work.
            Template = 2,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::Example => "EXAMPLE",
                    Type::Template => "TEMPLATE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "EXAMPLE" => Some(Self::Example),
                    "TEMPLATE" => Some(Self::Template),
                    _ => None,
                }
            }
        }
    }
    /// Represents intent parameters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// The unique identifier of this parameter.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Required. The name of the parameter.
        #[prost(string, tag = "2")]
        pub display_name: ::prost::alloc::string::String,
        /// Optional. The definition of the parameter value. It can be:
        ///
        /// - a constant string,
        /// - a parameter value defined as `$parameter_name`,
        /// - an original parameter value defined as `$parameter_name.original`,
        /// - a parameter value from some context defined as
        ///    `#context_name.parameter_name`.
        #[prost(string, tag = "3")]
        pub value: ::prost::alloc::string::String,
        /// Optional. The default value to use when the `value` yields an empty
        /// result.
        /// Default values can be extracted from contexts by using the following
        /// syntax: `#context_name.parameter_name`.
        #[prost(string, tag = "4")]
        pub default_value: ::prost::alloc::string::String,
        /// Optional. The name of the entity type, prefixed with `@`, that
        /// describes values of the parameter. If the parameter is
        /// required, this must be provided.
        #[prost(string, tag = "5")]
        pub entity_type_display_name: ::prost::alloc::string::String,
        /// Optional. Indicates whether the parameter is required. That is,
        /// whether the intent cannot be completed without collecting the parameter
        /// value.
        #[prost(bool, tag = "6")]
        pub mandatory: bool,
        /// Optional. The collection of prompts that the agent can present to the
        /// user in order to collect a value for the parameter.
        #[prost(string, repeated, tag = "7")]
        pub prompts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. Indicates whether the parameter represents a list of values.
        #[prost(bool, tag = "8")]
        pub is_list: bool,
    }
    /// A rich response message.
    /// Corresponds to the intent `Response` field in the Dialogflow console.
    /// For more information, see
    /// [Rich response
    /// messages](<https://cloud.google.com/dialogflow/docs/intents-rich-messages>).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        /// Optional. The platform that this message is intended for.
        #[prost(enumeration = "message::Platform", tag = "6")]
        pub platform: i32,
        /// Required. The rich response message.
        #[prost(
            oneof = "message::Message",
            tags = "1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 22, 23, 24"
        )]
        pub message: ::core::option::Option<message::Message>,
    }
    /// Nested message and enum types in `Message`.
    pub mod message {
        /// The text response message.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Text {
            /// Optional. The collection of the agent's responses.
            #[prost(string, repeated, tag = "1")]
            pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// The image response message.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Image {
            /// Optional. The public URI to an image file.
            #[prost(string, tag = "1")]
            pub image_uri: ::prost::alloc::string::String,
            /// Optional. A text description of the image to be used for accessibility,
            /// e.g., screen readers.
            #[prost(string, tag = "2")]
            pub accessibility_text: ::prost::alloc::string::String,
        }
        /// The quick replies response message.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QuickReplies {
            /// Optional. The title of the collection of quick replies.
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
            /// Optional. The collection of quick replies.
            #[prost(string, repeated, tag = "2")]
            pub quick_replies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// The card response message.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Card {
            /// Optional. The title of the card.
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
            /// Optional. The subtitle of the card.
            #[prost(string, tag = "2")]
            pub subtitle: ::prost::alloc::string::String,
            /// Optional. The public URI to an image file for the card.
            #[prost(string, tag = "3")]
            pub image_uri: ::prost::alloc::string::String,
            /// Optional. The collection of card buttons.
            #[prost(message, repeated, tag = "4")]
            pub buttons: ::prost::alloc::vec::Vec<card::Button>,
        }
        /// Nested message and enum types in `Card`.
        pub mod card {
            /// Contains information about a button.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Button {
                /// Optional. The text to show on the button.
                #[prost(string, tag = "1")]
                pub text: ::prost::alloc::string::String,
                /// Optional. The text to send back to the Dialogflow API or a URI to
                /// open.
                #[prost(string, tag = "2")]
                pub postback: ::prost::alloc::string::String,
            }
        }
        /// The simple response message containing speech or text.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SimpleResponse {
            /// One of text_to_speech or ssml must be provided. The plain text of the
            /// speech output. Mutually exclusive with ssml.
            #[prost(string, tag = "1")]
            pub text_to_speech: ::prost::alloc::string::String,
            /// One of text_to_speech or ssml must be provided. Structured spoken
            /// response to the user in the SSML format. Mutually exclusive with
            /// text_to_speech.
            #[prost(string, tag = "2")]
            pub ssml: ::prost::alloc::string::String,
            /// Optional. The text to display.
            #[prost(string, tag = "3")]
            pub display_text: ::prost::alloc::string::String,
        }
        /// The collection of simple response candidates.
        /// This message in `QueryResult.fulfillment_messages` and
        /// `WebhookResponse.fulfillment_messages` should contain only one
        /// `SimpleResponse`.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SimpleResponses {
            /// Required. The list of simple responses.
            #[prost(message, repeated, tag = "1")]
            pub simple_responses: ::prost::alloc::vec::Vec<SimpleResponse>,
        }
        /// The basic card message. Useful for displaying information.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BasicCard {
            /// Optional. The title of the card.
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
            /// Optional. The subtitle of the card.
            #[prost(string, tag = "2")]
            pub subtitle: ::prost::alloc::string::String,
            /// Required, unless image is present. The body text of the card.
            #[prost(string, tag = "3")]
            pub formatted_text: ::prost::alloc::string::String,
            /// Optional. The image for the card.
            #[prost(message, optional, tag = "4")]
            pub image: ::core::option::Option<Image>,
            /// Optional. The collection of card buttons.
            #[prost(message, repeated, tag = "5")]
            pub buttons: ::prost::alloc::vec::Vec<basic_card::Button>,
        }
        /// Nested message and enum types in `BasicCard`.
        pub mod basic_card {
            /// The button object that appears at the bottom of a card.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Button {
                /// Required. The title of the button.
                #[prost(string, tag = "1")]
                pub title: ::prost::alloc::string::String,
                /// Required. Action to take when a user taps on the button.
                #[prost(message, optional, tag = "2")]
                pub open_uri_action: ::core::option::Option<button::OpenUriAction>,
            }
            /// Nested message and enum types in `Button`.
            pub mod button {
                /// Opens the given URI.
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct OpenUriAction {
                    /// Required. The HTTP or HTTPS scheme URI.
                    #[prost(string, tag = "1")]
                    pub uri: ::prost::alloc::string::String,
                }
            }
        }
        /// The suggestion chip message that the user can tap to quickly post a reply
        /// to the conversation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Suggestion {
            /// Required. The text shown the in the suggestion chip.
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
        }
        /// The collection of suggestions.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Suggestions {
            /// Required. The list of suggested replies.
            #[prost(message, repeated, tag = "1")]
            pub suggestions: ::prost::alloc::vec::Vec<Suggestion>,
        }
        /// The suggestion chip message that allows the user to jump out to the app
        /// or website associated with this agent.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LinkOutSuggestion {
            /// Required. The name of the app or site this chip is linking to.
            #[prost(string, tag = "1")]
            pub destination_name: ::prost::alloc::string::String,
            /// Required. The URI of the app or site to open when the user taps the
            /// suggestion chip.
            #[prost(string, tag = "2")]
            pub uri: ::prost::alloc::string::String,
        }
        /// The card for presenting a list of options to select from.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ListSelect {
            /// Optional. The overall title of the list.
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
            /// Required. List items.
            #[prost(message, repeated, tag = "2")]
            pub items: ::prost::alloc::vec::Vec<list_select::Item>,
            /// Optional. Subtitle of the list.
            #[prost(string, tag = "3")]
            pub subtitle: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `ListSelect`.
        pub mod list_select {
            /// An item in the list.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Item {
                /// Required. Additional information about this option.
                #[prost(message, optional, tag = "1")]
                pub info: ::core::option::Option<super::SelectItemInfo>,
                /// Required. The title of the list item.
                #[prost(string, tag = "2")]
                pub title: ::prost::alloc::string::String,
                /// Optional. The main text describing the item.
                #[prost(string, tag = "3")]
                pub description: ::prost::alloc::string::String,
                /// Optional. The image to display.
                #[prost(message, optional, tag = "4")]
                pub image: ::core::option::Option<super::Image>,
            }
        }
        /// The card for presenting a carousel of options to select from.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CarouselSelect {
            /// Required. Carousel items.
            #[prost(message, repeated, tag = "1")]
            pub items: ::prost::alloc::vec::Vec<carousel_select::Item>,
        }
        /// Nested message and enum types in `CarouselSelect`.
        pub mod carousel_select {
            /// An item in the carousel.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Item {
                /// Required. Additional info about the option item.
                #[prost(message, optional, tag = "1")]
                pub info: ::core::option::Option<super::SelectItemInfo>,
                /// Required. Title of the carousel item.
                #[prost(string, tag = "2")]
                pub title: ::prost::alloc::string::String,
                /// Optional. The body text of the card.
                #[prost(string, tag = "3")]
                pub description: ::prost::alloc::string::String,
                /// Optional. The image to display.
                #[prost(message, optional, tag = "4")]
                pub image: ::core::option::Option<super::Image>,
            }
        }
        /// Additional info about the select item for when it is triggered in a
        /// dialog.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SelectItemInfo {
            /// Required. A unique key that will be sent back to the agent if this
            /// response is given.
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
            /// Optional. A list of synonyms that can also be used to trigger this
            /// item in dialog.
            #[prost(string, repeated, tag = "2")]
            pub synonyms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// The media content card for Actions on Google.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MediaContent {
            /// Optional. What type of media is the content (ie "audio").
            #[prost(enumeration = "media_content::ResponseMediaType", tag = "1")]
            pub media_type: i32,
            /// Required. List of media objects.
            #[prost(message, repeated, tag = "2")]
            pub media_objects: ::prost::alloc::vec::Vec<
                media_content::ResponseMediaObject,
            >,
        }
        /// Nested message and enum types in `MediaContent`.
        pub mod media_content {
            /// Response media object for media content card.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ResponseMediaObject {
                /// Required. Name of media card.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
                /// Optional. Description of media card.
                #[prost(string, tag = "2")]
                pub description: ::prost::alloc::string::String,
                /// Required. Url where the media is stored.
                #[prost(string, tag = "5")]
                pub content_url: ::prost::alloc::string::String,
                /// Image to show with the media card.
                #[prost(oneof = "response_media_object::Image", tags = "3, 4")]
                pub image: ::core::option::Option<response_media_object::Image>,
            }
            /// Nested message and enum types in `ResponseMediaObject`.
            pub mod response_media_object {
                /// Image to show with the media card.
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Image {
                    /// Optional. Image to display above media content.
                    #[prost(message, tag = "3")]
                    LargeImage(super::super::Image),
                    /// Optional. Icon to display above media content.
                    #[prost(message, tag = "4")]
                    Icon(super::super::Image),
                }
            }
            /// Format of response media type.
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
            pub enum ResponseMediaType {
                /// Unspecified.
                Unspecified = 0,
                /// Response media type is audio.
                Audio = 1,
            }
            impl ResponseMediaType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ResponseMediaType::Unspecified => {
                            "RESPONSE_MEDIA_TYPE_UNSPECIFIED"
                        }
                        ResponseMediaType::Audio => "AUDIO",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "RESPONSE_MEDIA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                        "AUDIO" => Some(Self::Audio),
                        _ => None,
                    }
                }
            }
        }
        /// Browse Carousel Card for Actions on Google.
        /// <https://developers.google.com/actions/assistant/responses#browsing_carousel>
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BrowseCarouselCard {
            /// Required. List of items in the Browse Carousel Card. Minimum of two
            /// items, maximum of ten.
            #[prost(message, repeated, tag = "1")]
            pub items: ::prost::alloc::vec::Vec<
                browse_carousel_card::BrowseCarouselCardItem,
            >,
            /// Optional. Settings for displaying the image. Applies to every image in
            /// \[items][google.cloud.dialogflow.v2.Intent.Message.BrowseCarouselCard.items\].
            #[prost(
                enumeration = "browse_carousel_card::ImageDisplayOptions",
                tag = "2"
            )]
            pub image_display_options: i32,
        }
        /// Nested message and enum types in `BrowseCarouselCard`.
        pub mod browse_carousel_card {
            /// Browsing carousel tile
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct BrowseCarouselCardItem {
                /// Required. Action to present to the user.
                #[prost(message, optional, tag = "1")]
                pub open_uri_action: ::core::option::Option<
                    browse_carousel_card_item::OpenUrlAction,
                >,
                /// Required. Title of the carousel item. Maximum of two lines of text.
                #[prost(string, tag = "2")]
                pub title: ::prost::alloc::string::String,
                /// Optional. Description of the carousel item. Maximum of four lines of
                /// text.
                #[prost(string, tag = "3")]
                pub description: ::prost::alloc::string::String,
                /// Optional. Hero image for the carousel item.
                #[prost(message, optional, tag = "4")]
                pub image: ::core::option::Option<super::Image>,
                /// Optional. Text that appears at the bottom of the Browse Carousel
                /// Card. Maximum of one line of text.
                #[prost(string, tag = "5")]
                pub footer: ::prost::alloc::string::String,
            }
            /// Nested message and enum types in `BrowseCarouselCardItem`.
            pub mod browse_carousel_card_item {
                /// Actions on Google action to open a given url.
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct OpenUrlAction {
                    /// Required. URL
                    #[prost(string, tag = "1")]
                    pub url: ::prost::alloc::string::String,
                    /// Optional. Specifies the type of viewer that is used when opening
                    /// the URL. Defaults to opening via web browser.
                    #[prost(enumeration = "open_url_action::UrlTypeHint", tag = "3")]
                    pub url_type_hint: i32,
                }
                /// Nested message and enum types in `OpenUrlAction`.
                pub mod open_url_action {
                    /// Type of the URI.
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
                    pub enum UrlTypeHint {
                        /// Unspecified
                        Unspecified = 0,
                        /// Url would be an amp action
                        AmpAction = 1,
                        /// URL that points directly to AMP content, or to a canonical URL
                        /// which refers to AMP content via <link rel="amphtml">.
                        AmpContent = 2,
                    }
                    impl UrlTypeHint {
                        /// String value of the enum field names used in the ProtoBuf definition.
                        ///
                        /// The values are not transformed in any way and thus are considered stable
                        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                        pub fn as_str_name(&self) -> &'static str {
                            match self {
                                UrlTypeHint::Unspecified => "URL_TYPE_HINT_UNSPECIFIED",
                                UrlTypeHint::AmpAction => "AMP_ACTION",
                                UrlTypeHint::AmpContent => "AMP_CONTENT",
                            }
                        }
                        /// Creates an enum from field names used in the ProtoBuf definition.
                        pub fn from_str_name(
                            value: &str,
                        ) -> ::core::option::Option<Self> {
                            match value {
                                "URL_TYPE_HINT_UNSPECIFIED" => Some(Self::Unspecified),
                                "AMP_ACTION" => Some(Self::AmpAction),
                                "AMP_CONTENT" => Some(Self::AmpContent),
                                _ => None,
                            }
                        }
                    }
                }
            }
            /// Image display options for Actions on Google. This should be used for
            /// when the image's aspect ratio does not match the image container's
            /// aspect ratio.
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
            pub enum ImageDisplayOptions {
                /// Fill the gaps between the image and the image container with gray
                /// bars.
                Unspecified = 0,
                /// Fill the gaps between the image and the image container with gray
                /// bars.
                Gray = 1,
                /// Fill the gaps between the image and the image container with white
                /// bars.
                White = 2,
                /// Image is scaled such that the image width and height match or exceed
                /// the container dimensions. This may crop the top and bottom of the
                /// image if the scaled image height is greater than the container
                /// height, or crop the left and right of the image if the scaled image
                /// width is greater than the container width. This is similar to "Zoom
                /// Mode" on a widescreen TV when playing a 4:3 video.
                Cropped = 3,
                /// Pad the gaps between image and image frame with a blurred copy of the
                /// same image.
                BlurredBackground = 4,
            }
            impl ImageDisplayOptions {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ImageDisplayOptions::Unspecified => {
                            "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED"
                        }
                        ImageDisplayOptions::Gray => "GRAY",
                        ImageDisplayOptions::White => "WHITE",
                        ImageDisplayOptions::Cropped => "CROPPED",
                        ImageDisplayOptions::BlurredBackground => "BLURRED_BACKGROUND",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED" => Some(Self::Unspecified),
                        "GRAY" => Some(Self::Gray),
                        "WHITE" => Some(Self::White),
                        "CROPPED" => Some(Self::Cropped),
                        "BLURRED_BACKGROUND" => Some(Self::BlurredBackground),
                        _ => None,
                    }
                }
            }
        }
        /// Table card for Actions on Google.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TableCard {
            /// Required. Title of the card.
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
            /// Optional. Subtitle to the title.
            #[prost(string, tag = "2")]
            pub subtitle: ::prost::alloc::string::String,
            /// Optional. Image which should be displayed on the card.
            #[prost(message, optional, tag = "3")]
            pub image: ::core::option::Option<Image>,
            /// Optional. Display properties for the columns in this table.
            #[prost(message, repeated, tag = "4")]
            pub column_properties: ::prost::alloc::vec::Vec<ColumnProperties>,
            /// Optional. Rows in this table of data.
            #[prost(message, repeated, tag = "5")]
            pub rows: ::prost::alloc::vec::Vec<TableCardRow>,
            /// Optional. List of buttons for the card.
            #[prost(message, repeated, tag = "6")]
            pub buttons: ::prost::alloc::vec::Vec<basic_card::Button>,
        }
        /// Column properties for \[TableCard][google.cloud.dialogflow.v2.Intent.Message.TableCard\].
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ColumnProperties {
            /// Required. Column heading.
            #[prost(string, tag = "1")]
            pub header: ::prost::alloc::string::String,
            /// Optional. Defines text alignment for all cells in this column.
            #[prost(enumeration = "column_properties::HorizontalAlignment", tag = "2")]
            pub horizontal_alignment: i32,
        }
        /// Nested message and enum types in `ColumnProperties`.
        pub mod column_properties {
            /// Text alignments within a cell.
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
            pub enum HorizontalAlignment {
                /// Text is aligned to the leading edge of the column.
                Unspecified = 0,
                /// Text is aligned to the leading edge of the column.
                Leading = 1,
                /// Text is centered in the column.
                Center = 2,
                /// Text is aligned to the trailing edge of the column.
                Trailing = 3,
            }
            impl HorizontalAlignment {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        HorizontalAlignment::Unspecified => {
                            "HORIZONTAL_ALIGNMENT_UNSPECIFIED"
                        }
                        HorizontalAlignment::Leading => "LEADING",
                        HorizontalAlignment::Center => "CENTER",
                        HorizontalAlignment::Trailing => "TRAILING",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "HORIZONTAL_ALIGNMENT_UNSPECIFIED" => Some(Self::Unspecified),
                        "LEADING" => Some(Self::Leading),
                        "CENTER" => Some(Self::Center),
                        "TRAILING" => Some(Self::Trailing),
                        _ => None,
                    }
                }
            }
        }
        /// Row of \[TableCard][google.cloud.dialogflow.v2.Intent.Message.TableCard\].
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TableCardRow {
            /// Optional. List of cells that make up this row.
            #[prost(message, repeated, tag = "1")]
            pub cells: ::prost::alloc::vec::Vec<TableCardCell>,
            /// Optional. Whether to add a visual divider after this row.
            #[prost(bool, tag = "2")]
            pub divider_after: bool,
        }
        /// Cell of \[TableCardRow][google.cloud.dialogflow.v2.Intent.Message.TableCardRow\].
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TableCardCell {
            /// Required. Text in this cell.
            #[prost(string, tag = "1")]
            pub text: ::prost::alloc::string::String,
        }
        /// The rich response message integration platform. See
        /// \[Integrations\](<https://cloud.google.com/dialogflow/docs/integrations>).
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
        pub enum Platform {
            /// Default platform.
            Unspecified = 0,
            /// Facebook.
            Facebook = 1,
            /// Slack.
            Slack = 2,
            /// Telegram.
            Telegram = 3,
            /// Kik.
            Kik = 4,
            /// Skype.
            Skype = 5,
            /// Line.
            Line = 6,
            /// Viber.
            Viber = 7,
            /// Google Assistant
            /// See [Dialogflow webhook
            /// format](<https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json>)
            ActionsOnGoogle = 8,
            /// Google Hangouts.
            GoogleHangouts = 11,
        }
        impl Platform {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Platform::Unspecified => "PLATFORM_UNSPECIFIED",
                    Platform::Facebook => "FACEBOOK",
                    Platform::Slack => "SLACK",
                    Platform::Telegram => "TELEGRAM",
                    Platform::Kik => "KIK",
                    Platform::Skype => "SKYPE",
                    Platform::Line => "LINE",
                    Platform::Viber => "VIBER",
                    Platform::ActionsOnGoogle => "ACTIONS_ON_GOOGLE",
                    Platform::GoogleHangouts => "GOOGLE_HANGOUTS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "PLATFORM_UNSPECIFIED" => Some(Self::Unspecified),
                    "FACEBOOK" => Some(Self::Facebook),
                    "SLACK" => Some(Self::Slack),
                    "TELEGRAM" => Some(Self::Telegram),
                    "KIK" => Some(Self::Kik),
                    "SKYPE" => Some(Self::Skype),
                    "LINE" => Some(Self::Line),
                    "VIBER" => Some(Self::Viber),
                    "ACTIONS_ON_GOOGLE" => Some(Self::ActionsOnGoogle),
                    "GOOGLE_HANGOUTS" => Some(Self::GoogleHangouts),
                    _ => None,
                }
            }
        }
        /// Required. The rich response message.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Message {
            /// The text response.
            #[prost(message, tag = "1")]
            Text(Text),
            /// The image response.
            #[prost(message, tag = "2")]
            Image(Image),
            /// The quick replies response.
            #[prost(message, tag = "3")]
            QuickReplies(QuickReplies),
            /// The card response.
            #[prost(message, tag = "4")]
            Card(Card),
            /// A custom platform-specific response.
            #[prost(message, tag = "5")]
            Payload(::prost_types::Struct),
            /// The voice and text-only responses for Actions on Google.
            #[prost(message, tag = "7")]
            SimpleResponses(SimpleResponses),
            /// The basic card response for Actions on Google.
            #[prost(message, tag = "8")]
            BasicCard(BasicCard),
            /// The suggestion chips for Actions on Google.
            #[prost(message, tag = "9")]
            Suggestions(Suggestions),
            /// The link out suggestion chip for Actions on Google.
            #[prost(message, tag = "10")]
            LinkOutSuggestion(LinkOutSuggestion),
            /// The list card response for Actions on Google.
            #[prost(message, tag = "11")]
            ListSelect(ListSelect),
            /// The carousel card response for Actions on Google.
            #[prost(message, tag = "12")]
            CarouselSelect(CarouselSelect),
            /// Browse carousel card for Actions on Google.
            #[prost(message, tag = "22")]
            BrowseCarouselCard(BrowseCarouselCard),
            /// Table card for Actions on Google.
            #[prost(message, tag = "23")]
            TableCard(TableCard),
            /// The media content card for Actions on Google.
            #[prost(message, tag = "24")]
            MediaContent(MediaContent),
        }
    }
    /// Represents a single followup intent in the chain.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FollowupIntentInfo {
        /// The unique identifier of the followup intent.
        /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
        #[prost(string, tag = "1")]
        pub followup_intent_name: ::prost::alloc::string::String,
        /// The unique identifier of the followup intent's parent.
        /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
        #[prost(string, tag = "2")]
        pub parent_followup_intent_name: ::prost::alloc::string::String,
    }
    /// Represents the different states that webhooks can be in.
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
    pub enum WebhookState {
        /// Webhook is disabled in the agent and in the intent.
        Unspecified = 0,
        /// Webhook is enabled in the agent and in the intent.
        Enabled = 1,
        /// Webhook is enabled in the agent and in the intent. Also, each slot
        /// filling prompt is forwarded to the webhook.
        EnabledForSlotFilling = 2,
    }
    impl WebhookState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WebhookState::Unspecified => "WEBHOOK_STATE_UNSPECIFIED",
                WebhookState::Enabled => "WEBHOOK_STATE_ENABLED",
                WebhookState::EnabledForSlotFilling => {
                    "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WEBHOOK_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "WEBHOOK_STATE_ENABLED" => Some(Self::Enabled),
                "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING" => {
                    Some(Self::EnabledForSlotFilling)
                }
                _ => None,
            }
        }
    }
}
/// The request message for \[Intents.ListIntents][google.cloud.dialogflow.v2.Intents.ListIntents\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIntentsRequest {
    /// Required. The agent to list all intents from.
    /// Format: `projects/<Project ID>/agent` or `projects/<Project
    /// ID>/locations/<Location ID>/agent`.
    ///
    /// Alternatively, you can specify the environment to list intents for.
    /// Format: `projects/<Project ID>/agent/environments/<Environment ID>`
    /// or `projects/<Project ID>/locations/<Location
    /// ID>/agent/environments/<Environment ID>`.
    /// Note: training phrases of the intents will not be returned for non-draft
    /// environment.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "3")]
    pub intent_view: i32,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Intents.ListIntents][google.cloud.dialogflow.v2.Intents.ListIntents\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIntentsResponse {
    /// The list of agent intents. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub intents: ::prost::alloc::vec::Vec<Intent>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[Intents.GetIntent][google.cloud.dialogflow.v2.Intents.GetIntent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIntentRequest {
    /// Required. The name of the intent.
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "3")]
    pub intent_view: i32,
}
/// The request message for \[Intents.CreateIntent][google.cloud.dialogflow.v2.Intents.CreateIntent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIntentRequest {
    /// Required. The agent to create a intent for.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The intent to create.
    #[prost(message, optional, tag = "2")]
    pub intent: ::core::option::Option<Intent>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "4")]
    pub intent_view: i32,
}
/// The request message for \[Intents.UpdateIntent][google.cloud.dialogflow.v2.Intents.UpdateIntent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIntentRequest {
    /// Required. The intent to update.
    #[prost(message, optional, tag = "1")]
    pub intent: ::core::option::Option<Intent>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "4")]
    pub intent_view: i32,
}
/// The request message for \[Intents.DeleteIntent][google.cloud.dialogflow.v2.Intents.DeleteIntent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIntentRequest {
    /// Required. The name of the intent to delete. If this intent has direct or
    /// indirect followup intents, we also delete them.
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateIntentsRequest {
    /// Required. The name of the agent to update or create intents in.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "6")]
    pub intent_view: i32,
    /// The source of the intent batch.
    #[prost(oneof = "batch_update_intents_request::IntentBatch", tags = "2, 3")]
    pub intent_batch: ::core::option::Option<batch_update_intents_request::IntentBatch>,
}
/// Nested message and enum types in `BatchUpdateIntentsRequest`.
pub mod batch_update_intents_request {
    /// The source of the intent batch.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum IntentBatch {
        /// The URI to a Google Cloud Storage file containing intents to update or
        /// create. The file format can either be a serialized proto (of IntentBatch
        /// type) or JSON object. Note: The URI must start with "gs://".
        #[prost(string, tag = "2")]
        IntentBatchUri(::prost::alloc::string::String),
        /// The collection of intents to update or create.
        #[prost(message, tag = "3")]
        IntentBatchInline(super::IntentBatch),
    }
}
/// The response message for \[Intents.BatchUpdateIntents][google.cloud.dialogflow.v2.Intents.BatchUpdateIntents\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateIntentsResponse {
    /// The collection of updated or created intents.
    #[prost(message, repeated, tag = "1")]
    pub intents: ::prost::alloc::vec::Vec<Intent>,
}
/// The request message for \[Intents.BatchDeleteIntents][google.cloud.dialogflow.v2.Intents.BatchDeleteIntents\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteIntentsRequest {
    /// Required. The name of the agent to delete all entities types for. Format:
    /// `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The collection of intents to delete. Only intent `name` must be
    /// filled in.
    #[prost(message, repeated, tag = "2")]
    pub intents: ::prost::alloc::vec::Vec<Intent>,
}
/// This message is a wrapper around a collection of intents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentBatch {
    /// A collection of intents.
    #[prost(message, repeated, tag = "1")]
    pub intents: ::prost::alloc::vec::Vec<Intent>,
}
/// Represents the options for views of an intent.
/// An intent can be a sizable object. Therefore, we provide a resource view that
/// does not return training phrases in the response by default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntentView {
    /// Training phrases field is not populated in the response.
    Unspecified = 0,
    /// All fields are populated.
    Full = 1,
}
impl IntentView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IntentView::Unspecified => "INTENT_VIEW_UNSPECIFIED",
            IntentView::Full => "INTENT_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTENT_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "INTENT_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod intents_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Intents][google.cloud.dialogflow.v2.Intent].
    #[derive(Debug, Clone)]
    pub struct IntentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IntentsClient<T>
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
        ) -> IntentsClient<InterceptedService<T, F>>
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
            IntentsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all intents in the specified agent.
        pub async fn list_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIntentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListIntentsResponse>,
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
                "/google.cloud.dialogflow.v2.Intents/ListIntents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Intents", "ListIntents"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified intent.
        pub async fn get_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIntentRequest>,
        ) -> std::result::Result<tonic::Response<super::Intent>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Intents/GetIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Intents", "GetIntent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an intent in the specified agent.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn create_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIntentRequest>,
        ) -> std::result::Result<tonic::Response<super::Intent>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Intents/CreateIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Intents", "CreateIntent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified intent.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn update_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIntentRequest>,
        ) -> std::result::Result<tonic::Response<super::Intent>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Intents/UpdateIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Intents", "UpdateIntent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified intent and its direct or indirect followup intents.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn delete_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIntentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Intents/DeleteIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Intents", "DeleteIntent"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates/Creates multiple intents in the specified agent.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: [BatchUpdateIntentsResponse][google.cloud.dialogflow.v2.BatchUpdateIntentsResponse]
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn batch_update_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateIntentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Intents/BatchUpdateIntents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Intents",
                        "BatchUpdateIntents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes intents in the specified agent.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn batch_delete_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteIntentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Intents/BatchDeleteIntents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Intents",
                        "BatchDeleteIntents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Each intent parameter has a type, called the entity type, which dictates
/// exactly how data from an end-user expression is extracted.
///
/// Dialogflow provides predefined system entities that can match many common
/// types of data. For example, there are system entities for matching dates,
/// times, colors, email addresses, and so on. You can also create your own
/// custom entities for matching custom data. For example, you could define a
/// vegetable entity that can match the types of vegetables available for
/// purchase with a grocery store agent.
///
/// For more information, see the
/// [Entity guide](<https://cloud.google.com/dialogflow/docs/entities-overview>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityType {
    /// The unique identifier of the entity type.
    /// Required for \[EntityTypes.UpdateEntityType][google.cloud.dialogflow.v2.EntityTypes.UpdateEntityType\] and
    /// \[EntityTypes.BatchUpdateEntityTypes][google.cloud.dialogflow.v2.EntityTypes.BatchUpdateEntityTypes\] methods.
    /// Format: `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the entity type.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Indicates the kind of entity type.
    #[prost(enumeration = "entity_type::Kind", tag = "3")]
    pub kind: i32,
    /// Optional. Indicates whether the entity type can be automatically
    /// expanded.
    #[prost(enumeration = "entity_type::AutoExpansionMode", tag = "4")]
    pub auto_expansion_mode: i32,
    /// Optional. The collection of entity entries associated with the entity type.
    #[prost(message, repeated, tag = "6")]
    pub entities: ::prost::alloc::vec::Vec<entity_type::Entity>,
    /// Optional. Enables fuzzy entity extraction during classification.
    #[prost(bool, tag = "7")]
    pub enable_fuzzy_extraction: bool,
}
/// Nested message and enum types in `EntityType`.
pub mod entity_type {
    /// An **entity entry** for an associated entity type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Required. The primary value associated with this entity entry.
        /// For example, if the entity type is *vegetable*, the value could be
        /// *scallions*.
        ///
        /// For `KIND_MAP` entity types:
        ///
        /// *   A reference value to be used in place of synonyms.
        ///
        /// For `KIND_LIST` entity types:
        ///
        /// *   A string that can contain references to other entity types (with or
        ///      without aliases).
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
        /// Required. A collection of value synonyms. For example, if the entity type
        /// is *vegetable*, and `value` is *scallions*, a synonym could be *green
        /// onions*.
        ///
        /// For `KIND_LIST` entity types:
        ///
        /// *   This collection must contain exactly one synonym equal to `value`.
        #[prost(string, repeated, tag = "2")]
        pub synonyms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Represents kinds of entities.
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
    pub enum Kind {
        /// Not specified. This value should be never used.
        Unspecified = 0,
        /// Map entity types allow mapping of a group of synonyms to a reference
        /// value.
        Map = 1,
        /// List entity types contain a set of entries that do not map to reference
        /// values. However, list entity types can contain references to other entity
        /// types (with or without aliases).
        List = 2,
        /// Regexp entity types allow to specify regular expressions in entries
        /// values.
        Regexp = 3,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Unspecified => "KIND_UNSPECIFIED",
                Kind::Map => "KIND_MAP",
                Kind::List => "KIND_LIST",
                Kind::Regexp => "KIND_REGEXP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KIND_UNSPECIFIED" => Some(Self::Unspecified),
                "KIND_MAP" => Some(Self::Map),
                "KIND_LIST" => Some(Self::List),
                "KIND_REGEXP" => Some(Self::Regexp),
                _ => None,
            }
        }
    }
    /// Represents different entity type expansion modes. Automated expansion
    /// allows an agent to recognize values that have not been explicitly listed in
    /// the entity (for example, new kinds of shopping list items).
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
    pub enum AutoExpansionMode {
        /// Auto expansion disabled for the entity.
        Unspecified = 0,
        /// Allows an agent to recognize values that have not been explicitly
        /// listed in the entity.
        Default = 1,
    }
    impl AutoExpansionMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AutoExpansionMode::Unspecified => "AUTO_EXPANSION_MODE_UNSPECIFIED",
                AutoExpansionMode::Default => "AUTO_EXPANSION_MODE_DEFAULT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AUTO_EXPANSION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTO_EXPANSION_MODE_DEFAULT" => Some(Self::Default),
                _ => None,
            }
        }
    }
}
/// The request message for \[EntityTypes.ListEntityTypes][google.cloud.dialogflow.v2.EntityTypes.ListEntityTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityTypesRequest {
    /// Required. The agent to list all entity types from.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[EntityTypes.ListEntityTypes][google.cloud.dialogflow.v2.EntityTypes.ListEntityTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityTypesResponse {
    /// The list of agent entity types. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub entity_types: ::prost::alloc::vec::Vec<EntityType>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[EntityTypes.GetEntityType][google.cloud.dialogflow.v2.EntityTypes.GetEntityType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityTypeRequest {
    /// Required. The name of the entity type.
    /// Format: `projects/<Project ID>/agent/entityTypes/<EntityType ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for \[EntityTypes.CreateEntityType][google.cloud.dialogflow.v2.EntityTypes.CreateEntityType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntityTypeRequest {
    /// Required. The agent to create a entity type for.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The entity type to create.
    #[prost(message, optional, tag = "2")]
    pub entity_type: ::core::option::Option<EntityType>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for \[EntityTypes.UpdateEntityType][google.cloud.dialogflow.v2.EntityTypes.UpdateEntityType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntityTypeRequest {
    /// Required. The entity type to update.
    #[prost(message, optional, tag = "1")]
    pub entity_type: ::core::option::Option<EntityType>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for \[EntityTypes.DeleteEntityType][google.cloud.dialogflow.v2.EntityTypes.DeleteEntityType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntityTypeRequest {
    /// Required. The name of the entity type to delete.
    /// Format: `projects/<Project ID>/agent/entityTypes/<EntityType ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[EntityTypes.BatchUpdateEntityTypes][google.cloud.dialogflow.v2.EntityTypes.BatchUpdateEntityTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateEntityTypesRequest {
    /// Required. The name of the agent to update or create entity types in.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The source of the entity type batch.
    ///
    /// For each entity type in the batch:
    ///
    /// *   If `name` is specified, we update an existing entity type.
    /// *   If `name` is not specified, we create a new entity type.
    #[prost(oneof = "batch_update_entity_types_request::EntityTypeBatch", tags = "2, 3")]
    pub entity_type_batch: ::core::option::Option<
        batch_update_entity_types_request::EntityTypeBatch,
    >,
}
/// Nested message and enum types in `BatchUpdateEntityTypesRequest`.
pub mod batch_update_entity_types_request {
    /// The source of the entity type batch.
    ///
    /// For each entity type in the batch:
    ///
    /// *   If `name` is specified, we update an existing entity type.
    /// *   If `name` is not specified, we create a new entity type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EntityTypeBatch {
        /// The URI to a Google Cloud Storage file containing entity types to update
        /// or create. The file format can either be a serialized proto (of
        /// EntityBatch type) or a JSON object. Note: The URI must start with
        /// "gs://".
        #[prost(string, tag = "2")]
        EntityTypeBatchUri(::prost::alloc::string::String),
        /// The collection of entity types to update or create.
        #[prost(message, tag = "3")]
        EntityTypeBatchInline(super::EntityTypeBatch),
    }
}
/// The response message for \[EntityTypes.BatchUpdateEntityTypes][google.cloud.dialogflow.v2.EntityTypes.BatchUpdateEntityTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateEntityTypesResponse {
    /// The collection of updated or created entity types.
    #[prost(message, repeated, tag = "1")]
    pub entity_types: ::prost::alloc::vec::Vec<EntityType>,
}
/// The request message for \[EntityTypes.BatchDeleteEntityTypes][google.cloud.dialogflow.v2.EntityTypes.BatchDeleteEntityTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteEntityTypesRequest {
    /// Required. The name of the agent to delete all entities types for. Format:
    /// `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The names entity types to delete. All names must point to the
    /// same agent as `parent`.
    #[prost(string, repeated, tag = "2")]
    pub entity_type_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request message for \[EntityTypes.BatchCreateEntities][google.cloud.dialogflow.v2.EntityTypes.BatchCreateEntities\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateEntitiesRequest {
    /// Required. The name of the entity type to create entities in. Format:
    /// `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The entities to create.
    #[prost(message, repeated, tag = "2")]
    pub entities: ::prost::alloc::vec::Vec<entity_type::Entity>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// The request message for \[EntityTypes.BatchUpdateEntities][google.cloud.dialogflow.v2.EntityTypes.BatchUpdateEntities\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateEntitiesRequest {
    /// Required. The name of the entity type to update or create entities in.
    /// Format: `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The entities to update or create.
    #[prost(message, repeated, tag = "2")]
    pub entities: ::prost::alloc::vec::Vec<entity_type::Entity>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for \[EntityTypes.BatchDeleteEntities][google.cloud.dialogflow.v2.EntityTypes.BatchDeleteEntities\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteEntitiesRequest {
    /// Required. The name of the entity type to delete entries for. Format:
    /// `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The reference `values` of the entities to delete. Note that
    /// these are not fully-qualified names, i.e. they don't start with
    /// `projects/<Project ID>`.
    #[prost(string, repeated, tag = "2")]
    pub entity_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](<https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity>).
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// This message is a wrapper around a collection of entity types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityTypeBatch {
    /// A collection of entity types.
    #[prost(message, repeated, tag = "1")]
    pub entity_types: ::prost::alloc::vec::Vec<EntityType>,
}
/// Generated client implementations.
pub mod entity_types_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [EntityTypes][google.cloud.dialogflow.v2.EntityType].
    #[derive(Debug, Clone)]
    pub struct EntityTypesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EntityTypesClient<T>
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
        ) -> EntityTypesClient<InterceptedService<T, F>>
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
            EntityTypesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all entity types in the specified agent.
        pub async fn list_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntityTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEntityTypesResponse>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/ListEntityTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "ListEntityTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified entity type.
        pub async fn get_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntityTypeRequest>,
        ) -> std::result::Result<tonic::Response<super::EntityType>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.EntityTypes/GetEntityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "GetEntityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an entity type in the specified agent.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn create_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntityTypeRequest>,
        ) -> std::result::Result<tonic::Response<super::EntityType>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.EntityTypes/CreateEntityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "CreateEntityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified entity type.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn update_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntityTypeRequest>,
        ) -> std::result::Result<tonic::Response<super::EntityType>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.EntityTypes/UpdateEntityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "UpdateEntityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified entity type.
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn delete_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntityTypeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.EntityTypes/DeleteEntityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "DeleteEntityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates/Creates multiple entity types in the specified agent.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: [BatchUpdateEntityTypesResponse][google.cloud.dialogflow.v2.BatchUpdateEntityTypesResponse]
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn batch_update_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateEntityTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchUpdateEntityTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "BatchUpdateEntityTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes entity types in the specified agent.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn batch_delete_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteEntityTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchDeleteEntityTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "BatchDeleteEntityTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates multiple new entities in the specified entity type.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn batch_create_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateEntitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchCreateEntities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "BatchCreateEntities",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates or creates multiple entities in the specified entity type. This
        /// method does not affect entities in the entity type that aren't explicitly
        /// specified in the request.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        ///
        pub async fn batch_update_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateEntitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchUpdateEntities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "BatchUpdateEntities",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes entities in the specified entity type.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: An empty [Struct
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct)
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        ///
        /// Note: You should always train an agent prior to sending it queries. See the
        /// [training
        /// documentation](https://cloud.google.com/dialogflow/es/docs/training).
        pub async fn batch_delete_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteEntitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchDeleteEntities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.EntityTypes",
                        "BatchDeleteEntities",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A session represents a conversation between a Dialogflow agent and an
/// end-user. You can create special entities, called session entities, during a
/// session. Session entities can extend or replace custom entity types and only
/// exist during the session that they were created for. All session data,
/// including session entities, is stored by Dialogflow for 20 minutes.
///
/// For more information, see the [session entity
/// guide](<https://cloud.google.com/dialogflow/docs/entities-session>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionEntityType {
    /// Required. The unique identifier of this session entity type. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/entityTypes/<Entity Type
    /// Display Name>`, or `projects/<Project ID>/agent/environments/<Environment
    /// ID>/users/<User ID>/sessions/<Session ID>/entityTypes/<Entity Type Display
    /// Name>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    ///
    /// `<Entity Type Display Name>` must be the display name of an existing entity
    /// type in the same agent that will be overridden or supplemented.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Indicates whether the additional data should override or
    /// supplement the custom entity type definition.
    #[prost(enumeration = "session_entity_type::EntityOverrideMode", tag = "2")]
    pub entity_override_mode: i32,
    /// Required. The collection of entities associated with this session entity
    /// type.
    #[prost(message, repeated, tag = "3")]
    pub entities: ::prost::alloc::vec::Vec<entity_type::Entity>,
}
/// Nested message and enum types in `SessionEntityType`.
pub mod session_entity_type {
    /// The types of modifications for a session entity type.
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
    pub enum EntityOverrideMode {
        /// Not specified. This value should be never used.
        Unspecified = 0,
        /// The collection of session entities overrides the collection of entities
        /// in the corresponding custom entity type.
        Override = 1,
        /// The collection of session entities extends the collection of entities in
        /// the corresponding custom entity type.
        ///
        /// Note: Even in this override mode calls to `ListSessionEntityTypes`,
        /// `GetSessionEntityType`, `CreateSessionEntityType` and
        /// `UpdateSessionEntityType` only return the additional entities added in
        /// this session entity type. If you want to get the supplemented list,
        /// please call \[EntityTypes.GetEntityType][google.cloud.dialogflow.v2.EntityTypes.GetEntityType\] on the custom entity type
        /// and merge.
        Supplement = 2,
    }
    impl EntityOverrideMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EntityOverrideMode::Unspecified => "ENTITY_OVERRIDE_MODE_UNSPECIFIED",
                EntityOverrideMode::Override => "ENTITY_OVERRIDE_MODE_OVERRIDE",
                EntityOverrideMode::Supplement => "ENTITY_OVERRIDE_MODE_SUPPLEMENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENTITY_OVERRIDE_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENTITY_OVERRIDE_MODE_OVERRIDE" => Some(Self::Override),
                "ENTITY_OVERRIDE_MODE_SUPPLEMENT" => Some(Self::Supplement),
                _ => None,
            }
        }
    }
}
/// The request message for \[SessionEntityTypes.ListSessionEntityTypes][google.cloud.dialogflow.v2.SessionEntityTypes.ListSessionEntityTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionEntityTypesRequest {
    /// Required. The session to list all session entity types from.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>` or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User ID>/
    /// sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[SessionEntityTypes.ListSessionEntityTypes][google.cloud.dialogflow.v2.SessionEntityTypes.ListSessionEntityTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionEntityTypesResponse {
    /// The list of session entity types. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub session_entity_types: ::prost::alloc::vec::Vec<SessionEntityType>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[SessionEntityTypes.GetSessionEntityType][google.cloud.dialogflow.v2.SessionEntityTypes.GetSessionEntityType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionEntityTypeRequest {
    /// Required. The name of the session entity type. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/entityTypes/<Entity Type
    /// Display Name>` or `projects/<Project ID>/agent/environments/<Environment
    /// ID>/users/<User ID>/sessions/<Session ID>/entityTypes/<Entity Type Display
    /// Name>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[SessionEntityTypes.CreateSessionEntityType][google.cloud.dialogflow.v2.SessionEntityTypes.CreateSessionEntityType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionEntityTypeRequest {
    /// Required. The session to create a session entity type for.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>` or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User ID>/
    /// sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The session entity type to create.
    #[prost(message, optional, tag = "2")]
    pub session_entity_type: ::core::option::Option<SessionEntityType>,
}
/// The request message for \[SessionEntityTypes.UpdateSessionEntityType][google.cloud.dialogflow.v2.SessionEntityTypes.UpdateSessionEntityType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSessionEntityTypeRequest {
    /// Required. The session entity type to update.
    #[prost(message, optional, tag = "1")]
    pub session_entity_type: ::core::option::Option<SessionEntityType>,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for \[SessionEntityTypes.DeleteSessionEntityType][google.cloud.dialogflow.v2.SessionEntityTypes.DeleteSessionEntityType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionEntityTypeRequest {
    /// Required. The name of the entity type to delete. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/entityTypes/<Entity Type
    /// Display Name>` or `projects/<Project ID>/agent/environments/<Environment
    /// ID>/users/<User ID>/sessions/<Session ID>/entityTypes/<Entity Type Display
    /// Name>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod session_entity_types_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [SessionEntityTypes][google.cloud.dialogflow.v2.SessionEntityType].
    #[derive(Debug, Clone)]
    pub struct SessionEntityTypesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SessionEntityTypesClient<T>
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
        ) -> SessionEntityTypesClient<InterceptedService<T, F>>
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
            SessionEntityTypesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all session entity types in the specified session.
        ///
        /// This method doesn't work with Google Assistant integration.
        /// Contact Dialogflow support if you need to use session entities
        /// with Google Assistant integration.
        pub async fn list_session_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSessionEntityTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSessionEntityTypesResponse>,
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/ListSessionEntityTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.SessionEntityTypes",
                        "ListSessionEntityTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified session entity type.
        ///
        /// This method doesn't work with Google Assistant integration.
        /// Contact Dialogflow support if you need to use session entities
        /// with Google Assistant integration.
        pub async fn get_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSessionEntityTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SessionEntityType>,
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/GetSessionEntityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.SessionEntityTypes",
                        "GetSessionEntityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a session entity type.
        ///
        /// If the specified session entity type already exists, overrides the session
        /// entity type.
        ///
        /// This method doesn't work with Google Assistant integration.
        /// Contact Dialogflow support if you need to use session entities
        /// with Google Assistant integration.
        pub async fn create_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSessionEntityTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SessionEntityType>,
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/CreateSessionEntityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.SessionEntityTypes",
                        "CreateSessionEntityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified session entity type.
        ///
        /// This method doesn't work with Google Assistant integration.
        /// Contact Dialogflow support if you need to use session entities
        /// with Google Assistant integration.
        pub async fn update_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSessionEntityTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SessionEntityType>,
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/UpdateSessionEntityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.SessionEntityTypes",
                        "UpdateSessionEntityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified session entity type.
        ///
        /// This method doesn't work with Google Assistant integration.
        /// Contact Dialogflow support if you need to use session entities
        /// with Google Assistant integration.
        pub async fn delete_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSessionEntityTypeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/DeleteSessionEntityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.SessionEntityTypes",
                        "DeleteSessionEntityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The request to detect user's intent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectIntentRequest {
    /// Required. The name of the session this query is sent to. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>`, or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`. If `Environment ID` is not specified, we assume
    /// default 'draft' environment (`Environment ID` might be referred to as
    /// environment name at some places). If `User ID` is not specified, we are
    /// using "-". It's up to the API caller to choose an appropriate `Session ID`
    /// and `User Id`. They can be a random number or some type of user and session
    /// identifiers (preferably hashed). The length of the `Session ID` and
    /// `User ID` must not exceed 36 characters.
    ///
    /// For more information, see the [API interactions
    /// guide](<https://cloud.google.com/dialogflow/docs/api-overview>).
    ///
    /// Note: Always use agent versions for production traffic.
    /// See [Versions and
    /// environments](<https://cloud.google.com/dialogflow/es/docs/agents-versions>).
    #[prost(string, tag = "1")]
    pub session: ::prost::alloc::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::core::option::Option<QueryParameters>,
    /// Required. The input specification. It can be set to:
    ///
    /// 1.  an audio config
    ///      which instructs the speech recognizer how to process the speech audio,
    ///
    /// 2.  a conversational query in the form of text, or
    ///
    /// 3.  an event that specifies which intent to trigger.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::core::option::Option<QueryInput>,
    /// Instructs the speech synthesizer how to generate the output
    /// audio. If this field is not set and agent-level speech synthesizer is not
    /// configured, no output audio is generated.
    #[prost(message, optional, tag = "4")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
    /// Mask for \[output_audio_config][google.cloud.dialogflow.v2.DetectIntentRequest.output_audio_config\] indicating which settings in this
    /// request-level config should override speech synthesizer settings defined at
    /// agent-level.
    ///
    /// If unspecified or empty, \[output_audio_config][google.cloud.dialogflow.v2.DetectIntentRequest.output_audio_config\] replaces the agent-level
    /// config in its entirety.
    #[prost(message, optional, tag = "7")]
    pub output_audio_config_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The natural language speech audio to be processed. This field
    /// should be populated iff `query_input` is set to an input audio config.
    /// A single request can contain up to 1 minute of speech audio data.
    #[prost(bytes = "vec", tag = "5")]
    pub input_audio: ::prost::alloc::vec::Vec<u8>,
}
/// The message returned from the DetectIntent method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectIntentResponse {
    /// The unique identifier of the response. It can be used to
    /// locate a response in the training example set or for reporting issues.
    #[prost(string, tag = "1")]
    pub response_id: ::prost::alloc::string::String,
    /// The selected results of the conversational query or event processing.
    /// See `alternative_query_results` for additional potential results.
    #[prost(message, optional, tag = "2")]
    pub query_result: ::core::option::Option<QueryResult>,
    /// Specifies the status of the webhook request.
    #[prost(message, optional, tag = "3")]
    pub webhook_status: ::core::option::Option<super::super::super::rpc::Status>,
    /// The audio data bytes encoded as specified in the request.
    /// Note: The output audio is generated based on the values of default platform
    /// text responses found in the `query_result.fulfillment_messages` field. If
    /// multiple default text responses exist, they will be concatenated when
    /// generating audio. If no default platform text responses exist, the
    /// generated audio content will be empty.
    ///
    /// In some scenarios, multiple output audio fields may be present in the
    /// response structure. In these cases, only the top-most-level audio output
    /// has content.
    #[prost(bytes = "vec", tag = "4")]
    pub output_audio: ::prost::alloc::vec::Vec<u8>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[prost(message, optional, tag = "6")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
}
/// Represents the parameters of the conversational query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParameters {
    /// The time zone of this conversational query from the
    /// [time zone database](<https://www.iana.org/time-zones>), e.g.,
    /// America/New_York, Europe/Paris. If not provided, the time zone specified in
    /// agent settings is used.
    #[prost(string, tag = "1")]
    pub time_zone: ::prost::alloc::string::String,
    /// The geo location of this conversational query.
    #[prost(message, optional, tag = "2")]
    pub geo_location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The collection of contexts to be activated before this query is
    /// executed.
    #[prost(message, repeated, tag = "3")]
    pub contexts: ::prost::alloc::vec::Vec<Context>,
    /// Specifies whether to delete all contexts in the current session
    /// before the new ones are activated.
    #[prost(bool, tag = "4")]
    pub reset_contexts: bool,
    /// Additional session entity types to replace or extend developer
    /// entity types with. The entity synonyms apply to all languages and persist
    /// for the session of this query.
    #[prost(message, repeated, tag = "5")]
    pub session_entity_types: ::prost::alloc::vec::Vec<SessionEntityType>,
    /// This field can be used to pass custom data to your webhook.
    /// Arbitrary JSON objects are supported.
    /// If supplied, the value is used to populate the
    /// `WebhookRequest.original_detect_intent_request.payload`
    /// field sent to your webhook.
    #[prost(message, optional, tag = "6")]
    pub payload: ::core::option::Option<::prost_types::Struct>,
    /// Configures the type of sentiment analysis to perform. If not
    /// provided, sentiment analysis is not performed.
    #[prost(message, optional, tag = "10")]
    pub sentiment_analysis_request_config: ::core::option::Option<
        SentimentAnalysisRequestConfig,
    >,
    /// This field can be used to pass HTTP headers for a webhook
    /// call. These headers will be sent to webhook along with the headers that
    /// have been configured through the Dialogflow web console. The headers
    /// defined within this field will overwrite the headers configured through the
    /// Dialogflow console if there is a conflict. Header names are
    /// case-insensitive. Google's specified headers are not allowed. Including:
    /// "Host", "Content-Length", "Connection", "From", "User-Agent",
    /// "Accept-Encoding", "If-Modified-Since", "If-None-Match", "X-Forwarded-For",
    /// etc.
    #[prost(map = "string, string", tag = "14")]
    pub webhook_headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Represents the query input. It can contain either:
///
/// 1.  An audio config which
///      instructs the speech recognizer how to process the speech audio.
///
/// 2.  A conversational query in the form of text,.
///
/// 3.  An event that specifies which intent to trigger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInput {
    /// Required. The input specification.
    #[prost(oneof = "query_input::Input", tags = "1, 2, 3")]
    pub input: ::core::option::Option<query_input::Input>,
}
/// Nested message and enum types in `QueryInput`.
pub mod query_input {
    /// Required. The input specification.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        /// Instructs the speech recognizer how to process the speech audio.
        #[prost(message, tag = "1")]
        AudioConfig(super::InputAudioConfig),
        /// The natural language text to be processed.
        #[prost(message, tag = "2")]
        Text(super::TextInput),
        /// The event to be processed.
        #[prost(message, tag = "3")]
        Event(super::EventInput),
    }
}
/// Represents the result of conversational query or event processing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    /// The original conversational query text:
    ///
    /// - If natural language text was provided as input, `query_text` contains
    ///    a copy of the input.
    /// - If natural language speech audio was provided as input, `query_text`
    ///    contains the speech recognition result. If speech recognizer produced
    ///    multiple alternatives, a particular one is picked.
    /// - If automatic spell correction is enabled, `query_text` will contain the
    ///    corrected user input.
    #[prost(string, tag = "1")]
    pub query_text: ::prost::alloc::string::String,
    /// The language that was triggered during intent detection.
    /// See [Language
    /// Support](<https://cloud.google.com/dialogflow/docs/reference/language>)
    /// for a list of the currently supported language codes.
    #[prost(string, tag = "15")]
    pub language_code: ::prost::alloc::string::String,
    /// The Speech recognition confidence between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. The default of 0.0 is a sentinel value indicating that confidence
    /// was not set.
    ///
    /// This field is not guaranteed to be accurate or set. In particular this
    /// field isn't set for StreamingDetectIntent since the streaming endpoint has
    /// separate confidence estimates per portion of the audio in
    /// StreamingRecognitionResult.
    #[prost(float, tag = "2")]
    pub speech_recognition_confidence: f32,
    /// The action name from the matched intent.
    #[prost(string, tag = "3")]
    pub action: ::prost::alloc::string::String,
    /// The collection of extracted parameters.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///      -   If parameter's entity type is a composite entity: map
    ///      -   Else: depending on parameter value type, could be one of string,
    ///          number, boolean, null, list or map
    /// -   MapValue value:
    ///      -   If parameter's entity type is a composite entity:
    ///          map from composite entity property names to property values
    ///      -   Else: parameter value
    #[prost(message, optional, tag = "4")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
    /// This field is set to:
    ///
    /// - `false` if the matched intent has required parameters and not all of
    ///     the required parameter values have been collected.
    /// - `true` if all required parameter values have been collected, or if the
    ///     matched intent doesn't contain any required parameters.
    #[prost(bool, tag = "5")]
    pub all_required_params_present: bool,
    /// Indicates whether the conversational query triggers a cancellation for slot
    /// filling.
    #[prost(bool, tag = "21")]
    pub cancels_slot_filling: bool,
    /// The text to be pronounced to the user or shown on the screen.
    /// Note: This is a legacy field, `fulfillment_messages` should be preferred.
    #[prost(string, tag = "6")]
    pub fulfillment_text: ::prost::alloc::string::String,
    /// The collection of rich messages to present to the user.
    #[prost(message, repeated, tag = "7")]
    pub fulfillment_messages: ::prost::alloc::vec::Vec<intent::Message>,
    /// If the query was fulfilled by a webhook call, this field is set to the
    /// value of the `source` field returned in the webhook response.
    #[prost(string, tag = "8")]
    pub webhook_source: ::prost::alloc::string::String,
    /// If the query was fulfilled by a webhook call, this field is set to the
    /// value of the `payload` field returned in the webhook response.
    #[prost(message, optional, tag = "9")]
    pub webhook_payload: ::core::option::Option<::prost_types::Struct>,
    /// The collection of output contexts. If applicable,
    /// `output_contexts.parameters` contains entries with name
    /// `<parameter name>.original` containing the original parameter values
    /// before the query.
    #[prost(message, repeated, tag = "10")]
    pub output_contexts: ::prost::alloc::vec::Vec<Context>,
    /// The intent that matched the conversational query. Some, not
    /// all fields are filled in this message, including but not limited to:
    /// `name`, `display_name`, `end_interaction` and `is_fallback`.
    #[prost(message, optional, tag = "11")]
    pub intent: ::core::option::Option<Intent>,
    /// The intent detection confidence. Values range from 0.0
    /// (completely uncertain) to 1.0 (completely certain).
    /// This value is for informational purpose only and is only used to
    /// help match the best intent within the classification threshold.
    /// This value may change for the same end-user expression at any time due to a
    /// model retraining or change in implementation.
    /// If there are `multiple knowledge_answers` messages, this value is set to
    /// the greatest `knowledgeAnswers.match_confidence` value in the list.
    #[prost(float, tag = "12")]
    pub intent_detection_confidence: f32,
    /// Free-form diagnostic information for the associated detect intent request.
    /// The fields of this data can change without notice, so you should not write
    /// code that depends on its structure.
    /// The data may contain:
    ///
    /// - webhook call latency
    /// - webhook errors
    #[prost(message, optional, tag = "14")]
    pub diagnostic_info: ::core::option::Option<::prost_types::Struct>,
    /// The sentiment analysis result, which depends on the
    /// `sentiment_analysis_request_config` specified in the request.
    #[prost(message, optional, tag = "17")]
    pub sentiment_analysis_result: ::core::option::Option<SentimentAnalysisResult>,
}
/// The top-level message sent by the client to the
/// \[Sessions.StreamingDetectIntent][google.cloud.dialogflow.v2.Sessions.StreamingDetectIntent\] method.
///
/// Multiple request messages should be sent in order:
///
/// 1.  The first message must contain
/// \[session][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.session\],
///      \[query_input][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.query_input\] plus optionally
///      \[query_params][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.query_params\]. If the client
///      wants to receive an audio response, it should also contain
///      \[output_audio_config][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.output_audio_config\].
///      The message must not contain
///      \[input_audio][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.input_audio\].
/// 2.  If \[query_input][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.query_input\] was set to
///      \[query_input.audio_config][google.cloud.dialogflow.v2.InputAudioConfig\], all subsequent
///      messages must contain
///      \[input_audio][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.input_audio\] to continue with
///      Speech recognition.
///      If you decide to rather detect an intent from text input after you
///      already started Speech recognition, please send a message with
///      \[query_input.text][google.cloud.dialogflow.v2.QueryInput.text\].
///
///      However, note that:
///
///      * Dialogflow will bill you for the audio duration so far.
///      * Dialogflow discards all Speech recognition results in favor of the
///        input text.
///      * Dialogflow will use the language code from the first message.
///
/// After you sent all input, you must half-close or abort the request stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDetectIntentRequest {
    /// Required. The name of the session the query is sent to.
    /// Format of the session name:
    /// `projects/<Project ID>/agent/sessions/<Session ID>`, or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`. If `Environment ID` is not specified, we assume
    /// default 'draft' environment. If `User ID` is not specified, we are using
    /// "-". It's up to the API caller to choose an appropriate `Session ID` and
    /// `User Id`. They can be a random number or some type of user and session
    /// identifiers (preferably hashed). The length of the `Session ID` and
    /// `User ID` must not exceed 36 characters.
    ///
    /// For more information, see the [API interactions
    /// guide](<https://cloud.google.com/dialogflow/docs/api-overview>).
    ///
    /// Note: Always use agent versions for production traffic.
    /// See [Versions and
    /// environments](<https://cloud.google.com/dialogflow/es/docs/agents-versions>).
    #[prost(string, tag = "1")]
    pub session: ::prost::alloc::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::core::option::Option<QueryParameters>,
    /// Required. The input specification. It can be set to:
    ///
    /// 1.  an audio config which instructs the speech recognizer how to process
    ///      the speech audio,
    ///
    /// 2.  a conversational query in the form of text, or
    ///
    /// 3.  an event that specifies which intent to trigger.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::core::option::Option<QueryInput>,
    /// Please use \[InputAudioConfig.single_utterance][google.cloud.dialogflow.v2.InputAudioConfig.single_utterance\] instead.
    /// If `false` (default), recognition does not cease until
    /// the client closes the stream. If `true`, the recognizer will detect a
    /// single spoken utterance in input audio. Recognition ceases when it detects
    /// the audio's voice has stopped or paused. In this case, once a detected
    /// intent is received, the client should close the stream and start a new
    /// request with a new stream as needed.
    /// This setting is ignored when `query_input` is a piece of text or an event.
    #[deprecated]
    #[prost(bool, tag = "4")]
    pub single_utterance: bool,
    /// Instructs the speech synthesizer how to generate the output
    /// audio. If this field is not set and agent-level speech synthesizer is not
    /// configured, no output audio is generated.
    #[prost(message, optional, tag = "5")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
    /// Mask for \[output_audio_config][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.output_audio_config\] indicating which settings in this
    /// request-level config should override speech synthesizer settings defined at
    /// agent-level.
    ///
    /// If unspecified or empty, \[output_audio_config][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.output_audio_config\] replaces the agent-level
    /// config in its entirety.
    #[prost(message, optional, tag = "7")]
    pub output_audio_config_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The input audio content to be recognized. Must be sent if
    /// `query_input` was set to a streaming input audio config. The complete audio
    /// over all streaming messages must not exceed 1 minute.
    #[prost(bytes = "vec", tag = "6")]
    pub input_audio: ::prost::alloc::vec::Vec<u8>,
}
/// The top-level message returned from the
/// `StreamingDetectIntent` method.
///
/// Multiple response messages can be returned in order:
///
/// 1.  If the `StreamingDetectIntentRequest.input_audio` field was
///      set, the `recognition_result` field is populated for one
///      or more messages.
///      See the \[StreamingRecognitionResult][google.cloud.dialogflow.v2.StreamingRecognitionResult\] message for details
///      about the result message sequence.
///
/// 2.  The next message contains `response_id`, `query_result`
///      and optionally `webhook_status` if a WebHook was called.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDetectIntentResponse {
    /// The unique identifier of the response. It can be used to
    /// locate a response in the training example set or for reporting issues.
    #[prost(string, tag = "1")]
    pub response_id: ::prost::alloc::string::String,
    /// The result of speech recognition.
    #[prost(message, optional, tag = "2")]
    pub recognition_result: ::core::option::Option<StreamingRecognitionResult>,
    /// The result of the conversational query or event processing.
    #[prost(message, optional, tag = "3")]
    pub query_result: ::core::option::Option<QueryResult>,
    /// Specifies the status of the webhook request.
    #[prost(message, optional, tag = "4")]
    pub webhook_status: ::core::option::Option<super::super::super::rpc::Status>,
    /// The audio data bytes encoded as specified in the request.
    /// Note: The output audio is generated based on the values of default platform
    /// text responses found in the `query_result.fulfillment_messages` field. If
    /// multiple default text responses exist, they will be concatenated when
    /// generating audio. If no default platform text responses exist, the
    /// generated audio content will be empty.
    ///
    /// In some scenarios, multiple output audio fields may be present in the
    /// response structure. In these cases, only the top-most-level audio output
    /// has content.
    #[prost(bytes = "vec", tag = "5")]
    pub output_audio: ::prost::alloc::vec::Vec<u8>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[prost(message, optional, tag = "6")]
    pub output_audio_config: ::core::option::Option<OutputAudioConfig>,
}
/// Contains a speech recognition result corresponding to a portion of the audio
/// that is currently being processed or an indication that this is the end
/// of the single requested utterance.
///
/// While end-user audio is being processed, Dialogflow sends a series of
/// results. Each result may contain a `transcript` value. A transcript
/// represents a portion of the utterance. While the recognizer is processing
/// audio, transcript values may be interim values or finalized values.
/// Once a transcript is finalized, the `is_final` value is set to true and
/// processing continues for the next transcript.
///
/// If `StreamingDetectIntentRequest.query_input.audio_config.single_utterance`
/// was true, and the recognizer has completed processing audio,
/// the `message_type` value is set to `END_OF_SINGLE_UTTERANCE and the
/// following (last) result contains the last finalized transcript.
///
/// The complete end-user utterance is determined by concatenating the
/// finalized transcript values received for the series of results.
///
/// In the following example, single utterance is enabled. In the case where
/// single utterance is not enabled, result 7 would not occur.
///
/// ```
/// Num | transcript              | message_type            | is_final
/// --- | ----------------------- | ----------------------- | --------
/// 1   | "tube"                  | TRANSCRIPT              | false
/// 2   | "to be a"               | TRANSCRIPT              | false
/// 3   | "to be"                 | TRANSCRIPT              | false
/// 4   | "to be or not to be"    | TRANSCRIPT              | true
/// 5   | "that's"                | TRANSCRIPT              | false
/// 6   | "that is                | TRANSCRIPT              | false
/// 7   | unset                   | END_OF_SINGLE_UTTERANCE | unset
/// 8   | " that is the question" | TRANSCRIPT              | true
/// ```
///
/// Concatenating the finalized transcripts with `is_final` set to true,
/// the complete utterance becomes "to be or not to be that is the question".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionResult {
    /// Type of the result message.
    #[prost(enumeration = "streaming_recognition_result::MessageType", tag = "1")]
    pub message_type: i32,
    /// Transcript text representing the words that the user spoke.
    /// Populated if and only if `message_type` = `TRANSCRIPT`.
    #[prost(string, tag = "2")]
    pub transcript: ::prost::alloc::string::String,
    /// If `false`, the `StreamingRecognitionResult` represents an
    /// interim result that may change. If `true`, the recognizer will not return
    /// any further hypotheses about this piece of the audio. May only be populated
    /// for `message_type` = `TRANSCRIPT`.
    #[prost(bool, tag = "3")]
    pub is_final: bool,
    /// The Speech confidence between 0.0 and 1.0 for the current portion of audio.
    /// A higher number indicates an estimated greater likelihood that the
    /// recognized words are correct. The default of 0.0 is a sentinel value
    /// indicating that confidence was not set.
    ///
    /// This field is typically only provided if `is_final` is true and you should
    /// not rely on it being accurate or even set.
    #[prost(float, tag = "4")]
    pub confidence: f32,
    /// Word-specific information for the words recognized by Speech in
    /// \[transcript][google.cloud.dialogflow.v2.StreamingRecognitionResult.transcript\]. Populated if and only if `message_type` = `TRANSCRIPT` and
    /// \[InputAudioConfig.enable_word_info\] is set.
    #[prost(message, repeated, tag = "7")]
    pub speech_word_info: ::prost::alloc::vec::Vec<SpeechWordInfo>,
    /// Time offset of the end of this Speech recognition result relative to the
    /// beginning of the audio. Only populated for `message_type` = `TRANSCRIPT`.
    #[prost(message, optional, tag = "8")]
    pub speech_end_offset: ::core::option::Option<::prost_types::Duration>,
    /// Detected language code for the transcript.
    #[prost(string, tag = "10")]
    pub language_code: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StreamingRecognitionResult`.
pub mod streaming_recognition_result {
    /// Type of the response message.
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
    pub enum MessageType {
        /// Not specified. Should never be used.
        Unspecified = 0,
        /// Message contains a (possibly partial) transcript.
        Transcript = 1,
        /// Event indicates that the server has detected the end of the user's speech
        /// utterance and expects no additional inputs.
        /// Therefore, the server will not process additional audio (although it may subsequently return additional results). The
        /// client should stop sending additional audio data, half-close the gRPC
        /// connection, and wait for any additional results until the server closes
        /// the gRPC connection. This message is only sent if `single_utterance` was
        /// set to `true`, and is not used otherwise.
        EndOfSingleUtterance = 2,
    }
    impl MessageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MessageType::Unspecified => "MESSAGE_TYPE_UNSPECIFIED",
                MessageType::Transcript => "TRANSCRIPT",
                MessageType::EndOfSingleUtterance => "END_OF_SINGLE_UTTERANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MESSAGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TRANSCRIPT" => Some(Self::Transcript),
                "END_OF_SINGLE_UTTERANCE" => Some(Self::EndOfSingleUtterance),
                _ => None,
            }
        }
    }
}
/// Represents the natural language text to be processed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInput {
    /// Required. The UTF-8 encoded natural language text to be processed.
    /// Text length must not exceed 256 characters.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// Required. The language of this conversational query. See [Language
    /// Support](<https://cloud.google.com/dialogflow/docs/reference/language>)
    /// for a list of the currently supported language codes. Note that queries in
    /// the same session do not necessarily need to specify the same language.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// Events allow for matching intents by event name instead of the natural
/// language input. For instance, input `<event: { name: "welcome_event",
/// parameters: { name: "Sam" } }>` can trigger a personalized welcome response.
/// The parameter `name` may be used by the agent in the response:
/// `"Hello #welcome_event.name! What can I do for you today?"`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventInput {
    /// Required. The unique identifier of the event.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The collection of parameters associated with the event.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///      -   If parameter's entity type is a composite entity: map
    ///      -   Else: depending on parameter value type, could be one of string,
    ///          number, boolean, null, list or map
    /// -   MapValue value:
    ///      -   If parameter's entity type is a composite entity:
    ///          map from composite entity property names to property values
    ///      -   Else: parameter value
    #[prost(message, optional, tag = "2")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
    /// Required. The language of this query. See [Language
    /// Support](<https://cloud.google.com/dialogflow/docs/reference/language>)
    /// for a list of the currently supported language codes. Note that queries in
    /// the same session do not necessarily need to specify the same language.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Configures the types of sentiment analysis to perform.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentimentAnalysisRequestConfig {
    /// Instructs the service to perform sentiment analysis on
    /// `query_text`. If not provided, sentiment analysis is not performed on
    /// `query_text`.
    #[prost(bool, tag = "1")]
    pub analyze_query_text_sentiment: bool,
}
/// The result of sentiment analysis. Sentiment analysis inspects user input
/// and identifies the prevailing subjective opinion, especially to determine a
/// user's attitude as positive, negative, or neutral.
/// For \[Participants.DetectIntent][\], it needs to be configured in
/// \[DetectIntentRequest.query_params][google.cloud.dialogflow.v2.DetectIntentRequest.query_params\]. For
/// \[Participants.StreamingDetectIntent][\], it needs to be configured in
/// \[StreamingDetectIntentRequest.query_params][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.query_params\].
/// And for \[Participants.AnalyzeContent][google.cloud.dialogflow.v2.Participants.AnalyzeContent\] and
/// \[Participants.StreamingAnalyzeContent][google.cloud.dialogflow.v2.Participants.StreamingAnalyzeContent\], it needs to be configured in
/// \[ConversationProfile.human_agent_assistant_config][google.cloud.dialogflow.v2.ConversationProfile.human_agent_assistant_config\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentimentAnalysisResult {
    /// The sentiment analysis result for `query_text`.
    #[prost(message, optional, tag = "1")]
    pub query_text_sentiment: ::core::option::Option<Sentiment>,
}
/// The sentiment, such as positive/negative feeling or association, for a unit
/// of analysis, such as the query text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sentiment {
    /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive
    /// sentiment).
    #[prost(float, tag = "1")]
    pub score: f32,
    /// A non-negative number in the [0, +inf) range, which represents the absolute
    /// magnitude of sentiment, regardless of score (positive or negative).
    #[prost(float, tag = "2")]
    pub magnitude: f32,
}
/// Generated client implementations.
pub mod sessions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service used for session interactions.
    ///
    /// For more information, see the [API interactions
    /// guide](https://cloud.google.com/dialogflow/docs/api-overview).
    #[derive(Debug, Clone)]
    pub struct SessionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SessionsClient<T>
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
        ) -> SessionsClient<InterceptedService<T, F>>
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
            SessionsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Processes a natural language query and returns structured, actionable data
        /// as a result. This method is not idempotent, because it may cause contexts
        /// and session entity types to be updated, which in turn might affect
        /// results of future queries.
        ///
        /// If you might use
        /// [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa)
        /// or other CCAI products now or in the future, consider using
        /// [AnalyzeContent][google.cloud.dialogflow.v2.Participants.AnalyzeContent]
        /// instead of `DetectIntent`. `AnalyzeContent` has additional
        /// functionality for Agent Assist and other CCAI products.
        ///
        /// Note: Always use agent versions for production traffic.
        /// See [Versions and
        /// environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
        pub async fn detect_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::DetectIntentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DetectIntentResponse>,
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
                "/google.cloud.dialogflow.v2.Sessions/DetectIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Sessions",
                        "DetectIntent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Processes a natural language query in audio format in a streaming fashion
        /// and returns structured, actionable data as a result. This method is only
        /// available via the gRPC API (not REST).
        ///
        /// If you might use
        /// [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa)
        /// or other CCAI products now or in the future, consider using
        /// [StreamingAnalyzeContent][google.cloud.dialogflow.v2.Participants.StreamingAnalyzeContent]
        /// instead of `StreamingDetectIntent`. `StreamingAnalyzeContent` has
        /// additional functionality for Agent Assist and other CCAI products.
        ///
        /// Note: Always use agent versions for production traffic.
        /// See [Versions and
        /// environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
        pub async fn streaming_detect_intent(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::StreamingDetectIntentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamingDetectIntentResponse>,
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
                "/google.cloud.dialogflow.v2.Sessions/StreamingDetectIntent",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Sessions",
                        "StreamingDetectIntent",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Represents a conversation participant (human agent, virtual agent, end-user).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Participant {
    /// Optional. The unique identifier of this participant.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/participants/<Participant ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The role this participant plays in the conversation. This field
    /// must be set during participant creation and is then immutable.
    #[prost(enumeration = "participant::Role", tag = "2")]
    pub role: i32,
    /// Optional. Label applied to streams representing this participant in SIPREC
    /// XML metadata and SDP. This is used to assign transcriptions from that
    /// media stream to this participant. This field can be updated.
    #[prost(string, tag = "6")]
    pub sip_recording_media_label: ::prost::alloc::string::String,
    /// Optional. Key-value filters on the metadata of documents returned by
    /// article suggestion. If specified, article suggestion only returns suggested
    /// documents that match all filters in their
    /// \[Document.metadata][google.cloud.dialogflow.v2.Document.metadata\]. Multiple
    /// values for a metadata key should be concatenated by comma. For example,
    /// filters to match all documents that have 'US' or 'CA' in their market
    /// metadata values and 'agent' in their user metadata values will be
    /// ```
    /// documents_metadata_filters {
    ///    key: "market"
    ///    value: "US,CA"
    /// }
    /// documents_metadata_filters {
    ///    key: "user"
    ///    value: "agent"
    /// }
    /// ```
    #[prost(map = "string, string", tag = "8")]
    pub documents_metadata_filters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `Participant`.
pub mod participant {
    /// Enumeration of the roles a participant can play in a conversation.
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
    pub enum Role {
        /// Participant role not set.
        Unspecified = 0,
        /// Participant is a human agent.
        HumanAgent = 1,
        /// Participant is an automated agent, such as a Dialogflow agent.
        AutomatedAgent = 2,
        /// Participant is an end user that has called or chatted with
        /// Dialogflow services.
        EndUser = 3,
    }
    impl Role {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Role::Unspecified => "ROLE_UNSPECIFIED",
                Role::HumanAgent => "HUMAN_AGENT",
                Role::AutomatedAgent => "AUTOMATED_AGENT",
                Role::EndUser => "END_USER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLE_UNSPECIFIED" => Some(Self::Unspecified),
                "HUMAN_AGENT" => Some(Self::HumanAgent),
                "AUTOMATED_AGENT" => Some(Self::AutomatedAgent),
                "END_USER" => Some(Self::EndUser),
                _ => None,
            }
        }
    }
}
/// Represents a message posted into a conversation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// The unique identifier of the message.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/messages/<Message ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The message content.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// Optional. The message language.
    /// This should be a \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>)
    /// language tag. Example: "en-US".
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// Output only. The participant that sends this message.
    #[prost(string, tag = "4")]
    pub participant: ::prost::alloc::string::String,
    /// Output only. The role of the participant.
    #[prost(enumeration = "participant::Role", tag = "5")]
    pub participant_role: i32,
    /// Output only. The time when the message was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The annotation for the message.
    #[prost(message, optional, tag = "7")]
    pub message_annotation: ::core::option::Option<MessageAnnotation>,
}
/// The request message for
/// \[Participants.CreateParticipant][google.cloud.dialogflow.v2.Participants.CreateParticipant\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateParticipantRequest {
    /// Required. Resource identifier of the conversation adding the participant.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The participant to create.
    #[prost(message, optional, tag = "2")]
    pub participant: ::core::option::Option<Participant>,
}
/// The request message for
/// \[Participants.GetParticipant][google.cloud.dialogflow.v2.Participants.GetParticipant\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantRequest {
    /// Required. The name of the participant. Format:
    /// `projects/<Project ID>/locations/<Location ID>/conversations/<Conversation
    /// ID>/participants/<Participant ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for
/// \[Participants.ListParticipants][google.cloud.dialogflow.v2.Participants.ListParticipants\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParticipantsRequest {
    /// Required. The conversation to list all participants from.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for
/// \[Participants.ListParticipants][google.cloud.dialogflow.v2.Participants.ListParticipants\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParticipantsResponse {
    /// The list of participants. There is a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub participants: ::prost::alloc::vec::Vec<Participant>,
    /// Token to retrieve the next page of results or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for
/// \[Participants.UpdateParticipant][google.cloud.dialogflow.v2.Participants.UpdateParticipant\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateParticipantRequest {
    /// Required. The participant to update.
    #[prost(message, optional, tag = "1")]
    pub participant: ::core::option::Option<Participant>,
    /// Required. The mask to specify which fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for
/// \[Participants.AnalyzeContent][google.cloud.dialogflow.v2.Participants.AnalyzeContent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeContentRequest {
    /// Required. The name of the participant this text comes from.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/participants/<Participant ID>`.
    #[prost(string, tag = "1")]
    pub participant: ::prost::alloc::string::String,
    /// Speech synthesis configuration.
    /// The speech synthesis settings for a virtual agent that may be configured
    /// for the associated conversation profile are not used when calling
    /// AnalyzeContent. If this configuration is not supplied, speech synthesis
    /// is disabled.
    #[prost(message, optional, tag = "5")]
    pub reply_audio_config: ::core::option::Option<OutputAudioConfig>,
    /// Parameters for a Dialogflow virtual-agent query.
    #[prost(message, optional, tag = "9")]
    pub query_params: ::core::option::Option<QueryParameters>,
    /// Parameters for a human assist query.
    #[prost(message, optional, tag = "14")]
    pub assist_query_params: ::core::option::Option<AssistQueryParameters>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters.
    /// A random UUID is recommended.
    /// This request is only idempotent if a `request_id` is provided.
    #[prost(string, tag = "11")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. The input content.
    #[prost(oneof = "analyze_content_request::Input", tags = "6, 8")]
    pub input: ::core::option::Option<analyze_content_request::Input>,
}
/// Nested message and enum types in `AnalyzeContentRequest`.
pub mod analyze_content_request {
    /// Required. The input content.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        /// The natural language text to be processed.
        #[prost(message, tag = "6")]
        TextInput(super::TextInput),
        /// An input event to send to Dialogflow.
        #[prost(message, tag = "8")]
        EventInput(super::EventInput),
    }
}
/// The message in the response that indicates the parameters of DTMF.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtmfParameters {
    /// Indicates whether DTMF input can be handled in the next request.
    #[prost(bool, tag = "1")]
    pub accepts_dtmf_input: bool,
}
/// The response message for
/// \[Participants.AnalyzeContent][google.cloud.dialogflow.v2.Participants.AnalyzeContent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeContentResponse {
    /// The output text content.
    /// This field is set if the automated agent responded with text to show to
    /// the user.
    #[prost(string, tag = "1")]
    pub reply_text: ::prost::alloc::string::String,
    /// The audio data bytes encoded as specified in the request.
    /// This field is set if:
    ///
    ///   - `reply_audio_config` was specified in the request, or
    ///   - The automated agent responded with audio to play to the user. In such
    ///     case, `reply_audio.config` contains settings used to synthesize the
    ///     speech.
    ///
    /// In some scenarios, multiple output audio fields may be present in the
    /// response structure. In these cases, only the top-most-level audio output
    /// has content.
    #[prost(message, optional, tag = "2")]
    pub reply_audio: ::core::option::Option<OutputAudio>,
    /// Only set if a Dialogflow automated agent has responded.
    /// Note that: \[AutomatedAgentReply.detect_intent_response.output_audio][\]
    /// and \[AutomatedAgentReply.detect_intent_response.output_audio_config][\]
    /// are always empty, use
    /// \[reply_audio][google.cloud.dialogflow.v2.AnalyzeContentResponse.reply_audio\]
    /// instead.
    #[prost(message, optional, tag = "3")]
    pub automated_agent_reply: ::core::option::Option<AutomatedAgentReply>,
    /// Message analyzed by CCAI.
    #[prost(message, optional, tag = "5")]
    pub message: ::core::option::Option<Message>,
    /// The suggestions for most recent human agent. The order is the same as
    /// \[HumanAgentAssistantConfig.SuggestionConfig.feature_configs][google.cloud.dialogflow.v2.HumanAgentAssistantConfig.SuggestionConfig.feature_configs\]
    /// of
    /// \[HumanAgentAssistantConfig.human_agent_suggestion_config][google.cloud.dialogflow.v2.HumanAgentAssistantConfig.human_agent_suggestion_config\].
    #[prost(message, repeated, tag = "6")]
    pub human_agent_suggestion_results: ::prost::alloc::vec::Vec<SuggestionResult>,
    /// The suggestions for end user. The order is the same as
    /// \[HumanAgentAssistantConfig.SuggestionConfig.feature_configs][google.cloud.dialogflow.v2.HumanAgentAssistantConfig.SuggestionConfig.feature_configs\]
    /// of
    /// \[HumanAgentAssistantConfig.end_user_suggestion_config][google.cloud.dialogflow.v2.HumanAgentAssistantConfig.end_user_suggestion_config\].
    #[prost(message, repeated, tag = "7")]
    pub end_user_suggestion_results: ::prost::alloc::vec::Vec<SuggestionResult>,
    /// Indicates the parameters of DTMF.
    #[prost(message, optional, tag = "9")]
    pub dtmf_parameters: ::core::option::Option<DtmfParameters>,
}
/// The request message for
/// \[Participants.SuggestArticles][google.cloud.dialogflow.v2.Participants.SuggestArticles\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestArticlesRequest {
    /// Required. The name of the participant to fetch suggestion for.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/participants/<Participant ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The name of the latest conversation message to compile suggestion
    /// for. If empty, it will be the latest message of the conversation.
    ///
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/messages/<Message ID>`.
    #[prost(string, tag = "2")]
    pub latest_message: ::prost::alloc::string::String,
    /// Max number of messages prior to and including
    /// \[latest_message][google.cloud.dialogflow.v2.SuggestArticlesRequest.latest_message\]
    /// to use as context when compiling the suggestion. By default 20 and at
    /// most 50.
    #[prost(int32, tag = "3")]
    pub context_size: i32,
    /// Parameters for a human assist query.
    #[prost(message, optional, tag = "4")]
    pub assist_query_params: ::core::option::Option<AssistQueryParameters>,
}
/// The response message for
/// \[Participants.SuggestArticles][google.cloud.dialogflow.v2.Participants.SuggestArticles\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestArticlesResponse {
    /// Articles ordered by score in descending order.
    #[prost(message, repeated, tag = "1")]
    pub article_answers: ::prost::alloc::vec::Vec<ArticleAnswer>,
    /// The name of the latest conversation message used to compile
    /// suggestion for.
    ///
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/messages/<Message ID>`.
    #[prost(string, tag = "2")]
    pub latest_message: ::prost::alloc::string::String,
    /// Number of messages prior to and including
    /// \[latest_message][google.cloud.dialogflow.v2.SuggestArticlesResponse.latest_message\]
    /// to compile the suggestion. It may be smaller than the
    /// \[SuggestArticlesRequest.context_size][google.cloud.dialogflow.v2.SuggestArticlesRequest.context_size\]
    /// field in the request if there aren't that many messages in the
    /// conversation.
    #[prost(int32, tag = "3")]
    pub context_size: i32,
}
/// The request message for
/// \[Participants.SuggestFaqAnswers][google.cloud.dialogflow.v2.Participants.SuggestFaqAnswers\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestFaqAnswersRequest {
    /// Required. The name of the participant to fetch suggestion for.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/participants/<Participant ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The name of the latest conversation message to compile suggestion
    /// for. If empty, it will be the latest message of the conversation.
    ///
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/messages/<Message ID>`.
    #[prost(string, tag = "2")]
    pub latest_message: ::prost::alloc::string::String,
    /// Max number of messages prior to and including
    /// \[latest_message\] to use as context when compiling the
    /// suggestion. By default 20 and at most 50.
    #[prost(int32, tag = "3")]
    pub context_size: i32,
    /// Parameters for a human assist query.
    #[prost(message, optional, tag = "4")]
    pub assist_query_params: ::core::option::Option<AssistQueryParameters>,
}
/// The request message for
/// \[Participants.SuggestFaqAnswers][google.cloud.dialogflow.v2.Participants.SuggestFaqAnswers\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestFaqAnswersResponse {
    /// Answers extracted from FAQ documents.
    #[prost(message, repeated, tag = "1")]
    pub faq_answers: ::prost::alloc::vec::Vec<FaqAnswer>,
    /// The name of the latest conversation message used to compile
    /// suggestion for.
    ///
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>/messages/<Message ID>`.
    #[prost(string, tag = "2")]
    pub latest_message: ::prost::alloc::string::String,
    /// Number of messages prior to and including
    /// \[latest_message][google.cloud.dialogflow.v2.SuggestFaqAnswersResponse.latest_message\]
    /// to compile the suggestion. It may be smaller than the
    /// \[SuggestFaqAnswersRequest.context_size][google.cloud.dialogflow.v2.SuggestFaqAnswersRequest.context_size\]
    /// field in the request if there aren't that many messages in the
    /// conversation.
    #[prost(int32, tag = "3")]
    pub context_size: i32,
}
/// Represents the natural language speech audio to be played to the end user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputAudio {
    /// Instructs the speech synthesizer how to generate the speech
    /// audio.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<OutputAudioConfig>,
    /// The natural language speech audio.
    #[prost(bytes = "vec", tag = "2")]
    pub audio: ::prost::alloc::vec::Vec<u8>,
}
/// Represents a response from an automated agent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomatedAgentReply {
    /// Response of the Dialogflow
    /// \[Sessions.DetectIntent][google.cloud.dialogflow.v2.Sessions.DetectIntent\]
    /// call.
    #[prost(message, optional, tag = "1")]
    pub detect_intent_response: ::core::option::Option<DetectIntentResponse>,
    /// AutomatedAgentReply type.
    #[prost(enumeration = "automated_agent_reply::AutomatedAgentReplyType", tag = "7")]
    pub automated_agent_reply_type: i32,
    /// Indicates whether the partial automated agent reply is interruptible when a
    /// later reply message arrives. e.g. if the agent specified some music as
    /// partial response, it can be cancelled.
    #[prost(bool, tag = "8")]
    pub allow_cancellation: bool,
}
/// Nested message and enum types in `AutomatedAgentReply`.
pub mod automated_agent_reply {
    /// Represents different automated agent reply types.
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
    pub enum AutomatedAgentReplyType {
        /// Not specified. This should never happen.
        Unspecified = 0,
        /// Partial reply. e.g. Aggregated responses in a `Fulfillment` that enables
        /// `return_partial_response` can be returned as partial reply.
        /// WARNING: partial reply is not eligible for barge-in.
        Partial = 1,
        /// Final reply.
        Final = 2,
    }
    impl AutomatedAgentReplyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AutomatedAgentReplyType::Unspecified => {
                    "AUTOMATED_AGENT_REPLY_TYPE_UNSPECIFIED"
                }
                AutomatedAgentReplyType::Partial => "PARTIAL",
                AutomatedAgentReplyType::Final => "FINAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AUTOMATED_AGENT_REPLY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PARTIAL" => Some(Self::Partial),
                "FINAL" => Some(Self::Final),
                _ => None,
            }
        }
    }
}
/// Represents article answer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArticleAnswer {
    /// The article title.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// The article URI.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Article snippets.
    #[prost(string, repeated, tag = "3")]
    pub snippets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Article match confidence.
    /// The system's confidence score that this article is a good match for this
    /// conversation, as a value from 0.0 (completely uncertain) to 1.0
    /// (completely certain).
    #[prost(float, tag = "4")]
    pub confidence: f32,
    /// A map that contains metadata about the answer and the
    /// document from which it originates.
    #[prost(map = "string, string", tag = "5")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The name of answer record, in the format of
    /// "projects/<Project ID>/locations/<Location ID>/answerRecords/<Answer Record
    /// ID>"
    #[prost(string, tag = "6")]
    pub answer_record: ::prost::alloc::string::String,
}
/// Represents answer from "frequently asked questions".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaqAnswer {
    /// The piece of text from the `source` knowledge base document.
    #[prost(string, tag = "1")]
    pub answer: ::prost::alloc::string::String,
    /// The system's confidence score that this Knowledge answer is a good match
    /// for this conversational query, range from 0.0 (completely uncertain)
    /// to 1.0 (completely certain).
    #[prost(float, tag = "2")]
    pub confidence: f32,
    /// The corresponding FAQ question.
    #[prost(string, tag = "3")]
    pub question: ::prost::alloc::string::String,
    /// Indicates which Knowledge Document this answer was extracted
    /// from.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/agent/knowledgeBases/<Knowledge Base ID>/documents/<Document ID>`.
    #[prost(string, tag = "4")]
    pub source: ::prost::alloc::string::String,
    /// A map that contains metadata about the answer and the
    /// document from which it originates.
    #[prost(map = "string, string", tag = "5")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The name of answer record, in the format of
    /// "projects/<Project ID>/locations/<Location ID>/answerRecords/<Answer Record
    /// ID>"
    #[prost(string, tag = "6")]
    pub answer_record: ::prost::alloc::string::String,
}
/// One response of different type of suggestion response which is used in
/// the response of
/// \[Participants.AnalyzeContent][google.cloud.dialogflow.v2.Participants.AnalyzeContent\]
/// and
/// \[Participants.AnalyzeContent][google.cloud.dialogflow.v2.Participants.AnalyzeContent\],
/// as well as
/// \[HumanAgentAssistantEvent][google.cloud.dialogflow.v2.HumanAgentAssistantEvent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestionResult {
    /// Different type of suggestion response.
    #[prost(oneof = "suggestion_result::SuggestionResponse", tags = "1, 2, 3")]
    pub suggestion_response: ::core::option::Option<
        suggestion_result::SuggestionResponse,
    >,
}
/// Nested message and enum types in `SuggestionResult`.
pub mod suggestion_result {
    /// Different type of suggestion response.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SuggestionResponse {
        /// Error status if the request failed.
        #[prost(message, tag = "1")]
        Error(super::super::super::super::rpc::Status),
        /// SuggestArticlesResponse if request is for ARTICLE_SUGGESTION.
        #[prost(message, tag = "2")]
        SuggestArticlesResponse(super::SuggestArticlesResponse),
        /// SuggestFaqAnswersResponse if request is for FAQ_ANSWER.
        #[prost(message, tag = "3")]
        SuggestFaqAnswersResponse(super::SuggestFaqAnswersResponse),
    }
}
/// Represents a part of a message possibly annotated with an entity. The part
/// can be an entity or purely a part of the message between two entities or
/// message start/end.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotatedMessagePart {
    /// A part of a message possibly annotated with an entity.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// The [Dialogflow system entity
    /// type](<https://cloud.google.com/dialogflow/docs/reference/system-entities>)
    /// of this message part. If this is empty, Dialogflow could not annotate the
    /// phrase part with a system entity.
    #[prost(string, tag = "2")]
    pub entity_type: ::prost::alloc::string::String,
    /// The [Dialogflow system entity formatted value
    /// ](<https://cloud.google.com/dialogflow/docs/reference/system-entities>) of
    /// this message part. For example for a system entity of type
    /// `@sys.unit-currency`, this may contain:
    /// <pre>
    /// {
    ///    "amount": 5,
    ///    "currency": "USD"
    /// }
    /// </pre>
    #[prost(message, optional, tag = "3")]
    pub formatted_value: ::core::option::Option<::prost_types::Value>,
}
/// Represents the result of annotation for the message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAnnotation {
    /// The collection of annotated message parts ordered by their
    /// position in the message. You can recover the annotated message by
    /// concatenating \[AnnotatedMessagePart.text\].
    #[prost(message, repeated, tag = "1")]
    pub parts: ::prost::alloc::vec::Vec<AnnotatedMessagePart>,
    /// Indicates whether the text message contains entities.
    #[prost(bool, tag = "2")]
    pub contain_entities: bool,
}
/// Represents the parameters of human assist query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssistQueryParameters {
    /// Key-value filters on the metadata of documents returned by article
    /// suggestion. If specified, article suggestion only returns suggested
    /// documents that match all filters in their
    /// \[Document.metadata][google.cloud.dialogflow.v2.Document.metadata\]. Multiple
    /// values for a metadata key should be concatenated by comma. For example,
    /// filters to match all documents that have 'US' or 'CA' in their market
    /// metadata values and 'agent' in their user metadata values will be
    /// documents_metadata_filters {
    ///    key: "market"
    ///    value: "US,CA"
    /// }
    /// documents_metadata_filters {
    ///    key: "user"
    ///    value: "agent"
    /// }
    #[prost(map = "string, string", tag = "1")]
    pub documents_metadata_filters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Generated client implementations.
pub mod participants_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Participants][google.cloud.dialogflow.v2.Participant].
    #[derive(Debug, Clone)]
    pub struct ParticipantsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ParticipantsClient<T>
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
        ) -> ParticipantsClient<InterceptedService<T, F>>
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
            ParticipantsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new participant in a conversation.
        pub async fn create_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateParticipantRequest>,
        ) -> std::result::Result<tonic::Response<super::Participant>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Participants/CreateParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Participants",
                        "CreateParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a conversation participant.
        pub async fn get_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParticipantRequest>,
        ) -> std::result::Result<tonic::Response<super::Participant>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Participants/GetParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Participants",
                        "GetParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the list of all participants in the specified conversation.
        pub async fn list_participants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListParticipantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListParticipantsResponse>,
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
                "/google.cloud.dialogflow.v2.Participants/ListParticipants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Participants",
                        "ListParticipants",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified participant.
        pub async fn update_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateParticipantRequest>,
        ) -> std::result::Result<tonic::Response<super::Participant>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Participants/UpdateParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Participants",
                        "UpdateParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Adds a text (chat, for example), or audio (phone recording, for example)
        /// message from a participant into the conversation.
        ///
        /// Note: Always use agent versions for production traffic
        /// sent to virtual agents. See [Versions and
        /// environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).
        pub async fn analyze_content(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeContentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnalyzeContentResponse>,
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
                "/google.cloud.dialogflow.v2.Participants/AnalyzeContent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Participants",
                        "AnalyzeContent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets suggested articles for a participant based on specific historical
        /// messages.
        pub async fn suggest_articles(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestArticlesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuggestArticlesResponse>,
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
                "/google.cloud.dialogflow.v2.Participants/SuggestArticles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Participants",
                        "SuggestArticles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets suggested faq answers for a participant based on specific historical
        /// messages.
        pub async fn suggest_faq_answers(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestFaqAnswersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuggestFaqAnswersResponse>,
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
                "/google.cloud.dialogflow.v2.Participants/SuggestFaqAnswers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Participants",
                        "SuggestFaqAnswers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Answer records are records to manage answer history and feedbacks for
/// Dialogflow.
///
/// Currently, answer record includes:
///
/// - human agent assistant article suggestion
/// - human agent assistant faq article
///
/// It doesn't include:
///
/// - `DetectIntent` intent matching
/// - `DetectIntent` knowledge
///
/// Answer records are not related to the conversation history in the
/// Dialogflow Console. A Record is generated even when the end-user disables
/// conversation history in the console. Records are created when there's a human
/// agent assistant suggestion generated.
///
/// A typical workflow for customers provide feedback to an answer is:
///
/// 1. For human agent assistant, customers get suggestion via ListSuggestions
///     API. Together with the answers, \[AnswerRecord.name][google.cloud.dialogflow.v2.AnswerRecord.name\] are returned to the
///     customers.
/// 2. The customer uses the \[AnswerRecord.name][google.cloud.dialogflow.v2.AnswerRecord.name\] to call the
///     \[UpdateAnswerRecord][\] method to send feedback about a specific answer
///     that they believe is wrong.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerRecord {
    /// The unique identifier of this answer record.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/answerRecords/<Answer Record ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The AnswerFeedback for this record. You can set this with
    /// \[AnswerRecords.UpdateAnswerRecord][google.cloud.dialogflow.v2.AnswerRecords.UpdateAnswerRecord\] in order to give us feedback about
    /// this answer.
    #[prost(message, optional, tag = "2")]
    pub answer_feedback: ::core::option::Option<AnswerFeedback>,
    /// The record for this answer.
    #[prost(oneof = "answer_record::Record", tags = "4")]
    pub record: ::core::option::Option<answer_record::Record>,
}
/// Nested message and enum types in `AnswerRecord`.
pub mod answer_record {
    /// The record for this answer.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Record {
        /// Output only. The record for human agent assistant.
        #[prost(message, tag = "4")]
        AgentAssistantRecord(super::AgentAssistantRecord),
    }
}
/// Request message for \[AnswerRecords.ListAnswerRecords][google.cloud.dialogflow.v2.AnswerRecords.ListAnswerRecords\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnswerRecordsRequest {
    /// Required. The project to list all answer records for in reverse
    /// chronological order. Format: `projects/<Project ID>/locations/<Location
    /// ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Filters to restrict results to specific answer records.
    /// Filter on answer record type. Currently predicates on `type` is supported,
    /// valid values are `ARTICLE_ANSWER`, `FAQ_ANSWER`.
    ///
    /// For more information about filtering, see
    /// [API Filtering](<https://aip.dev/160>).
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The maximum number of records to return in a single page.
    /// The server may return fewer records than this. If unspecified, we use 10.
    /// The maximum is 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. The
    /// \[ListAnswerRecordsResponse.next_page_token][google.cloud.dialogflow.v2.ListAnswerRecordsResponse.next_page_token\]
    /// value returned from a previous list request used to continue listing on
    /// the next page.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[AnswerRecords.ListAnswerRecords][google.cloud.dialogflow.v2.AnswerRecords.ListAnswerRecords\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnswerRecordsResponse {
    /// The list of answer records.
    #[prost(message, repeated, tag = "1")]
    pub answer_records: ::prost::alloc::vec::Vec<AnswerRecord>,
    /// A token to retrieve next page of results. Or empty if there are no more
    /// results.
    /// Pass this value in the
    /// \[ListAnswerRecordsRequest.page_token][google.cloud.dialogflow.v2.ListAnswerRecordsRequest.page_token\]
    /// field in the subsequent call to `ListAnswerRecords` method to retrieve the
    /// next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[AnswerRecords.UpdateAnswerRecord][google.cloud.dialogflow.v2.AnswerRecords.UpdateAnswerRecord\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAnswerRecordRequest {
    /// Required. Answer record to update.
    #[prost(message, optional, tag = "1")]
    pub answer_record: ::core::option::Option<AnswerRecord>,
    /// Required. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Represents feedback the customer has about the quality & correctness of a
/// certain answer in a conversation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerFeedback {
    /// The correctness level of the specific answer.
    #[prost(enumeration = "answer_feedback::CorrectnessLevel", tag = "1")]
    pub correctness_level: i32,
    /// Indicates whether the answer/item was clicked by the human agent
    /// or not. Default to false.
    #[prost(bool, tag = "3")]
    pub clicked: bool,
    /// Time when the answer/item was clicked.
    #[prost(message, optional, tag = "5")]
    pub click_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates whether the answer/item was displayed to the human
    /// agent in the agent desktop UI. Default to false.
    #[prost(bool, tag = "4")]
    pub displayed: bool,
    /// Time when the answer/item was displayed.
    #[prost(message, optional, tag = "6")]
    pub display_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Normally, detail feedback is provided when answer is not fully correct.
    #[prost(oneof = "answer_feedback::DetailFeedback", tags = "2")]
    pub detail_feedback: ::core::option::Option<answer_feedback::DetailFeedback>,
}
/// Nested message and enum types in `AnswerFeedback`.
pub mod answer_feedback {
    /// The correctness level of an answer.
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
    pub enum CorrectnessLevel {
        /// Correctness level unspecified.
        Unspecified = 0,
        /// Answer is totally wrong.
        NotCorrect = 1,
        /// Answer is partially correct.
        PartiallyCorrect = 2,
        /// Answer is fully correct.
        FullyCorrect = 3,
    }
    impl CorrectnessLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CorrectnessLevel::Unspecified => "CORRECTNESS_LEVEL_UNSPECIFIED",
                CorrectnessLevel::NotCorrect => "NOT_CORRECT",
                CorrectnessLevel::PartiallyCorrect => "PARTIALLY_CORRECT",
                CorrectnessLevel::FullyCorrect => "FULLY_CORRECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CORRECTNESS_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                "NOT_CORRECT" => Some(Self::NotCorrect),
                "PARTIALLY_CORRECT" => Some(Self::PartiallyCorrect),
                "FULLY_CORRECT" => Some(Self::FullyCorrect),
                _ => None,
            }
        }
    }
    /// Normally, detail feedback is provided when answer is not fully correct.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DetailFeedback {
        /// Detail feedback of agent assist suggestions.
        #[prost(message, tag = "2")]
        AgentAssistantDetailFeedback(super::AgentAssistantFeedback),
    }
}
/// Detail feedback of Agent Assist result.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentAssistantFeedback {
    /// Optional. Whether or not the suggested answer is relevant.
    ///
    /// For example:
    ///
    /// * Query: "Can I change my mailing address?"
    /// * Suggested document says: "Items must be returned/exchanged within 60
    ///    days of the purchase date."
    /// * \[answer_relevance][google.cloud.dialogflow.v2.AgentAssistantFeedback.answer_relevance\]: \[AnswerRelevance.IRRELEVANT][google.cloud.dialogflow.v2.AgentAssistantFeedback.AnswerRelevance.IRRELEVANT\]
    #[prost(enumeration = "agent_assistant_feedback::AnswerRelevance", tag = "1")]
    pub answer_relevance: i32,
    /// Optional. Whether or not the information in the document is correct.
    ///
    /// For example:
    ///
    /// * Query: "Can I return the package in 2 days once received?"
    /// * Suggested document says: "Items must be returned/exchanged within 60
    ///    days of the purchase date."
    /// * Ground truth: "No return or exchange is allowed."
    /// * \[document_correctness\]: INCORRECT
    #[prost(enumeration = "agent_assistant_feedback::DocumentCorrectness", tag = "2")]
    pub document_correctness: i32,
    /// Optional. Whether or not the suggested document is efficient. For example,
    /// if the document is poorly written, hard to understand, hard to use or
    /// too long to find useful information, \[document_efficiency][google.cloud.dialogflow.v2.AgentAssistantFeedback.document_efficiency\] is
    /// \[DocumentEfficiency.INEFFICIENT][google.cloud.dialogflow.v2.AgentAssistantFeedback.DocumentEfficiency.INEFFICIENT\].
    #[prost(enumeration = "agent_assistant_feedback::DocumentEfficiency", tag = "3")]
    pub document_efficiency: i32,
}
/// Nested message and enum types in `AgentAssistantFeedback`.
pub mod agent_assistant_feedback {
    /// Relevance of an answer.
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
    pub enum AnswerRelevance {
        /// Answer relevance unspecified.
        Unspecified = 0,
        /// Answer is irrelevant to query.
        Irrelevant = 1,
        /// Answer is relevant to query.
        Relevant = 2,
    }
    impl AnswerRelevance {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AnswerRelevance::Unspecified => "ANSWER_RELEVANCE_UNSPECIFIED",
                AnswerRelevance::Irrelevant => "IRRELEVANT",
                AnswerRelevance::Relevant => "RELEVANT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ANSWER_RELEVANCE_UNSPECIFIED" => Some(Self::Unspecified),
                "IRRELEVANT" => Some(Self::Irrelevant),
                "RELEVANT" => Some(Self::Relevant),
                _ => None,
            }
        }
    }
    /// Correctness of document.
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
    pub enum DocumentCorrectness {
        /// Document correctness unspecified.
        Unspecified = 0,
        /// Information in document is incorrect.
        Incorrect = 1,
        /// Information in document is correct.
        Correct = 2,
    }
    impl DocumentCorrectness {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DocumentCorrectness::Unspecified => "DOCUMENT_CORRECTNESS_UNSPECIFIED",
                DocumentCorrectness::Incorrect => "INCORRECT",
                DocumentCorrectness::Correct => "CORRECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DOCUMENT_CORRECTNESS_UNSPECIFIED" => Some(Self::Unspecified),
                "INCORRECT" => Some(Self::Incorrect),
                "CORRECT" => Some(Self::Correct),
                _ => None,
            }
        }
    }
    /// Efficiency of document.
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
    pub enum DocumentEfficiency {
        /// Document efficiency unspecified.
        Unspecified = 0,
        /// Document is inefficient.
        Inefficient = 1,
        /// Document is efficient.
        Efficient = 2,
    }
    impl DocumentEfficiency {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DocumentEfficiency::Unspecified => "DOCUMENT_EFFICIENCY_UNSPECIFIED",
                DocumentEfficiency::Inefficient => "INEFFICIENT",
                DocumentEfficiency::Efficient => "EFFICIENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DOCUMENT_EFFICIENCY_UNSPECIFIED" => Some(Self::Unspecified),
                "INEFFICIENT" => Some(Self::Inefficient),
                "EFFICIENT" => Some(Self::Efficient),
                _ => None,
            }
        }
    }
}
/// Represents a record of a human agent assist answer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentAssistantRecord {
    /// Output only. The agent assist answer.
    #[prost(oneof = "agent_assistant_record::Answer", tags = "5, 6")]
    pub answer: ::core::option::Option<agent_assistant_record::Answer>,
}
/// Nested message and enum types in `AgentAssistantRecord`.
pub mod agent_assistant_record {
    /// Output only. The agent assist answer.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Answer {
        /// Output only. The article suggestion answer.
        #[prost(message, tag = "5")]
        ArticleSuggestionAnswer(super::ArticleAnswer),
        /// Output only. The FAQ answer.
        #[prost(message, tag = "6")]
        FaqAnswer(super::FaqAnswer),
    }
}
/// Generated client implementations.
pub mod answer_records_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [AnswerRecords][google.cloud.dialogflow.v2.AnswerRecord].
    #[derive(Debug, Clone)]
    pub struct AnswerRecordsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AnswerRecordsClient<T>
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
        ) -> AnswerRecordsClient<InterceptedService<T, F>>
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
            AnswerRecordsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all answer records in the specified project in reverse
        /// chronological order.
        pub async fn list_answer_records(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAnswerRecordsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAnswerRecordsResponse>,
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
                "/google.cloud.dialogflow.v2.AnswerRecords/ListAnswerRecords",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.AnswerRecords",
                        "ListAnswerRecords",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified answer record.
        pub async fn update_answer_record(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAnswerRecordRequest>,
        ) -> std::result::Result<tonic::Response<super::AnswerRecord>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.AnswerRecords/UpdateAnswerRecord",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.AnswerRecords",
                        "UpdateAnswerRecord",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Represents a conversation.
/// A conversation is an interaction between an agent, including live agents
/// and Dialogflow agents, and a support customer. Conversations can
/// include phone calls and text-based chat sessions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Conversation {
    /// Output only. The unique identifier of this conversation.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The current state of the Conversation.
    #[prost(enumeration = "conversation::LifecycleState", tag = "2")]
    pub lifecycle_state: i32,
    /// Required. The Conversation Profile to be used to configure this
    /// Conversation. This field cannot be updated.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversationProfiles/<Conversation Profile ID>`.
    #[prost(string, tag = "3")]
    pub conversation_profile: ::prost::alloc::string::String,
    /// Output only. It will not be empty if the conversation is to be connected over
    /// telephony.
    #[prost(message, optional, tag = "4")]
    pub phone_number: ::core::option::Option<ConversationPhoneNumber>,
    /// Output only. The time the conversation was started.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the conversation was finished.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The stage of a conversation. It indicates whether the virtual agent or a
    /// human agent is handling the conversation.
    ///
    /// If the conversation is created with the conversation profile that has
    /// Dialogflow config set, defaults to
    /// \[ConversationStage.VIRTUAL_AGENT_STAGE][google.cloud.dialogflow.v2.Conversation.ConversationStage.VIRTUAL_AGENT_STAGE\]; Otherwise, defaults to
    /// \[ConversationStage.HUMAN_ASSIST_STAGE][google.cloud.dialogflow.v2.Conversation.ConversationStage.HUMAN_ASSIST_STAGE\].
    ///
    /// If the conversation is created with the conversation profile that has
    /// Dialogflow config set but explicitly sets conversation_stage to
    /// \[ConversationStage.HUMAN_ASSIST_STAGE][google.cloud.dialogflow.v2.Conversation.ConversationStage.HUMAN_ASSIST_STAGE\], it skips
    /// \[ConversationStage.VIRTUAL_AGENT_STAGE][google.cloud.dialogflow.v2.Conversation.ConversationStage.VIRTUAL_AGENT_STAGE\] stage and directly goes to
    /// \[ConversationStage.HUMAN_ASSIST_STAGE][google.cloud.dialogflow.v2.Conversation.ConversationStage.HUMAN_ASSIST_STAGE\].
    #[prost(enumeration = "conversation::ConversationStage", tag = "7")]
    pub conversation_stage: i32,
}
/// Nested message and enum types in `Conversation`.
pub mod conversation {
    /// Enumeration of the completion status of the conversation.
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
    pub enum LifecycleState {
        /// Unknown.
        Unspecified = 0,
        /// Conversation is currently open for media analysis.
        InProgress = 1,
        /// Conversation has been completed.
        Completed = 2,
    }
    impl LifecycleState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LifecycleState::Unspecified => "LIFECYCLE_STATE_UNSPECIFIED",
                LifecycleState::InProgress => "IN_PROGRESS",
                LifecycleState::Completed => "COMPLETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIFECYCLE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "COMPLETED" => Some(Self::Completed),
                _ => None,
            }
        }
    }
    /// Enumeration of the different conversation stages a conversation can be in.
    /// Reference:
    /// <https://cloud.google.com/dialogflow/priv/docs/contact-center/basics#stages>
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
    pub enum ConversationStage {
        /// Unknown. Should never be used after a conversation is successfully
        /// created.
        Unspecified = 0,
        /// The conversation should return virtual agent responses into the
        /// conversation.
        VirtualAgentStage = 1,
        /// The conversation should not provide responses, just listen and provide
        /// suggestions.
        HumanAssistStage = 2,
    }
    impl ConversationStage {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversationStage::Unspecified => "CONVERSATION_STAGE_UNSPECIFIED",
                ConversationStage::VirtualAgentStage => "VIRTUAL_AGENT_STAGE",
                ConversationStage::HumanAssistStage => "HUMAN_ASSIST_STAGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONVERSATION_STAGE_UNSPECIFIED" => Some(Self::Unspecified),
                "VIRTUAL_AGENT_STAGE" => Some(Self::VirtualAgentStage),
                "HUMAN_ASSIST_STAGE" => Some(Self::HumanAssistStage),
                _ => None,
            }
        }
    }
}
/// The request message for \[Conversations.CreateConversation][google.cloud.dialogflow.v2.Conversations.CreateConversation\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversationRequest {
    /// Required. Resource identifier of the project creating the conversation.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The conversation to create.
    #[prost(message, optional, tag = "2")]
    pub conversation: ::core::option::Option<Conversation>,
    /// Optional. Identifier of the conversation. Generally it's auto generated by Google.
    /// Only set it if you cannot wait for the response to return a
    /// auto-generated one to you.
    ///
    /// The conversation ID must be compliant with the regression fomula
    /// "\[a-zA-Z][a-zA-Z0-9_-\]*" with the characters length in range of \[3,64\].
    /// If the field is provided, the caller is resposible for
    /// 1. the uniqueness of the ID, otherwise the request will be rejected.
    /// 2. the consistency for whether to use custom ID or not under a project to
    /// better ensure uniqueness.
    #[prost(string, tag = "3")]
    pub conversation_id: ::prost::alloc::string::String,
}
/// The request message for \[Conversations.ListConversations][google.cloud.dialogflow.v2.Conversations.ListConversations\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationsRequest {
    /// Required. The project from which to list all conversation.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters conversations listed in the response. In
    /// general, the expression must specify the field name, a comparison operator,
    /// and the value to use for filtering:
    /// <ul>
    ///    <li>The value must be a string, a number, or a boolean.</li>
    ///    <li>The comparison operator must be either `=`,`!=`, `>`, or `<`.</li>
    ///    <li>To filter on multiple expressions, separate the
    ///        expressions with `AND` or `OR` (omitting both implies `AND`).</li>
    ///    <li>For clarity, expressions can be enclosed in parentheses.</li>
    /// </ul>
    /// Only `lifecycle_state` can be filtered on in this way. For example,
    /// the following expression only returns `COMPLETED` conversations:
    ///
    /// `lifecycle_state = "COMPLETED"`
    ///
    /// For more information about filtering, see
    /// [API Filtering](<https://aip.dev/160>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// The response message for \[Conversations.ListConversations][google.cloud.dialogflow.v2.Conversations.ListConversations\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationsResponse {
    /// The list of conversations. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub conversations: ::prost::alloc::vec::Vec<Conversation>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[Conversations.GetConversation][google.cloud.dialogflow.v2.Conversations.GetConversation\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversationRequest {
    /// Required. The name of the conversation. Format:
    /// `projects/<Project ID>/locations/<Location ID>/conversations/<Conversation
    /// ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[Conversations.CompleteConversation][google.cloud.dialogflow.v2.Conversations.CompleteConversation\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteConversationRequest {
    /// Required. Resource identifier of the conversation to close.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[Conversations.ListMessages][google.cloud.dialogflow.v2.Conversations.ListMessages\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMessagesRequest {
    /// Required. The name of the conversation to list messages for.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversations/<Conversation ID>`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Filter on message fields. Currently predicates on `create_time`
    /// and `create_time_epoch_microseconds` are supported. `create_time` only
    /// support milliseconds accuracy. E.g.,
    /// `create_time_epoch_microseconds > 1551790877964485` or
    /// `create_time > 2017-01-15T01:30:15.01Z`.
    ///
    /// For more information about filtering, see
    /// [API Filtering](<https://aip.dev/160>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Conversations.ListMessages][google.cloud.dialogflow.v2.Conversations.ListMessages\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMessagesResponse {
    /// The list of messages. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    /// `messages` is sorted by `create_time` in descending order.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
    /// Token to retrieve the next page of results, or empty if there are
    /// no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Represents a phone number for telephony integration. It allows for connecting
/// a particular conversation over telephony.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationPhoneNumber {
    /// Output only. The phone number to connect to this conversation.
    #[prost(string, tag = "3")]
    pub phone_number: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod conversations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Conversations][google.cloud.dialogflow.v2.Conversation].
    #[derive(Debug, Clone)]
    pub struct ConversationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversationsClient<T>
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
        ) -> ConversationsClient<InterceptedService<T, F>>
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
            ConversationsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new conversation. Conversations are auto-completed after 24
        /// hours.
        ///
        /// Conversation Lifecycle:
        /// There are two stages during a conversation: Automated Agent Stage and
        /// Assist Stage.
        ///
        /// For Automated Agent Stage, there will be a dialogflow agent responding to
        /// user queries.
        ///
        /// For Assist Stage, there's no dialogflow agent responding to user queries.
        /// But we will provide suggestions which are generated from conversation.
        ///
        /// If [Conversation.conversation_profile][google.cloud.dialogflow.v2.Conversation.conversation_profile] is configured for a dialogflow
        /// agent, conversation will start from `Automated Agent Stage`, otherwise, it
        /// will start from `Assist Stage`. And during `Automated Agent Stage`, once an
        /// [Intent][google.cloud.dialogflow.v2.Intent] with [Intent.live_agent_handoff][google.cloud.dialogflow.v2.Intent.live_agent_handoff] is triggered, conversation
        /// will transfer to Assist Stage.
        pub async fn create_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Conversations/CreateConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Conversations",
                        "CreateConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the list of all conversations in the specified project.
        pub async fn list_conversations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationsResponse>,
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
                "/google.cloud.dialogflow.v2.Conversations/ListConversations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Conversations",
                        "ListConversations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specific conversation.
        pub async fn get_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Conversations/GetConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Conversations",
                        "GetConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Completes the specified conversation. Finished conversations are purged
        /// from the database after 30 days.
        pub async fn complete_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Conversations/CompleteConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Conversations",
                        "CompleteConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists messages that belong to a given conversation.
        /// `messages` are ordered by `create_time` in descending order. To fetch
        /// updates without duplication, send request with filter
        /// `create_time_epoch_microseconds >
        /// [first item's create_time of previous request]` and empty page_token.
        pub async fn list_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMessagesResponse>,
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
                "/google.cloud.dialogflow.v2.Conversations/ListMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Conversations",
                        "ListMessages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Represents a notification sent to Pub/Sub subscribers for conversation
/// lifecycle events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationEvent {
    /// The unique identifier of the conversation this notification
    /// refers to.
    /// Format: `projects/<Project ID>/conversations/<Conversation ID>`.
    #[prost(string, tag = "1")]
    pub conversation: ::prost::alloc::string::String,
    /// The type of the event that this notification refers to.
    #[prost(enumeration = "conversation_event::Type", tag = "2")]
    pub r#type: i32,
    /// More detailed information about an error. Only set for type
    /// UNRECOVERABLE_ERROR_IN_PHONE_CALL.
    #[prost(message, optional, tag = "3")]
    pub error_status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Payload of conversation event.
    #[prost(oneof = "conversation_event::Payload", tags = "4")]
    pub payload: ::core::option::Option<conversation_event::Payload>,
}
/// Nested message and enum types in `ConversationEvent`.
pub mod conversation_event {
    /// Enumeration of the types of events available.
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
    pub enum Type {
        /// Type not set.
        Unspecified = 0,
        /// A new conversation has been opened. This is fired when a telephone call
        /// is answered, or a conversation is created via the API.
        ConversationStarted = 1,
        /// An existing conversation has closed. This is fired when a telephone call
        /// is terminated, or a conversation is closed via the API.
        ConversationFinished = 2,
        /// An existing conversation has received notification from Dialogflow that
        /// human intervention is required.
        HumanInterventionNeeded = 3,
        /// An existing conversation has received a new message, either from API or
        /// telephony. It is configured in
        /// \[ConversationProfile.new_message_event_notification_config][google.cloud.dialogflow.v2.ConversationProfile.new_message_event_notification_config\]
        NewMessage = 5,
        /// Unrecoverable error during a telephone call.
        ///
        /// In general non-recoverable errors only occur if something was
        /// misconfigured in the ConversationProfile corresponding to the call. After
        /// a non-recoverable error, Dialogflow may stop responding.
        ///
        /// We don't fire this event:
        ///
        /// * in an API call because we can directly return the error, or,
        /// * when we can recover from an error.
        UnrecoverableError = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::ConversationStarted => "CONVERSATION_STARTED",
                Type::ConversationFinished => "CONVERSATION_FINISHED",
                Type::HumanInterventionNeeded => "HUMAN_INTERVENTION_NEEDED",
                Type::NewMessage => "NEW_MESSAGE",
                Type::UnrecoverableError => "UNRECOVERABLE_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CONVERSATION_STARTED" => Some(Self::ConversationStarted),
                "CONVERSATION_FINISHED" => Some(Self::ConversationFinished),
                "HUMAN_INTERVENTION_NEEDED" => Some(Self::HumanInterventionNeeded),
                "NEW_MESSAGE" => Some(Self::NewMessage),
                "UNRECOVERABLE_ERROR" => Some(Self::UnrecoverableError),
                _ => None,
            }
        }
    }
    /// Payload of conversation event.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Payload of NEW_MESSAGE event.
        #[prost(message, tag = "4")]
        NewMessagePayload(super::Message),
    }
}
/// Defines the services to connect to incoming Dialogflow conversations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationProfile {
    /// The unique identifier of this conversation profile.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversationProfiles/<Conversation Profile ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Human readable name for this profile. Max length 1024 bytes.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Create time of the conversation profile.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time of the conversation profile.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Configuration for an automated agent to use with this profile.
    #[prost(message, optional, tag = "3")]
    pub automated_agent_config: ::core::option::Option<AutomatedAgentConfig>,
    /// Configuration for agent assistance to use with this profile.
    #[prost(message, optional, tag = "4")]
    pub human_agent_assistant_config: ::core::option::Option<HumanAgentAssistantConfig>,
    /// Configuration for connecting to a live agent.
    ///
    /// Currently, this feature is not general available, please contact Google
    /// to get access.
    #[prost(message, optional, tag = "5")]
    pub human_agent_handoff_config: ::core::option::Option<HumanAgentHandoffConfig>,
    /// Configuration for publishing conversation lifecycle events.
    #[prost(message, optional, tag = "6")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// Configuration for logging conversation lifecycle events.
    #[prost(message, optional, tag = "7")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
    /// Configuration for publishing new message events. Event will be sent in
    /// format of \[ConversationEvent][google.cloud.dialogflow.v2.ConversationEvent\]
    #[prost(message, optional, tag = "8")]
    pub new_message_event_notification_config: ::core::option::Option<
        NotificationConfig,
    >,
    /// Settings for speech transcription.
    #[prost(message, optional, tag = "9")]
    pub stt_config: ::core::option::Option<SpeechToTextConfig>,
    /// Language code for the conversation profile. If not specified, the language
    /// is en-US. Language at ConversationProfile should be set for all non en-US
    /// languages.
    /// This should be a \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>)
    /// language tag. Example: "en-US".
    #[prost(string, tag = "10")]
    pub language_code: ::prost::alloc::string::String,
    /// The time zone of this conversational profile from the
    /// [time zone database](<https://www.iana.org/time-zones>), e.g.,
    /// America/New_York, Europe/Paris. Defaults to America/New_York.
    #[prost(string, tag = "14")]
    pub time_zone: ::prost::alloc::string::String,
    /// Name of the CX SecuritySettings reference for the agent.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/securitySettings/<Security Settings ID>`.
    #[prost(string, tag = "13")]
    pub security_settings: ::prost::alloc::string::String,
}
/// The request message for \[ConversationProfiles.ListConversationProfiles][google.cloud.dialogflow.v2.ConversationProfiles.ListConversationProfiles\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationProfilesRequest {
    /// Required. The project to list all conversation profiles from.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[ConversationProfiles.ListConversationProfiles][google.cloud.dialogflow.v2.ConversationProfiles.ListConversationProfiles\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationProfilesResponse {
    /// The list of project conversation profiles. There is a maximum number
    /// of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub conversation_profiles: ::prost::alloc::vec::Vec<ConversationProfile>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[ConversationProfiles.GetConversationProfile][google.cloud.dialogflow.v2.ConversationProfiles.GetConversationProfile\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversationProfileRequest {
    /// Required. The resource name of the conversation profile.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversationProfiles/<Conversation Profile ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[ConversationProfiles.CreateConversationProfile][google.cloud.dialogflow.v2.ConversationProfiles.CreateConversationProfile\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversationProfileRequest {
    /// Required. The project to create a conversation profile for.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The conversation profile to create.
    #[prost(message, optional, tag = "2")]
    pub conversation_profile: ::core::option::Option<ConversationProfile>,
}
/// The request message for \[ConversationProfiles.UpdateConversationProfile][google.cloud.dialogflow.v2.ConversationProfiles.UpdateConversationProfile\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConversationProfileRequest {
    /// Required. The conversation profile to update.
    #[prost(message, optional, tag = "1")]
    pub conversation_profile: ::core::option::Option<ConversationProfile>,
    /// Required. The mask to control which fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for \[ConversationProfiles.DeleteConversationProfile][google.cloud.dialogflow.v2.ConversationProfiles.DeleteConversationProfile\].
///
/// This operation fails if the conversation profile is still referenced from
/// a phone number.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConversationProfileRequest {
    /// Required. The name of the conversation profile to delete.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/conversationProfiles/<Conversation Profile ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Defines the Automated Agent to connect to a conversation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomatedAgentConfig {
    /// Required. ID of the Dialogflow agent environment to use.
    ///
    /// This project needs to either be the same project as the conversation or you
    /// need to grant `service-<Conversation Project
    /// Number>@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow API
    /// Service Agent` role in this project.
    ///
    /// - For ES agents, use format: `projects/<Project ID>/locations/<Location
    /// ID>/agent/environments/<Environment ID or '-'>`. If environment is not
    /// specified, the default `draft` environment is used. Refer to
    /// \[DetectIntentRequest\](/dialogflow/docs/reference/rpc/google.cloud.dialogflow.v2#google.cloud.dialogflow.v2.DetectIntentRequest)
    /// for more details.
    ///
    /// - For CX agents, use format `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID
    /// or '-'>`. If environment is not specified, the default `draft` environment
    /// is used.
    #[prost(string, tag = "1")]
    pub agent: ::prost::alloc::string::String,
}
/// Defines the Human Agent Assist to connect to a conversation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanAgentAssistantConfig {
    /// Pub/Sub topic on which to publish new agent assistant events.
    #[prost(message, optional, tag = "2")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// Configuration for agent assistance of human agent participant.
    #[prost(message, optional, tag = "3")]
    pub human_agent_suggestion_config: ::core::option::Option<
        human_agent_assistant_config::SuggestionConfig,
    >,
    /// Configuration for agent assistance of end user participant.
    ///
    /// Currently, this feature is not general available, please contact Google
    /// to get access.
    #[prost(message, optional, tag = "4")]
    pub end_user_suggestion_config: ::core::option::Option<
        human_agent_assistant_config::SuggestionConfig,
    >,
    /// Configuration for message analysis.
    #[prost(message, optional, tag = "5")]
    pub message_analysis_config: ::core::option::Option<
        human_agent_assistant_config::MessageAnalysisConfig,
    >,
}
/// Nested message and enum types in `HumanAgentAssistantConfig`.
pub mod human_agent_assistant_config {
    /// Settings of suggestion trigger.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuggestionTriggerSettings {
        /// Do not trigger if last utterance is small talk.
        #[prost(bool, tag = "1")]
        pub no_smalltalk: bool,
        /// Only trigger suggestion if participant role of last utterance is
        /// END_USER.
        #[prost(bool, tag = "2")]
        pub only_end_user: bool,
    }
    /// Config for suggestion features.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuggestionFeatureConfig {
        /// The suggestion feature.
        #[prost(message, optional, tag = "5")]
        pub suggestion_feature: ::core::option::Option<super::SuggestionFeature>,
        /// Automatically iterates all participants and tries to compile
        /// suggestions.
        ///
        /// Supported features: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST.
        #[prost(bool, tag = "3")]
        pub enable_event_based_suggestion: bool,
        /// Settings of suggestion trigger.
        ///
        /// Currently, only ARTICLE_SUGGESTION and FAQ will use this field.
        #[prost(message, optional, tag = "10")]
        pub suggestion_trigger_settings: ::core::option::Option<
            SuggestionTriggerSettings,
        >,
        /// Configs of query.
        #[prost(message, optional, tag = "6")]
        pub query_config: ::core::option::Option<SuggestionQueryConfig>,
        /// Configs of custom conversation model.
        #[prost(message, optional, tag = "7")]
        pub conversation_model_config: ::core::option::Option<ConversationModelConfig>,
    }
    /// Detail human agent assistant config.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuggestionConfig {
        /// Configuration of different suggestion features. One feature can have only
        /// one config.
        #[prost(message, repeated, tag = "2")]
        pub feature_configs: ::prost::alloc::vec::Vec<SuggestionFeatureConfig>,
        /// If `group_suggestion_responses` is false, and there are multiple
        /// `feature_configs` in `event based suggestion` or
        /// StreamingAnalyzeContent, we will try to deliver suggestions to customers
        /// as soon as we get new suggestion. Different type of suggestions based on
        /// the same context will be in  separate Pub/Sub event or
        /// `StreamingAnalyzeContentResponse`.
        ///
        /// If `group_suggestion_responses` set to true. All the suggestions to the
        /// same participant based on the same context will be grouped into a single
        /// Pub/Sub event or StreamingAnalyzeContentResponse.
        #[prost(bool, tag = "3")]
        pub group_suggestion_responses: bool,
    }
    /// Config for suggestion query.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuggestionQueryConfig {
        /// Maximum number of results to return. Currently, if unset, defaults to 10.
        /// And the max number is 20.
        #[prost(int32, tag = "4")]
        pub max_results: i32,
        /// Confidence threshold of query result.
        ///
        /// Agent Assist gives each suggestion a score in the range [0.0, 1.0], based
        /// on the relevance between the suggestion and the current conversation
        /// context. A score of 0.0 has no relevance, while a score of 1.0 has high
        /// relevance. Only suggestions with a score greater than or equal to the
        /// value of this field are included in the results.
        ///
        /// For a baseline model (the default), the recommended value is in the range
        /// [0.05, 0.1].
        ///
        /// For a custom model, there is no recommended value. Tune this value by
        /// starting from a very low value and slowly increasing until you have
        /// desired results.
        ///
        /// If this field is not set, it defaults to 0.0, which means that all
        /// suggestions are returned.
        ///
        /// Supported features: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE.
        #[prost(float, tag = "5")]
        pub confidence_threshold: f32,
        /// Determines how recent conversation context is filtered when generating
        /// suggestions. If unspecified, no messages will be dropped.
        #[prost(message, optional, tag = "7")]
        pub context_filter_settings: ::core::option::Option<
            suggestion_query_config::ContextFilterSettings,
        >,
        /// Source of query.
        #[prost(oneof = "suggestion_query_config::QuerySource", tags = "1, 2, 3")]
        pub query_source: ::core::option::Option<suggestion_query_config::QuerySource>,
    }
    /// Nested message and enum types in `SuggestionQueryConfig`.
    pub mod suggestion_query_config {
        /// Knowledge base source settings.
        ///
        /// Supported features: ARTICLE_SUGGESTION, FAQ.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct KnowledgeBaseQuerySource {
            /// Required. Knowledge bases to query. Format:
            /// `projects/<Project ID>/locations/<Location
            /// ID>/knowledgeBases/<Knowledge Base ID>`. Currently, at most 5 knowledge
            /// bases are supported.
            #[prost(string, repeated, tag = "1")]
            pub knowledge_bases: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
        }
        /// Document source settings.
        ///
        /// Supported features: SMART_REPLY, SMART_COMPOSE.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DocumentQuerySource {
            /// Required. Knowledge documents to query from. Format:
            /// `projects/<Project ID>/locations/<Location
            /// ID>/knowledgeBases/<KnowledgeBase ID>/documents/<Document ID>`.
            /// Currently, at most 5 documents are supported.
            #[prost(string, repeated, tag = "1")]
            pub documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// Dialogflow source setting.
        ///
        /// Supported feature: DIALOGFLOW_ASSIST.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DialogflowQuerySource {
            /// Required. The name of a Dialogflow virtual agent used for end user side intent
            /// detection and suggestion. Format: `projects/<Project Number/
            /// ID>/locations/<Location ID>/agent`. When multiple agents are allowed in
            /// the same Dialogflow project.
            #[prost(string, tag = "1")]
            pub agent: ::prost::alloc::string::String,
        }
        /// Settings that determine how to filter recent conversation context when
        /// generating suggestions.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ContextFilterSettings {
            /// If set to true, the last message from virtual agent (hand off message)
            /// and the message before it (trigger message of hand off) are dropped.
            #[prost(bool, tag = "1")]
            pub drop_handoff_messages: bool,
            /// If set to true, all messages from virtual agent are dropped.
            #[prost(bool, tag = "2")]
            pub drop_virtual_agent_messages: bool,
            /// If set to true, all messages from ivr stage are dropped.
            #[prost(bool, tag = "3")]
            pub drop_ivr_messages: bool,
        }
        /// Source of query.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum QuerySource {
            /// Query from knowledgebase. It is used by:
            /// ARTICLE_SUGGESTION, FAQ.
            #[prost(message, tag = "1")]
            KnowledgeBaseQuerySource(KnowledgeBaseQuerySource),
            /// Query from knowledge base document. It is used by:
            /// SMART_REPLY, SMART_COMPOSE.
            #[prost(message, tag = "2")]
            DocumentQuerySource(DocumentQuerySource),
            /// Query from Dialogflow agent. It is used by DIALOGFLOW_ASSIST.
            #[prost(message, tag = "3")]
            DialogflowQuerySource(DialogflowQuerySource),
        }
    }
    /// Custom conversation models used in agent assist feature.
    ///
    /// Supported feature: ARTICLE_SUGGESTION, SMART_COMPOSE, SMART_REPLY.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConversationModelConfig {
        /// Conversation model resource name. Format: `projects/<Project
        /// ID>/conversationModels/<Model ID>`.
        #[prost(string, tag = "1")]
        pub model: ::prost::alloc::string::String,
    }
    /// Configuration for analyses to run on each conversation message.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageAnalysisConfig {
        /// Enable entity extraction in conversation messages on [agent assist
        /// stage](<https://cloud.google.com/dialogflow/priv/docs/contact-center/basics#stages>).
        /// If unspecified, defaults to false.
        ///
        /// Currently, this feature is not general available, please contact Google
        /// to get access.
        #[prost(bool, tag = "2")]
        pub enable_entity_extraction: bool,
        /// Enable sentiment analysis in conversation messages on [agent assist
        /// stage](<https://cloud.google.com/dialogflow/priv/docs/contact-center/basics#stages>).
        /// If unspecified, defaults to false. Sentiment analysis inspects user input
        /// and identifies the prevailing subjective opinion, especially to determine
        /// a user's attitude as positive, negative, or neutral:
        /// <https://cloud.google.com/natural-language/docs/basics#sentiment_analysis>
        /// For \[Participants.StreamingAnalyzeContent][google.cloud.dialogflow.v2.Participants.StreamingAnalyzeContent\] method, result will be in
        /// \[StreamingAnalyzeContentResponse.message.SentimentAnalysisResult][google.cloud.dialogflow.v2.StreamingAnalyzeContentResponse.message\].
        /// For \[Participants.AnalyzeContent][google.cloud.dialogflow.v2.Participants.AnalyzeContent\] method, result will be in
        /// \[AnalyzeContentResponse.message.SentimentAnalysisResult][google.cloud.dialogflow.v2.AnalyzeContentResponse.message\]
        /// For \[Conversations.ListMessages][google.cloud.dialogflow.v2.Conversations.ListMessages\] method, result will be in
        /// \[ListMessagesResponse.messages.SentimentAnalysisResult][google.cloud.dialogflow.v2.ListMessagesResponse.messages\]
        /// If Pub/Sub notification is configured, result will be in
        /// \[ConversationEvent.new_message_payload.SentimentAnalysisResult][google.cloud.dialogflow.v2.ConversationEvent.new_message_payload\].
        #[prost(bool, tag = "3")]
        pub enable_sentiment_analysis: bool,
    }
}
/// Defines the hand off to a live agent, typically on which external agent
/// service provider to connect to a conversation.
///
/// Currently, this feature is not general available, please contact Google
/// to get access.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanAgentHandoffConfig {
    /// Required. Specifies which agent service to connect for human agent handoff.
    #[prost(oneof = "human_agent_handoff_config::AgentService", tags = "1, 2")]
    pub agent_service: ::core::option::Option<human_agent_handoff_config::AgentService>,
}
/// Nested message and enum types in `HumanAgentHandoffConfig`.
pub mod human_agent_handoff_config {
    /// Configuration specific to LivePerson (<https://www.liveperson.com>).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LivePersonConfig {
        /// Required. Account number of the LivePerson account to connect. This is
        /// the account number you input at the login page.
        #[prost(string, tag = "1")]
        pub account_number: ::prost::alloc::string::String,
    }
    /// Configuration specific to Salesforce Live Agent.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SalesforceLiveAgentConfig {
        /// Required. The organization ID of the Salesforce account.
        #[prost(string, tag = "1")]
        pub organization_id: ::prost::alloc::string::String,
        /// Required. Live Agent deployment ID.
        #[prost(string, tag = "2")]
        pub deployment_id: ::prost::alloc::string::String,
        /// Required. Live Agent chat button ID.
        #[prost(string, tag = "3")]
        pub button_id: ::prost::alloc::string::String,
        /// Required. Domain of the Live Agent endpoint for this agent. You can find
        /// the endpoint URL in the `Live Agent settings` page. For example if URL
        /// has the form <https://d.la4-c2-phx.salesforceliveagent.com/...,>
        /// you should fill in d.la4-c2-phx.salesforceliveagent.com.
        #[prost(string, tag = "4")]
        pub endpoint_domain: ::prost::alloc::string::String,
    }
    /// Required. Specifies which agent service to connect for human agent handoff.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AgentService {
        /// Uses LivePerson (<https://www.liveperson.com>).
        #[prost(message, tag = "1")]
        LivePersonConfig(LivePersonConfig),
        /// Uses Salesforce Live Agent.
        #[prost(message, tag = "2")]
        SalesforceLiveAgentConfig(SalesforceLiveAgentConfig),
    }
}
/// Defines notification behavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationConfig {
    /// Name of the Pub/Sub topic to publish conversation
    /// events like
    /// \[CONVERSATION_STARTED][google.cloud.dialogflow.v2.ConversationEvent.Type.CONVERSATION_STARTED\] as
    /// serialized \[ConversationEvent][google.cloud.dialogflow.v2.ConversationEvent\] protos.
    ///
    /// Notification works for phone calls, if this topic either is in the same
    /// project as the conversation or you grant `service-<Conversation Project
    /// Number>@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow Service
    /// Agent` role in the topic project.
    ///
    /// Format: `projects/<Project ID>/locations/<Location ID>/topics/<Topic ID>`.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// Format of message.
    #[prost(enumeration = "notification_config::MessageFormat", tag = "2")]
    pub message_format: i32,
}
/// Nested message and enum types in `NotificationConfig`.
pub mod notification_config {
    /// Format of cloud pub/sub message.
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
    pub enum MessageFormat {
        /// If it is unspecified, PROTO will be used.
        Unspecified = 0,
        /// Pub/Sub message will be serialized proto.
        Proto = 1,
        /// Pub/Sub message will be json.
        Json = 2,
    }
    impl MessageFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MessageFormat::Unspecified => "MESSAGE_FORMAT_UNSPECIFIED",
                MessageFormat::Proto => "PROTO",
                MessageFormat::Json => "JSON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MESSAGE_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "PROTO" => Some(Self::Proto),
                "JSON" => Some(Self::Json),
                _ => None,
            }
        }
    }
}
/// Defines logging behavior for conversation lifecycle events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingConfig {
    /// Whether to log conversation events like
    /// \[CONVERSATION_STARTED][google.cloud.dialogflow.v2.ConversationEvent.Type.CONVERSATION_STARTED\] to
    /// Stackdriver in the conversation project as JSON format
    /// \[ConversationEvent][google.cloud.dialogflow.v2.ConversationEvent\] protos.
    #[prost(bool, tag = "3")]
    pub enable_stackdriver_logging: bool,
}
/// The type of Human Agent Assistant API suggestion to perform, and the maximum
/// number of results to return for that type. Multiple `Feature` objects can
/// be specified in the `features` list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestionFeature {
    /// Type of Human Agent Assistant API feature to request.
    #[prost(enumeration = "suggestion_feature::Type", tag = "1")]
    pub r#type: i32,
}
/// Nested message and enum types in `SuggestionFeature`.
pub mod suggestion_feature {
    /// Defines the type of Human Agent Assistant feature.
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
    pub enum Type {
        /// Unspecified feature type.
        Unspecified = 0,
        /// Run article suggestion model.
        ArticleSuggestion = 1,
        /// Run FAQ model.
        Faq = 2,
        /// Run smart reply model.
        SmartReply = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::ArticleSuggestion => "ARTICLE_SUGGESTION",
                Type::Faq => "FAQ",
                Type::SmartReply => "SMART_REPLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ARTICLE_SUGGESTION" => Some(Self::ArticleSuggestion),
                "FAQ" => Some(Self::Faq),
                "SMART_REPLY" => Some(Self::SmartReply),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod conversation_profiles_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [ConversationProfiles][google.cloud.dialogflow.v2.ConversationProfile].
    #[derive(Debug, Clone)]
    pub struct ConversationProfilesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversationProfilesClient<T>
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
        ) -> ConversationProfilesClient<InterceptedService<T, F>>
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
            ConversationProfilesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all conversation profiles in the specified project.
        pub async fn list_conversation_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversationProfilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationProfilesResponse>,
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
                "/google.cloud.dialogflow.v2.ConversationProfiles/ListConversationProfiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.ConversationProfiles",
                        "ListConversationProfiles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified conversation profile.
        pub async fn get_conversation_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversationProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConversationProfile>,
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
                "/google.cloud.dialogflow.v2.ConversationProfiles/GetConversationProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.ConversationProfiles",
                        "GetConversationProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a conversation profile in the specified project.
        ///
        /// [ConversationProfile.CreateTime][] and [ConversationProfile.UpdateTime][]
        /// aren't populated in the response. You can retrieve them via
        /// [GetConversationProfile][google.cloud.dialogflow.v2.ConversationProfiles.GetConversationProfile] API.
        pub async fn create_conversation_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversationProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConversationProfile>,
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
                "/google.cloud.dialogflow.v2.ConversationProfiles/CreateConversationProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.ConversationProfiles",
                        "CreateConversationProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified conversation profile.
        ///
        /// [ConversationProfile.CreateTime][] and [ConversationProfile.UpdateTime][]
        /// aren't populated in the response. You can retrieve them via
        /// [GetConversationProfile][google.cloud.dialogflow.v2.ConversationProfiles.GetConversationProfile] API.
        pub async fn update_conversation_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConversationProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConversationProfile>,
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
                "/google.cloud.dialogflow.v2.ConversationProfiles/UpdateConversationProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.ConversationProfiles",
                        "UpdateConversationProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified conversation profile.
        pub async fn delete_conversation_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConversationProfileRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.ConversationProfiles/DeleteConversationProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.ConversationProfiles",
                        "DeleteConversationProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A knowledge document to be used by a \[KnowledgeBase][google.cloud.dialogflow.v2.KnowledgeBase\].
///
/// For more information, see the [knowledge base
/// guide](<https://cloud.google.com/dialogflow/docs/how/knowledge-bases>).
///
/// Note: The `projects.agent.knowledgeBases.documents` resource is deprecated;
/// only use `projects.knowledgeBases.documents`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// Optional. The document resource name.
    /// The name must be empty when creating a document.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>/documents/<Document ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the document. The name must be 1024 bytes or
    /// less; otherwise, the creation request fails.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The MIME type of this document.
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
    /// Required. The knowledge type of document content.
    #[prost(
        enumeration = "document::KnowledgeType",
        repeated,
        packed = "false",
        tag = "4"
    )]
    pub knowledge_types: ::prost::alloc::vec::Vec<i32>,
    /// Optional. If true, we try to automatically reload the document every day
    /// (at a time picked by the system). If false or unspecified, we don't try
    /// to automatically reload the document.
    ///
    /// Currently you can only enable automatic reload for documents sourced from
    /// a public url, see `source` field for the source types.
    ///
    /// Reload status can be tracked in `latest_reload_status`. If a reload
    /// fails, we will keep the document unchanged.
    ///
    /// If a reload fails with internal errors, the system will try to reload the
    /// document on the next day.
    /// If a reload fails with non-retriable errors (e.g. PERMISION_DENIED), the
    /// system will not try to reload the document anymore. You need to manually
    /// reload the document successfully by calling `ReloadDocument` and clear the
    /// errors.
    #[prost(bool, tag = "11")]
    pub enable_auto_reload: bool,
    /// Output only. The time and status of the latest reload.
    /// This reload may have been triggered automatically or manually
    /// and may not have succeeded.
    #[prost(message, optional, tag = "12")]
    pub latest_reload_status: ::core::option::Option<document::ReloadStatus>,
    /// Optional. Metadata for the document. The metadata supports arbitrary
    /// key-value pairs. Suggested use cases include storing a document's title,
    /// an external URL distinct from the document's content_uri, etc.
    /// The max size of a `key` or a `value` of the metadata is 1024 bytes.
    #[prost(map = "string, string", tag = "7")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The source of this document.
    #[prost(oneof = "document::Source", tags = "5, 9")]
    pub source: ::core::option::Option<document::Source>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// The status of a reload attempt.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReloadStatus {
        /// The time of a reload attempt.
        /// This reload may have been triggered automatically or manually and may
        /// not have succeeded.
        #[prost(message, optional, tag = "1")]
        pub time: ::core::option::Option<::prost_types::Timestamp>,
        /// The status of a reload attempt or the initial load.
        #[prost(message, optional, tag = "2")]
        pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
    }
    /// The knowledge type of document content.
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
    pub enum KnowledgeType {
        /// The type is unspecified or arbitrary.
        Unspecified = 0,
        /// The document content contains question and answer pairs as either HTML or
        /// CSV. Typical FAQ HTML formats are parsed accurately, but unusual formats
        /// may fail to be parsed.
        ///
        /// CSV must have questions in the first column and answers in the second,
        /// with no header. Because of this explicit format, they are always parsed
        /// accurately.
        Faq = 1,
        /// Documents for which unstructured text is extracted and used for
        /// question answering.
        ExtractiveQa = 2,
        /// The entire document content as a whole can be used for query results.
        /// Only for Contact Center Solutions on Dialogflow.
        ArticleSuggestion = 3,
    }
    impl KnowledgeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KnowledgeType::Unspecified => "KNOWLEDGE_TYPE_UNSPECIFIED",
                KnowledgeType::Faq => "FAQ",
                KnowledgeType::ExtractiveQa => "EXTRACTIVE_QA",
                KnowledgeType::ArticleSuggestion => "ARTICLE_SUGGESTION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KNOWLEDGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "FAQ" => Some(Self::Faq),
                "EXTRACTIVE_QA" => Some(Self::ExtractiveQa),
                "ARTICLE_SUGGESTION" => Some(Self::ArticleSuggestion),
                _ => None,
            }
        }
    }
    /// Required. The source of this document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The URI where the file content is located.
        ///
        /// For documents stored in Google Cloud Storage, these URIs must have
        /// the form `gs://<bucket-name>/<object-name>`.
        ///
        /// NOTE: External URLs must correspond to public webpages, i.e., they must
        /// be indexed by Google Search. In particular, URLs for showing documents in
        /// Google Cloud Storage (i.e. the URL in your browser) are not supported.
        /// Instead use the `gs://` format URI described above.
        #[prost(string, tag = "5")]
        ContentUri(::prost::alloc::string::String),
        /// The raw content of the document. This field is only permitted for
        /// EXTRACTIVE_QA and FAQ knowledge types.
        #[prost(bytes, tag = "9")]
        RawContent(::prost::alloc::vec::Vec<u8>),
    }
}
/// Request message for \[Documents.GetDocument][google.cloud.dialogflow.v2.Documents.GetDocument\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentRequest {
    /// Required. The name of the document to retrieve.
    /// Format `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>/documents/<Document ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[Documents.ListDocuments][google.cloud.dialogflow.v2.Documents.ListDocuments\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsRequest {
    /// Required. The knowledge base to list all documents for.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By
    /// default 10 and at most 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[Documents.ListDocuments][google.cloud.dialogflow.v2.Documents.ListDocuments\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsResponse {
    /// The list of documents.
    #[prost(message, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<Document>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[Documents.CreateDocument][google.cloud.dialogflow.v2.Documents.CreateDocument\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentRequest {
    /// Required. The knowledge base to create a document for.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The document to create.
    #[prost(message, optional, tag = "2")]
    pub document: ::core::option::Option<Document>,
}
/// Request message for \[Documents.DeleteDocument][google.cloud.dialogflow.v2.Documents.DeleteDocument\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentRequest {
    /// Required. The name of the document to delete.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>/documents/<Document ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[Documents.UpdateDocument][google.cloud.dialogflow.v2.Documents.UpdateDocument\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentRequest {
    /// Required. The document to update.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// Optional. Not specified means `update all`.
    /// Currently, only `display_name` can be updated, an InvalidArgument will be
    /// returned for attempting to update other fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for \[Documents.ReloadDocument][google.cloud.dialogflow.v2.Documents.ReloadDocument\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadDocumentRequest {
    /// Required. The name of the document to reload.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>/documents/<Document ID>`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The source for document reloading.
    /// If provided, the service will load the contents from the source
    /// and update document in the knowledge base.
    #[prost(oneof = "reload_document_request::Source", tags = "3")]
    pub source: ::core::option::Option<reload_document_request::Source>,
}
/// Nested message and enum types in `ReloadDocumentRequest`.
pub mod reload_document_request {
    /// The source for document reloading.
    /// If provided, the service will load the contents from the source
    /// and update document in the knowledge base.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Optional. The path of gcs source file for reloading document content. For now,
        /// only gcs uri is supported.
        ///
        /// For documents stored in Google Cloud Storage, these URIs must have
        /// the form `gs://<bucket-name>/<object-name>`.
        #[prost(string, tag = "3")]
        ContentUri(::prost::alloc::string::String),
    }
}
/// Metadata in google::longrunning::Operation for Knowledge operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KnowledgeOperationMetadata {
    /// Output only. The current state of this operation.
    #[prost(enumeration = "knowledge_operation_metadata::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `KnowledgeOperationMetadata`.
pub mod knowledge_operation_metadata {
    /// States of the operation.
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
        /// State unspecified.
        Unspecified = 0,
        /// The operation has been created.
        Pending = 1,
        /// The operation is currently running.
        Running = 2,
        /// The operation is done, either cancelled or completed.
        Done = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "PENDING",
                State::Running => "RUNNING",
                State::Done => "DONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "RUNNING" => Some(Self::Running),
                "DONE" => Some(Self::Done),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod documents_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing knowledge [Documents][google.cloud.dialogflow.v2.Document].
    #[derive(Debug, Clone)]
    pub struct DocumentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DocumentsClient<T>
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
        ) -> DocumentsClient<InterceptedService<T, F>>
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
            DocumentsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all documents of the knowledge base.
        pub async fn list_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDocumentsResponse>,
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
                "/google.cloud.dialogflow.v2.Documents/ListDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Documents",
                        "ListDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified document.
        pub async fn get_document(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Documents/GetDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Documents",
                        "GetDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new document.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: [KnowledgeOperationMetadata][google.cloud.dialogflow.v2.KnowledgeOperationMetadata]
        /// - `response`: [Document][google.cloud.dialogflow.v2.Document]
        pub async fn create_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Documents/CreateDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Documents",
                        "CreateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified document.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: [KnowledgeOperationMetadata][google.cloud.dialogflow.v2.KnowledgeOperationMetadata]
        /// - `response`: An [Empty
        ///   message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)
        pub async fn delete_document(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Documents/DeleteDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Documents",
                        "DeleteDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified document.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: [KnowledgeOperationMetadata][google.cloud.dialogflow.v2.KnowledgeOperationMetadata]
        /// - `response`: [Document][google.cloud.dialogflow.v2.Document]
        pub async fn update_document(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Documents/UpdateDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Documents",
                        "UpdateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Reloads the specified document from its specified source, content_uri or
        /// content. The previously loaded content of the document will be deleted.
        /// Note: Even when the content of the document has not changed, there still
        /// may be side effects because of internal implementation changes.
        ///
        /// This method is a [long-running
        /// operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation).
        /// The returned `Operation` type has the following method-specific fields:
        ///
        /// - `metadata`: [KnowledgeOperationMetadata][google.cloud.dialogflow.v2.KnowledgeOperationMetadata]
        /// - `response`: [Document][google.cloud.dialogflow.v2.Document]
        ///
        /// Note: The `projects.agent.knowledgeBases.documents` resource is deprecated;
        /// only use `projects.knowledgeBases.documents`.
        pub async fn reload_document(
            &mut self,
            request: impl tonic::IntoRequest<super::ReloadDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.dialogflow.v2.Documents/ReloadDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Documents",
                        "ReloadDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// By default, your agent responds to a matched intent with a static response.
/// As an alternative, you can provide a more dynamic response by using
/// fulfillment. When you enable fulfillment for an intent, Dialogflow responds
/// to that intent by calling a service that you define. For example, if an
/// end-user wants to schedule a haircut on Friday, your service can check your
/// database and respond to the end-user with availability information for
/// Friday.
///
/// For more information, see the [fulfillment
/// guide](<https://cloud.google.com/dialogflow/docs/fulfillment-overview>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fulfillment {
    /// Required. The unique identifier of the fulfillment.
    /// Supported formats:
    /// - `projects/<Project ID>/agent/fulfillment`
    /// - `projects/<Project ID>/locations/<Location ID>/agent/fulfillment`
    ///
    /// This field is not used for Fulfillment in an Environment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The human-readable name of the fulfillment, unique within the agent.
    ///
    /// This field is not used for Fulfillment in an Environment.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Whether fulfillment is enabled.
    #[prost(bool, tag = "4")]
    pub enabled: bool,
    /// Optional. The field defines whether the fulfillment is enabled for certain features.
    #[prost(message, repeated, tag = "5")]
    pub features: ::prost::alloc::vec::Vec<fulfillment::Feature>,
    /// Required. The fulfillment configuration.
    #[prost(oneof = "fulfillment::Fulfillment", tags = "3")]
    pub fulfillment: ::core::option::Option<fulfillment::Fulfillment>,
}
/// Nested message and enum types in `Fulfillment`.
pub mod fulfillment {
    /// Represents configuration for a generic web service.
    /// Dialogflow supports two mechanisms for authentications:
    /// - Basic authentication with username and password.
    /// - Authentication with additional authentication headers.
    /// More information could be found at:
    /// <https://cloud.google.com/dialogflow/docs/fulfillment-configure.>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GenericWebService {
        /// Required. The fulfillment URI for receiving POST requests.
        /// It must use https protocol.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
        /// Optional. The user name for HTTP Basic authentication.
        #[prost(string, tag = "2")]
        pub username: ::prost::alloc::string::String,
        /// Optional. The password for HTTP Basic authentication.
        #[prost(string, tag = "3")]
        pub password: ::prost::alloc::string::String,
        /// Optional. The HTTP request headers to send together with fulfillment requests.
        #[prost(map = "string, string", tag = "4")]
        pub request_headers: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Optional. Indicates if generic web service is created through Cloud Functions
        /// integration. Defaults to false.
        ///
        /// is_cloud_function is deprecated. Cloud functions can be configured by
        /// its uri as a regular web service now.
        #[deprecated]
        #[prost(bool, tag = "5")]
        pub is_cloud_function: bool,
    }
    /// Whether fulfillment is enabled for the specific feature.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Feature {
        /// The type of the feature that enabled for fulfillment.
        #[prost(enumeration = "feature::Type", tag = "1")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `Feature`.
    pub mod feature {
        /// The type of the feature.
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
        pub enum Type {
            /// Feature type not specified.
            Unspecified = 0,
            /// Fulfillment is enabled for SmallTalk.
            Smalltalk = 1,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::Smalltalk => "SMALLTALK",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "SMALLTALK" => Some(Self::Smalltalk),
                    _ => None,
                }
            }
        }
    }
    /// Required. The fulfillment configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Fulfillment {
        /// Configuration for a generic web service.
        #[prost(message, tag = "3")]
        GenericWebService(GenericWebService),
    }
}
/// The request message for \[Fulfillments.GetFulfillment][google.cloud.dialogflow.v2.Fulfillments.GetFulfillment\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFulfillmentRequest {
    /// Required. The name of the fulfillment.
    /// Format: `projects/<Project ID>/agent/fulfillment`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[Fulfillments.UpdateFulfillment][google.cloud.dialogflow.v2.Fulfillments.UpdateFulfillment\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFulfillmentRequest {
    /// Required. The fulfillment to update.
    #[prost(message, optional, tag = "1")]
    pub fulfillment: ::core::option::Option<Fulfillment>,
    /// Required. The mask to control which fields get updated. If the mask is not
    /// present, all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Generated client implementations.
pub mod fulfillments_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Fulfillments][google.cloud.dialogflow.v2.Fulfillment].
    #[derive(Debug, Clone)]
    pub struct FulfillmentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FulfillmentsClient<T>
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
        ) -> FulfillmentsClient<InterceptedService<T, F>>
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
            FulfillmentsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves the fulfillment.
        pub async fn get_fulfillment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFulfillmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Fulfillment>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Fulfillments/GetFulfillment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Fulfillments",
                        "GetFulfillment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the fulfillment.
        pub async fn update_fulfillment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFulfillmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Fulfillment>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Fulfillments/UpdateFulfillment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Fulfillments",
                        "UpdateFulfillment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// You can create multiple versions of your agent and publish them to separate
/// environments.
///
/// When you edit an agent, you are editing the draft agent. At any point, you
/// can save the draft agent as an agent version, which is an immutable snapshot
/// of your agent.
///
/// When you save the draft agent, it is published to the default environment.
/// When you create agent versions, you can publish them to custom environments.
/// You can create a variety of custom environments for:
///
/// - testing
/// - development
/// - production
/// - etc.
///
/// For more information, see the [versions and environments
/// guide](<https://cloud.google.com/dialogflow/docs/agents-versions>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Output only. The unique identifier of this agent environment.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/environments/<Environment ID>`
    /// - `projects/<Project ID>/locations/<Location
    ///    ID>/agent/environments/<Environment ID>`
    ///
    /// The environment ID for the default environment is `-`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The developer-provided description for this environment.
    /// The maximum length is 500 characters. If exceeded, the request is rejected.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Optional. The agent version loaded into this environment.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/versions/<Version ID>`
    /// - `projects/<Project ID>/locations/<Location ID>/agent/versions/<Version
    ///    ID>`
    #[prost(string, tag = "3")]
    pub agent_version: ::prost::alloc::string::String,
    /// Output only. The state of this environment. This field is read-only, i.e., it cannot be
    /// set by create and update methods.
    #[prost(enumeration = "environment::State", tag = "4")]
    pub state: i32,
    /// Output only. The last update time of this environment. This field is read-only, i.e., it
    /// cannot be set by create and update methods.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Text to speech settings for this environment.
    #[prost(message, optional, tag = "7")]
    pub text_to_speech_settings: ::core::option::Option<TextToSpeechSettings>,
    /// Optional. The fulfillment settings to use for this environment.
    #[prost(message, optional, tag = "8")]
    pub fulfillment: ::core::option::Option<Fulfillment>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Represents an environment state. When an environment is pointed to a new
    /// agent version, the environment is temporarily set to the `LOADING` state.
    /// During that time, the environment keeps on serving the previous version of
    /// the agent. After the new agent version is done loading, the environment is
    /// set back to the `RUNNING` state.
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
        /// Not specified. This value is not used.
        Unspecified = 0,
        /// Stopped.
        Stopped = 1,
        /// Loading.
        Loading = 2,
        /// Running.
        Running = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Stopped => "STOPPED",
                State::Loading => "LOADING",
                State::Running => "RUNNING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STOPPED" => Some(Self::Stopped),
                "LOADING" => Some(Self::Loading),
                "RUNNING" => Some(Self::Running),
                _ => None,
            }
        }
    }
}
/// Instructs the speech synthesizer on how to generate the output audio content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextToSpeechSettings {
    /// Optional. Indicates whether text to speech is enabled. Even when this field is false,
    /// other settings in this proto are still retained.
    #[prost(bool, tag = "1")]
    pub enable_text_to_speech: bool,
    /// Required. Audio encoding of the synthesized audio content.
    #[prost(enumeration = "OutputAudioEncoding", tag = "2")]
    pub output_audio_encoding: i32,
    /// Optional. The synthesis sample rate (in hertz) for this audio. If not provided, then
    /// the synthesizer will use the default sample rate based on the audio
    /// encoding. If this is different from the voice's natural sample rate, then
    /// the synthesizer will honor this request by converting to the desired sample
    /// rate (which might result in worse audio quality).
    #[prost(int32, tag = "3")]
    pub sample_rate_hertz: i32,
    /// Optional. Configuration of how speech should be synthesized, mapping from language
    /// (<https://cloud.google.com/dialogflow/docs/reference/language>) to
    /// SynthesizeSpeechConfig.
    #[prost(map = "string, message", tag = "4")]
    pub synthesize_speech_configs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        SynthesizeSpeechConfig,
    >,
}
/// The request message for \[Environments.ListEnvironments][google.cloud.dialogflow.v2.Environments.ListEnvironments\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsRequest {
    /// Required. The agent to list all environments from.
    /// Format:
    ///
    /// - `projects/<Project ID>/agent`
    /// - `projects/<Project ID>/locations/<Location ID>/agent`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Environments.ListEnvironments][google.cloud.dialogflow.v2.Environments.ListEnvironments\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsResponse {
    /// The list of agent environments. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub environments: ::prost::alloc::vec::Vec<Environment>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[Environments.GetEnvironment][google.cloud.dialogflow.v2.Environments.GetEnvironment\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    /// Required. The name of the environment.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/environments/<Environment ID>`
    /// - `projects/<Project ID>/locations/<Location
    ///    ID>/agent/environments/<Environment ID>`
    ///
    /// The environment ID for the default environment is `-`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[Environments.CreateEnvironment][google.cloud.dialogflow.v2.Environments.CreateEnvironment\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentRequest {
    /// Required. The agent to create an environment for.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent`
    /// - `projects/<Project ID>/locations/<Location ID>/agent`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The environment to create.
    #[prost(message, optional, tag = "2")]
    pub environment: ::core::option::Option<Environment>,
    /// Required. The unique id of the new environment.
    #[prost(string, tag = "3")]
    pub environment_id: ::prost::alloc::string::String,
}
/// The request message for \[Environments.UpdateEnvironment][google.cloud.dialogflow.v2.Environments.UpdateEnvironment\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnvironmentRequest {
    /// Required. The environment to update.
    #[prost(message, optional, tag = "1")]
    pub environment: ::core::option::Option<Environment>,
    /// Required. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. This field is used to prevent accidental overwrite of the default
    /// environment, which is an operation that cannot be undone. To confirm that
    /// the caller desires this overwrite, this field must be explicitly set to
    /// true when updating the default environment (environment ID = `-`).
    #[prost(bool, tag = "3")]
    pub allow_load_to_draft_and_discard_changes: bool,
}
/// The request message for \[Environments.DeleteEnvironment][google.cloud.dialogflow.v2.Environments.DeleteEnvironment\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentRequest {
    /// Required. The name of the environment to delete.
    /// / Format:
    ///
    /// - `projects/<Project ID>/agent/environments/<Environment ID>`
    /// - `projects/<Project ID>/locations/<Location
    ///    ID>/agent/environments/<Environment ID>`
    ///
    /// The environment ID for the default environment is `-`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[Environments.GetEnvironmentHistory][google.cloud.dialogflow.v2.Environments.GetEnvironmentHistory\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentHistoryRequest {
    /// Required. The name of the environment to retrieve history for.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/environments/<Environment ID>`
    /// - `projects/<Project ID>/locations/<Location
    ///    ID>/agent/environments/<Environment ID>`
    ///
    /// The environment ID for the default environment is `-`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Environments.GetEnvironmentHistory][google.cloud.dialogflow.v2.Environments.GetEnvironmentHistory\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentHistory {
    /// Output only. The name of the environment this history is for.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/environments/<Environment ID>`
    /// - `projects/<Project ID>/locations/<Location
    ///     ID>/agent/environments/<Environment ID>`
    ///
    /// The environment ID for the default environment is `-`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Output only. The list of agent environments. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<environment_history::Entry>,
    /// Output only. Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EnvironmentHistory`.
pub mod environment_history {
    /// Represents an environment history entry.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        /// The agent version loaded into this environment history entry.
        #[prost(string, tag = "1")]
        pub agent_version: ::prost::alloc::string::String,
        /// The developer-provided description for this environment history entry.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// The creation time of this environment history entry.
        #[prost(message, optional, tag = "3")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Generated client implementations.
pub mod environments_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Environments][google.cloud.dialogflow.v2.Environment].
    #[derive(Debug, Clone)]
    pub struct EnvironmentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EnvironmentsClient<T>
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
        ) -> EnvironmentsClient<InterceptedService<T, F>>
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
            EnvironmentsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all non-default environments of the specified agent.
        pub async fn list_environments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEnvironmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEnvironmentsResponse>,
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
                "/google.cloud.dialogflow.v2.Environments/ListEnvironments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Environments",
                        "ListEnvironments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified agent environment.
        pub async fn get_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Environment>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Environments/GetEnvironment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Environments",
                        "GetEnvironment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an agent environment.
        pub async fn create_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Environment>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Environments/CreateEnvironment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Environments",
                        "CreateEnvironment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified agent environment.
        ///
        /// This method allows you to deploy new agent versions into the environment.
        /// When an environment is pointed to a new agent version by setting
        /// `environment.agent_version`, the environment is temporarily set to the
        /// `LOADING` state. During that time, the environment continues serving the
        /// previous version of the agent. After the new agent version is done loading,
        /// the environment is set back to the `RUNNING` state.
        /// You can use "-" as Environment ID in environment name to update an agent
        /// version in the default environment. WARNING: this will negate all recent
        /// changes to the draft agent and can't be undone. You may want to save the
        /// draft agent to a version before calling this method.
        pub async fn update_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Environment>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Environments/UpdateEnvironment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Environments",
                        "UpdateEnvironment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified agent environment.
        pub async fn delete_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEnvironmentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Environments/DeleteEnvironment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Environments",
                        "DeleteEnvironment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the history of the specified environment.
        pub async fn get_environment_history(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnvironmentHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EnvironmentHistory>,
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
                "/google.cloud.dialogflow.v2.Environments/GetEnvironmentHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Environments",
                        "GetEnvironmentHistory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Represents a notification sent to Cloud Pub/Sub subscribers for
/// human agent assistant events in a specific conversation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanAgentAssistantEvent {
    /// The conversation this notification refers to.
    /// Format: `projects/<Project ID>/conversations/<Conversation ID>`.
    #[prost(string, tag = "1")]
    pub conversation: ::prost::alloc::string::String,
    /// The participant that the suggestion is compiled for.
    /// Format: `projects/<Project ID>/conversations/<Conversation
    /// ID>/participants/<Participant ID>`. It will not be set in legacy workflow.
    #[prost(string, tag = "3")]
    pub participant: ::prost::alloc::string::String,
    /// The suggestion results payload that this notification refers to.
    #[prost(message, repeated, tag = "5")]
    pub suggestion_results: ::prost::alloc::vec::Vec<SuggestionResult>,
}
/// A knowledge base represents a collection of knowledge documents that you
/// provide to Dialogflow. Your knowledge documents contain information that may
/// be useful during conversations with end-users. Some Dialogflow features use
/// knowledge bases when looking for a response to an end-user input.
///
/// For more information, see the [knowledge base
/// guide](<https://cloud.google.com/dialogflow/docs/how/knowledge-bases>).
///
/// Note: The `projects.agent.knowledgeBases` resource is deprecated;
/// only use `projects.knowledgeBases`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KnowledgeBase {
    /// The knowledge base resource name.
    /// The name must be empty when creating a knowledge base.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the knowledge base. The name must be 1024
    /// bytes or less; otherwise, the creation request fails.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Language which represents the KnowledgeBase. When the KnowledgeBase is
    /// created/updated, expect this to be present for non en-us languages. When
    /// unspecified, the default language code en-us applies.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
}
/// Request message for \[KnowledgeBases.ListKnowledgeBases][google.cloud.dialogflow.v2.KnowledgeBases.ListKnowledgeBases\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKnowledgeBasesRequest {
    /// Required. The project to list of knowledge bases for.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return in a single page. By
    /// default 10 and at most 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[KnowledgeBases.ListKnowledgeBases][google.cloud.dialogflow.v2.KnowledgeBases.ListKnowledgeBases\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKnowledgeBasesResponse {
    /// The list of knowledge bases.
    #[prost(message, repeated, tag = "1")]
    pub knowledge_bases: ::prost::alloc::vec::Vec<KnowledgeBase>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[KnowledgeBases.GetKnowledgeBase][google.cloud.dialogflow.v2.KnowledgeBases.GetKnowledgeBase\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKnowledgeBaseRequest {
    /// Required. The name of the knowledge base to retrieve.
    /// Format `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[KnowledgeBases.CreateKnowledgeBase][google.cloud.dialogflow.v2.KnowledgeBases.CreateKnowledgeBase\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKnowledgeBaseRequest {
    /// Required. The project to create a knowledge base for.
    /// Format: `projects/<Project ID>/locations/<Location ID>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The knowledge base to create.
    #[prost(message, optional, tag = "2")]
    pub knowledge_base: ::core::option::Option<KnowledgeBase>,
}
/// Request message for \[KnowledgeBases.DeleteKnowledgeBase][google.cloud.dialogflow.v2.KnowledgeBases.DeleteKnowledgeBase\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKnowledgeBaseRequest {
    /// Required. The name of the knowledge base to delete.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/knowledgeBases/<Knowledge Base ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Force deletes the knowledge base. When set to true, any documents
    /// in the knowledge base are also deleted.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for \[KnowledgeBases.UpdateKnowledgeBase][google.cloud.dialogflow.v2.KnowledgeBases.UpdateKnowledgeBase\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateKnowledgeBaseRequest {
    /// Required. The knowledge base to update.
    #[prost(message, optional, tag = "1")]
    pub knowledge_base: ::core::option::Option<KnowledgeBase>,
    /// Optional. Not specified means `update all`.
    /// Currently, only `display_name` can be updated, an InvalidArgument will be
    /// returned for attempting to update other fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Generated client implementations.
pub mod knowledge_bases_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [KnowledgeBases][google.cloud.dialogflow.v2.KnowledgeBase].
    #[derive(Debug, Clone)]
    pub struct KnowledgeBasesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KnowledgeBasesClient<T>
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
        ) -> KnowledgeBasesClient<InterceptedService<T, F>>
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
            KnowledgeBasesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all knowledge bases of the specified agent.
        pub async fn list_knowledge_bases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKnowledgeBasesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListKnowledgeBasesResponse>,
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
                "/google.cloud.dialogflow.v2.KnowledgeBases/ListKnowledgeBases",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.KnowledgeBases",
                        "ListKnowledgeBases",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified knowledge base.
        pub async fn get_knowledge_base(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKnowledgeBaseRequest>,
        ) -> std::result::Result<tonic::Response<super::KnowledgeBase>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.KnowledgeBases/GetKnowledgeBase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.KnowledgeBases",
                        "GetKnowledgeBase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a knowledge base.
        pub async fn create_knowledge_base(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKnowledgeBaseRequest>,
        ) -> std::result::Result<tonic::Response<super::KnowledgeBase>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.KnowledgeBases/CreateKnowledgeBase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.KnowledgeBases",
                        "CreateKnowledgeBase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified knowledge base.
        pub async fn delete_knowledge_base(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteKnowledgeBaseRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.KnowledgeBases/DeleteKnowledgeBase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.KnowledgeBases",
                        "DeleteKnowledgeBase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified knowledge base.
        pub async fn update_knowledge_base(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateKnowledgeBaseRequest>,
        ) -> std::result::Result<tonic::Response<super::KnowledgeBase>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.KnowledgeBases/UpdateKnowledgeBase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.KnowledgeBases",
                        "UpdateKnowledgeBase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// You can create multiple versions of your agent and publish them to separate
/// environments.
///
/// When you edit an agent, you are editing the draft agent. At any point, you
/// can save the draft agent as an agent version, which is an immutable snapshot
/// of your agent.
///
/// When you save the draft agent, it is published to the default environment.
/// When you create agent versions, you can publish them to custom environments.
/// You can create a variety of custom environments for:
///
/// - testing
/// - development
/// - production
/// - etc.
///
/// For more information, see the [versions and environments
/// guide](<https://cloud.google.com/dialogflow/docs/agents-versions>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Output only. The unique identifier of this agent version.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/versions/<Version ID>`
    /// - `projects/<Project ID>/locations/<Location ID>/agent/versions/<Version
    ///    ID>`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The developer-provided description of this version.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The sequential number of this version. This field is read-only which means
    /// it cannot be set by create and update methods.
    #[prost(int32, tag = "3")]
    pub version_number: i32,
    /// Output only. The creation time of this version. This field is read-only, i.e., it cannot
    /// be set by create and update methods.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The status of this version. This field is read-only and cannot be set by
    /// create and update methods.
    #[prost(enumeration = "version::VersionStatus", tag = "6")]
    pub status: i32,
}
/// Nested message and enum types in `Version`.
pub mod version {
    /// The status of a version.
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
    pub enum VersionStatus {
        /// Not specified. This value is not used.
        Unspecified = 0,
        /// Version is not ready to serve (e.g. training is in progress).
        InProgress = 1,
        /// Version is ready to serve.
        Ready = 2,
        /// Version training failed.
        Failed = 3,
    }
    impl VersionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VersionStatus::Unspecified => "VERSION_STATUS_UNSPECIFIED",
                VersionStatus::InProgress => "IN_PROGRESS",
                VersionStatus::Ready => "READY",
                VersionStatus::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERSION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "READY" => Some(Self::Ready),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// The request message for \[Versions.ListVersions][google.cloud.dialogflow.v2.Versions.ListVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsRequest {
    /// Required. The agent to list all versions from.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent`
    /// - `projects/<Project ID>/locations/<Location ID>/agent`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Versions.ListVersions][google.cloud.dialogflow.v2.Versions.ListVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsResponse {
    /// The list of agent versions. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[Versions.GetVersion][google.cloud.dialogflow.v2.Versions.GetVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// Required. The name of the version.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/versions/<Version ID>`
    /// - `projects/<Project ID>/locations/<Location ID>/agent/versions/<Version
    ///    ID>`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for \[Versions.CreateVersion][google.cloud.dialogflow.v2.Versions.CreateVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionRequest {
    /// Required. The agent to create a version for.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent`
    /// - `projects/<Project ID>/locations/<Location ID>/agent`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The version to create.
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<Version>,
}
/// The request message for \[Versions.UpdateVersion][google.cloud.dialogflow.v2.Versions.UpdateVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVersionRequest {
    /// Required. The version to update.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/versions/<Version ID>`
    /// - `projects/<Project ID>/locations/<Location ID>/agent/versions/<Version
    ///    ID>`
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<Version>,
    /// Required. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for \[Versions.DeleteVersion][google.cloud.dialogflow.v2.Versions.DeleteVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionRequest {
    /// Required. The name of the version to delete.
    /// Supported formats:
    ///
    /// - `projects/<Project ID>/agent/versions/<Version ID>`
    /// - `projects/<Project ID>/locations/<Location ID>/agent/versions/<Version
    ///    ID>`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod versions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Versions][google.cloud.dialogflow.v2.Version].
    #[derive(Debug, Clone)]
    pub struct VersionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VersionsClient<T>
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
        ) -> VersionsClient<InterceptedService<T, F>>
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
            VersionsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the list of all versions of the specified agent.
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVersionsResponse>,
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
                "/google.cloud.dialogflow.v2.Versions/ListVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Versions",
                        "ListVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the specified agent version.
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> std::result::Result<tonic::Response<super::Version>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Versions/GetVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.dialogflow.v2.Versions", "GetVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an agent version.
        ///
        /// The new version points to the agent instance in the "default" environment.
        pub async fn create_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVersionRequest>,
        ) -> std::result::Result<tonic::Response<super::Version>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Versions/CreateVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Versions",
                        "CreateVersion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified agent version.
        ///
        /// Note that this method does not allow you to update the state of the agent
        /// the given version points to. It allows you to update only mutable
        /// properties of the version resource.
        pub async fn update_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVersionRequest>,
        ) -> std::result::Result<tonic::Response<super::Version>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Versions/UpdateVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Versions",
                        "UpdateVersion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete the specified agent version.
        pub async fn delete_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVersionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.dialogflow.v2.Versions/DeleteVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dialogflow.v2.Versions",
                        "DeleteVersion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The request message for a webhook call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookRequest {
    /// The unique identifier of detectIntent request session.
    /// Can be used to identify end-user inside webhook implementation.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>`, or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`.
    #[prost(string, tag = "4")]
    pub session: ::prost::alloc::string::String,
    /// The unique identifier of the response. Contains the same value as
    /// `\[Streaming\]DetectIntentResponse.response_id`.
    #[prost(string, tag = "1")]
    pub response_id: ::prost::alloc::string::String,
    /// The result of the conversational query or event processing. Contains the
    /// same value as `\[Streaming\]DetectIntentResponse.query_result`.
    #[prost(message, optional, tag = "2")]
    pub query_result: ::core::option::Option<QueryResult>,
    /// Optional. The contents of the original request that was passed to
    /// `\[Streaming\]DetectIntent` call.
    #[prost(message, optional, tag = "3")]
    pub original_detect_intent_request: ::core::option::Option<
        OriginalDetectIntentRequest,
    >,
}
/// The response message for a webhook call.
///
/// This response is validated by the Dialogflow server. If validation fails,
/// an error will be returned in the \[QueryResult.diagnostic_info][google.cloud.dialogflow.v2.QueryResult.diagnostic_info\] field.
/// Setting JSON fields to an empty value with the wrong type is a common error.
/// To avoid this error:
///
/// - Use `""` for empty strings
/// - Use `{}` or `null` for empty objects
/// - Use `[]` or `null` for empty arrays
///
/// For more information, see the
/// [Protocol Buffers Language
/// Guide](<https://developers.google.com/protocol-buffers/docs/proto3#json>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookResponse {
    /// Optional. The text response message intended for the end-user.
    /// It is recommended to use `fulfillment_messages.text.text\[0\]` instead.
    /// When provided, Dialogflow uses this field to populate
    /// \[QueryResult.fulfillment_text][google.cloud.dialogflow.v2.QueryResult.fulfillment_text\] sent to the integration or API caller.
    #[prost(string, tag = "1")]
    pub fulfillment_text: ::prost::alloc::string::String,
    /// Optional. The rich response messages intended for the end-user.
    /// When provided, Dialogflow uses this field to populate
    /// \[QueryResult.fulfillment_messages][google.cloud.dialogflow.v2.QueryResult.fulfillment_messages\] sent to the integration or API caller.
    #[prost(message, repeated, tag = "2")]
    pub fulfillment_messages: ::prost::alloc::vec::Vec<intent::Message>,
    /// Optional. A custom field used to identify the webhook source.
    /// Arbitrary strings are supported.
    /// When provided, Dialogflow uses this field to populate
    /// \[QueryResult.webhook_source][google.cloud.dialogflow.v2.QueryResult.webhook_source\] sent to the integration or API caller.
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    /// Optional. This field can be used to pass custom data from your webhook to the
    /// integration or API caller. Arbitrary JSON objects are supported.
    /// When provided, Dialogflow uses this field to populate
    /// \[QueryResult.webhook_payload][google.cloud.dialogflow.v2.QueryResult.webhook_payload\] sent to the integration or API caller.
    /// This field is also used by the
    /// [Google Assistant
    /// integration](<https://cloud.google.com/dialogflow/docs/integrations/aog>)
    /// for rich response messages.
    /// See the format definition at [Google Assistant Dialogflow webhook
    /// format](<https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json>)
    #[prost(message, optional, tag = "4")]
    pub payload: ::core::option::Option<::prost_types::Struct>,
    /// Optional. The collection of output contexts that will overwrite currently
    /// active contexts for the session and reset their lifespans.
    /// When provided, Dialogflow uses this field to populate
    /// \[QueryResult.output_contexts][google.cloud.dialogflow.v2.QueryResult.output_contexts\] sent to the integration or API caller.
    #[prost(message, repeated, tag = "5")]
    pub output_contexts: ::prost::alloc::vec::Vec<Context>,
    /// Optional. Invokes the supplied events.
    /// When this field is set, Dialogflow ignores the `fulfillment_text`,
    /// `fulfillment_messages`, and `payload` fields.
    #[prost(message, optional, tag = "6")]
    pub followup_event_input: ::core::option::Option<EventInput>,
    /// Optional. Additional session entity types to replace or extend developer
    /// entity types with. The entity synonyms apply to all languages and persist
    /// for the session. Setting this data from a webhook overwrites
    /// the session entity types that have been set using `detectIntent`,
    /// `streamingDetectIntent` or \[SessionEntityType][google.cloud.dialogflow.v2.SessionEntityType\] management methods.
    #[prost(message, repeated, tag = "10")]
    pub session_entity_types: ::prost::alloc::vec::Vec<SessionEntityType>,
}
/// Represents the contents of the original request that was passed to
/// the `\[Streaming\]DetectIntent` call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginalDetectIntentRequest {
    /// The source of this request, e.g., `google`, `facebook`, `slack`. It is set
    /// by Dialogflow-owned servers.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// Optional. The version of the protocol used for this request.
    /// This field is AoG-specific.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Optional. This field is set to the value of the `QueryParameters.payload`
    /// field passed in the request. Some integrations that query a Dialogflow
    /// agent may provide additional information in the payload.
    ///
    /// In particular, for the Dialogflow Phone Gateway integration, this field has
    /// the form:
    /// <pre>{
    ///   "telephony": {
    ///     "caller_id": "+18558363987"
    ///   }
    /// }</pre>
    /// Note: The caller ID field (`caller_id`) will be redacted for Trial
    /// Edition agents and populated with the caller ID in [E.164
    /// format](<https://en.wikipedia.org/wiki/E.164>) for Essentials Edition agents.
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<::prost_types::Struct>,
}
