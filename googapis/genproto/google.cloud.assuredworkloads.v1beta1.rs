/// Request for creating a workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadRequest {
    /// Required. The resource name of the new Workload's parent.
    /// Must be of the form `organizations/{org_id}/locations/{location_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Assured Workload to create
    #[prost(message, optional, tag = "2")]
    pub workload: ::core::option::Option<Workload>,
    /// Optional. A identifier associated with the workload and underlying projects which
    /// allows for the break down of billing costs for a workload. The value
    /// provided for the identifier will add a label to the workload and contained
    /// projects with the identifier as the value.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
}
/// Request for Updating a workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkloadRequest {
    /// Required. The workload to update.
    /// The workload’s `name` field is used to identify the workload to be updated.
    /// Format:
    /// organizations/{org_id}/locations/{location_id}/workloads/{workload_id}
    #[prost(message, optional, tag = "1")]
    pub workload: ::core::option::Option<Workload>,
    /// Required. The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for deleting a Workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkloadRequest {
    /// Required. The `name` field is used to identify the workload.
    /// Format:
    /// organizations/{org_id}/locations/{location_id}/workloads/{workload_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The etag of the workload.
    /// If this is provided, it must match the server's etag.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request for fetching a workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkloadRequest {
    /// Required. The resource name of the Workload to fetch. This is the workloads's
    /// relative path in the API, formatted as
    /// "organizations/{organization_id}/locations/{location_id}/workloads/{workload_id}".
    /// For example,
    /// "organizations/123/locations/us-east1/workloads/assured-workload-1".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for fetching workloads in an organization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsRequest {
    /// Required. Parent Resource to list workloads from.
    /// Must be of the form `organizations/{org_id}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token returned from previous request. Page token contains context from
    /// previous request. Page token needs to be passed in the second and following
    /// requests.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A custom filter for filtering by properties of a workload. At this time,
    /// only filtering by labels is supported.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response of ListWorkloads endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsResponse {
    /// List of Workloads under a given parent.
    #[prost(message, repeated, tag = "1")]
    pub workloads: ::prost::alloc::vec::Vec<Workload>,
    /// The next page token. Return empty if reached the last page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// An Workload object for managing highly regulated workloads of cloud
/// customers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workload {
    /// Optional. The resource name of the workload.
    /// Format:
    /// organizations/{organization}/locations/{location}/workloads/{workload}
    ///
    /// Read-only.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-assigned display name of the Workload.
    /// When present it must be between 4 to 30 characters.
    /// Allowed characters are: lowercase and uppercase letters, numbers,
    /// hyphen, and spaces.
    ///
    /// Example: My Workload
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The resources associated with this workload.
    /// These resources will be created when creating the workload.
    /// If any of the projects already exist, the workload creation will fail.
    /// Always read only.
    #[prost(message, repeated, tag = "3")]
    pub resources: ::prost::alloc::vec::Vec<workload::ResourceInfo>,
    /// Required. Immutable. Compliance Regime associated with this workload.
    #[prost(enumeration = "workload::ComplianceRegime", tag = "4")]
    pub compliance_regime: i32,
    /// Output only. Immutable. The Workload creation timestamp.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. The billing account used for the resources which are
    /// direct children of workload. This billing account is initially associated
    /// with the resources created as part of Workload creation.
    /// After the initial creation of these resources, the customer can change
    /// the assigned billing account.
    /// The resource name has the form
    /// `billingAccounts/{billing_account_id}`. For example,
    /// `billingAccounts/012345-567890-ABCDEF`.
    #[prost(string, tag = "6")]
    pub billing_account: ::prost::alloc::string::String,
    /// Optional. ETag of the workload, it is calculated on the basis
    /// of the Workload contents. It will be used in Update & Delete operations.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. Labels applied to the workload.
    #[prost(map = "string, string", tag = "10")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Input only. The parent resource for the resources managed by this Assured Workload. May
    /// be either empty or a folder resource which is a child of the
    /// Workload parent. If not specified all resources are created under the
    /// parent organization.
    /// Format:
    /// folders/{folder_id}
    #[prost(string, tag = "13")]
    pub provisioned_resources_parent: ::prost::alloc::string::String,
    /// Input only. Settings used to create a CMEK crypto key. When set a project with a KMS
    /// CMEK key is provisioned. This field is mandatory for a subset of Compliance
    /// Regimes.
    #[prost(message, optional, tag = "14")]
    pub kms_settings: ::core::option::Option<workload::KmsSettings>,
    /// Input only. Resource properties that are used to customize workload resources.
    /// These properties (such as custom project id) will be used to create
    /// workload resources if possible. This field is optional.
    #[prost(message, repeated, tag = "15")]
    pub resource_settings: ::prost::alloc::vec::Vec<workload::ResourceSettings>,
    /// Settings specific to the selected \[compliance_regime\]
    #[prost(oneof = "workload::ComplianceRegimeSettings", tags = "7, 8, 11, 12")]
    pub compliance_regime_settings: ::core::option::Option<
        workload::ComplianceRegimeSettings,
    >,
}
/// Nested message and enum types in `Workload`.
pub mod workload {
    /// Represent the resources that are children of this Workload.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceInfo {
        /// Resource identifier.
        /// For a project this represents project_number.
        #[prost(int64, tag = "1")]
        pub resource_id: i64,
        /// Indicates the type of resource.
        #[prost(enumeration = "resource_info::ResourceType", tag = "2")]
        pub resource_type: i32,
    }
    /// Nested message and enum types in `ResourceInfo`.
    pub mod resource_info {
        /// The type of resource.
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
        pub enum ResourceType {
            /// Unknown resource type.
            Unspecified = 0,
            /// Deprecated. Existing workloads will continue to support this, but new
            /// CreateWorkloadRequests should not specify this as an input value.
            ConsumerProject = 1,
            /// Consumer Folder.
            ConsumerFolder = 4,
            /// Consumer project containing encryption keys.
            EncryptionKeysProject = 2,
            /// Keyring resource that hosts encryption keys.
            Keyring = 3,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::Unspecified => "RESOURCE_TYPE_UNSPECIFIED",
                    ResourceType::ConsumerProject => "CONSUMER_PROJECT",
                    ResourceType::ConsumerFolder => "CONSUMER_FOLDER",
                    ResourceType::EncryptionKeysProject => "ENCRYPTION_KEYS_PROJECT",
                    ResourceType::Keyring => "KEYRING",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "RESOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "CONSUMER_PROJECT" => Some(Self::ConsumerProject),
                    "CONSUMER_FOLDER" => Some(Self::ConsumerFolder),
                    "ENCRYPTION_KEYS_PROJECT" => Some(Self::EncryptionKeysProject),
                    "KEYRING" => Some(Self::Keyring),
                    _ => None,
                }
            }
        }
    }
    /// Settings specific to the Key Management Service.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KmsSettings {
        /// Required. Input only. Immutable. The time at which the Key Management Service will automatically create a
        /// new version of the crypto key and mark it as the primary.
        #[prost(message, optional, tag = "1")]
        pub next_rotation_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Required. Input only. Immutable. \[next_rotation_time\] will be advanced by this period when the Key
        /// Management Service automatically rotates a key. Must be at least 24 hours
        /// and at most 876,000 hours.
        #[prost(message, optional, tag = "2")]
        pub rotation_period: ::core::option::Option<::prost_types::Duration>,
    }
    /// Settings specific to resources needed for IL4.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Il4Settings {
        /// Required. Input only. Immutable. Settings used to create a CMEK crypto key.
        #[prost(message, optional, tag = "1")]
        pub kms_settings: ::core::option::Option<KmsSettings>,
    }
    /// Settings specific to resources needed for CJIS.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CjisSettings {
        /// Required. Input only. Immutable. Settings used to create a CMEK crypto key.
        #[prost(message, optional, tag = "1")]
        pub kms_settings: ::core::option::Option<KmsSettings>,
    }
    /// Settings specific to resources needed for FedRAMP High.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FedrampHighSettings {
        /// Required. Input only. Immutable. Settings used to create a CMEK crypto key.
        #[prost(message, optional, tag = "1")]
        pub kms_settings: ::core::option::Option<KmsSettings>,
    }
    /// Settings specific to resources needed for FedRAMP Moderate.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FedrampModerateSettings {
        /// Required. Input only. Immutable. Settings used to create a CMEK crypto key.
        #[prost(message, optional, tag = "1")]
        pub kms_settings: ::core::option::Option<KmsSettings>,
    }
    /// Represent the custom settings for the resources to be created.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSettings {
        /// Resource identifier.
        /// For a project this represents project_id. If the project is already
        /// taken, the workload creation will fail.
        #[prost(string, tag = "1")]
        pub resource_id: ::prost::alloc::string::String,
        /// Indicates the type of resource. This field should be specified to
        /// correspond the id to the right project type (CONSUMER_PROJECT or
        /// ENCRYPTION_KEYS_PROJECT)
        #[prost(enumeration = "resource_info::ResourceType", tag = "2")]
        pub resource_type: i32,
        /// User-assigned resource display name.
        /// If not empty it will be used to create a resource with the specified
        /// name.
        #[prost(string, tag = "3")]
        pub display_name: ::prost::alloc::string::String,
    }
    /// Supported Compliance Regimes.
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
    pub enum ComplianceRegime {
        /// Unknown compliance regime.
        Unspecified = 0,
        /// Information protection as per DoD IL4 requirements.
        Il4 = 1,
        /// Criminal Justice Information Services (CJIS) Security policies.
        Cjis = 2,
        /// FedRAMP High data protection controls
        FedrampHigh = 3,
        /// FedRAMP Moderate data protection controls
        FedrampModerate = 4,
        /// Assured Workloads For US Regions data protection controls
        UsRegionalAccess = 5,
        /// Health Insurance Portability and Accountability Act controls
        Hipaa = 6,
        /// Health Information Trust Alliance controls
        Hitrust = 7,
        /// Assured Workloads For EU Regions and Support controls
        EuRegionsAndSupport = 8,
        /// Assured Workloads For Canada Regions and Support controls
        CaRegionsAndSupport = 9,
    }
    impl ComplianceRegime {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ComplianceRegime::Unspecified => "COMPLIANCE_REGIME_UNSPECIFIED",
                ComplianceRegime::Il4 => "IL4",
                ComplianceRegime::Cjis => "CJIS",
                ComplianceRegime::FedrampHigh => "FEDRAMP_HIGH",
                ComplianceRegime::FedrampModerate => "FEDRAMP_MODERATE",
                ComplianceRegime::UsRegionalAccess => "US_REGIONAL_ACCESS",
                ComplianceRegime::Hipaa => "HIPAA",
                ComplianceRegime::Hitrust => "HITRUST",
                ComplianceRegime::EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT",
                ComplianceRegime::CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLIANCE_REGIME_UNSPECIFIED" => Some(Self::Unspecified),
                "IL4" => Some(Self::Il4),
                "CJIS" => Some(Self::Cjis),
                "FEDRAMP_HIGH" => Some(Self::FedrampHigh),
                "FEDRAMP_MODERATE" => Some(Self::FedrampModerate),
                "US_REGIONAL_ACCESS" => Some(Self::UsRegionalAccess),
                "HIPAA" => Some(Self::Hipaa),
                "HITRUST" => Some(Self::Hitrust),
                "EU_REGIONS_AND_SUPPORT" => Some(Self::EuRegionsAndSupport),
                "CA_REGIONS_AND_SUPPORT" => Some(Self::CaRegionsAndSupport),
                _ => None,
            }
        }
    }
    /// Settings specific to the selected \[compliance_regime\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ComplianceRegimeSettings {
        /// Required. Input only. Immutable. Settings specific to resources needed for IL4.
        #[prost(message, tag = "7")]
        Il4Settings(Il4Settings),
        /// Required. Input only. Immutable. Settings specific to resources needed for CJIS.
        #[prost(message, tag = "8")]
        CjisSettings(CjisSettings),
        /// Required. Input only. Immutable. Settings specific to resources needed for FedRAMP High.
        #[prost(message, tag = "11")]
        FedrampHighSettings(FedrampHighSettings),
        /// Required. Input only. Immutable. Settings specific to resources needed for FedRAMP Moderate.
        #[prost(message, tag = "12")]
        FedrampModerateSettings(FedrampModerateSettings),
    }
}
/// Operation metadata to give request details of CreateWorkload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadOperationMetadata {
    /// Optional. Time when the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The display name of the workload.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The parent of the workload.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Compliance controls that should be applied to the resources managed by
    /// the workload.
    #[prost(enumeration = "workload::ComplianceRegime", tag = "4")]
    pub compliance_regime: i32,
    /// Optional. Resource properties in the input that are used for creating/customizing
    /// workload resources.
    #[prost(message, repeated, tag = "5")]
    pub resource_settings: ::prost::alloc::vec::Vec<workload::ResourceSettings>,
}
/// Generated client implementations.
pub mod assured_workloads_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage AssuredWorkloads.
    #[derive(Debug, Clone)]
    pub struct AssuredWorkloadsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssuredWorkloadsServiceClient<T>
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
        ) -> AssuredWorkloadsServiceClient<InterceptedService<T, F>>
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
            AssuredWorkloadsServiceClient::new(
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
        /// Creates Assured Workload.
        pub async fn create_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkloadRequest>,
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
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/CreateWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService",
                        "CreateWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing workload.
        /// Currently allows updating of workload display_name and labels.
        /// For force updates don't set etag field in the Workload.
        /// Only one update operation per workload can be in progress.
        pub async fn update_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkloadRequest>,
        ) -> std::result::Result<tonic::Response<super::Workload>, tonic::Status> {
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
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/UpdateWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService",
                        "UpdateWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the workload. Make sure that workload's direct children are already
        /// in a deleted state, otherwise the request will fail with a
        /// FAILED_PRECONDITION error.
        pub async fn delete_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkloadRequest>,
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
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/DeleteWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService",
                        "DeleteWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets Assured Workload associated with a CRM Node
        pub async fn get_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkloadRequest>,
        ) -> std::result::Result<tonic::Response<super::Workload>, tonic::Status> {
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
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/GetWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService",
                        "GetWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Assured Workloads under a CRM Node.
        pub async fn list_workloads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkloadsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkloadsResponse>,
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
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/ListWorkloads",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService",
                        "ListWorkloads",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
