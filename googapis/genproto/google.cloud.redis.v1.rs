/// Node specific properties.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    /// Output only. Node identifying string. e.g. 'node-0', 'node-1'
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Location of the node.
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
}
/// A Google Cloud Redis instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Required. Unique name of the resource in this scope including project and
    /// location using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///
    /// Note: Redis instances are managed and addressed at regional level so
    /// location_id here refers to a GCP region; however, users may choose which
    /// specific zone (or collection of zones for cross-zone instances) an instance
    /// should be provisioned in. Refer to \[location_id][google.cloud.redis.v1.Instance.location_id\] and
    /// \[alternative_location_id][google.cloud.redis.v1.Instance.alternative_location_id\] fields for more details.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An arbitrary and optional user-provided name for the instance.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Resource labels to represent user provided metadata
    #[prost(map = "string, string", tag = "3")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The zone where the instance will be provisioned. If not provided,
    /// the service will choose a zone from the specified region for the instance.
    /// For standard tier, additional nodes will be added across multiple zones for
    /// protection against zonal failures. If specified, at least one node will be
    /// provisioned in this zone.
    #[prost(string, tag = "4")]
    pub location_id: ::prost::alloc::string::String,
    /// Optional. If specified, at least one node will be provisioned in this zone
    /// in addition to the zone specified in location_id. Only applicable to
    /// standard tier. If provided, it must be a different zone from the one
    /// provided in \[location_id\]. Additional nodes beyond the first 2 will be
    /// placed in zones selected by the service.
    #[prost(string, tag = "5")]
    pub alternative_location_id: ::prost::alloc::string::String,
    /// Optional. The version of Redis software.
    /// If not provided, latest supported version will be used. Currently, the
    /// supported values are:
    ///
    ///   *   `REDIS_3_2` for Redis 3.2 compatibility
    ///   *   `REDIS_4_0` for Redis 4.0 compatibility (default)
    ///   *   `REDIS_5_0` for Redis 5.0 compatibility
    ///   *   `REDIS_6_X` for Redis 6.x compatibility
    #[prost(string, tag = "7")]
    pub redis_version: ::prost::alloc::string::String,
    /// Optional. For DIRECT_PEERING mode, the CIDR range of internal addresses
    /// that are reserved for this instance. Range must
    /// be unique and non-overlapping with existing subnets in an authorized
    /// network. For PRIVATE_SERVICE_ACCESS mode, the name of one allocated IP
    /// address ranges associated with this private service access connection.
    /// If not provided, the service will choose an unused /29 block, for
    /// example, 10.0.0.0/29 or 192.168.0.0/29.  For READ_REPLICAS_ENABLED
    /// the default block size is /28.
    #[prost(string, tag = "9")]
    pub reserved_ip_range: ::prost::alloc::string::String,
    /// Output only. Hostname or IP address of the exposed Redis endpoint used by
    /// clients to connect to the service.
    #[prost(string, tag = "10")]
    pub host: ::prost::alloc::string::String,
    /// Output only. The port number of the exposed Redis endpoint.
    #[prost(int32, tag = "11")]
    pub port: i32,
    /// Output only. The current zone where the Redis primary node is located. In
    /// basic tier, this will always be the same as \[location_id\]. In
    /// standard tier, this can be the zone of any node in the instance.
    #[prost(string, tag = "12")]
    pub current_location_id: ::prost::alloc::string::String,
    /// Output only. The time the instance was created.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of this instance.
    #[prost(enumeration = "instance::State", tag = "14")]
    pub state: i32,
    /// Output only. Additional information about the current status of this
    /// instance, if available.
    #[prost(string, tag = "15")]
    pub status_message: ::prost::alloc::string::String,
    /// Optional. Redis configuration parameters, according to
    /// <http://redis.io/topics/config.> Currently, the only supported parameters
    /// are:
    ///
    ///   Redis version 3.2 and newer:
    ///
    ///   *   maxmemory-policy
    ///   *   notify-keyspace-events
    ///
    ///   Redis version 4.0 and newer:
    ///
    ///   *   activedefrag
    ///   *   lfu-decay-time
    ///   *   lfu-log-factor
    ///   *   maxmemory-gb
    ///
    ///   Redis version 5.0 and newer:
    ///
    ///   *   stream-node-max-bytes
    ///   *   stream-node-max-entries
    #[prost(map = "string, string", tag = "16")]
    pub redis_configs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The service tier of the instance.
    #[prost(enumeration = "instance::Tier", tag = "17")]
    pub tier: i32,
    /// Required. Redis memory size in GiB.
    #[prost(int32, tag = "18")]
    pub memory_size_gb: i32,
    /// Optional. The full name of the Google Compute Engine
    /// \[network\](<https://cloud.google.com/vpc/docs/vpc>) to which the
    /// instance is connected. If left unspecified, the `default` network
    /// will be used.
    #[prost(string, tag = "20")]
    pub authorized_network: ::prost::alloc::string::String,
    /// Output only. Cloud IAM identity used by import / export operations to
    /// transfer data to/from Cloud Storage. Format is
    /// "serviceAccount:<service_account_email>". The value may change over time
    /// for a given instance so should be checked before each import/export
    /// operation.
    #[prost(string, tag = "21")]
    pub persistence_iam_identity: ::prost::alloc::string::String,
    /// Optional. The network connect mode of the Redis instance.
    /// If not provided, the connect mode defaults to DIRECT_PEERING.
    #[prost(enumeration = "instance::ConnectMode", tag = "22")]
    pub connect_mode: i32,
    /// Optional. The number of replica nodes. Valid range for standard tier
    /// is \[1-5\] and defaults to 1. Valid value for basic tier is 0 and defaults
    /// to 0.
    #[prost(int32, tag = "31")]
    pub replica_count: i32,
    /// Output only. Info per node.
    #[prost(message, repeated, tag = "32")]
    pub nodes: ::prost::alloc::vec::Vec<NodeInfo>,
    /// Output only. Hostname or IP address of the exposed readonly Redis
    /// endpoint. Standard tier only. Targets all healthy replica nodes in
    /// instance. Replication is asynchronous and replica nodes will exhibit some
    /// lag behind the primary. Write requests must target 'host'.
    #[prost(string, tag = "33")]
    pub read_endpoint: ::prost::alloc::string::String,
    /// Output only. The port number of the exposed readonly redis
    /// endpoint. Standard tier only. Write requests should target 'port'.
    #[prost(int32, tag = "34")]
    pub read_endpoint_port: i32,
    /// Optional. Read replica mode.
    #[prost(enumeration = "instance::ReadReplicasMode", tag = "35")]
    pub read_replicas_mode: i32,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Represents the different states of a Redis instance.
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
        /// Not set.
        Unspecified = 0,
        /// Redis instance is being created.
        Creating = 1,
        /// Redis instance has been created and is fully usable.
        Ready = 2,
        /// Redis instance configuration is being updated. Certain kinds of updates
        /// may cause the instance to become unusable while the update is in
        /// progress.
        Updating = 3,
        /// Redis instance is being deleted.
        Deleting = 4,
        /// Redis instance is being repaired and may be unusable.
        Repairing = 5,
        /// Maintenance is being performed on this Redis instance.
        Maintenance = 6,
        /// Redis instance is importing data (availability may be affected).
        Importing = 8,
        /// Redis instance is failing over (availability may be affected).
        FailingOver = 9,
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
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Repairing => "REPAIRING",
                State::Maintenance => "MAINTENANCE",
                State::Importing => "IMPORTING",
                State::FailingOver => "FAILING_OVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "REPAIRING" => Some(Self::Repairing),
                "MAINTENANCE" => Some(Self::Maintenance),
                "IMPORTING" => Some(Self::Importing),
                "FAILING_OVER" => Some(Self::FailingOver),
                _ => None,
            }
        }
    }
    /// Available service tiers to choose from
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
        /// Not set.
        Unspecified = 0,
        /// BASIC tier: standalone instance
        Basic = 1,
        /// STANDARD_HA tier: highly available primary/replica instances
        StandardHa = 3,
    }
    impl Tier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Tier::Unspecified => "TIER_UNSPECIFIED",
                Tier::Basic => "BASIC",
                Tier::StandardHa => "STANDARD_HA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIER_UNSPECIFIED" => Some(Self::Unspecified),
                "BASIC" => Some(Self::Basic),
                "STANDARD_HA" => Some(Self::StandardHa),
                _ => None,
            }
        }
    }
    /// Available connection modes.
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
    pub enum ConnectMode {
        /// Not set.
        Unspecified = 0,
        /// Connect via direct peering to the Memorystore for Redis hosted service.
        DirectPeering = 1,
        /// Connect your Memorystore for Redis instance using Private Service
        /// Access. Private services access provides an IP address range for multiple
        /// Google Cloud services, including Memorystore.
        PrivateServiceAccess = 2,
    }
    impl ConnectMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectMode::Unspecified => "CONNECT_MODE_UNSPECIFIED",
                ConnectMode::DirectPeering => "DIRECT_PEERING",
                ConnectMode::PrivateServiceAccess => "PRIVATE_SERVICE_ACCESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECT_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "DIRECT_PEERING" => Some(Self::DirectPeering),
                "PRIVATE_SERVICE_ACCESS" => Some(Self::PrivateServiceAccess),
                _ => None,
            }
        }
    }
    /// Read replicas mode.
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
    pub enum ReadReplicasMode {
        /// If not set, Memorystore Redis backend will pick the mode based on other fields in
        /// the request.
        Unspecified = 0,
        /// If disabled, read endpoint will not be provided and the instance cannot
        /// scale up or down the number of replicas.
        ReadReplicasDisabled = 1,
        /// If enabled, read endpoint will be provided and the instance can scale
        /// up and down the number of replicas.
        ReadReplicasEnabled = 2,
    }
    impl ReadReplicasMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReadReplicasMode::Unspecified => "READ_REPLICAS_MODE_UNSPECIFIED",
                ReadReplicasMode::ReadReplicasDisabled => "READ_REPLICAS_DISABLED",
                ReadReplicasMode::ReadReplicasEnabled => "READ_REPLICAS_ENABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "READ_REPLICAS_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "READ_REPLICAS_DISABLED" => Some(Self::ReadReplicasDisabled),
                "READ_REPLICAS_ENABLED" => Some(Self::ReadReplicasEnabled),
                _ => None,
            }
        }
    }
}
/// Request for \[ListInstances][google.cloud.redis.v1.CloudRedis.ListInstances\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The resource name of the instance location using the form:
    ///      `projects/{project_id}/locations/{location_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 1000 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// \[`next_page_token`][google.cloud.redis.v1.ListInstancesResponse.next_page_token\]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `next_page_token` value returned from a previous
    /// \[ListInstances][google.cloud.redis.v1.CloudRedis.ListInstances\] request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for \[ListInstances][google.cloud.redis.v1.CloudRedis.ListInstances\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of Redis instances in the project in the specified location,
    /// or across all locations.
    ///
    /// If the `location_id` in the parent field of the request is "-", all regions
    /// available to the project are queried, and the results aggregated.
    /// If in such an aggregated query a location is unavailable, a placeholder
    /// Redis entry is included in the response with the `name` field set to a
    /// value of the form
    /// `projects/{project_id}/locations/{location_id}/instances/`- and the
    /// `status` field set to ERROR and `status_message` field set to "location not
    /// available for ListInstances".
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for \[GetInstance][google.cloud.redis.v1.CloudRedis.GetInstance\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[CreateInstance][google.cloud.redis.v1.CloudRedis.CreateInstance\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The resource name of the instance location using the form:
    ///      `projects/{project_id}/locations/{location_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The logical name of the Redis instance in the customer project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-40 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project / location
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. A Redis \[Instance\] resource
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// Request for \[UpdateInstance][google.cloud.redis.v1.CloudRedis.UpdateInstance\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field may only include these
    /// fields from \[Instance][google.cloud.redis.v1.Instance\]:
    ///
    ///   *   `displayName`
    ///   *   `labels`
    ///   *   `memorySizeGb`
    ///   *   `redisConfig`
    ///   *   `replica_count`
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub instance: ::core::option::Option<Instance>,
}
/// Request for \[UpgradeInstance][google.cloud.redis.v1.CloudRedis.UpgradeInstance\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Specifies the target version of Redis software to upgrade to.
    #[prost(string, tag = "2")]
    pub redis_version: ::prost::alloc::string::String,
}
/// Request for \[DeleteInstance][google.cloud.redis.v1.CloudRedis.DeleteInstance\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The Cloud Storage location for the input content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Source data URI. (e.g. 'gs://my_bucket/my_object').
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// The input content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Required. Specify source location of input data
    #[prost(oneof = "input_config::Source", tags = "1")]
    pub source: ::core::option::Option<input_config::Source>,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// Required. Specify source location of input data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Google Cloud Storage location where input content is located.
        #[prost(message, tag = "1")]
        GcsSource(super::GcsSource),
    }
}
/// Request for \[Import][google.cloud.redis.v1.CloudRedis.ImportInstance\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Specify data to be imported.
    #[prost(message, optional, tag = "3")]
    pub input_config: ::core::option::Option<InputConfig>,
}
/// The Cloud Storage location for the output content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required. Data destination URI (e.g.
    /// 'gs://my_bucket/my_object'). Existing files will be overwritten.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// The output content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Required. Specify destination location of output data
    #[prost(oneof = "output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// Required. Specify destination location of output data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Google Cloud Storage destination for output content.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// Request for \[Export][google.cloud.redis.v1.CloudRedis.ExportInstance\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Specify data to be exported.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Request for \[Failover][google.cloud.redis.v1.CloudRedis.FailoverInstance\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailoverInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Available data protection modes that the user can choose. If it's
    /// unspecified, data protection mode will be LIMITED_DATA_LOSS by default.
    #[prost(enumeration = "failover_instance_request::DataProtectionMode", tag = "2")]
    pub data_protection_mode: i32,
}
/// Nested message and enum types in `FailoverInstanceRequest`.
pub mod failover_instance_request {
    /// Specifies different modes of operation in relation to the data retention.
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
    pub enum DataProtectionMode {
        /// Defaults to LIMITED_DATA_LOSS if a data protection mode is not
        /// specified.
        Unspecified = 0,
        /// Instance failover will be protected with data loss control. More
        /// specifically, the failover will only be performed if the current
        /// replication offset diff between primary and replica is under a certain
        /// threshold.
        LimitedDataLoss = 1,
        /// Instance failover will be performed without data loss control.
        ForceDataLoss = 2,
    }
    impl DataProtectionMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataProtectionMode::Unspecified => "DATA_PROTECTION_MODE_UNSPECIFIED",
                DataProtectionMode::LimitedDataLoss => "LIMITED_DATA_LOSS",
                DataProtectionMode::ForceDataLoss => "FORCE_DATA_LOSS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATA_PROTECTION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "LIMITED_DATA_LOSS" => Some(Self::LimitedDataLoss),
                "FORCE_DATA_LOSS" => Some(Self::ForceDataLoss),
                _ => None,
            }
        }
    }
}
/// Represents the v1 metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Creation timestamp.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End timestamp.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation target.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Operation verb.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Operation status details.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Specifies if cancellation was requested for the operation.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// This location metadata represents additional configuration options for a
/// given location where a Redis instance may be created. All fields are output
/// only. It is returned as content of the
/// `google.cloud.location.Location.metadata` field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// Output only. The set of available zones in the location. The map is keyed
    /// by the lowercase ID of each zone, as defined by GCE. These keys can be
    /// specified in `location_id` or `alternative_location_id` fields when
    /// creating a Redis instance.
    #[prost(map = "string, message", tag = "1")]
    pub available_zones: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ZoneMetadata,
    >,
}
/// Defines specific information for a particular zone. Currently empty and
/// reserved for future use only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZoneMetadata {}
/// Generated client implementations.
pub mod cloud_redis_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Configures and manages Cloud Memorystore for Redis instances
    ///
    /// Google Cloud Memorystore for Redis v1
    ///
    /// The `redis.googleapis.com` service implements the Google Cloud Memorystore
    /// for Redis API and defines the following resource model for managing Redis
    /// instances:
    /// * The service works with a collection of cloud projects, named: `/projects/*`
    /// * Each project has a collection of available locations, named: `/locations/*`
    /// * Each location has a collection of Redis instances, named: `/instances/*`
    /// * As such, Redis instances are resources of the form:
    ///   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///
    /// Note that location_id must be referring to a GCP `region`; for example:
    /// * `projects/redpepper-1290/locations/us-central1/instances/my-redis`
    #[derive(Debug, Clone)]
    pub struct CloudRedisClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudRedisClient<T>
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
        ) -> CloudRedisClient<InterceptedService<T, F>>
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
            CloudRedisClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all Redis instances owned by a project in either the specified
        /// location (region) or all locations.
        ///
        /// The location should have the following format:
        ///
        /// * `projects/{project_id}/locations/{location_id}`
        ///
        /// If `location_id` is specified as `-` (wildcard), then all regions
        /// available to the project are queried, and the results are aggregated.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstancesResponse>,
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
                "/google.cloud.redis.v1.CloudRedis/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.redis.v1.CloudRedis", "ListInstances"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a specific Redis instance.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::Instance>, tonic::Status> {
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
                "/google.cloud.redis.v1.CloudRedis/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.redis.v1.CloudRedis", "GetInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Redis instance based on the specified tier and memory size.
        ///
        /// By default, the instance is accessible from the project's
        /// [default network](https://cloud.google.com/vpc/docs/vpc).
        ///
        /// The creation is executed asynchronously and callers may check the returned
        /// operation to track its progress. Once the operation is completed the Redis
        /// instance will be fully functional. Completed longrunning.Operation will
        /// contain the new instance object in the response field.
        ///
        /// The returned operation is automatically deleted after a few hours, so there
        /// is no need to call DeleteOperation.
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/CreateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.redis.v1.CloudRedis", "CreateInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the metadata and configuration of a specific Redis instance.
        ///
        /// Completed longrunning.Operation will contain the new instance object
        /// in the response field. The returned operation is automatically deleted
        /// after a few hours, so there is no need to call DeleteOperation.
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/UpdateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.redis.v1.CloudRedis", "UpdateInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Upgrades Redis instance to the newer Redis version specified in the
        /// request.
        pub async fn upgrade_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/UpgradeInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.redis.v1.CloudRedis",
                        "UpgradeInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Import a Redis RDB snapshot file from Cloud Storage into a Redis instance.
        ///
        /// Redis may stop serving during this operation. Instance state will be
        /// IMPORTING for entire operation. When complete, the instance will contain
        /// only data from the imported file.
        ///
        /// The returned operation is automatically deleted after a few hours, so
        /// there is no need to call DeleteOperation.
        pub async fn import_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/ImportInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.redis.v1.CloudRedis", "ImportInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Export Redis instance data into a Redis RDB format file in Cloud Storage.
        ///
        /// Redis will continue serving during this operation.
        ///
        /// The returned operation is automatically deleted after a few hours, so
        /// there is no need to call DeleteOperation.
        pub async fn export_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/ExportInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.redis.v1.CloudRedis", "ExportInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Initiates a failover of the primary node to current replica node for a
        /// specific STANDARD tier Cloud Memorystore for Redis instance.
        pub async fn failover_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::FailoverInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/FailoverInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.redis.v1.CloudRedis",
                        "FailoverInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a specific Redis instance.  Instance stops serving and data is
        /// deleted.
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/DeleteInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.redis.v1.CloudRedis", "DeleteInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
