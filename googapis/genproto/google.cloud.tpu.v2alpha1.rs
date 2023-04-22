/// A guest attributes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestAttributes {
    /// The path to be queried. This can be the default namespace ('/') or a
    /// nested namespace ('/\<namespace\>/') or a specified key
    /// ('/\<namespace\>/\<key\>')
    #[prost(string, tag = "1")]
    pub query_path: ::prost::alloc::string::String,
    /// The value of the requested queried path.
    #[prost(message, optional, tag = "2")]
    pub query_value: ::core::option::Option<GuestAttributesValue>,
}
/// Array of guest attribute namespace/key/value tuples.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestAttributesValue {
    /// The list of guest attributes entries.
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<GuestAttributesEntry>,
}
/// A guest attributes namespace/key/value entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestAttributesEntry {
    /// Namespace for the guest attribute entry.
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// Key for the guest attribute entry.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// Value for the guest attribute entry.
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
}
/// A node-attached disk resource.
/// Next ID: 8;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachedDisk {
    /// Specifies the full path to an existing disk.
    /// For example: "projects/my-project/zones/us-central1-c/disks/my-disk".
    #[prost(string, tag = "3")]
    pub source_disk: ::prost::alloc::string::String,
    /// The mode in which to attach this disk.
    /// If not specified, the default is READ_WRITE mode.
    /// Only applicable to data_disks.
    #[prost(enumeration = "attached_disk::DiskMode", tag = "4")]
    pub mode: i32,
}
/// Nested message and enum types in `AttachedDisk`.
pub mod attached_disk {
    /// The different mode of the attached disk.
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
    pub enum DiskMode {
        /// The disk mode is not known/set.
        Unspecified = 0,
        /// Attaches the disk in read-write mode. Only one TPU node can attach a disk
        /// in read-write mode at a time.
        ReadWrite = 1,
        /// Attaches the disk in read-only mode. Multiple TPU nodes can attach
        /// a disk in read-only mode at a time.
        ReadOnly = 2,
    }
    impl DiskMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DiskMode::Unspecified => "DISK_MODE_UNSPECIFIED",
                DiskMode::ReadWrite => "READ_WRITE",
                DiskMode::ReadOnly => "READ_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DISK_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "READ_WRITE" => Some(Self::ReadWrite),
                "READ_ONLY" => Some(Self::ReadOnly),
                _ => None,
            }
        }
    }
}
/// Sets the scheduling options for this node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulingConfig {
    /// Defines whether the node is preemptible.
    #[prost(bool, tag = "1")]
    pub preemptible: bool,
    /// Whether the node is created under a reservation.
    #[prost(bool, tag = "2")]
    pub reserved: bool,
}
/// A network endpoint over which a TPU worker can be reached.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkEndpoint {
    /// The internal IP address of this network endpoint.
    #[prost(string, tag = "1")]
    pub ip_address: ::prost::alloc::string::String,
    /// The port of this network endpoint.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// The access config for the TPU worker.
    #[prost(message, optional, tag = "5")]
    pub access_config: ::core::option::Option<AccessConfig>,
}
/// An access config attached to the TPU worker.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessConfig {
    /// Output only. An external IP address associated with the TPU worker.
    #[prost(string, tag = "1")]
    pub external_ip: ::prost::alloc::string::String,
}
/// Network related configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// The name of the network for the TPU node. It must be a preexisting Google
    /// Compute Engine network. If none is provided, "default" will be used.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// The name of the subnetwork for the TPU node. It must be a preexisting
    /// Google Compute Engine subnetwork. If none is provided, "default" will be
    /// used.
    #[prost(string, tag = "2")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Indicates that external IP addresses would be associated with the TPU
    /// workers. If set to false, the specified subnetwork or network should have
    /// Private Google Access enabled.
    #[prost(bool, tag = "3")]
    pub enable_external_ips: bool,
}
/// A service account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// Email address of the service account. If empty, default Compute service
    /// account will be used.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// The list of scopes to be made available for this service account. If empty,
    /// access to all Cloud APIs will be allowed.
    #[prost(string, repeated, tag = "2")]
    pub scope: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A TPU instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Output only. Immutable. The name of the TPU.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The user-supplied description of the TPU. Maximum of 512 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. The type of hardware accelerators associated with this node.
    #[prost(string, tag = "5")]
    pub accelerator_type: ::prost::alloc::string::String,
    /// Output only. The current state for the TPU Node.
    #[prost(enumeration = "node::State", tag = "9")]
    pub state: i32,
    /// Output only. If this field is populated, it contains a description of why
    /// the TPU Node is unhealthy.
    #[prost(string, tag = "10")]
    pub health_description: ::prost::alloc::string::String,
    /// Required. The runtime version running in the Node.
    #[prost(string, tag = "11")]
    pub runtime_version: ::prost::alloc::string::String,
    /// Network configurations for the TPU node.
    #[prost(message, optional, tag = "36")]
    pub network_config: ::core::option::Option<NetworkConfig>,
    /// The CIDR block that the TPU node will use when selecting an IP address.
    /// This CIDR block must be a /29 block; the Compute Engine networks API
    /// forbids a smaller block, and using a larger block would be wasteful (a
    /// node can only consume one IP address). Errors will occur if the CIDR block
    /// has already been used for a currently existing TPU node, the CIDR block
    /// conflicts with any subnetworks in the user's provided network, or the
    /// provided network is peered with another network that is using that CIDR
    /// block.
    #[prost(string, tag = "13")]
    pub cidr_block: ::prost::alloc::string::String,
    /// The Google Cloud Platform Service Account to be used by the TPU node VMs.
    /// If None is specified, the default compute service account will be used.
    #[prost(message, optional, tag = "37")]
    pub service_account: ::core::option::Option<ServiceAccount>,
    /// Output only. The time when the node was created.
    #[prost(message, optional, tag = "16")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The scheduling options for this node.
    #[prost(message, optional, tag = "17")]
    pub scheduling_config: ::core::option::Option<SchedulingConfig>,
    /// Output only. The network endpoints where TPU workers can be accessed and
    /// sent work. It is recommended that runtime clients of the node reach out
    /// to the 0th entry in this map first.
    #[prost(message, repeated, tag = "21")]
    pub network_endpoints: ::prost::alloc::vec::Vec<NetworkEndpoint>,
    /// The health status of the TPU node.
    #[prost(enumeration = "node::Health", tag = "22")]
    pub health: i32,
    /// Resource labels to represent user-provided metadata.
    #[prost(map = "string, string", tag = "24")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Custom metadata to apply to the TPU Node.
    /// Can set startup-script and shutdown-script
    #[prost(map = "string, string", tag = "34")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Tags to apply to the TPU Node. Tags are used to identify valid sources or
    /// targets for network firewalls.
    #[prost(string, repeated, tag = "40")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The unique identifier for the TPU Node.
    #[prost(int64, tag = "33")]
    pub id: i64,
    /// The additional data disks for the Node.
    #[prost(message, repeated, tag = "41")]
    pub data_disks: ::prost::alloc::vec::Vec<AttachedDisk>,
    /// Output only. The API version that created this Node.
    #[prost(enumeration = "node::ApiVersion", tag = "38")]
    pub api_version: i32,
    /// Output only. The Symptoms that have occurred to the TPU Node.
    #[prost(message, repeated, tag = "39")]
    pub symptoms: ::prost::alloc::vec::Vec<Symptom>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    /// Represents the different states of a TPU node during its lifecycle.
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
        /// TPU node state is not known/set.
        Unspecified = 0,
        /// TPU node is being created.
        Creating = 1,
        /// TPU node has been created.
        Ready = 2,
        /// TPU node is restarting.
        Restarting = 3,
        /// TPU node is undergoing reimaging.
        Reimaging = 4,
        /// TPU node is being deleted.
        Deleting = 5,
        /// TPU node is being repaired and may be unusable. Details can be
        /// found in the `help_description` field.
        Repairing = 6,
        /// TPU node is stopped.
        Stopped = 8,
        /// TPU node is currently stopping.
        Stopping = 9,
        /// TPU node is currently starting.
        Starting = 10,
        /// TPU node has been preempted. Only applies to Preemptible TPU Nodes.
        Preempted = 11,
        /// TPU node has been terminated due to maintenance or has reached the end of
        /// its life cycle (for preemptible nodes).
        Terminated = 12,
        /// TPU node is currently hiding.
        Hiding = 13,
        /// TPU node has been hidden.
        Hidden = 14,
        /// TPU node is currently unhiding.
        Unhiding = 15,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Restarting => "RESTARTING",
                State::Reimaging => "REIMAGING",
                State::Deleting => "DELETING",
                State::Repairing => "REPAIRING",
                State::Stopped => "STOPPED",
                State::Stopping => "STOPPING",
                State::Starting => "STARTING",
                State::Preempted => "PREEMPTED",
                State::Terminated => "TERMINATED",
                State::Hiding => "HIDING",
                State::Hidden => "HIDDEN",
                State::Unhiding => "UNHIDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "RESTARTING" => Some(Self::Restarting),
                "REIMAGING" => Some(Self::Reimaging),
                "DELETING" => Some(Self::Deleting),
                "REPAIRING" => Some(Self::Repairing),
                "STOPPED" => Some(Self::Stopped),
                "STOPPING" => Some(Self::Stopping),
                "STARTING" => Some(Self::Starting),
                "PREEMPTED" => Some(Self::Preempted),
                "TERMINATED" => Some(Self::Terminated),
                "HIDING" => Some(Self::Hiding),
                "HIDDEN" => Some(Self::Hidden),
                "UNHIDING" => Some(Self::Unhiding),
                _ => None,
            }
        }
    }
    /// Health defines the status of a TPU node as reported by
    /// Health Monitor.
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
    pub enum Health {
        /// Health status is unknown: not initialized or failed to retrieve.
        Unspecified = 0,
        /// The resource is healthy.
        Healthy = 1,
        /// The resource is unresponsive.
        Timeout = 3,
        /// The in-guest ML stack is unhealthy.
        UnhealthyTensorflow = 4,
        /// The node is under maintenance/priority boost caused rescheduling and
        /// will resume running once rescheduled.
        UnhealthyMaintenance = 5,
    }
    impl Health {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Health::Unspecified => "HEALTH_UNSPECIFIED",
                Health::Healthy => "HEALTHY",
                Health::Timeout => "TIMEOUT",
                Health::UnhealthyTensorflow => "UNHEALTHY_TENSORFLOW",
                Health::UnhealthyMaintenance => "UNHEALTHY_MAINTENANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HEALTH_UNSPECIFIED" => Some(Self::Unspecified),
                "HEALTHY" => Some(Self::Healthy),
                "TIMEOUT" => Some(Self::Timeout),
                "UNHEALTHY_TENSORFLOW" => Some(Self::UnhealthyTensorflow),
                "UNHEALTHY_MAINTENANCE" => Some(Self::UnhealthyMaintenance),
                _ => None,
            }
        }
    }
    /// TPU API Version.
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
        /// API version is unknown.
        Unspecified = 0,
        /// TPU API V1Alpha1 version.
        V1Alpha1 = 1,
        /// TPU API V1 version.
        V1 = 2,
        /// TPU API V2Alpha1 version.
        V2Alpha1 = 3,
    }
    impl ApiVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApiVersion::Unspecified => "API_VERSION_UNSPECIFIED",
                ApiVersion::V1Alpha1 => "V1_ALPHA1",
                ApiVersion::V1 => "V1",
                ApiVersion::V2Alpha1 => "V2_ALPHA1",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "API_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                "V1_ALPHA1" => Some(Self::V1Alpha1),
                "V1" => Some(Self::V1),
                "V2_ALPHA1" => Some(Self::V2Alpha1),
                _ => None,
            }
        }
    }
}
/// Request for \[ListNodes][google.cloud.tpu.v2alpha1.Tpu.ListNodes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for \[ListNodes][google.cloud.tpu.v2alpha1.Tpu.ListNodes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for \[GetNode][google.cloud.tpu.v2alpha1.Tpu.GetNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[CreateNode][google.cloud.tpu.v2alpha1.Tpu.CreateNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodeRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The unqualified resource name.
    #[prost(string, tag = "2")]
    pub node_id: ::prost::alloc::string::String,
    /// Required. The node.
    #[prost(message, optional, tag = "3")]
    pub node: ::core::option::Option<Node>,
}
/// Request for \[DeleteNode][google.cloud.tpu.v2alpha1.Tpu.DeleteNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[StopNode][google.cloud.tpu.v2alpha1.Tpu.StopNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopNodeRequest {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[StartNode][google.cloud.tpu.v2alpha1.Tpu.StartNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNodeRequest {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[UpdateNode][google.cloud.tpu.v2alpha1.Tpu.UpdateNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNodeRequest {
    /// Required. Mask of fields from \[Node][Tpu.Node\] to update.
    /// Supported fields: None.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The node. Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub node: ::core::option::Option<Node>,
}
/// The per-product per-project service identity for Cloud TPU service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceIdentity {
    /// The email address of the service identity.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
}
/// Request for
/// \[GenerateServiceIdentity][google.cloud.tpu.v2alpha1.Tpu.GenerateServiceIdentity\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateServiceIdentityRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response for
/// \[GenerateServiceIdentity][google.cloud.tpu.v2alpha1.Tpu.GenerateServiceIdentity\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateServiceIdentityResponse {
    /// ServiceIdentity that was created or retrieved.
    #[prost(message, optional, tag = "1")]
    pub identity: ::core::option::Option<ServiceIdentity>,
}
/// A accelerator type that a Node can be configured with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorType {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// the accelerator type.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
}
/// Request for
/// \[GetAcceleratorType][google.cloud.tpu.v2alpha1.Tpu.GetAcceleratorType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAcceleratorTypeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// \[ListAcceleratorTypes][google.cloud.tpu.v2alpha1.Tpu.ListAcceleratorTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAcceleratorTypesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// \[ListAcceleratorTypes][google.cloud.tpu.v2alpha1.Tpu.ListAcceleratorTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAcceleratorTypesResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub accelerator_types: ::prost::alloc::vec::Vec<AcceleratorType>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata describing an \[Operation][google.longrunning.Operation\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Target of the operation - for example
    /// projects/project-1/connectivityTests/test-1
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Specifies if cancellation was requested for the operation.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// A runtime version that a Node can be configured with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeVersion {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The runtime version.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Request for
/// \[GetRuntimeVersion][google.cloud.tpu.v2alpha1.Tpu.GetRuntimeVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuntimeVersionRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// \[ListRuntimeVersions][google.cloud.tpu.v2alpha1.Tpu.ListRuntimeVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimeVersionsRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// \[ListRuntimeVersions][google.cloud.tpu.v2alpha1.Tpu.ListRuntimeVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimeVersionsResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub runtime_versions: ::prost::alloc::vec::Vec<RuntimeVersion>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Symptom instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symptom {
    /// Timestamp when the Symptom is created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the Symptom.
    #[prost(enumeration = "symptom::SymptomType", tag = "2")]
    pub symptom_type: i32,
    /// Detailed information of the current Symptom.
    #[prost(string, tag = "3")]
    pub details: ::prost::alloc::string::String,
    /// A string used to uniquely distinguish a worker within a TPU node.
    #[prost(string, tag = "4")]
    pub worker_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Symptom`.
pub mod symptom {
    /// SymptomType represents the different types of Symptoms that a TPU can be
    /// at.
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
    pub enum SymptomType {
        /// Unspecified symptom.
        Unspecified = 0,
        /// TPU VM memory is low.
        LowMemory = 1,
        /// TPU runtime is out of memory.
        OutOfMemory = 2,
        /// TPU runtime execution has timed out.
        ExecuteTimedOut = 3,
        /// TPU runtime fails to construct a mesh that recognizes each TPU device's
        /// neighbors.
        MeshBuildFail = 4,
        /// TPU HBM is out of memory.
        HbmOutOfMemory = 5,
        /// Abusive behaviors have been identified on the current project.
        ProjectAbuse = 6,
    }
    impl SymptomType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SymptomType::Unspecified => "SYMPTOM_TYPE_UNSPECIFIED",
                SymptomType::LowMemory => "LOW_MEMORY",
                SymptomType::OutOfMemory => "OUT_OF_MEMORY",
                SymptomType::ExecuteTimedOut => "EXECUTE_TIMED_OUT",
                SymptomType::MeshBuildFail => "MESH_BUILD_FAIL",
                SymptomType::HbmOutOfMemory => "HBM_OUT_OF_MEMORY",
                SymptomType::ProjectAbuse => "PROJECT_ABUSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYMPTOM_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "LOW_MEMORY" => Some(Self::LowMemory),
                "OUT_OF_MEMORY" => Some(Self::OutOfMemory),
                "EXECUTE_TIMED_OUT" => Some(Self::ExecuteTimedOut),
                "MESH_BUILD_FAIL" => Some(Self::MeshBuildFail),
                "HBM_OUT_OF_MEMORY" => Some(Self::HbmOutOfMemory),
                "PROJECT_ABUSE" => Some(Self::ProjectAbuse),
                _ => None,
            }
        }
    }
}
/// Request for
/// \[GetGuestAttributes][google.cloud.tpu.v2alpha1.Tpu.GetGuestAttributes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuestAttributesRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The guest attributes path to be queried.
    #[prost(string, tag = "2")]
    pub query_path: ::prost::alloc::string::String,
    /// The 0-based worker ID. If it is empty, all workers' GuestAttributes will be
    /// returned.
    #[prost(string, repeated, tag = "3")]
    pub worker_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response for
/// \[GetGuestAttributes][google.cloud.tpu.v2alpha1.Tpu.GetGuestAttributes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuestAttributesResponse {
    /// The guest attributes for the TPU workers.
    #[prost(message, repeated, tag = "1")]
    pub guest_attributes: ::prost::alloc::vec::Vec<GuestAttributes>,
}
/// Generated client implementations.
pub mod tpu_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages TPU nodes and other resources
    ///
    /// TPU API v2alpha1
    #[derive(Debug, Clone)]
    pub struct TpuClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TpuClient<T>
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
        ) -> TpuClient<InterceptedService<T, F>>
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
            TpuClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists nodes.
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNodesResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/ListNodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "ListNodes"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a node.
        pub async fn get_node(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeRequest>,
        ) -> std::result::Result<tonic::Response<super::Node>, tonic::Status> {
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "GetNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a node.
        pub async fn create_node(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/CreateNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "CreateNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a node.
        pub async fn delete_node(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/DeleteNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "DeleteNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Stops a node. This operation is only available with single TPU nodes.
        pub async fn stop_node(
            &mut self,
            request: impl tonic::IntoRequest<super::StopNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/StopNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "StopNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Starts a node.
        pub async fn start_node(
            &mut self,
            request: impl tonic::IntoRequest<super::StartNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/StartNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "StartNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates the configurations of a node.
        pub async fn update_node(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/UpdateNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "UpdateNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Generates the Cloud TPU service identity for the project.
        pub async fn generate_service_identity(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateServiceIdentityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateServiceIdentityResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/GenerateServiceIdentity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "GenerateServiceIdentity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists accelerator types supported by this API.
        pub async fn list_accelerator_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAcceleratorTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAcceleratorTypesResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/ListAcceleratorTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "ListAcceleratorTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets AcceleratorType.
        pub async fn get_accelerator_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAcceleratorTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceleratorType>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetAcceleratorType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "GetAcceleratorType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists runtime versions supported by this API.
        pub async fn list_runtime_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuntimeVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRuntimeVersionsResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/ListRuntimeVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "ListRuntimeVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a runtime version.
        pub async fn get_runtime_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRuntimeVersionRequest>,
        ) -> std::result::Result<tonic::Response<super::RuntimeVersion>, tonic::Status> {
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetRuntimeVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "GetRuntimeVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the guest attributes for the node.
        pub async fn get_guest_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGuestAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGuestAttributesResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetGuestAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "GetGuestAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
