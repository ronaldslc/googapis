/// A message defining the database engine and provider.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseType {
    /// The database provider.
    #[prost(enumeration = "DatabaseProvider", tag = "1")]
    pub provider: i32,
    /// The database engine.
    #[prost(enumeration = "DatabaseEngine", tag = "2")]
    pub engine: i32,
}
/// Migration job as stored in Cloud Logging logs.
/// NEXT_TAG = 36.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggedMigrationJob {
    /// Required. The unique identifier for a migration job.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Labels.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The display name.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The current migration job state.
    #[prost(enumeration = "logged_migration_job::State", tag = "4")]
    pub state: i32,
    /// Required. The current migration job phase.
    #[prost(enumeration = "logged_migration_job::Phase", tag = "5")]
    pub phase: i32,
    /// Required. The migration job type.
    #[prost(enumeration = "logged_migration_job::Type", tag = "6")]
    pub r#type: i32,
    /// Optional. An optional dump path (gs://\[BUCKET_NAME]/[OBJECT_NAME\]).
    #[prost(string, tag = "7")]
    pub dump_path: ::prost::alloc::string::String,
    /// Required. The migration job source connection profile name.
    #[prost(string, tag = "8")]
    pub source: ::prost::alloc::string::String,
    /// Required. The migration job destination connection profile name.
    #[prost(string, tag = "9")]
    pub destination: ::prost::alloc::string::String,
    /// Required. the migration job duration in seconds.
    #[prost(message, optional, tag = "10")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Required. Type of connectivity to source database.
    #[prost(enumeration = "logged_migration_job::ConnectivityType", tag = "11")]
    pub connectivity_type: i32,
    /// Required. The error details in case of state FAILED.
    #[prost(message, optional, tag = "12")]
    pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Required. The time when this migration job was completed. Should only be set when the
    /// phase of the migration job is COMPLETED.
    #[prost(message, optional, tag = "13")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The indicative source database.
    #[prost(message, optional, tag = "14")]
    pub source_database: ::core::option::Option<DatabaseType>,
    /// Required. The indicative destination database.
    #[prost(message, optional, tag = "15")]
    pub destination_database: ::core::option::Option<DatabaseType>,
}
/// Nested message and enum types in `LoggedMigrationJob`.
pub mod logged_migration_job {
    /// The migration job states enum.
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
        /// The state of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is down for maintenance.
        Maintenance = 1,
        /// The migration job is in draft mode and fully editable.
        Draft = 2,
        /// The migration job is being created.
        Creating = 3,
        /// The migration job is created and not started.
        NotStarted = 4,
        /// The migration job is running
        Running = 5,
        /// The migration job failed.
        Failed = 6,
        /// The migration job has been completed.
        Completed = 7,
        /// The migration job is being deleted.
        Deleting = 8,
        /// The migration job is being stopped.
        Stopping = 9,
        /// The migration job is currently stopped.
        Stopped = 10,
        /// The migration job has been deleted.
        Deleted = 11,
        /// The migration job is being updated.
        Updating = 12,
        /// The migration job is starting.
        Starting = 13,
        /// The migration job is restarting.
        Restarting = 14,
        /// The migration job is resuming.
        Resuming = 15,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Maintenance => "MAINTENANCE",
                State::Draft => "DRAFT",
                State::Creating => "CREATING",
                State::NotStarted => "NOT_STARTED",
                State::Running => "RUNNING",
                State::Failed => "FAILED",
                State::Completed => "COMPLETED",
                State::Deleting => "DELETING",
                State::Stopping => "STOPPING",
                State::Stopped => "STOPPED",
                State::Deleted => "DELETED",
                State::Updating => "UPDATING",
                State::Starting => "STARTING",
                State::Restarting => "RESTARTING",
                State::Resuming => "RESUMING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "MAINTENANCE" => Some(Self::Maintenance),
                "DRAFT" => Some(Self::Draft),
                "CREATING" => Some(Self::Creating),
                "NOT_STARTED" => Some(Self::NotStarted),
                "RUNNING" => Some(Self::Running),
                "FAILED" => Some(Self::Failed),
                "COMPLETED" => Some(Self::Completed),
                "DELETING" => Some(Self::Deleting),
                "STOPPING" => Some(Self::Stopping),
                "STOPPED" => Some(Self::Stopped),
                "DELETED" => Some(Self::Deleted),
                "UPDATING" => Some(Self::Updating),
                "STARTING" => Some(Self::Starting),
                "RESTARTING" => Some(Self::Restarting),
                "RESUMING" => Some(Self::Resuming),
                _ => None,
            }
        }
    }
    /// The migration job phases enum.
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
    pub enum Phase {
        /// The phase of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is in the full dump phase.
        FullDump = 1,
        /// The migration job is CDC phase.
        Cdc = 2,
        /// The migration job is running the promote phase.
        PromoteInProgress = 3,
        /// Only RDS flow - waiting for source writes to stop
        WaitingForSourceWritesToStop = 4,
        /// Only RDS flow - the sources writes stopped, waiting for dump to begin
        PreparingTheDump = 5,
    }
    impl Phase {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Phase::Unspecified => "PHASE_UNSPECIFIED",
                Phase::FullDump => "FULL_DUMP",
                Phase::Cdc => "CDC",
                Phase::PromoteInProgress => "PROMOTE_IN_PROGRESS",
                Phase::WaitingForSourceWritesToStop => {
                    "WAITING_FOR_SOURCE_WRITES_TO_STOP"
                }
                Phase::PreparingTheDump => "PREPARING_THE_DUMP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PHASE_UNSPECIFIED" => Some(Self::Unspecified),
                "FULL_DUMP" => Some(Self::FullDump),
                "CDC" => Some(Self::Cdc),
                "PROMOTE_IN_PROGRESS" => Some(Self::PromoteInProgress),
                "WAITING_FOR_SOURCE_WRITES_TO_STOP" => {
                    Some(Self::WaitingForSourceWritesToStop)
                }
                "PREPARING_THE_DUMP" => Some(Self::PreparingTheDump),
                _ => None,
            }
        }
    }
    /// The migration job types.
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
        /// The type of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is a one time migration.
        OneTime = 1,
        /// The migration job is a continuous migration.
        Continuous = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::OneTime => "ONE_TIME",
                Type::Continuous => "CONTINUOUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ONE_TIME" => Some(Self::OneTime),
                "CONTINUOUS" => Some(Self::Continuous),
                _ => None,
            }
        }
    }
    /// Type of connectivity to source database.
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
    pub enum ConnectivityType {
        /// No data defined.
        Unspecified = 0,
        /// Connect using static IO
        StaticIp = 1,
        /// Use reverse SSH connectivity.
        ReverseSsh = 2,
        /// Use VPC Peering connectivity.
        VpcPeering = 3,
    }
    impl ConnectivityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectivityType::Unspecified => "CONNECTIVITY_TYPE_UNSPECIFIED",
                ConnectivityType::StaticIp => "STATIC_IP",
                ConnectivityType::ReverseSsh => "REVERSE_SSH",
                ConnectivityType::VpcPeering => "VPC_PEERING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECTIVITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATIC_IP" => Some(Self::StaticIp),
                "REVERSE_SSH" => Some(Self::ReverseSsh),
                "VPC_PEERING" => Some(Self::VpcPeering),
                _ => None,
            }
        }
    }
}
/// An MySQL database connection profile.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MySqlConnectionProfile {
    /// The database version.
    #[prost(enumeration = "my_sql_connection_profile::Version", tag = "1")]
    pub version: i32,
    /// The Cloud SQL id for a Cloud SQL instance.
    #[prost(string, tag = "2")]
    pub cloud_sql_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MySqlConnectionProfile`.
pub mod my_sql_connection_profile {
    /// The MySQL database version.
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
    pub enum Version {
        /// Unspecified version.
        Unspecified = 0,
        /// MySQL 5.5.
        V55 = 1,
        /// MySQL 5.6.
        V56 = 2,
        /// MySQL 5.7.
        V57 = 3,
        /// MySQL 8.0.
        V80 = 4,
    }
    impl Version {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Version::Unspecified => "VERSION_UNSPECIFIED",
                Version::V55 => "V5_5",
                Version::V56 => "V5_6",
                Version::V57 => "V5_7",
                Version::V80 => "V8_0",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                "V5_5" => Some(Self::V55),
                "V5_6" => Some(Self::V56),
                "V5_7" => Some(Self::V57),
                "V8_0" => Some(Self::V80),
                _ => None,
            }
        }
    }
}
/// An PostgreSQL connection profile.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostgreSqlConnectionProfile {
    /// The database version.
    #[prost(enumeration = "postgre_sql_connection_profile::Version", tag = "1")]
    pub version: i32,
    /// The Cloud SQL id for a Cloud SQL instance.
    #[prost(string, tag = "2")]
    pub cloud_sql_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PostgreSqlConnectionProfile`.
pub mod postgre_sql_connection_profile {
    /// The PostgreSQL database version.
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
    pub enum Version {
        /// Unspecified version.
        Unspecified = 0,
        /// PostgreSQL 9.6.
        V96 = 1,
        /// PostgreSQL 11.
        V11 = 2,
        /// PostgreSQL 10.
        V10 = 3,
        /// PostgreSQL 12.
        V12 = 4,
        /// PostgreSQL 13.
        V13 = 5,
    }
    impl Version {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Version::Unspecified => "VERSION_UNSPECIFIED",
                Version::V96 => "V9_6",
                Version::V11 => "V11",
                Version::V10 => "V10",
                Version::V12 => "V12",
                Version::V13 => "V13",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                "V9_6" => Some(Self::V96),
                "V11" => Some(Self::V11),
                "V10" => Some(Self::V10),
                "V12" => Some(Self::V12),
                "V13" => Some(Self::V13),
                _ => None,
            }
        }
    }
}
/// A CloudSQL connection profile.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlConnectionProfile {
    /// The Cloud SQL id.
    #[prost(string, tag = "1")]
    pub cloud_sql_id: ::prost::alloc::string::String,
}
/// An producer connection profile definition.
/// NEXT_TAG = 18.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggedConnectionProfile {
    /// The unique identifier for a connection profile.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Labels.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The current connection profile state.
    #[prost(enumeration = "logged_connection_profile::State", tag = "3")]
    pub state: i32,
    /// The display name.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// The error details in case of state FAILED.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// The database provider.
    #[prost(enumeration = "DatabaseProvider", tag = "6")]
    pub provider: i32,
    /// The connection profile definition
    #[prost(
        oneof = "logged_connection_profile::ConnectionProfile",
        tags = "100, 101, 102"
    )]
    pub connection_profile: ::core::option::Option<
        logged_connection_profile::ConnectionProfile,
    >,
}
/// Nested message and enum types in `LoggedConnectionProfile`.
pub mod logged_connection_profile {
    /// The connection profile states enum.
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
        /// The state of the connection profile is unknown.
        Unspecified = 0,
        /// The connection profile is in draft mode and fully editable.
        Draft = 1,
        /// The connection profile is being created.
        Creating = 2,
        /// The connection profile is ready.
        Ready = 3,
        /// The connection profile is being updated.
        Updating = 4,
        /// The connection profile is being deleted.
        Deleting = 5,
        /// The connection profile has been deleted.
        Deleted = 6,
        /// The last action on the connection profile failed.
        Failed = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Draft => "DRAFT",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Deleted => "DELETED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DRAFT" => Some(Self::Draft),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "DELETED" => Some(Self::Deleted),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// The connection profile definition
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConnectionProfile {
        /// A MySQL database connection profile.
        #[prost(message, tag = "100")]
        Mysql(super::MySqlConnectionProfile),
        /// A PostgreSQL database connection profile.
        #[prost(message, tag = "101")]
        Postgresql(super::PostgreSqlConnectionProfile),
        /// A CloudSQL database connection profile.
        #[prost(message, tag = "102")]
        Cloudsql(super::CloudSqlConnectionProfile),
    }
}
/// Log definition for Migration Job event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationJobEventLog {
    /// The migration job resource
    #[prost(message, optional, tag = "1")]
    pub migration_job: ::core::option::Option<LoggedMigrationJob>,
    /// Timestamp of the event
    #[prost(message, optional, tag = "2")]
    pub occurrence_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Event code
    #[prost(int32, tag = "3")]
    pub code: i32,
    /// Event message
    #[prost(string, tag = "4")]
    pub text_message: ::prost::alloc::string::String,
    /// Original event data
    #[prost(oneof = "migration_job_event_log::OriginalCause", tags = "200, 201")]
    pub original_cause: ::core::option::Option<migration_job_event_log::OriginalCause>,
}
/// Nested message and enum types in `MigrationJobEventLog`.
pub mod migration_job_event_log {
    /// Original event data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OriginalCause {
        /// Original event code
        #[prost(int32, tag = "200")]
        OriginalCode(i32),
        /// Original event message
        #[prost(string, tag = "201")]
        OriginalMessage(::prost::alloc::string::String),
    }
}
/// Log definition for Connection Profile event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionProfileEventLog {
    /// The connection profilr resource
    #[prost(message, optional, tag = "1")]
    pub connection_profile: ::core::option::Option<LoggedConnectionProfile>,
    /// Timestamp of the event
    #[prost(message, optional, tag = "2")]
    pub occurrence_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Event code
    #[prost(int32, tag = "3")]
    pub code: i32,
    /// Event message
    #[prost(string, tag = "4")]
    pub text_message: ::prost::alloc::string::String,
    /// Original event data
    #[prost(oneof = "connection_profile_event_log::OriginalCause", tags = "200, 201")]
    pub original_cause: ::core::option::Option<
        connection_profile_event_log::OriginalCause,
    >,
}
/// Nested message and enum types in `ConnectionProfileEventLog`.
pub mod connection_profile_event_log {
    /// Original event data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OriginalCause {
        /// Original event code
        #[prost(int32, tag = "200")]
        OriginalCode(i32),
        /// Original event message
        #[prost(string, tag = "201")]
        OriginalMessage(::prost::alloc::string::String),
    }
}
/// The database engines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseEngine {
    /// The source database engine of the migration job is unknown.
    Unspecified = 0,
    /// The source engine is MySQL.
    Mysql = 1,
    /// The source engine is PostgreSQL.
    Postgresql = 2,
    /// The source engine is SQL Server.
    Sqlserver = 3,
}
impl DatabaseEngine {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseEngine::Unspecified => "DATABASE_ENGINE_UNSPECIFIED",
            DatabaseEngine::Mysql => "MYSQL",
            DatabaseEngine::Postgresql => "POSTGRESQL",
            DatabaseEngine::Sqlserver => "SQLSERVER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATABASE_ENGINE_UNSPECIFIED" => Some(Self::Unspecified),
            "MYSQL" => Some(Self::Mysql),
            "POSTGRESQL" => Some(Self::Postgresql),
            "SQLSERVER" => Some(Self::Sqlserver),
            _ => None,
        }
    }
}
/// The database providers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseProvider {
    /// The database provider is unknown.
    Unspecified = 0,
    /// CloudSQL runs the database.
    Cloudsql = 1,
    /// RDS runs the database.
    Rds = 2,
}
impl DatabaseProvider {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseProvider::Unspecified => "DATABASE_PROVIDER_UNSPECIFIED",
            DatabaseProvider::Cloudsql => "CLOUDSQL",
            DatabaseProvider::Rds => "RDS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATABASE_PROVIDER_UNSPECIFIED" => Some(Self::Unspecified),
            "CLOUDSQL" => Some(Self::Cloudsql),
            "RDS" => Some(Self::Rds),
            _ => None,
        }
    }
}
