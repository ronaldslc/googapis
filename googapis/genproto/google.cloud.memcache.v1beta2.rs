#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Required. Unique name of the resource in this scope including project and
    /// location using the form:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///
    /// Note: Memcached instances are managed and addressed at regional level so
    /// location_id here refers to a GCP region; however, users may choose which
    /// zones Memcached nodes within an instances should be provisioned in.
    /// Refer to [zones] field for more details.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. User provided name for the instance only used for display
    /// purposes. Cannot be more than 80 characters.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Optional. Resource labels to represent user-provided metadata.
    /// Refer to cloud documentation on labels for more details.
    /// https://cloud.google.com/compute/docs/labeling-resources
    #[prost(map = "string, string", tag = "3")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Optional. The full name of the Google Compute Engine
    /// [network](https://cloud.google.com/vpc/docs/vpc) to which the
    /// instance is connected. If left unspecified, the `default` network
    /// will be used.
    #[prost(string, tag = "4")]
    pub authorized_network: std::string::String,
    /// Optional. Zones where Memcached nodes should be provisioned in.
    /// Memcached nodes will be equally distributed across these zones. If not
    /// provided, the service will by default create nodes in all zones in the
    /// region for the instance.
    #[prost(string, repeated, tag = "5")]
    pub zones: ::std::vec::Vec<std::string::String>,
    /// Required. Number of nodes in the Memcached instance.
    #[prost(int32, tag = "6")]
    pub node_count: i32,
    /// Required. Configuration for Memcached nodes.
    #[prost(message, optional, tag = "7")]
    pub node_config: ::std::option::Option<instance::NodeConfig>,
    /// Optional. The major version of Memcached software.
    /// If not provided, latest supported version will be used. Currently the
    /// latest supported major version is MEMCACHE_1_5.
    /// The minor version will be automatically determined by our system based on
    /// the latest supported minor version.
    #[prost(enumeration = "MemcacheVersion", tag = "9")]
    pub memcache_version: i32,
    /// Optional: User defined parameters to apply to the memcached process
    /// on each node.
    #[prost(message, optional, tag = "11")]
    pub parameters: ::std::option::Option<MemcacheParameters>,
    /// Output only. List of Memcached nodes.
    /// Refer to [Node] message for more details.
    #[prost(message, repeated, tag = "12")]
    pub memcache_nodes: ::std::vec::Vec<instance::Node>,
    /// Output only. The time the instance was created.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the instance was updated.
    #[prost(message, optional, tag = "14")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The state of this Memcached instance.
    #[prost(enumeration = "instance::State", tag = "15")]
    pub state: i32,
    /// Output only. The full version of memcached server running on this instance.
    /// System automatically determines the full memcached version for an instance
    /// based on the input MemcacheVersion.
    /// The full version format will be "memcached-1.5.16".
    #[prost(string, tag = "18")]
    pub memcache_full_version: std::string::String,
    /// List of messages that describe current statuses of memcached instance.
    #[prost(message, repeated, tag = "19")]
    pub instance_messages: ::std::vec::Vec<instance::InstanceMessage>,
    /// Output only. Endpoint for Discovery API
    #[prost(string, tag = "20")]
    pub discovery_endpoint: std::string::String,
}
pub mod instance {
    /// Configuration for a Memcached Node.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodeConfig {
        /// Required. Number of cpus per Memcached node.
        #[prost(int32, tag = "1")]
        pub cpu_count: i32,
        /// Required. Memory size in MiB for each Memcached node.
        #[prost(int32, tag = "2")]
        pub memory_size_mb: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        /// Output only. Identifier of the Memcached node. The node id does not
        /// include project or location like the Memcached instance name.
        #[prost(string, tag = "1")]
        pub node_id: std::string::String,
        /// Output only. Location (GCP Zone) for the Memcached node.
        #[prost(string, tag = "2")]
        pub zone: std::string::String,
        /// Output only. Current state of the Memcached node.
        #[prost(enumeration = "node::State", tag = "3")]
        pub state: i32,
        /// Output only. Hostname or IP address of the Memcached node used by the
        /// clients to connect to the Memcached server on this node.
        #[prost(string, tag = "4")]
        pub host: std::string::String,
        /// Output only. The port number of the Memcached server on this node.
        #[prost(int32, tag = "5")]
        pub port: i32,
        /// User defined parameters currently applied to the node.
        #[prost(message, optional, tag = "6")]
        pub parameters: ::std::option::Option<super::MemcacheParameters>,
    }
    pub mod node {
        /// Different states of a Memcached node.
        /// LINT.IfChange
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            /// Node state is not set.
            Unspecified = 0,
            /// Node is being created.
            Creating = 1,
            /// Node has been created and ready to be used.
            Ready = 2,
            /// Node is being deleted.
            Deleting = 3,
            /// Node is being updated.
            Updating = 4,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceMessage {
        /// A code that correspond to one type of user-facing message.
        #[prost(enumeration = "instance_message::Code", tag = "1")]
        pub code: i32,
        /// Message on memcached instance which will be exposed to users.
        #[prost(string, tag = "2")]
        pub message: std::string::String,
    }
    pub mod instance_message {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Code {
            /// Message Code not set.
            Unspecified = 0,
            /// Memcached nodes are distributed unevenly.
            ZoneDistributionUnbalanced = 1,
        }
    }
    /// Different states of a Memcached instance.
    /// LINT.IfChange
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State not set.
        Unspecified = 0,
        /// Memcached instance is being created.
        Creating = 1,
        /// Memcached instance has been created and ready to be used.
        Ready = 2,
        /// Memcached instance is being deleted.
        Deleting = 4,
        /// Memcached instance is going through maintenance, e.g. data plane rollout.
        PerformingMaintenance = 5,
    }
}
/// Request for [ListInstances][google.cloud.memcache.v1beta2.CloudMemcache.ListInstances].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The resource name of the instance location using the form:
    ///     `projects/{project_id}/locations/{location_id}`
    /// where `location_id` refers to a GCP region
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 1000 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// [next_page_token][CloudMemcache.ListInstancesResponse.next_page_token]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// List filter. For example, exclude all Memcached instances with name as
    /// my-instance by specifying "name != my-instance".
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response for [ListInstances][google.cloud.memcache.v1beta2.CloudMemcache.ListInstances].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of Memcached instances in the project in the specified location,
    /// or across all locations.
    ///
    /// If the `location_id` in the parent field of the request is "-", all regions
    /// available to the project are queried, and the results aggregated.
    #[prost(message, repeated, tag = "1")]
    pub resources: ::std::vec::Vec<Instance>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request for [GetInstance][google.cloud.memcache.v1beta2.CloudMemcache.GetInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Memcached instance resource name in the format:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for [CreateInstance][google.cloud.memcache.v1beta2.CloudMemcache.CreateInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The resource name of the instance location using the form:
    ///     `projects/{project_id}/locations/{location_id}`
    /// where `location_id` refers to a GCP region
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The logical name of the Memcached instance in the user
    /// project with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-40 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the user project / location
    #[prost(string, tag = "2")]
    pub instance_id: std::string::String,
    /// Required. A Memcached [Instance] resource
    #[prost(message, optional, tag = "3")]
    pub resource: ::std::option::Option<Instance>,
}
/// Request for [UpdateInstance][google.cloud.memcache.v1beta2.CloudMemcache.UpdateInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. Mask of fields to update.
    ///  *   `displayName`
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. A Memcached [Instance] resource.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub resource: ::std::option::Option<Instance>,
}
/// Request for [DeleteInstance][google.cloud.memcache.v1beta2.CloudMemcache.DeleteInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Memcached instance resource name in the format:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for [ApplyParameters][google.cloud.memcache.v1beta2.CloudMemcache.ApplyParameters].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyParametersRequest {
    /// Required. Resource name of the Memcached instance for which parameter group updates
    /// should be applied.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Nodes to which we should apply the instance-level parameter group.
    #[prost(string, repeated, tag = "2")]
    pub node_ids: ::std::vec::Vec<std::string::String>,
    /// Whether to apply instance-level parameter group to all nodes. If set to
    /// true, will explicitly restrict users from specifying any nodes, and apply
    /// parameter group updates to all nodes within the instance.
    #[prost(bool, tag = "3")]
    pub apply_all: bool,
}
/// Request for [UpdateParameters][google.cloud.memcache.v1beta2.CloudMemcache.UpdateParameters].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateParametersRequest {
    /// Required. Resource name of the Memcached instance for which the parameters should be
    /// updated.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The parameters to apply to the instance.
    #[prost(message, optional, tag = "3")]
    pub parameters: ::std::option::Option<MemcacheParameters>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemcacheParameters {
    /// Output only. The unique ID associated with this set of parameters. Users
    /// can use this id to determine if the parameters associated with the instance
    /// differ from the parameters associated with the nodes and any action needs
    /// to be taken to apply parameters on nodes.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// User defined set of parameters to use in the memcached process.
    #[prost(map = "string, string", tag = "3")]
    pub params: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Represents the metadata of a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Time when the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time when the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: std::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: std::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: std::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: std::string::String,
}
/// Metadata for the given [google.cloud.location.Location][google.cloud.location.Location].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// Output only. The set of available zones in the location. The map is keyed
    /// by the lowercase ID of each zone, as defined by GCE. These keys can be
    /// specified in the `zones` field when creating a Memcached instance.
    #[prost(map = "string, message", tag = "1")]
    pub available_zones: ::std::collections::HashMap<std::string::String, ZoneMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZoneMetadata {}
/// Memcached versions supported by our service.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MemcacheVersion {
    Unspecified = 0,
    /// Memcached 1.5 version.
    Memcache15 = 1,
}
#[doc = r" Generated client implementations."]
pub mod cloud_memcache_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Configures and manages Cloud Memorystore for Memcached instances."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " The `memcache.googleapis.com` service implements the Google Cloud Memorystore"]
    #[doc = " for Memcached API and defines the following resource model for managing"]
    #[doc = " Memorystore Memcached (also called Memcached below) instances:"]
    #[doc = " * The service works with a collection of cloud projects, named: `/projects/*`"]
    #[doc = " * Each project has a collection of available locations, named: `/locations/*`"]
    #[doc = " * Each location has a collection of Memcached instances, named:"]
    #[doc = " `/instances/*`"]
    #[doc = " * As such, Memcached instances are resources of the form:"]
    #[doc = "   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`"]
    #[doc = ""]
    #[doc = " Note that location_id must be refering to a GCP `region`; for example:"]
    #[doc = " * `projects/my-memcached-project/locations/us-central1/instances/my-memcached`"]
    pub struct CloudMemcacheClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudMemcacheClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Lists Instances in a given project and location."]
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.memcache.v1beta2.CloudMemcache/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Instance."]
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.memcache.v1beta2.CloudMemcache/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Instance in a given project and location."]
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.memcache.v1beta2.CloudMemcache/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing Instance in a given project and location."]
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.memcache.v1beta2.CloudMemcache/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the defined Memcached Parameters for an existing Instance."]
        #[doc = " This method only stages the parameters, it must be followed by"]
        #[doc = " ApplyParameters to apply the parameters to nodes of the Memcached Instance."]
        pub async fn update_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateParametersRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.memcache.v1beta2.CloudMemcache/UpdateParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Instance."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.memcache.v1beta2.CloudMemcache/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ApplyParameters will update current set of Parameters to the set of"]
        #[doc = " specified nodes of the Memcached Instance."]
        pub async fn apply_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyParametersRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.memcache.v1beta2.CloudMemcache/ApplyParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudMemcacheClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudMemcacheClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudMemcacheClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cloud_memcache_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CloudMemcacheServer."]
    #[async_trait]
    pub trait CloudMemcache: Send + Sync + 'static {
        #[doc = " Lists Instances in a given project and location."]
        async fn list_instances(
            &self,
            request: tonic::Request<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status>;
        #[doc = " Gets details of a single Instance."]
        async fn get_instance(
            &self,
            request: tonic::Request<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status>;
        #[doc = " Creates a new Instance in a given project and location."]
        async fn create_instance(
            &self,
            request: tonic::Request<super::CreateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates an existing Instance in a given project and location."]
        async fn update_instance(
            &self,
            request: tonic::Request<super::UpdateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates the defined Memcached Parameters for an existing Instance."]
        #[doc = " This method only stages the parameters, it must be followed by"]
        #[doc = " ApplyParameters to apply the parameters to nodes of the Memcached Instance."]
        async fn update_parameters(
            &self,
            request: tonic::Request<super::UpdateParametersRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a single Instance."]
        async fn delete_instance(
            &self,
            request: tonic::Request<super::DeleteInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " ApplyParameters will update current set of Parameters to the set of"]
        #[doc = " specified nodes of the Memcached Instance."]
        async fn apply_parameters(
            &self,
            request: tonic::Request<super::ApplyParametersRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Configures and manages Cloud Memorystore for Memcached instances."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " The `memcache.googleapis.com` service implements the Google Cloud Memorystore"]
    #[doc = " for Memcached API and defines the following resource model for managing"]
    #[doc = " Memorystore Memcached (also called Memcached below) instances:"]
    #[doc = " * The service works with a collection of cloud projects, named: `/projects/*`"]
    #[doc = " * Each project has a collection of available locations, named: `/locations/*`"]
    #[doc = " * Each location has a collection of Memcached instances, named:"]
    #[doc = " `/instances/*`"]
    #[doc = " * As such, Memcached instances are resources of the form:"]
    #[doc = "   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`"]
    #[doc = ""]
    #[doc = " Note that location_id must be refering to a GCP `region`; for example:"]
    #[doc = " * `projects/my-memcached-project/locations/us-central1/instances/my-memcached`"]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CloudMemcacheServer<T: CloudMemcache> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CloudMemcache> CloudMemcacheServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for CloudMemcacheServer<T>
    where
        T: CloudMemcache,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.memcache.v1beta2.CloudMemcache/ListInstances" => {
                    #[allow(non_camel_case_types)]
                    struct ListInstancesSvc<T: CloudMemcache>(pub Arc<T>);
                    impl<T: CloudMemcache> tonic::server::UnaryService<super::ListInstancesRequest>
                        for ListInstancesSvc<T>
                    {
                        type Response = super::ListInstancesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInstancesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_instances(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListInstancesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.memcache.v1beta2.CloudMemcache/GetInstance" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstanceSvc<T: CloudMemcache>(pub Arc<T>);
                    impl<T: CloudMemcache> tonic::server::UnaryService<super::GetInstanceRequest>
                        for GetInstanceSvc<T>
                    {
                        type Response = super::Instance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.memcache.v1beta2.CloudMemcache/CreateInstance" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInstanceSvc<T: CloudMemcache>(pub Arc<T>);
                    impl<T: CloudMemcache> tonic::server::UnaryService<super::CreateInstanceRequest>
                        for CreateInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.memcache.v1beta2.CloudMemcache/UpdateInstance" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateInstanceSvc<T: CloudMemcache>(pub Arc<T>);
                    impl<T: CloudMemcache> tonic::server::UnaryService<super::UpdateInstanceRequest>
                        for UpdateInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.memcache.v1beta2.CloudMemcache/UpdateParameters" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateParametersSvc<T: CloudMemcache>(pub Arc<T>);
                    impl<T: CloudMemcache>
                        tonic::server::UnaryService<super::UpdateParametersRequest>
                        for UpdateParametersSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateParametersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_parameters(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.memcache.v1beta2.CloudMemcache/DeleteInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteInstanceSvc<T: CloudMemcache>(pub Arc<T>);
                    impl<T: CloudMemcache> tonic::server::UnaryService<super::DeleteInstanceRequest>
                        for DeleteInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteInstanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.memcache.v1beta2.CloudMemcache/ApplyParameters" => {
                    #[allow(non_camel_case_types)]
                    struct ApplyParametersSvc<T: CloudMemcache>(pub Arc<T>);
                    impl<T: CloudMemcache>
                        tonic::server::UnaryService<super::ApplyParametersRequest>
                        for ApplyParametersSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApplyParametersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.apply_parameters(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ApplyParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: CloudMemcache> Clone for CloudMemcacheServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CloudMemcache> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
