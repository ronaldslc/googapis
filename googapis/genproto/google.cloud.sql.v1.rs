/// An entry for an Access Control list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AclEntry {
    /// The allowlisted value for the access control list.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// The time when this access control entry expires in
    /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "2")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A label to identify this entry.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// This is always **sql#aclEntry**.
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
}
/// An Admin API warning message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiWarning {
    /// Code to uniquely identify the warning type.
    #[prost(enumeration = "api_warning::SqlApiWarningCode", tag = "1")]
    pub code: i32,
    /// The warning message.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// The region name for REGION_UNREACHABLE warning.
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ApiWarning`.
pub mod api_warning {
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
    pub enum SqlApiWarningCode {
        /// An unknown or unset warning type from Cloud SQL API.
        Unspecified = 0,
        /// Warning when one or more regions are not reachable.  The returned result
        /// set may be incomplete.
        RegionUnreachable = 1,
    }
    impl SqlApiWarningCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlApiWarningCode::Unspecified => "SQL_API_WARNING_CODE_UNSPECIFIED",
                SqlApiWarningCode::RegionUnreachable => "REGION_UNREACHABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_API_WARNING_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "REGION_UNREACHABLE" => Some(Self::RegionUnreachable),
                _ => None,
            }
        }
    }
}
/// We currently only support backup retention by specifying the number
/// of backups we will retain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRetentionSettings {
    /// The unit that 'retained_backups' represents.
    #[prost(enumeration = "backup_retention_settings::RetentionUnit", tag = "1")]
    pub retention_unit: i32,
    /// Depending on the value of retention_unit, this is used to determine
    /// if a backup needs to be deleted.  If retention_unit is 'COUNT', we will
    /// retain this many backups.
    #[prost(message, optional, tag = "2")]
    pub retained_backups: ::core::option::Option<i32>,
}
/// Nested message and enum types in `BackupRetentionSettings`.
pub mod backup_retention_settings {
    /// The units that retained_backups specifies, we only support COUNT.
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
    pub enum RetentionUnit {
        /// Backup retention unit is unspecified, will be treated as COUNT.
        Unspecified = 0,
        /// Retention will be by count, eg. "retain the most recent 7 backups".
        Count = 1,
    }
    impl RetentionUnit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RetentionUnit::Unspecified => "RETENTION_UNIT_UNSPECIFIED",
                RetentionUnit::Count => "COUNT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RETENTION_UNIT_UNSPECIFIED" => Some(Self::Unspecified),
                "COUNT" => Some(Self::Count),
                _ => None,
            }
        }
    }
}
/// Database instance backup configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupConfiguration {
    /// Start time for the daily backup configuration in UTC timezone in the 24
    /// hour format - **HH:MM**.
    #[prost(string, tag = "1")]
    pub start_time: ::prost::alloc::string::String,
    /// Whether this configuration is enabled.
    #[prost(message, optional, tag = "2")]
    pub enabled: ::core::option::Option<bool>,
    /// This is always **sql#backupConfiguration**.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    /// (MySQL only) Whether binary log is enabled. If backup configuration is
    /// disabled, binarylog must be disabled as well.
    #[prost(message, optional, tag = "4")]
    pub binary_log_enabled: ::core::option::Option<bool>,
    /// Reserved for future use.
    #[prost(message, optional, tag = "5")]
    pub replication_log_archiving_enabled: ::core::option::Option<bool>,
    /// Location of the backup
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// (Postgres only) Whether point in time recovery is enabled.
    #[prost(message, optional, tag = "7")]
    pub point_in_time_recovery_enabled: ::core::option::Option<bool>,
    /// Backup retention settings.
    #[prost(message, optional, tag = "8")]
    pub backup_retention_settings: ::core::option::Option<BackupRetentionSettings>,
    /// The number of days of transaction logs we retain for point in time
    /// restore, from 1-7.
    #[prost(message, optional, tag = "9")]
    pub transaction_log_retention_days: ::core::option::Option<i32>,
}
/// Backup context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupContext {
    /// The identifier of the backup.
    #[prost(int64, tag = "1")]
    pub backup_id: i64,
    /// This is always **sql#backupContext**.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// Represents a SQL database on the Cloud SQL instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// This is always **sql#database**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The Cloud SQL charset value.
    #[prost(string, tag = "2")]
    pub charset: ::prost::alloc::string::String,
    /// The Cloud SQL collation value.
    #[prost(string, tag = "3")]
    pub collation: ::prost::alloc::string::String,
    /// This field is deprecated and will be removed from a future version of the
    /// API.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
    /// The name of the database in the Cloud SQL instance. This does not include
    /// the project ID or instance name.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// The name of the Cloud SQL instance. This does not include the project ID.
    #[prost(string, tag = "6")]
    pub instance: ::prost::alloc::string::String,
    /// The URI of this resource.
    #[prost(string, tag = "7")]
    pub self_link: ::prost::alloc::string::String,
    /// The project ID of the project containing the Cloud SQL database. The Google
    /// apps domain is prefixed if applicable.
    #[prost(string, tag = "8")]
    pub project: ::prost::alloc::string::String,
    #[prost(oneof = "database::DatabaseDetails", tags = "9")]
    pub database_details: ::core::option::Option<database::DatabaseDetails>,
}
/// Nested message and enum types in `Database`.
pub mod database {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DatabaseDetails {
        #[prost(message, tag = "9")]
        SqlserverDatabaseDetails(super::SqlServerDatabaseDetails),
    }
}
/// Represents a Sql Server database on the Cloud SQL instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlServerDatabaseDetails {
    /// The version of SQL Server with which the database is to be made compatible
    #[prost(int32, tag = "1")]
    pub compatibility_level: i32,
    /// The recovery model of a SQL Server database
    #[prost(string, tag = "2")]
    pub recovery_model: ::prost::alloc::string::String,
}
/// Database flags for Cloud SQL instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseFlags {
    /// The name of the flag. These flags are passed at instance startup, so
    /// include both server options and system variables. Flags are
    /// specified with underscores, not hyphens. For more information, see
    /// [Configuring Database Flags](<https://cloud.google.com/sql/docs/mysql/flags>)
    /// in the Cloud SQL documentation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The value of the flag. Booleans are set to **on** for true
    /// and **off** for false. This field must be omitted if the flag
    /// doesn't take a value.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// MySQL-specific external server sync settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MySqlSyncConfig {
    /// Flags to use for the initial dump.
    #[prost(message, repeated, tag = "1")]
    pub initial_sync_flags: ::prost::alloc::vec::Vec<SyncFlags>,
}
/// Initial sync flags for certain Cloud SQL APIs.
/// Currently used for the MySQL external server initial dump.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncFlags {
    /// The name of the flag.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The value of the flag. This field must be omitted if the flag
    /// doesn't take a value.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Reference to another Cloud SQL instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceReference {
    /// The name of the Cloud SQL instance being referenced.
    /// This does not include the project ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The region of the Cloud SQL instance being referenced.
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
    /// The project ID of the Cloud SQL instance being referenced.
    /// The default is the same project ID as the instance references it.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
/// Read-replica configuration for connecting to the on-premises primary
/// instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemoteMasterConfiguration {
    /// This is always **sql#demoteMasterConfiguration**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// MySQL specific configuration when replicating from a MySQL on-premises
    /// primary instance. Replication configuration information such as the
    /// username, password, certificates, and keys are not stored in the instance
    /// metadata. The configuration information is used only to set up the
    /// replication connection and is stored by MySQL in a file named
    /// **master.info** in the data directory.
    #[prost(message, optional, tag = "2")]
    pub mysql_replica_configuration: ::core::option::Option<
        DemoteMasterMySqlReplicaConfiguration,
    >,
}
/// Read-replica configuration specific to MySQL databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemoteMasterMySqlReplicaConfiguration {
    /// This is always **sql#demoteMasterMysqlReplicaConfiguration**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The username for the replication connection.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// The password for the replication connection.
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
    /// PEM representation of the replica's private key. The corresponsing public
    /// key is encoded in the client's certificate. The format of the replica's
    /// private key can be either PKCS #1 or PKCS #8.
    #[prost(string, tag = "4")]
    pub client_key: ::prost::alloc::string::String,
    /// PEM representation of the replica's x509 certificate.
    #[prost(string, tag = "5")]
    pub client_certificate: ::prost::alloc::string::String,
    /// PEM representation of the trusted CA's x509 certificate.
    #[prost(string, tag = "6")]
    pub ca_certificate: ::prost::alloc::string::String,
}
/// Database instance export context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportContext {
    /// The path to the file in Google Cloud Storage where the export will be
    /// stored. The URI is in the form **gs://bucketName/fileName**. If the file
    /// already exists, the request succeeds, but the operation fails. If
    /// **fileType** is **SQL** and the filename ends with .gz,
    /// the contents are compressed.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Databases to be exported.
    /// *  **MySQL instances:** If **fileType** is **SQL** and no database is
    /// specified, all databases are exported, except for the **mysql** system
    /// database. If **fileType** is **CSV**, you can specify one database,
    /// either by using this property or by using the
    /// **csvExportOptions.selectQuery** property, which takes precedence
    /// over this property.
    /// *  **PostgreSQL instances:** You must specify one database to be exported.
    /// If **fileType** is **CSV**, this database must match the one specified in
    /// the **csvExportOptions.selectQuery** property.
    /// *  **SQL Server instances:** You must specify one database to be exported,
    /// and the **fileType** must be **BAK**.
    #[prost(string, repeated, tag = "2")]
    pub databases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This is always **sql#exportContext**.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    /// Options for exporting data as SQL statements.
    #[prost(message, optional, tag = "4")]
    pub sql_export_options: ::core::option::Option<export_context::SqlExportOptions>,
    /// Options for exporting data as CSV. **MySQL** and **PostgreSQL**
    /// instances only.
    #[prost(message, optional, tag = "5")]
    pub csv_export_options: ::core::option::Option<export_context::SqlCsvExportOptions>,
    /// The file type for the specified uri.
    /// *  **SQL**: The file contains SQL statements.
    /// *  **CSV**: The file contains CSV data.
    /// *  **BAK**: The file contains backup data for a SQL Server instance.
    #[prost(enumeration = "SqlFileType", tag = "6")]
    pub file_type: i32,
    /// Option for export offload.
    #[prost(message, optional, tag = "8")]
    pub offload: ::core::option::Option<bool>,
}
/// Nested message and enum types in `ExportContext`.
pub mod export_context {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlCsvExportOptions {
        /// The select query used to extract the data.
        #[prost(string, tag = "1")]
        pub select_query: ::prost::alloc::string::String,
        /// Specifies the character that should appear before a data character that
        /// needs to be escaped.
        #[prost(string, tag = "2")]
        pub escape_character: ::prost::alloc::string::String,
        /// Specifies the quoting character to be used when a data value is quoted.
        #[prost(string, tag = "3")]
        pub quote_character: ::prost::alloc::string::String,
        /// Specifies the character that separates columns within each row (line) of
        /// the file.
        #[prost(string, tag = "4")]
        pub fields_terminated_by: ::prost::alloc::string::String,
        /// This is used to separate lines. If a line does not contain all fields,
        /// the rest of the columns are set to their default values.
        #[prost(string, tag = "6")]
        pub lines_terminated_by: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlExportOptions {
        /// Tables to export, or that were exported, from the specified database. If
        /// you specify tables, specify one and only one database. For PostgreSQL
        /// instances, you can specify only one table.
        #[prost(string, repeated, tag = "1")]
        pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Export only schemas.
        #[prost(message, optional, tag = "2")]
        pub schema_only: ::core::option::Option<bool>,
        #[prost(message, optional, tag = "3")]
        pub mysql_export_options: ::core::option::Option<
            sql_export_options::MysqlExportOptions,
        >,
    }
    /// Nested message and enum types in `SqlExportOptions`.
    pub mod sql_export_options {
        /// Options for exporting from MySQL.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MysqlExportOptions {
            /// Option to include SQL statement required to set up replication.
            /// *  If set to **1**, the dump file includes
            ///   a CHANGE MASTER TO statement with the binary log coordinates,
            ///   and --set-gtid-purged is set to ON.
            /// *  If set to **2**, the CHANGE MASTER TO statement is written as
            ///   a SQL comment and has no effect.
            /// *  If set to any value other than **1**, --set-gtid-purged is set
            /// to OFF.
            #[prost(message, optional, tag = "1")]
            pub master_data: ::core::option::Option<i32>,
        }
    }
}
/// Database instance import context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportContext {
    /// Path to the import file in Cloud Storage, in the form
    /// **gs://bucketName/fileName**. Compressed gzip files (.gz) are supported
    /// when **fileType** is **SQL**. The instance must have
    /// write permissions to the bucket and read access to the file.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// The target database for the import. If **fileType** is **SQL**, this field
    /// is required only if the import file does not specify a database, and is
    /// overridden by any database specification in the import file. If
    /// **fileType** is **CSV**, one database must be specified.
    #[prost(string, tag = "2")]
    pub database: ::prost::alloc::string::String,
    /// This is always **sql#importContext**.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    /// The file type for the specified uri. <br>**SQL**: The file
    /// contains SQL statements. <br>**CSV**: The file contains CSV data.
    #[prost(enumeration = "SqlFileType", tag = "4")]
    pub file_type: i32,
    /// Options for importing data as CSV.
    #[prost(message, optional, tag = "5")]
    pub csv_import_options: ::core::option::Option<import_context::SqlCsvImportOptions>,
    /// The PostgreSQL user for this import operation. PostgreSQL instances only.
    #[prost(string, tag = "6")]
    pub import_user: ::prost::alloc::string::String,
    /// Import parameters specific to SQL Server .BAK files
    #[prost(message, optional, tag = "7")]
    pub bak_import_options: ::core::option::Option<import_context::SqlBakImportOptions>,
}
/// Nested message and enum types in `ImportContext`.
pub mod import_context {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlCsvImportOptions {
        /// The table to which CSV data is imported.
        #[prost(string, tag = "1")]
        pub table: ::prost::alloc::string::String,
        /// The columns to which CSV data is imported. If not specified, all columns
        /// of the database table are loaded with CSV data.
        #[prost(string, repeated, tag = "2")]
        pub columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies the character that should appear before a data character that
        /// needs to be escaped.
        #[prost(string, tag = "4")]
        pub escape_character: ::prost::alloc::string::String,
        /// Specifies the quoting character to be used when a data value is quoted.
        #[prost(string, tag = "5")]
        pub quote_character: ::prost::alloc::string::String,
        /// Specifies the character that separates columns within each row (line) of
        /// the file.
        #[prost(string, tag = "6")]
        pub fields_terminated_by: ::prost::alloc::string::String,
        /// This is used to separate lines. If a line does not contain all fields,
        /// the rest of the columns are set to their default values.
        #[prost(string, tag = "8")]
        pub lines_terminated_by: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlBakImportOptions {
        #[prost(message, optional, tag = "1")]
        pub encryption_options: ::core::option::Option<
            sql_bak_import_options::EncryptionOptions,
        >,
    }
    /// Nested message and enum types in `SqlBakImportOptions`.
    pub mod sql_bak_import_options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EncryptionOptions {
            /// Path to the Certificate (.cer) in Cloud Storage, in the form
            /// **gs://bucketName/fileName**. The instance must have
            /// write permissions to the bucket and read access to the file.
            #[prost(string, tag = "1")]
            pub cert_path: ::prost::alloc::string::String,
            /// Path to the Certificate Private Key (.pvk)  in Cloud Storage, in the
            /// form **gs://bucketName/fileName**. The instance must have
            /// write permissions to the bucket and read access to the file.
            #[prost(string, tag = "2")]
            pub pvk_path: ::prost::alloc::string::String,
            /// Password that encrypts the private key
            #[prost(string, tag = "3")]
            pub pvk_password: ::prost::alloc::string::String,
        }
    }
}
/// IP Management configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpConfiguration {
    /// Whether the instance is assigned a public IP address or not.
    #[prost(message, optional, tag = "1")]
    pub ipv4_enabled: ::core::option::Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is
    /// accessible for private IP. For example,
    /// **/projects/myProject/global/networks/default**. This setting can
    /// be updated, but it cannot be removed after it is set.
    #[prost(string, tag = "2")]
    pub private_network: ::prost::alloc::string::String,
    /// Whether SSL connections over IP are enforced or not.
    #[prost(message, optional, tag = "3")]
    pub require_ssl: ::core::option::Option<bool>,
    /// The list of external networks that are allowed to connect to the instance
    /// using the IP. In 'CIDR' notation, also known as 'slash' notation (for
    /// example: **192.168.100.0/24**).
    #[prost(message, repeated, tag = "4")]
    pub authorized_networks: ::prost::alloc::vec::Vec<AclEntry>,
    /// The name of the allocated ip range for the private ip CloudSQL instance.
    /// For example: "google-managed-services-default". If set, the instance ip
    /// will be created in the allocated range. The range name must comply with
    /// [RFC 1035](<https://tools.ietf.org/html/rfc1035>). Specifically, the name
    /// must be 1-63 characters long and match the regular expression
    /// `\[a-z]([-a-z0-9]*[a-z0-9\])?.`
    /// Reserved for future use.
    #[prost(string, tag = "6")]
    pub allocated_ip_range: ::prost::alloc::string::String,
}
/// Preferred location. This specifies where a Cloud SQL instance is located.
/// Note that if the preferred location is not available, the instance will be
/// located as close as possible within the region. Only one location may be
/// specified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationPreference {
    /// The App Engine application to follow, it must be in the same region as the
    /// Cloud SQL instance.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub follow_gae_application: ::prost::alloc::string::String,
    /// The preferred Compute Engine zone (for example: us-central1-a,
    /// us-central1-b, etc.).
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// The preferred Compute Engine zone for the secondary/failover
    /// (for example: us-central1-a, us-central1-b, etc.).
    /// Reserved for future use.
    #[prost(string, tag = "4")]
    pub secondary_zone: ::prost::alloc::string::String,
    /// This is always **sql#locationPreference**.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
}
/// Maintenance window. This specifies when a Cloud SQL instance is
/// restarted for system maintenance purposes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceWindow {
    /// hour of day - 0 to 23.
    #[prost(message, optional, tag = "1")]
    pub hour: ::core::option::Option<i32>,
    /// day of week (1-7), starting on Monday.
    #[prost(message, optional, tag = "2")]
    pub day: ::core::option::Option<i32>,
    /// Maintenance timing setting: **canary** (Earlier) or **stable** (Later).
    /// [Learn
    /// more](<https://cloud.google.com/sql/docs/mysql/instance-settings#maintenance-timing-2ndgen>).
    #[prost(enumeration = "SqlUpdateTrack", tag = "3")]
    pub update_track: i32,
    /// This is always **sql#maintenanceWindow**.
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
}
/// Deny maintenance Periods. This specifies a date range during when all CSA
/// rollout will be denied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenyMaintenancePeriod {
    /// "deny maintenance period" start date. If the year of the start date is
    /// empty, the year of the end date also must be empty. In this case, it means
    /// the deny maintenance period recurs every year. The date is in format
    /// yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01
    #[prost(string, tag = "1")]
    pub start_date: ::prost::alloc::string::String,
    /// "deny maintenance period" end date. If the year of the end date is empty,
    /// the year of the start date also must be empty. In this case, it means the
    /// no maintenance interval recurs every year. The date is in format yyyy-mm-dd
    /// i.e., 2020-11-01, or mm-dd, i.e., 11-01
    #[prost(string, tag = "2")]
    pub end_date: ::prost::alloc::string::String,
    /// Time in UTC when the "deny maintenance period" starts on start_date and
    /// ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00
    #[prost(string, tag = "3")]
    pub time: ::prost::alloc::string::String,
}
/// Insights configuration. This specifies when Cloud SQL Insights feature is
/// enabled and optional configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsightsConfig {
    /// Whether Query Insights feature is enabled.
    #[prost(bool, tag = "1")]
    pub query_insights_enabled: bool,
    /// Whether Query Insights will record client address when enabled.
    #[prost(bool, tag = "2")]
    pub record_client_address: bool,
    /// Whether Query Insights will record application tags from query when
    /// enabled.
    #[prost(bool, tag = "3")]
    pub record_application_tags: bool,
    /// Maximum query length stored in bytes. Default value: 1024 bytes.
    /// Range: 256-4500 bytes. Query length more than this field value will be
    /// truncated to this value. When unset, query length will be the default
    /// value. Changing query length will restart the database.
    #[prost(message, optional, tag = "4")]
    pub query_string_length: ::core::option::Option<i32>,
    /// Number of query execution plans captured by Insights per minute
    /// for all queries combined. Default is 5.
    #[prost(message, optional, tag = "5")]
    pub query_plans_per_minute: ::core::option::Option<i32>,
}
/// Read-replica configuration specific to MySQL databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MySqlReplicaConfiguration {
    /// Path to a SQL dump file in Google Cloud Storage from which the replica
    /// instance is to be created. The URI is in the form gs://bucketName/fileName.
    /// Compressed gzip files (.gz) are also supported.
    /// Dumps have the binlog co-ordinates from which replication
    /// begins. This can be accomplished by setting --master-data to 1 when using
    /// mysqldump.
    #[prost(string, tag = "1")]
    pub dump_file_path: ::prost::alloc::string::String,
    /// The username for the replication connection.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// The password for the replication connection.
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
    /// Seconds to wait between connect retries. MySQL's default is 60 seconds.
    #[prost(message, optional, tag = "4")]
    pub connect_retry_interval: ::core::option::Option<i32>,
    /// Interval in milliseconds between replication heartbeats.
    #[prost(message, optional, tag = "5")]
    pub master_heartbeat_period: ::core::option::Option<i64>,
    /// PEM representation of the trusted CA's x509 certificate.
    #[prost(string, tag = "6")]
    pub ca_certificate: ::prost::alloc::string::String,
    /// PEM representation of the replica's x509 certificate.
    #[prost(string, tag = "7")]
    pub client_certificate: ::prost::alloc::string::String,
    /// PEM representation of the replica's private key. The corresponsing public
    /// key is encoded in the client's certificate.
    #[prost(string, tag = "8")]
    pub client_key: ::prost::alloc::string::String,
    /// A list of permissible ciphers to use for SSL encryption.
    #[prost(string, tag = "9")]
    pub ssl_cipher: ::prost::alloc::string::String,
    /// Whether or not to check the primary instance's Common Name value in the
    /// certificate that it sends during the SSL handshake.
    #[prost(message, optional, tag = "10")]
    pub verify_server_certificate: ::core::option::Option<bool>,
    /// This is always **sql#mysqlReplicaConfiguration**.
    #[prost(string, tag = "11")]
    pub kind: ::prost::alloc::string::String,
}
/// Disk encryption configuration for an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskEncryptionConfiguration {
    /// Resource name of KMS key for disk encryption
    #[prost(string, tag = "1")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// This is always **sql#diskEncryptionConfiguration**.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// Disk encryption status for an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskEncryptionStatus {
    /// KMS key version used to encrypt the Cloud SQL instance resource
    #[prost(string, tag = "1")]
    pub kms_key_version_name: ::prost::alloc::string::String,
    /// This is always **sql#diskEncryptionStatus**.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// Database instance IP Mapping.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpMapping {
    /// The type of this IP address. A **PRIMARY** address is a public address that
    /// can accept incoming connections. A **PRIVATE** address is a private address
    /// that can accept incoming connections. An **OUTGOING** address is the source
    /// address of connections originating from the instance, if supported.
    #[prost(enumeration = "SqlIpAddressType", tag = "1")]
    pub r#type: i32,
    /// The IP address assigned.
    #[prost(string, tag = "2")]
    pub ip_address: ::prost::alloc::string::String,
    /// The due time for this IP to be retired in
    /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**. This field is only available when
    /// the IP is scheduled to be retired.
    #[prost(message, optional, tag = "3")]
    pub time_to_retire: ::core::option::Option<::prost_types::Timestamp>,
}
/// An Operation resource.&nbsp;For successful operations that return an
/// Operation resource, only the fields relevant to the operation are populated
/// in the resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// This is always **sql#operation**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_link: ::prost::alloc::string::String,
    /// The status of an operation. Valid values are:
    /// *  **PENDING**
    /// *  **RUNNING**
    /// *  **DONE**
    /// *  **SQL_OPERATION_STATUS_UNSPECIFIED**
    #[prost(enumeration = "operation::SqlOperationStatus", tag = "3")]
    pub status: i32,
    /// The email address of the user who initiated this operation.
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
    /// The time this operation was enqueued in UTC timezone in [RFC
    /// 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "5")]
    pub insert_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time this operation actually started in UTC timezone in [RFC
    /// 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time this operation finished in UTC timezone in [RFC
    /// 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If errors occurred during processing of this operation, this field will be
    /// populated.
    #[prost(message, optional, tag = "8")]
    pub error: ::core::option::Option<OperationErrors>,
    /// The type of the operation. Valid values are:
    /// *  **CREATE**
    /// *  **DELETE**
    /// *  **UPDATE**
    /// *  **RESTART**
    /// *  **IMPORT**
    /// *  **EXPORT**
    /// *  **BACKUP_VOLUME**
    /// *  **RESTORE_VOLUME**
    /// *  **CREATE_USER**
    /// *  **DELETE_USER**
    /// *  **CREATE_DATABASE**
    /// *  **DELETE_DATABASE**
    #[prost(enumeration = "operation::SqlOperationType", tag = "9")]
    pub operation_type: i32,
    /// The context for import operation, if applicable.
    #[prost(message, optional, tag = "10")]
    pub import_context: ::core::option::Option<ImportContext>,
    /// The context for export operation, if applicable.
    #[prost(message, optional, tag = "11")]
    pub export_context: ::core::option::Option<ExportContext>,
    /// The context for backup operation, if applicable.
    #[prost(message, optional, tag = "17")]
    pub backup_context: ::core::option::Option<BackupContext>,
    /// An identifier that uniquely identifies the operation. You can use this
    /// identifier to retrieve the Operations resource that has information about
    /// the operation.
    #[prost(string, tag = "12")]
    pub name: ::prost::alloc::string::String,
    /// Name of the database instance related to this operation.
    #[prost(string, tag = "13")]
    pub target_id: ::prost::alloc::string::String,
    /// The URI of this resource.
    #[prost(string, tag = "14")]
    pub self_link: ::prost::alloc::string::String,
    /// The project ID of the target instance related to this operation.
    #[prost(string, tag = "15")]
    pub target_project: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    /// The type of Cloud SQL operation.
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
    pub enum SqlOperationType {
        /// Unknown operation type.
        Unspecified = 0,
        /// Imports data into a Cloud SQL instance.
        Import = 1,
        /// Exports data from a Cloud SQL instance to a Cloud Storage
        /// bucket.
        Export = 2,
        /// Creates a new Cloud SQL instance.
        Create = 3,
        /// Updates the settings of a Cloud SQL instance.
        Update = 4,
        /// Deletes a Cloud SQL instance.
        Delete = 5,
        /// Restarts the Cloud SQL instance.
        Restart = 6,
        Backup = 7,
        Snapshot = 8,
        /// Performs instance backup.
        BackupVolume = 9,
        /// Deletes an instance backup.
        DeleteVolume = 10,
        /// Restores an instance backup.
        RestoreVolume = 11,
        /// Injects a privileged user in mysql for MOB instances.
        InjectUser = 12,
        /// Clones a Cloud SQL instance.
        Clone = 14,
        /// Stops replication on a Cloud SQL read replica instance.
        StopReplica = 15,
        /// Starts replication on a Cloud SQL read replica instance.
        StartReplica = 16,
        /// Promotes a Cloud SQL replica instance.
        PromoteReplica = 17,
        /// Creates a Cloud SQL replica instance.
        CreateReplica = 18,
        /// Creates a new user in a Cloud SQL instance.
        CreateUser = 19,
        /// Deletes a user from a Cloud SQL instance.
        DeleteUser = 20,
        /// Updates an existing user in a Cloud SQL instance.
        UpdateUser = 21,
        /// Creates a database in the Cloud SQL instance.
        CreateDatabase = 22,
        /// Deletes a database in the Cloud SQL instance.
        DeleteDatabase = 23,
        /// Updates a database in the Cloud SQL instance.
        UpdateDatabase = 24,
        /// Performs failover of an HA-enabled Cloud SQL
        /// failover replica.
        Failover = 25,
        /// Deletes the backup taken by a backup run.
        DeleteBackup = 26,
        RecreateReplica = 27,
        /// Truncates a general or slow log table in MySQL.
        TruncateLog = 28,
        /// Demotes the stand-alone instance to be a Cloud SQL
        /// read replica for an external database server.
        DemoteMaster = 29,
        /// Indicates that the instance is currently in maintenance. Maintenance
        /// typically causes the instance to be unavailable for 1-3 minutes.
        Maintenance = 30,
        /// This field is deprecated, and will be removed in future version of API.
        EnablePrivateIp = 31,
        DeferMaintenance = 32,
        /// Creates clone instance.
        CreateClone = 33,
        /// Reschedule maintenance to another time.
        RescheduleMaintenance = 34,
        /// Starts external sync of a Cloud SQL EM replica to an external primary
        /// instance.
        StartExternalSync = 35,
    }
    impl SqlOperationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlOperationType::Unspecified => "SQL_OPERATION_TYPE_UNSPECIFIED",
                SqlOperationType::Import => "IMPORT",
                SqlOperationType::Export => "EXPORT",
                SqlOperationType::Create => "CREATE",
                SqlOperationType::Update => "UPDATE",
                SqlOperationType::Delete => "DELETE",
                SqlOperationType::Restart => "RESTART",
                SqlOperationType::Backup => "BACKUP",
                SqlOperationType::Snapshot => "SNAPSHOT",
                SqlOperationType::BackupVolume => "BACKUP_VOLUME",
                SqlOperationType::DeleteVolume => "DELETE_VOLUME",
                SqlOperationType::RestoreVolume => "RESTORE_VOLUME",
                SqlOperationType::InjectUser => "INJECT_USER",
                SqlOperationType::Clone => "CLONE",
                SqlOperationType::StopReplica => "STOP_REPLICA",
                SqlOperationType::StartReplica => "START_REPLICA",
                SqlOperationType::PromoteReplica => "PROMOTE_REPLICA",
                SqlOperationType::CreateReplica => "CREATE_REPLICA",
                SqlOperationType::CreateUser => "CREATE_USER",
                SqlOperationType::DeleteUser => "DELETE_USER",
                SqlOperationType::UpdateUser => "UPDATE_USER",
                SqlOperationType::CreateDatabase => "CREATE_DATABASE",
                SqlOperationType::DeleteDatabase => "DELETE_DATABASE",
                SqlOperationType::UpdateDatabase => "UPDATE_DATABASE",
                SqlOperationType::Failover => "FAILOVER",
                SqlOperationType::DeleteBackup => "DELETE_BACKUP",
                SqlOperationType::RecreateReplica => "RECREATE_REPLICA",
                SqlOperationType::TruncateLog => "TRUNCATE_LOG",
                SqlOperationType::DemoteMaster => "DEMOTE_MASTER",
                SqlOperationType::Maintenance => "MAINTENANCE",
                SqlOperationType::EnablePrivateIp => "ENABLE_PRIVATE_IP",
                SqlOperationType::DeferMaintenance => "DEFER_MAINTENANCE",
                SqlOperationType::CreateClone => "CREATE_CLONE",
                SqlOperationType::RescheduleMaintenance => "RESCHEDULE_MAINTENANCE",
                SqlOperationType::StartExternalSync => "START_EXTERNAL_SYNC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "IMPORT" => Some(Self::Import),
                "EXPORT" => Some(Self::Export),
                "CREATE" => Some(Self::Create),
                "UPDATE" => Some(Self::Update),
                "DELETE" => Some(Self::Delete),
                "RESTART" => Some(Self::Restart),
                "BACKUP" => Some(Self::Backup),
                "SNAPSHOT" => Some(Self::Snapshot),
                "BACKUP_VOLUME" => Some(Self::BackupVolume),
                "DELETE_VOLUME" => Some(Self::DeleteVolume),
                "RESTORE_VOLUME" => Some(Self::RestoreVolume),
                "INJECT_USER" => Some(Self::InjectUser),
                "CLONE" => Some(Self::Clone),
                "STOP_REPLICA" => Some(Self::StopReplica),
                "START_REPLICA" => Some(Self::StartReplica),
                "PROMOTE_REPLICA" => Some(Self::PromoteReplica),
                "CREATE_REPLICA" => Some(Self::CreateReplica),
                "CREATE_USER" => Some(Self::CreateUser),
                "DELETE_USER" => Some(Self::DeleteUser),
                "UPDATE_USER" => Some(Self::UpdateUser),
                "CREATE_DATABASE" => Some(Self::CreateDatabase),
                "DELETE_DATABASE" => Some(Self::DeleteDatabase),
                "UPDATE_DATABASE" => Some(Self::UpdateDatabase),
                "FAILOVER" => Some(Self::Failover),
                "DELETE_BACKUP" => Some(Self::DeleteBackup),
                "RECREATE_REPLICA" => Some(Self::RecreateReplica),
                "TRUNCATE_LOG" => Some(Self::TruncateLog),
                "DEMOTE_MASTER" => Some(Self::DemoteMaster),
                "MAINTENANCE" => Some(Self::Maintenance),
                "ENABLE_PRIVATE_IP" => Some(Self::EnablePrivateIp),
                "DEFER_MAINTENANCE" => Some(Self::DeferMaintenance),
                "CREATE_CLONE" => Some(Self::CreateClone),
                "RESCHEDULE_MAINTENANCE" => Some(Self::RescheduleMaintenance),
                "START_EXTERNAL_SYNC" => Some(Self::StartExternalSync),
                _ => None,
            }
        }
    }
    /// The status of an operation.
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
    pub enum SqlOperationStatus {
        /// The state of the operation is unknown.
        Unspecified = 0,
        /// The operation has been queued, but has not started yet.
        Pending = 1,
        /// The operation is running.
        Running = 2,
        /// The operation completed.
        Done = 3,
    }
    impl SqlOperationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlOperationStatus::Unspecified => "SQL_OPERATION_STATUS_UNSPECIFIED",
                SqlOperationStatus::Pending => "PENDING",
                SqlOperationStatus::Running => "RUNNING",
                SqlOperationStatus::Done => "DONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_OPERATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "RUNNING" => Some(Self::Running),
                "DONE" => Some(Self::Done),
                _ => None,
            }
        }
    }
}
/// Database instance operation error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationError {
    /// This is always **sql#operationError**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Identifies the specific error that occurred.
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// Additional information about the error encountered.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Database instance operation errors list wrapper.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationErrors {
    /// This is always **sql#operationErrors**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The list of errors encountered while processing this operation.
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<OperationError>,
}
/// Database instance settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// The version of instance settings. This is a required field for update
    /// method to make sure concurrent updates are handled properly. During update,
    /// use the most recent settingsVersion value for this instance and do not try
    /// to update this value.
    #[prost(message, optional, tag = "1")]
    pub settings_version: ::core::option::Option<i64>,
    /// The App Engine app IDs that can access this instance.
    /// (Deprecated) Applied to First Generation instances only.
    #[deprecated]
    #[prost(string, repeated, tag = "2")]
    pub authorized_gae_applications: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// The tier (or machine type) for this instance, for example
    /// **db-custom-1-3840**.
    #[prost(string, tag = "3")]
    pub tier: ::prost::alloc::string::String,
    /// This is always **sql#settings**.
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
    /// User-provided labels, represented as a dictionary where each label is a
    /// single key value pair.
    #[prost(map = "string, string", tag = "5")]
    pub user_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Availability type. Potential values:
    /// *  **ZONAL**: The instance serves data from only one zone. Outages in that
    /// zone affect data accessibility.
    /// *  **REGIONAL**: The instance can serve data from more than one zone in a
    /// region (it is highly available)./
    ///
    /// For more information, see [Overview of the High Availability
    /// Configuration](<https://cloud.google.com/sql/docs/mysql/high-availability>).
    #[prost(enumeration = "SqlAvailabilityType", tag = "6")]
    pub availability_type: i32,
    /// The pricing plan for this instance. This can be either **PER_USE** or
    /// **PACKAGE**. Only **PER_USE** is supported for Second Generation instances.
    #[prost(enumeration = "SqlPricingPlan", tag = "7")]
    pub pricing_plan: i32,
    /// The type of replication this instance uses. This can be either
    /// **ASYNCHRONOUS** or **SYNCHRONOUS**. (Deprecated) This property was only
    /// applicable to First Generation instances.
    #[deprecated]
    #[prost(enumeration = "SqlReplicationType", tag = "8")]
    pub replication_type: i32,
    /// The maximum size to which storage capacity can be automatically increased.
    /// The default value is 0, which specifies that there is no limit.
    #[prost(message, optional, tag = "9")]
    pub storage_auto_resize_limit: ::core::option::Option<i64>,
    /// The activation policy specifies when the instance is activated; it is
    /// applicable only when the instance state is RUNNABLE. Valid values:
    /// *  **ALWAYS**: The instance is on, and remains so even in the absence of
    /// connection requests.
    /// *  **NEVER**: The instance is off; it is not activated, even if a
    /// connection request arrives.
    #[prost(enumeration = "settings::SqlActivationPolicy", tag = "10")]
    pub activation_policy: i32,
    /// The settings for IP Management. This allows to enable or disable the
    /// instance IP and manage which external networks can connect to the instance.
    /// The IPv4 address cannot be disabled for Second Generation instances.
    #[prost(message, optional, tag = "11")]
    pub ip_configuration: ::core::option::Option<IpConfiguration>,
    /// Configuration to increase storage size automatically. The default value is
    /// true.
    #[prost(message, optional, tag = "12")]
    pub storage_auto_resize: ::core::option::Option<bool>,
    /// The location preference settings. This allows the instance to be located as
    /// near as possible to either an App Engine app or Compute Engine zone for
    /// better performance. App Engine co-location was only applicable to First
    /// Generation instances.
    #[prost(message, optional, tag = "13")]
    pub location_preference: ::core::option::Option<LocationPreference>,
    /// The database flags passed to the instance at startup.
    #[prost(message, repeated, tag = "14")]
    pub database_flags: ::prost::alloc::vec::Vec<DatabaseFlags>,
    /// The type of data disk: **PD_SSD** (default) or **PD_HDD**. Not used for
    /// First Generation instances.
    #[prost(enumeration = "SqlDataDiskType", tag = "15")]
    pub data_disk_type: i32,
    /// The maintenance window for this instance. This specifies when the instance
    /// can be restarted for maintenance purposes.
    #[prost(message, optional, tag = "16")]
    pub maintenance_window: ::core::option::Option<MaintenanceWindow>,
    /// The daily backup configuration for the instance.
    #[prost(message, optional, tag = "17")]
    pub backup_configuration: ::core::option::Option<BackupConfiguration>,
    /// Configuration specific to read replica instances. Indicates whether
    /// replication is enabled or not.
    #[prost(message, optional, tag = "18")]
    pub database_replication_enabled: ::core::option::Option<bool>,
    /// Configuration specific to read replica instances. Indicates whether
    /// database flags for crash-safe replication are enabled. This property was
    /// only applicable to First Generation instances.
    #[deprecated]
    #[prost(message, optional, tag = "19")]
    pub crash_safe_replication_enabled: ::core::option::Option<bool>,
    /// The size of data disk, in GB. The data disk size minimum is 10GB.
    #[prost(message, optional, tag = "20")]
    pub data_disk_size_gb: ::core::option::Option<i64>,
    /// Active Directory configuration, relevant only for Cloud SQL for SQL Server.
    #[prost(message, optional, tag = "22")]
    pub active_directory_config: ::core::option::Option<SqlActiveDirectoryConfig>,
    /// The name of server Instance collation.
    #[prost(string, tag = "23")]
    pub collation: ::prost::alloc::string::String,
    /// Deny maintenance periods
    #[prost(message, repeated, tag = "24")]
    pub deny_maintenance_periods: ::prost::alloc::vec::Vec<DenyMaintenancePeriod>,
    /// Insights configuration, for now relevant only for Postgres.
    #[prost(message, optional, tag = "25")]
    pub insights_config: ::core::option::Option<InsightsConfig>,
    /// SQL Server specific audit configuration.
    #[prost(message, optional, tag = "29")]
    pub sql_server_audit_config: ::core::option::Option<SqlServerAuditConfig>,
}
/// Nested message and enum types in `Settings`.
pub mod settings {
    /// Specifies when the instance is activated.
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
    pub enum SqlActivationPolicy {
        /// Unknown activation plan.
        Unspecified = 0,
        /// The instance is always up and running.
        Always = 1,
        /// The instance never starts.
        Never = 2,
        /// The instance starts upon receiving requests.
        OnDemand = 3,
    }
    impl SqlActivationPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlActivationPolicy::Unspecified => "SQL_ACTIVATION_POLICY_UNSPECIFIED",
                SqlActivationPolicy::Always => "ALWAYS",
                SqlActivationPolicy::Never => "NEVER",
                SqlActivationPolicy::OnDemand => "ON_DEMAND",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_ACTIVATION_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "ALWAYS" => Some(Self::Always),
                "NEVER" => Some(Self::Never),
                "ON_DEMAND" => Some(Self::OnDemand),
                _ => None,
            }
        }
    }
}
/// SslCerts Resource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCert {
    /// This is always **sql#sslCert**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Serial number, as extracted from the certificate.
    #[prost(string, tag = "2")]
    pub cert_serial_number: ::prost::alloc::string::String,
    /// PEM representation.
    #[prost(string, tag = "3")]
    pub cert: ::prost::alloc::string::String,
    /// The time when the certificate was created in [RFC
    /// 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User supplied name.  Constrained to [a-zA-Z.-_ ]+.
    #[prost(string, tag = "5")]
    pub common_name: ::prost::alloc::string::String,
    /// The time when the certificate expires in [RFC
    /// 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Sha1 Fingerprint.
    #[prost(string, tag = "7")]
    pub sha1_fingerprint: ::prost::alloc::string::String,
    /// Name of the database instance.
    #[prost(string, tag = "8")]
    pub instance: ::prost::alloc::string::String,
    /// The URI of this resource.
    #[prost(string, tag = "9")]
    pub self_link: ::prost::alloc::string::String,
}
/// SslCertDetail.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertDetail {
    /// The public information about the cert.
    #[prost(message, optional, tag = "1")]
    pub cert_info: ::core::option::Option<SslCert>,
    /// The private key for the client cert, in pem format.  Keep private in order
    /// to protect your security.
    #[prost(string, tag = "2")]
    pub cert_private_key: ::prost::alloc::string::String,
}
/// Active Directory configuration, relevant only for Cloud SQL for SQL Server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlActiveDirectoryConfig {
    /// This is always sql#activeDirectoryConfig.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The name of the domain (e.g., mydomain.com).
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
}
/// SQL Server specific audit configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlServerAuditConfig {
    /// This is always sql#sqlServerAuditConfig
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The name of the destination bucket (e.g., gs://mybucket).
    #[prost(string, tag = "2")]
    pub bucket: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlFileType {
    /// Unknown file type.
    Unspecified = 0,
    /// File containing SQL statements.
    Sql = 1,
    /// File in CSV format.
    Csv = 2,
    Bak = 4,
}
impl SqlFileType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlFileType::Unspecified => "SQL_FILE_TYPE_UNSPECIFIED",
            SqlFileType::Sql => "SQL",
            SqlFileType::Csv => "CSV",
            SqlFileType::Bak => "BAK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_FILE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SQL" => Some(Self::Sql),
            "CSV" => Some(Self::Csv),
            "BAK" => Some(Self::Bak),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlBackendType {
    /// This is an unknown backend type for instance.
    Unspecified = 0,
    /// V1 speckle instance.
    FirstGen = 1,
    /// V2 speckle instance.
    SecondGen = 2,
    /// On premises instance.
    External = 3,
}
impl SqlBackendType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlBackendType::Unspecified => "SQL_BACKEND_TYPE_UNSPECIFIED",
            SqlBackendType::FirstGen => "FIRST_GEN",
            SqlBackendType::SecondGen => "SECOND_GEN",
            SqlBackendType::External => "EXTERNAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_BACKEND_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "FIRST_GEN" => Some(Self::FirstGen),
            "SECOND_GEN" => Some(Self::SecondGen),
            "EXTERNAL" => Some(Self::External),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlIpAddressType {
    /// This is an unknown IP address type.
    Unspecified = 0,
    /// IP address the customer is supposed to connect to. Usually this is the
    /// load balancer's IP address
    Primary = 1,
    /// Source IP address of the connection a read replica establishes to its
    /// external primary instance. This IP address can be allowlisted by the
    /// customer in case it has a firewall that filters incoming connection to its
    /// on premises primary instance.
    Outgoing = 2,
    /// Private IP used when using private IPs and network peering.
    Private = 3,
    /// V1 IP of a migrated instance. We want the user to
    /// decommission this IP as soon as the migration is complete.
    /// Note: V1 instances with V1 ip addresses will be counted as PRIMARY.
    Migrated1stGen = 4,
}
impl SqlIpAddressType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlIpAddressType::Unspecified => "SQL_IP_ADDRESS_TYPE_UNSPECIFIED",
            SqlIpAddressType::Primary => "PRIMARY",
            SqlIpAddressType::Outgoing => "OUTGOING",
            SqlIpAddressType::Private => "PRIVATE",
            SqlIpAddressType::Migrated1stGen => "MIGRATED_1ST_GEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_IP_ADDRESS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PRIMARY" => Some(Self::Primary),
            "OUTGOING" => Some(Self::Outgoing),
            "PRIVATE" => Some(Self::Private),
            "MIGRATED_1ST_GEN" => Some(Self::Migrated1stGen),
            _ => None,
        }
    }
}
/// The database engine type and version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlDatabaseVersion {
    /// This is an unknown database version.
    Unspecified = 0,
    /// The database version is MySQL 5.1.
    Mysql51 = 2,
    /// The database version is MySQL 5.5.
    Mysql55 = 3,
    /// The database version is MySQL 5.6.
    Mysql56 = 5,
    /// The database version is MySQL 5.7.
    Mysql57 = 6,
    /// The database version is PostgreSQL 9.6.
    Postgres96 = 9,
    /// The database version is PostgreSQL 11.
    Postgres11 = 10,
    /// The database version is SQL Server 2017 Standard.
    Sqlserver2017Standard = 11,
    /// The database version is SQL Server 2017 Enterprise.
    Sqlserver2017Enterprise = 14,
    /// The database version is SQL Server 2017 Express.
    Sqlserver2017Express = 15,
    /// The database version is SQL Server 2017 Web.
    Sqlserver2017Web = 16,
    /// The database version is PostgreSQL 10.
    Postgres10 = 18,
    /// The database version is PostgreSQL 12.
    Postgres12 = 19,
    /// The database version is PostgreSQL 13.
    Postgres13 = 23,
    /// The database version is SQL Server 2019 Standard.
    Sqlserver2019Standard = 26,
    /// The database version is SQL Server 2019 Enterprise.
    Sqlserver2019Enterprise = 27,
    /// The database version is SQL Server 2019 Express.
    Sqlserver2019Express = 28,
    /// The database version is SQL Server 2019 Web.
    Sqlserver2019Web = 29,
}
impl SqlDatabaseVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlDatabaseVersion::Unspecified => "SQL_DATABASE_VERSION_UNSPECIFIED",
            SqlDatabaseVersion::Mysql51 => "MYSQL_5_1",
            SqlDatabaseVersion::Mysql55 => "MYSQL_5_5",
            SqlDatabaseVersion::Mysql56 => "MYSQL_5_6",
            SqlDatabaseVersion::Mysql57 => "MYSQL_5_7",
            SqlDatabaseVersion::Postgres96 => "POSTGRES_9_6",
            SqlDatabaseVersion::Postgres11 => "POSTGRES_11",
            SqlDatabaseVersion::Sqlserver2017Standard => "SQLSERVER_2017_STANDARD",
            SqlDatabaseVersion::Sqlserver2017Enterprise => "SQLSERVER_2017_ENTERPRISE",
            SqlDatabaseVersion::Sqlserver2017Express => "SQLSERVER_2017_EXPRESS",
            SqlDatabaseVersion::Sqlserver2017Web => "SQLSERVER_2017_WEB",
            SqlDatabaseVersion::Postgres10 => "POSTGRES_10",
            SqlDatabaseVersion::Postgres12 => "POSTGRES_12",
            SqlDatabaseVersion::Postgres13 => "POSTGRES_13",
            SqlDatabaseVersion::Sqlserver2019Standard => "SQLSERVER_2019_STANDARD",
            SqlDatabaseVersion::Sqlserver2019Enterprise => "SQLSERVER_2019_ENTERPRISE",
            SqlDatabaseVersion::Sqlserver2019Express => "SQLSERVER_2019_EXPRESS",
            SqlDatabaseVersion::Sqlserver2019Web => "SQLSERVER_2019_WEB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_DATABASE_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
            "MYSQL_5_1" => Some(Self::Mysql51),
            "MYSQL_5_5" => Some(Self::Mysql55),
            "MYSQL_5_6" => Some(Self::Mysql56),
            "MYSQL_5_7" => Some(Self::Mysql57),
            "POSTGRES_9_6" => Some(Self::Postgres96),
            "POSTGRES_11" => Some(Self::Postgres11),
            "SQLSERVER_2017_STANDARD" => Some(Self::Sqlserver2017Standard),
            "SQLSERVER_2017_ENTERPRISE" => Some(Self::Sqlserver2017Enterprise),
            "SQLSERVER_2017_EXPRESS" => Some(Self::Sqlserver2017Express),
            "SQLSERVER_2017_WEB" => Some(Self::Sqlserver2017Web),
            "POSTGRES_10" => Some(Self::Postgres10),
            "POSTGRES_12" => Some(Self::Postgres12),
            "POSTGRES_13" => Some(Self::Postgres13),
            "SQLSERVER_2019_STANDARD" => Some(Self::Sqlserver2019Standard),
            "SQLSERVER_2019_ENTERPRISE" => Some(Self::Sqlserver2019Enterprise),
            "SQLSERVER_2019_EXPRESS" => Some(Self::Sqlserver2019Express),
            "SQLSERVER_2019_WEB" => Some(Self::Sqlserver2019Web),
            _ => None,
        }
    }
}
/// The pricing plan for this instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlPricingPlan {
    /// This is an unknown pricing plan for this instance.
    Unspecified = 0,
    /// The instance is billed at a monthly flat rate.
    Package = 1,
    /// The instance is billed per usage.
    PerUse = 2,
}
impl SqlPricingPlan {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlPricingPlan::Unspecified => "SQL_PRICING_PLAN_UNSPECIFIED",
            SqlPricingPlan::Package => "PACKAGE",
            SqlPricingPlan::PerUse => "PER_USE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_PRICING_PLAN_UNSPECIFIED" => Some(Self::Unspecified),
            "PACKAGE" => Some(Self::Package),
            "PER_USE" => Some(Self::PerUse),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlReplicationType {
    /// This is an unknown replication type for a Cloud SQL instance.
    Unspecified = 0,
    /// The synchronous replication mode for First Generation instances. It is the
    /// default value.
    Synchronous = 1,
    /// The asynchronous replication mode for First Generation instances. It
    /// provides a slight performance gain, but if an outage occurs while this
    /// option is set to asynchronous, you can lose up to a few seconds of updates
    /// to your data.
    Asynchronous = 2,
}
impl SqlReplicationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlReplicationType::Unspecified => "SQL_REPLICATION_TYPE_UNSPECIFIED",
            SqlReplicationType::Synchronous => "SYNCHRONOUS",
            SqlReplicationType::Asynchronous => "ASYNCHRONOUS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_REPLICATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SYNCHRONOUS" => Some(Self::Synchronous),
            "ASYNCHRONOUS" => Some(Self::Asynchronous),
            _ => None,
        }
    }
}
/// The type of disk that is used for a v2 instance to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlDataDiskType {
    /// This is an unknown data disk type.
    Unspecified = 0,
    /// An SSD data disk.
    PdSsd = 1,
    /// An HDD data disk.
    PdHdd = 2,
    /// This field is deprecated and will be removed from a future version of the
    /// API.
    ObsoleteLocalSsd = 3,
}
impl SqlDataDiskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlDataDiskType::Unspecified => "SQL_DATA_DISK_TYPE_UNSPECIFIED",
            SqlDataDiskType::PdSsd => "PD_SSD",
            SqlDataDiskType::PdHdd => "PD_HDD",
            SqlDataDiskType::ObsoleteLocalSsd => "OBSOLETE_LOCAL_SSD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_DATA_DISK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PD_SSD" => Some(Self::PdSsd),
            "PD_HDD" => Some(Self::PdHdd),
            "OBSOLETE_LOCAL_SSD" => Some(Self::ObsoleteLocalSsd),
            _ => None,
        }
    }
}
/// The availability type of the given Cloud SQL instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlAvailabilityType {
    /// This is an unknown Availability type.
    Unspecified = 0,
    /// Zonal available instance.
    Zonal = 1,
    /// Regional available instance.
    Regional = 2,
}
impl SqlAvailabilityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlAvailabilityType::Unspecified => "SQL_AVAILABILITY_TYPE_UNSPECIFIED",
            SqlAvailabilityType::Zonal => "ZONAL",
            SqlAvailabilityType::Regional => "REGIONAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_AVAILABILITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ZONAL" => Some(Self::Zonal),
            "REGIONAL" => Some(Self::Regional),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlUpdateTrack {
    /// This is an unknown maintenance timing preference.
    Unspecified = 0,
    /// For instance update that requires a restart, this update track indicates
    /// your instance prefer to restart for new version early in maintenance
    /// window.
    Canary = 1,
    /// For instance update that requires a restart, this update track indicates
    /// your instance prefer to let Cloud SQL choose the timing of restart (within
    /// its Maintenance window, if applicable).
    Stable = 2,
}
impl SqlUpdateTrack {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlUpdateTrack::Unspecified => "SQL_UPDATE_TRACK_UNSPECIFIED",
            SqlUpdateTrack::Canary => "canary",
            SqlUpdateTrack::Stable => "stable",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_UPDATE_TRACK_UNSPECIFIED" => Some(Self::Unspecified),
            "canary" => Some(Self::Canary),
            "stable" => Some(Self::Stable),
            _ => None,
        }
    }
}
/// Backup runs delete request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlBackupRunsDeleteRequest {
    /// The ID of the backup run to delete. To find a backup run ID, use the
    /// \[list\](<https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/backupRuns/list>)
    /// method.
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
/// Backup runs get request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlBackupRunsGetRequest {
    /// The ID of this backup run.
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
/// Backup runs insert request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlBackupRunsInsertRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<BackupRun>,
}
/// Backup runs list request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlBackupRunsListRequest {
    /// Cloud SQL instance ID, or "-" for all instances. This does not include
    /// the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Maximum number of backup runs per response.
    #[prost(int32, tag = "2")]
    pub max_results: i32,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
/// A BackupRun resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRun {
    /// This is always **sql#backupRun**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The status of this run.
    #[prost(enumeration = "SqlBackupRunStatus", tag = "2")]
    pub status: i32,
    /// The time the run was enqueued in UTC timezone in
    /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "3")]
    pub enqueued_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The identifier for this backup run. Unique only for a specific Cloud SQL
    /// instance.
    #[prost(int64, tag = "4")]
    pub id: i64,
    /// The time the backup operation actually started in UTC timezone in
    /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the backup operation completed in UTC timezone in
    /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Information about why the backup operation failed. This is only present if
    /// the run has the FAILED status.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<OperationError>,
    /// The type of this run; can be either "AUTOMATED" or "ON_DEMAND". This field
    /// defaults to "ON_DEMAND" and is ignored, when specified for insert requests.
    #[prost(enumeration = "SqlBackupRunType", tag = "8")]
    pub r#type: i32,
    /// The description of this run, only applicable to on-demand backups.
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    /// The start time of the backup window during which this the backup was
    /// attempted in [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for
    /// example **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "10")]
    pub window_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Name of the database instance.
    #[prost(string, tag = "11")]
    pub instance: ::prost::alloc::string::String,
    /// The URI of this resource.
    #[prost(string, tag = "12")]
    pub self_link: ::prost::alloc::string::String,
    /// Location of the backups.
    #[prost(string, tag = "13")]
    pub location: ::prost::alloc::string::String,
    /// Encryption configuration specific to a backup.
    #[prost(message, optional, tag = "16")]
    pub disk_encryption_configuration: ::core::option::Option<
        DiskEncryptionConfiguration,
    >,
    /// Encryption status specific to a backup.
    #[prost(message, optional, tag = "17")]
    pub disk_encryption_status: ::core::option::Option<DiskEncryptionStatus>,
    /// Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT.
    #[prost(enumeration = "SqlBackupKind", tag = "19")]
    pub backup_kind: i32,
}
/// Backup run list results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRunsListResponse {
    /// This is always **sql#backupRunsList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// A list of backup runs in reverse chronological order of the enqueued time.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<BackupRun>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The status of a backup run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlBackupRunStatus {
    /// The status of the run is unknown.
    Unspecified = 0,
    /// The backup operation was enqueued.
    Enqueued = 1,
    /// The backup is overdue across a given backup window. Indicates a
    /// problem. Example: Long-running operation in progress during
    /// the whole window.
    Overdue = 2,
    /// The backup is in progress.
    Running = 3,
    /// The backup failed.
    Failed = 4,
    /// The backup was successful.
    Successful = 5,
    /// The backup was skipped (without problems) for a given backup
    /// window. Example: Instance was idle.
    Skipped = 6,
    /// The backup is about to be deleted.
    DeletionPending = 7,
    /// The backup deletion failed.
    DeletionFailed = 8,
    /// The backup has been deleted.
    Deleted = 9,
}
impl SqlBackupRunStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlBackupRunStatus::Unspecified => "SQL_BACKUP_RUN_STATUS_UNSPECIFIED",
            SqlBackupRunStatus::Enqueued => "ENQUEUED",
            SqlBackupRunStatus::Overdue => "OVERDUE",
            SqlBackupRunStatus::Running => "RUNNING",
            SqlBackupRunStatus::Failed => "FAILED",
            SqlBackupRunStatus::Successful => "SUCCESSFUL",
            SqlBackupRunStatus::Skipped => "SKIPPED",
            SqlBackupRunStatus::DeletionPending => "DELETION_PENDING",
            SqlBackupRunStatus::DeletionFailed => "DELETION_FAILED",
            SqlBackupRunStatus::Deleted => "DELETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_BACKUP_RUN_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "ENQUEUED" => Some(Self::Enqueued),
            "OVERDUE" => Some(Self::Overdue),
            "RUNNING" => Some(Self::Running),
            "FAILED" => Some(Self::Failed),
            "SUCCESSFUL" => Some(Self::Successful),
            "SKIPPED" => Some(Self::Skipped),
            "DELETION_PENDING" => Some(Self::DeletionPending),
            "DELETION_FAILED" => Some(Self::DeletionFailed),
            "DELETED" => Some(Self::Deleted),
            _ => None,
        }
    }
}
/// Defines the supported backup kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlBackupKind {
    /// This is an unknown BackupKind.
    Unspecified = 0,
    /// The snapshot based backups
    Snapshot = 1,
    /// Physical backups
    Physical = 2,
}
impl SqlBackupKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlBackupKind::Unspecified => "SQL_BACKUP_KIND_UNSPECIFIED",
            SqlBackupKind::Snapshot => "SNAPSHOT",
            SqlBackupKind::Physical => "PHYSICAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_BACKUP_KIND_UNSPECIFIED" => Some(Self::Unspecified),
            "SNAPSHOT" => Some(Self::Snapshot),
            "PHYSICAL" => Some(Self::Physical),
            _ => None,
        }
    }
}
/// Type of backup (i.e. automated, on demand, etc).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlBackupRunType {
    /// This is an unknown BackupRun type.
    Unspecified = 0,
    /// The backup schedule automatically triggers a backup.
    Automated = 1,
    /// The user manually triggers a backup.
    OnDemand = 2,
}
impl SqlBackupRunType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlBackupRunType::Unspecified => "SQL_BACKUP_RUN_TYPE_UNSPECIFIED",
            SqlBackupRunType::Automated => "AUTOMATED",
            SqlBackupRunType::OnDemand => "ON_DEMAND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_BACKUP_RUN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTOMATED" => Some(Self::Automated),
            "ON_DEMAND" => Some(Self::OnDemand),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod sql_backup_runs_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing database backups.
    #[derive(Debug, Clone)]
    pub struct SqlBackupRunsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlBackupRunsServiceClient<T>
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
        ) -> SqlBackupRunsServiceClient<InterceptedService<T, F>>
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
            SqlBackupRunsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Deletes the backup taken by a backup run.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlBackupRunsDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlBackupRunsService/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlBackupRunsService", "Delete"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a resource containing information about a backup run.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlBackupRunsGetRequest>,
        ) -> std::result::Result<tonic::Response<super::BackupRun>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlBackupRunsService/Get",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlBackupRunsService", "Get"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new backup run on demand.
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlBackupRunsInsertRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlBackupRunsService/Insert",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlBackupRunsService", "Insert"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all backup runs associated with the project or a given instance
        /// and configuration in the reverse chronological order of the backup
        /// initiation time.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlBackupRunsListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BackupRunsListResponse>,
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
                "/google.cloud.sql.v1.SqlBackupRunsService/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlBackupRunsService", "List"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Connect settings retrieval request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectSettingsRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Optional. Optional snapshot read timestamp to trade freshness for performance.
    #[prost(message, optional, tag = "7")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Connect settings retrieval response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectSettings {
    /// This is always `sql#connectSettings`.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// SSL configuration.
    #[prost(message, optional, tag = "2")]
    pub server_ca_cert: ::core::option::Option<SslCert>,
    /// The assigned IP addresses for the instance.
    #[prost(message, repeated, tag = "3")]
    pub ip_addresses: ::prost::alloc::vec::Vec<IpMapping>,
    /// The cloud region for the instance. e.g. **us-central1**, **europe-west1**.
    /// The region cannot be changed after instance creation.
    #[prost(string, tag = "4")]
    pub region: ::prost::alloc::string::String,
    /// The database engine type and version. The **databaseVersion**
    /// field cannot be changed after instance creation.
    ///    MySQL instances: **MYSQL_8_0**, **MYSQL_5_7** (default),
    /// or **MYSQL_5_6**.
    ///    PostgreSQL instances: **POSTGRES_9_6**, **POSTGRES_10**,
    /// **POSTGRES_11** or **POSTGRES_12** (default).
    ///    SQL Server instances: **SQLSERVER_2017_STANDARD** (default),
    /// **SQLSERVER_2017_ENTERPRISE**, **SQLSERVER_2017_EXPRESS**, or
    /// **SQLSERVER_2017_WEB**.
    #[prost(enumeration = "SqlDatabaseVersion", tag = "31")]
    pub database_version: i32,
    /// **SECOND_GEN**: Cloud SQL database instance.
    /// **EXTERNAL**: A database server that is not managed by Google.
    /// This property is read-only; use the **tier** property in the **settings**
    /// object to determine the database type.
    #[prost(enumeration = "SqlBackendType", tag = "32")]
    pub backend_type: i32,
}
/// Ephemeral certificate creation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateEphemeralCertRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// PEM encoded public key to include in the signed certificate.
    #[prost(string, tag = "3")]
    pub public_key: ::prost::alloc::string::String,
    /// Optional. Access token to include in the signed certificate.
    #[prost(string, tag = "4")]
    pub access_token: ::prost::alloc::string::String,
    /// Optional. Optional snapshot read timestamp to trade freshness for performance.
    #[prost(message, optional, tag = "7")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Ephemeral certificate creation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateEphemeralCertResponse {
    /// Generated cert
    #[prost(message, optional, tag = "1")]
    pub ephemeral_cert: ::core::option::Option<SslCert>,
}
/// Generated client implementations.
pub mod sql_connect_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud SQL connect service.
    #[derive(Debug, Clone)]
    pub struct SqlConnectServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlConnectServiceClient<T>
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
        ) -> SqlConnectServiceClient<InterceptedService<T, F>>
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
            SqlConnectServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves connect settings about a Cloud SQL instance.
        pub async fn get_connect_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConnectSettings>,
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
                "/google.cloud.sql.v1.SqlConnectService/GetConnectSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlConnectService",
                        "GetConnectSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates a short-lived X509 certificate containing the provided public key
        /// and signed by a private key specific to the target instance. Users may use
        /// the certificate to authenticate as themselves when connecting to the
        /// database.
        pub async fn generate_ephemeral_cert(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateEphemeralCertRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateEphemeralCertResponse>,
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
                "/google.cloud.sql.v1.SqlConnectService/GenerateEphemeralCert",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlConnectService",
                        "GenerateEphemeralCert",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Database delete request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesDeleteRequest {
    /// Name of the database to be deleted in the instance.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
/// Database get request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesGetRequest {
    /// Name of the database in the instance.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
/// Database insert request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesInsertRequest {
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<Database>,
}
/// Database list request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesListRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Database update request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesUpdateRequest {
    /// Name of the database to be updated in the instance.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<Database>,
}
/// Database list response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabasesListResponse {
    /// This is always **sql#databasesList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of database resources in the instance.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Database>,
}
/// Generated client implementations.
pub mod sql_databases_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage databases.
    #[derive(Debug, Clone)]
    pub struct SqlDatabasesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlDatabasesServiceClient<T>
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
        ) -> SqlDatabasesServiceClient<InterceptedService<T, F>>
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
            SqlDatabasesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Deletes a database from a Cloud SQL instance.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlDatabasesService/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlDatabasesService", "Delete"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a resource containing information about a database inside a Cloud
        /// SQL instance.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesGetRequest>,
        ) -> std::result::Result<tonic::Response<super::Database>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlDatabasesService/Get",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlDatabasesService", "Get"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Inserts a resource containing information about a database inside a Cloud
        /// SQL instance.
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesInsertRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlDatabasesService/Insert",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlDatabasesService", "Insert"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists databases in the specified Cloud SQL instance.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DatabasesListResponse>,
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
                "/google.cloud.sql.v1.SqlDatabasesService/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlDatabasesService", "List"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Partially updates a resource containing information about a database inside
        /// a Cloud SQL instance. This method supports patch semantics.
        pub async fn patch(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesUpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlDatabasesService/Patch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlDatabasesService", "Patch"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a resource containing information about a database inside a Cloud
        /// SQL instance.
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesUpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlDatabasesService/Update",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlDatabasesService", "Update"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Flags list request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlFlagsListRequest {
    /// Database type and version you want to retrieve flags for. By default, this
    /// method returns flags for all database types and versions.
    #[prost(string, tag = "1")]
    pub database_version: ::prost::alloc::string::String,
}
/// Flags list response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlagsListResponse {
    /// This is always **sql#flagsList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of flags.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Flag>,
}
/// A flag resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flag {
    /// This is the name of the flag. Flag names always use underscores, not
    /// hyphens, for example: **max_allowed_packet**
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of the flag. Flags are typed to being **BOOLEAN**, **STRING**,
    /// **INTEGER** or **NONE**. **NONE** is used for flags which do not take a
    /// value, such as **skip_grant_tables**.
    #[prost(enumeration = "SqlFlagType", tag = "2")]
    pub r#type: i32,
    /// The database version this flag applies to. Can be **MYSQL_8_0**,
    /// **MYSQL_5_6**, or **MYSQL_5_7**.
    #[prost(enumeration = "SqlDatabaseVersion", repeated, tag = "3")]
    pub applies_to: ::prost::alloc::vec::Vec<i32>,
    /// For **STRING** flags, a list of strings that the value can be set to.
    #[prost(string, repeated, tag = "4")]
    pub allowed_string_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// For **INTEGER** flags, the minimum allowed value.
    #[prost(message, optional, tag = "5")]
    pub min_value: ::core::option::Option<i64>,
    /// For **INTEGER** flags, the maximum allowed value.
    #[prost(message, optional, tag = "6")]
    pub max_value: ::core::option::Option<i64>,
    /// Indicates whether changing this flag will trigger a database restart. Only
    /// applicable to Second Generation instances.
    #[prost(message, optional, tag = "7")]
    pub requires_restart: ::core::option::Option<bool>,
    /// This is always **sql#flag**.
    #[prost(string, tag = "8")]
    pub kind: ::prost::alloc::string::String,
    /// Whether or not the flag is considered in beta.
    #[prost(message, optional, tag = "9")]
    pub in_beta: ::core::option::Option<bool>,
    /// Use this field if only certain integers are accepted. Can be combined
    /// with min_value and max_value to add additional values.
    #[prost(int64, repeated, tag = "10")]
    pub allowed_int_values: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlFlagType {
    /// This is an unknown flag type.
    Unspecified = 0,
    /// Boolean type flag.
    Boolean = 1,
    /// String type flag.
    String = 2,
    /// Integer type flag.
    Integer = 3,
    /// Flag type used for a server startup option.
    None = 4,
    /// Type introduced specially for MySQL TimeZone offset. Accept a string value
    /// with the format [-12:59, 13:00].
    MysqlTimezoneOffset = 5,
    /// Float type flag.
    Float = 6,
    /// Comma-separated list of the strings in a SqlFlagType enum.
    RepeatedString = 7,
}
impl SqlFlagType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlFlagType::Unspecified => "SQL_FLAG_TYPE_UNSPECIFIED",
            SqlFlagType::Boolean => "BOOLEAN",
            SqlFlagType::String => "STRING",
            SqlFlagType::Integer => "INTEGER",
            SqlFlagType::None => "NONE",
            SqlFlagType::MysqlTimezoneOffset => "MYSQL_TIMEZONE_OFFSET",
            SqlFlagType::Float => "FLOAT",
            SqlFlagType::RepeatedString => "REPEATED_STRING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_FLAG_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BOOLEAN" => Some(Self::Boolean),
            "STRING" => Some(Self::String),
            "INTEGER" => Some(Self::Integer),
            "NONE" => Some(Self::None),
            "MYSQL_TIMEZONE_OFFSET" => Some(Self::MysqlTimezoneOffset),
            "FLOAT" => Some(Self::Float),
            "REPEATED_STRING" => Some(Self::RepeatedString),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod sql_flags_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage database flags for Cloud SQL instances.
    #[derive(Debug, Clone)]
    pub struct SqlFlagsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlFlagsServiceClient<T>
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
        ) -> SqlFlagsServiceClient<InterceptedService<T, F>>
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
            SqlFlagsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all available database flags for Cloud SQL instances.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlFlagsListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FlagsListResponse>,
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
                "/google.cloud.sql.v1.SqlFlagsService/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.sql.v1.SqlFlagsService", "List"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod sql_instance_names_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud SQL instance names service.
    #[derive(Debug, Clone)]
    pub struct SqlInstanceNamesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlInstanceNamesServiceClient<T>
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
        ) -> SqlInstanceNamesServiceClient<InterceptedService<T, F>>
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
            SqlInstanceNamesServiceClient::new(
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
    }
}
/// Instance add server CA request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesAddServerCaRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance clone request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesCloneRequest {
    /// The ID of the Cloud SQL instance to be cloned (source). This does not
    /// include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the source as well as the clone Cloud SQL instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesCloneRequest>,
}
/// Instance delete request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesDeleteRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance to be deleted.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance demote master request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesDemoteMasterRequest {
    /// Cloud SQL instance name.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesDemoteMasterRequest>,
}
/// Instance export request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesExportRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance to be exported.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesExportRequest>,
}
/// Instance failover request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesFailoverRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the read replica.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesFailoverRequest>,
}
/// Instance get request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesGetRequest {
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance import request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesImportRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesImportRequest>,
}
/// Instance insert request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesInsertRequest {
    /// Project ID of the project to which the newly created Cloud SQL instances
    /// should belong.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<DatabaseInstance>,
}
/// Instance list request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesListRequest {
    /// A filter expression that filters resources listed in the response.
    /// The expression is in the form of field:value. For example,
    /// 'instanceType:CLOUD_SQL_INSTANCE'. Fields can be nested as needed as per
    /// their JSON representation, such as 'settings.userLabels.auto_start:true'.
    ///
    /// Multiple filter queries are space-separated. For example.
    /// 'state:RUNNABLE instanceType:CLOUD_SQL_INSTANCE'. By default, each
    /// expression is an AND expression. However, you can include AND and OR
    /// expressions explicitly.
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of results to return per response.
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Project ID of the project for which to list Cloud SQL instances.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
/// Instance list server CAs request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesListServerCasRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance patch request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesPatchRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<DatabaseInstance>,
}
/// Instance promote replica request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesPromoteReplicaRequest {
    /// Cloud SQL read replica instance name.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the read replica.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance reset SSL config request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesResetSslConfigRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance restart request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRestartRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance to be restarted.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance restore backup request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRestoreBackupRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesRestoreBackupRequest>,
}
/// Instance rotate server CA request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRotateServerCaRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesRotateServerCaRequest>,
}
/// Instance start replica request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesStartReplicaRequest {
    /// Cloud SQL read replica instance name.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the read replica.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance stop replica request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesStopReplicaRequest {
    /// Cloud SQL read replica instance name.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the read replica.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Instance truncate log request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesTruncateLogRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the Cloud SQL project.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesTruncateLogRequest>,
}
/// Instance update request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesUpdateRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<DatabaseInstance>,
}
/// Instance reschedule maintenance request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRescheduleMaintenanceRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<SqlInstancesRescheduleMaintenanceRequestBody>,
}
/// Instance verify external sync settings request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesVerifyExternalSyncSettingsRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Flag to enable verifying connection only
    #[prost(bool, tag = "3")]
    pub verify_connection_only: bool,
    /// External sync mode
    #[prost(
        enumeration = "sql_instances_verify_external_sync_settings_request::ExternalSyncMode",
        tag = "4"
    )]
    pub sync_mode: i32,
    /// Optional. Flag to verify settings required by replication setup only
    #[prost(bool, tag = "5")]
    pub verify_replication_only: bool,
    #[prost(
        oneof = "sql_instances_verify_external_sync_settings_request::SyncConfig",
        tags = "6"
    )]
    pub sync_config: ::core::option::Option<
        sql_instances_verify_external_sync_settings_request::SyncConfig,
    >,
}
/// Nested message and enum types in `SqlInstancesVerifyExternalSyncSettingsRequest`.
pub mod sql_instances_verify_external_sync_settings_request {
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
    pub enum ExternalSyncMode {
        /// Unknown external sync mode, will be defaulted to ONLINE mode
        Unspecified = 0,
        /// Online external sync will set up replication after initial data external
        /// sync
        Online = 1,
        /// Offline external sync only dumps and loads a one-time snapshot of
        /// the primary instance's data
        Offline = 2,
    }
    impl ExternalSyncMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExternalSyncMode::Unspecified => "EXTERNAL_SYNC_MODE_UNSPECIFIED",
                ExternalSyncMode::Online => "ONLINE",
                ExternalSyncMode::Offline => "OFFLINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXTERNAL_SYNC_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ONLINE" => Some(Self::Online),
                "OFFLINE" => Some(Self::Offline),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SyncConfig {
        /// Optional. MySQL-specific settings for start external sync.
        #[prost(message, tag = "6")]
        MysqlSyncConfig(super::MySqlSyncConfig),
    }
}
/// Instance start external sync request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesStartExternalSyncRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// External sync mode.
    #[prost(
        enumeration = "sql_instances_verify_external_sync_settings_request::ExternalSyncMode",
        tag = "3"
    )]
    pub sync_mode: i32,
    /// Whether to skip the verification step (VESS).
    #[prost(bool, tag = "4")]
    pub skip_verification: bool,
    #[prost(oneof = "sql_instances_start_external_sync_request::SyncConfig", tags = "6")]
    pub sync_config: ::core::option::Option<
        sql_instances_start_external_sync_request::SyncConfig,
    >,
}
/// Nested message and enum types in `SqlInstancesStartExternalSyncRequest`.
pub mod sql_instances_start_external_sync_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SyncConfig {
        /// MySQL-specific settings for start external sync.
        #[prost(message, tag = "6")]
        MysqlSyncConfig(super::MySqlSyncConfig),
    }
}
/// Instance create ephemeral certificate request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesCreateEphemeralCertRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the Cloud SQL project.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<SslCertsCreateEphemeralRequest>,
}
/// Database instance clone request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesCloneRequest {
    /// Contains details about the clone operation.
    #[prost(message, optional, tag = "1")]
    pub clone_context: ::core::option::Option<CloneContext>,
}
/// Database demote primary instance request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesDemoteMasterRequest {
    /// Contains details about the demoteMaster operation.
    #[prost(message, optional, tag = "1")]
    pub demote_master_context: ::core::option::Option<DemoteMasterContext>,
}
/// Database instance export request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesExportRequest {
    /// Contains details about the export operation.
    #[prost(message, optional, tag = "1")]
    pub export_context: ::core::option::Option<ExportContext>,
}
/// Instance failover request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesFailoverRequest {
    /// Failover Context.
    #[prost(message, optional, tag = "1")]
    pub failover_context: ::core::option::Option<FailoverContext>,
}
/// SslCerts create ephemeral certificate request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertsCreateEphemeralRequest {
    /// PEM encoded public key to include in the signed certificate.
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    /// Access token to include in the signed certificate.
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
}
/// Database instance import request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesImportRequest {
    /// Contains details about the import operation.
    #[prost(message, optional, tag = "1")]
    pub import_context: ::core::option::Option<ImportContext>,
}
/// Database instances list response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesListResponse {
    /// This is always **sql#instancesList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of warnings that occurred while handling the request.
    #[prost(message, repeated, tag = "2")]
    pub warnings: ::prost::alloc::vec::Vec<ApiWarning>,
    /// List of database instance resources.
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<DatabaseInstance>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag = "4")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Instances ListServerCas response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesListServerCasResponse {
    /// List of server CA certificates for the instance.
    #[prost(message, repeated, tag = "1")]
    pub certs: ::prost::alloc::vec::Vec<SslCert>,
    #[prost(string, tag = "2")]
    pub active_version: ::prost::alloc::string::String,
    /// This is always **sql#instancesListServerCas**.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
}
/// Database instance restore backup request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesRestoreBackupRequest {
    /// Parameters required to perform the restore backup operation.
    #[prost(message, optional, tag = "1")]
    pub restore_backup_context: ::core::option::Option<RestoreBackupContext>,
}
/// Rotate server CA request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesRotateServerCaRequest {
    /// Contains details about the rotate server CA operation.
    #[prost(message, optional, tag = "1")]
    pub rotate_server_ca_context: ::core::option::Option<RotateServerCaContext>,
}
/// Instance truncate log request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesTruncateLogRequest {
    /// Contains details about the truncate log operation.
    #[prost(message, optional, tag = "1")]
    pub truncate_log_context: ::core::option::Option<TruncateLogContext>,
}
/// Instance verify external sync settings response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesVerifyExternalSyncSettingsResponse {
    /// This is always **sql#migrationSettingErrorList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of migration violations.
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<SqlExternalSyncSettingError>,
    /// List of migration warnings.
    #[prost(message, repeated, tag = "3")]
    pub warnings: ::prost::alloc::vec::Vec<SqlExternalSyncSettingError>,
}
/// Database instance clone context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneContext {
    /// This is always **sql#cloneContext**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Reserved for future use.
    #[prost(int64, tag = "2")]
    pub pitr_timestamp_ms: i64,
    /// Name of the Cloud SQL instance to be created as a clone.
    #[prost(string, tag = "3")]
    pub destination_instance_name: ::prost::alloc::string::String,
    /// Binary log coordinates, if specified, identify the position up to which the
    /// source instance is cloned. If not specified, the source instance is
    /// cloned up to the most recent binary log coordinates.
    #[prost(message, optional, tag = "4")]
    pub bin_log_coordinates: ::core::option::Option<BinLogCoordinates>,
    /// Timestamp, if specified, identifies the time to which the source instance
    /// is cloned.
    #[prost(message, optional, tag = "5")]
    pub point_in_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Binary log coordinates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinLogCoordinates {
    /// Name of the binary log file for a Cloud SQL instance.
    #[prost(string, tag = "1")]
    pub bin_log_file_name: ::prost::alloc::string::String,
    /// Position (offset) within the binary log file.
    #[prost(int64, tag = "2")]
    pub bin_log_position: i64,
    /// This is always **sql#binLogCoordinates**.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
}
/// A Cloud SQL instance resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseInstance {
    /// This is always **sql#instance**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The current serving state of the Cloud SQL instance. This can be one of the
    /// following:
    /// *  **SQL_INSTANCE_STATE_UNSPECIFIED**: The state of the instance is
    /// unknown.
    /// *  **RUNNABLE**: The instance is running, or has been stopped by owner.
    /// *  **SUSPENDED**: The instance is not available, for example due to
    /// problems with billing.
    /// *  **PENDING_DELETE**: The instance is being deleted.
    /// *  **PENDING_CREATE**: The instance is being created.
    /// *  **MAINTENANCE**: The instance is down for maintenance.
    /// *  **FAILED**: The instance creation failed.
    #[prost(enumeration = "database_instance::SqlInstanceState", tag = "2")]
    pub state: i32,
    /// The database engine type and version. The **databaseVersion** field cannot
    /// be changed after instance creation.
    /// *  **MySQL instances**: MYSQL_8_0, MYSQL_5_7 (default), or MYSQL_5_6.
    /// *  **PostgreSQL instances**: POSTGRES_9_6, POSTGRES_10, POSTGRES_11,
    /// POSTGRES_12, POSTGRES_13 (default).
    /// *  **SQL Server instances**: SQLSERVER_2019_STANDARD,
    /// SQLSERVER_2019_ENTERPRISE, SQLSERVER_2019_EXPRESS, or SQLSERVER_2019_WEB,
    /// SQLSERVER_2017_STANDARD (default), SQLSERVER_2017_ENTERPRISE,
    /// SQLSERVER_2017_EXPRESS, or SQLSERVER_2017_WEB.
    #[prost(enumeration = "SqlDatabaseVersion", tag = "3")]
    pub database_version: i32,
    /// The user settings.
    #[prost(message, optional, tag = "4")]
    pub settings: ::core::option::Option<Settings>,
    /// This field is deprecated and will be removed from a future version of the
    /// API. Use the **settings.settingsVersion** field instead.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
    /// The name and status of the failover replica.
    #[prost(message, optional, tag = "6")]
    pub failover_replica: ::core::option::Option<database_instance::SqlFailoverReplica>,
    /// The name of the instance which will act as primary in the replication
    /// setup.
    #[prost(string, tag = "7")]
    pub master_instance_name: ::prost::alloc::string::String,
    /// The replicas of the instance.
    #[prost(string, repeated, tag = "8")]
    pub replica_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The maximum disk size of the instance in bytes.
    #[deprecated]
    #[prost(message, optional, tag = "9")]
    pub max_disk_size: ::core::option::Option<i64>,
    /// The current disk usage of the instance in bytes. This property has been
    /// deprecated. Use the
    /// "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud
    /// Monitoring API instead. Please see [this
    /// announcement](<https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ>)
    /// for details.
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub current_disk_size: ::core::option::Option<i64>,
    /// The assigned IP addresses for the instance.
    #[prost(message, repeated, tag = "11")]
    pub ip_addresses: ::prost::alloc::vec::Vec<IpMapping>,
    /// SSL configuration.
    #[prost(message, optional, tag = "12")]
    pub server_ca_cert: ::core::option::Option<SslCert>,
    /// The instance type. This can be one of the following:
    /// *  **CLOUD_SQL_INSTANCE**: A Cloud SQL instance that is not replicating
    /// from a primary instance.
    /// *  **ON_PREMISES_INSTANCE**: An instance running on the customer's
    /// premises.
    /// *  **READ_REPLICA_INSTANCE**: A Cloud SQL instance configured as a
    /// read-replica.
    #[prost(enumeration = "SqlInstanceType", tag = "13")]
    pub instance_type: i32,
    /// The project ID of the project containing the Cloud SQL instance. The Google
    /// apps domain is prefixed if applicable.
    #[prost(string, tag = "14")]
    pub project: ::prost::alloc::string::String,
    /// The IPv6 address assigned to the instance.
    /// (Deprecated) This property was applicable only
    /// to First Generation instances.
    #[deprecated]
    #[prost(string, tag = "15")]
    pub ipv6_address: ::prost::alloc::string::String,
    /// The service account email address assigned to the instance. <br>This
    /// property is read-only.
    #[prost(string, tag = "16")]
    pub service_account_email_address: ::prost::alloc::string::String,
    /// Configuration specific to on-premises instances.
    #[prost(message, optional, tag = "17")]
    pub on_premises_configuration: ::core::option::Option<OnPremisesConfiguration>,
    /// Configuration specific to failover replicas and read replicas.
    #[prost(message, optional, tag = "18")]
    pub replica_configuration: ::core::option::Option<ReplicaConfiguration>,
    /// The backend type.
    /// **SECOND_GEN**: Cloud SQL database instance.
    /// **EXTERNAL**: A database server that is not managed by Google.
    ///
    /// This property is read-only; use the **tier** property in the **settings**
    /// object to determine the database type.
    #[prost(enumeration = "SqlBackendType", tag = "19")]
    pub backend_type: i32,
    /// The URI of this resource.
    #[prost(string, tag = "20")]
    pub self_link: ::prost::alloc::string::String,
    /// If the instance state is SUSPENDED, the reason for the suspension.
    #[prost(enumeration = "SqlSuspensionReason", repeated, tag = "21")]
    pub suspension_reason: ::prost::alloc::vec::Vec<i32>,
    /// Connection name of the Cloud SQL instance used in connection strings.
    #[prost(string, tag = "22")]
    pub connection_name: ::prost::alloc::string::String,
    /// Name of the Cloud SQL instance. This does not include the project ID.
    #[prost(string, tag = "23")]
    pub name: ::prost::alloc::string::String,
    /// The geographical region. Can be:
    /// *  **us-central** (**FIRST_GEN** instances only)
    /// *  **us-central1** (**SECOND_GEN** instances only)
    /// *  **asia-east1** or **europe-west1**.
    ///
    /// Defaults to **us-central** or **us-central1** depending on the instance
    /// type. The region cannot be changed after instance creation.
    #[prost(string, tag = "24")]
    pub region: ::prost::alloc::string::String,
    /// The Compute Engine zone that the instance is currently serving from. This
    /// value could be different from the zone that was specified when the instance
    /// was created if the instance has failed over to its secondary zone.
    #[prost(string, tag = "25")]
    pub gce_zone: ::prost::alloc::string::String,
    /// The Compute Engine zone that the failover instance is currently serving
    /// from for a regional instance. This value could be different
    /// from the zone that was specified when the instance
    /// was created if the instance has failed over to its secondary/failover zone.
    /// Reserved for future use.
    #[prost(string, tag = "34")]
    pub secondary_gce_zone: ::prost::alloc::string::String,
    /// Disk encryption configuration specific to an instance.
    #[prost(message, optional, tag = "26")]
    pub disk_encryption_configuration: ::core::option::Option<
        DiskEncryptionConfiguration,
    >,
    /// Disk encryption status specific to an instance.
    #[prost(message, optional, tag = "27")]
    pub disk_encryption_status: ::core::option::Option<DiskEncryptionStatus>,
    /// Initial root password. Use only on creation.
    #[prost(string, tag = "29")]
    pub root_password: ::prost::alloc::string::String,
    /// The start time of any upcoming scheduled maintenance for this instance.
    #[prost(message, optional, tag = "30")]
    pub scheduled_maintenance: ::core::option::Option<
        database_instance::SqlScheduledMaintenance,
    >,
    /// The status indicating if instance satisfiesPzs.
    /// Reserved for future use.
    #[prost(message, optional, tag = "35")]
    pub satisfies_pzs: ::core::option::Option<bool>,
    /// This field represents the report generated by the proactive database
    /// wellness job for OutOfDisk issues.
    /// *  Writers:
    ///    *  the proactive database wellness job for OOD.
    /// *  Readers:
    ///    *  the proactive database wellness job
    #[prost(message, optional, tag = "38")]
    pub out_of_disk_report: ::core::option::Option<
        database_instance::SqlOutOfDiskReport,
    >,
    /// Output only. The time when the instance was created in
    /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// **2012-11-15T16:19:00.094Z**.
    #[prost(message, optional, tag = "39")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `DatabaseInstance`.
pub mod database_instance {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlFailoverReplica {
        /// The name of the failover replica. If specified at instance creation, a
        /// failover replica is created for the instance. The name
        /// doesn't include the project ID.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The availability status of the failover replica. A false status indicates
        /// that the failover replica is out of sync. The primary instance can only
        /// failover to the failover replica when the status is true.
        #[prost(message, optional, tag = "2")]
        pub available: ::core::option::Option<bool>,
    }
    /// Any scheduled maintenancce for this instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlScheduledMaintenance {
        /// The start time of any upcoming scheduled maintenance for this instance.
        #[prost(message, optional, tag = "1")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        #[deprecated]
        #[prost(bool, tag = "2")]
        pub can_defer: bool,
        /// If the scheduled maintenance can be rescheduled.
        #[prost(bool, tag = "3")]
        pub can_reschedule: bool,
        /// Maintenance cannot be rescheduled to start beyond this deadline.
        #[prost(message, optional, tag = "4")]
        pub schedule_deadline_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// This message wraps up the information written by out-of-disk detection job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlOutOfDiskReport {
        /// This field represents the state generated by the proactive database
        /// wellness job for OutOfDisk issues.
        /// *  Writers:
        ///    *  the proactive database wellness job for OOD.
        /// *  Readers:
        ///    *  the proactive database wellness job
        #[prost(
            enumeration = "sql_out_of_disk_report::SqlOutOfDiskState",
            optional,
            tag = "1"
        )]
        pub sql_out_of_disk_state: ::core::option::Option<i32>,
        /// The minimum recommended increase size in GigaBytes
        /// This field is consumed by the frontend
        /// *  Writers:
        ///    *  the proactive database wellness job for OOD.
        /// *  Readers:
        #[prost(int32, optional, tag = "2")]
        pub sql_min_recommended_increase_size_gb: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `SqlOutOfDiskReport`.
    pub mod sql_out_of_disk_report {
        /// This enum lists all possible states regarding out-of-disk issues.
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
        pub enum SqlOutOfDiskState {
            /// Unspecified state
            Unspecified = 0,
            /// The instance has plenty space on data disk
            Normal = 1,
            /// Data disk is almost used up. It is shutdown to prevent data
            /// corruption.
            SoftShutdown = 2,
        }
        impl SqlOutOfDiskState {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SqlOutOfDiskState::Unspecified => "SQL_OUT_OF_DISK_STATE_UNSPECIFIED",
                    SqlOutOfDiskState::Normal => "NORMAL",
                    SqlOutOfDiskState::SoftShutdown => "SOFT_SHUTDOWN",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SQL_OUT_OF_DISK_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "NORMAL" => Some(Self::Normal),
                    "SOFT_SHUTDOWN" => Some(Self::SoftShutdown),
                    _ => None,
                }
            }
        }
    }
    /// The current serving state of the database instance.
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
    pub enum SqlInstanceState {
        /// The state of the instance is unknown.
        Unspecified = 0,
        /// The instance is running, or has been stopped by owner.
        Runnable = 1,
        /// The instance is not available, for example due to problems with billing.
        Suspended = 2,
        /// The instance is being deleted.
        PendingDelete = 3,
        /// The instance is being created.
        PendingCreate = 4,
        /// The instance is down for maintenance.
        Maintenance = 5,
        /// The creation of the instance failed or a fatal error occurred during
        /// maintenance.
        Failed = 6,
        /// The instance is under maintenance operations and the database is
        /// available.
        OnlineMaintenance = 7,
    }
    impl SqlInstanceState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlInstanceState::Unspecified => "SQL_INSTANCE_STATE_UNSPECIFIED",
                SqlInstanceState::Runnable => "RUNNABLE",
                SqlInstanceState::Suspended => "SUSPENDED",
                SqlInstanceState::PendingDelete => "PENDING_DELETE",
                SqlInstanceState::PendingCreate => "PENDING_CREATE",
                SqlInstanceState::Maintenance => "MAINTENANCE",
                SqlInstanceState::Failed => "FAILED",
                SqlInstanceState::OnlineMaintenance => "ONLINE_MAINTENANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_INSTANCE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNABLE" => Some(Self::Runnable),
                "SUSPENDED" => Some(Self::Suspended),
                "PENDING_DELETE" => Some(Self::PendingDelete),
                "PENDING_CREATE" => Some(Self::PendingCreate),
                "MAINTENANCE" => Some(Self::Maintenance),
                "FAILED" => Some(Self::Failed),
                "ONLINE_MAINTENANCE" => Some(Self::OnlineMaintenance),
                _ => None,
            }
        }
    }
}
/// Reschedule options for maintenance windows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRescheduleMaintenanceRequestBody {
    /// Required. The type of the reschedule the user wants.
    #[prost(message, optional, tag = "3")]
    pub reschedule: ::core::option::Option<
        sql_instances_reschedule_maintenance_request_body::Reschedule,
    >,
}
/// Nested message and enum types in `SqlInstancesRescheduleMaintenanceRequestBody`.
pub mod sql_instances_reschedule_maintenance_request_body {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reschedule {
        /// Required. The type of the reschedule.
        #[prost(enumeration = "RescheduleType", tag = "1")]
        pub reschedule_type: i32,
        /// Optional. Timestamp when the maintenance shall be rescheduled to if
        /// reschedule_type=SPECIFIC_TIME, in
        /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
        /// **2012-11-15T16:19:00.094Z**.
        #[prost(message, optional, tag = "2")]
        pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    }
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
    pub enum RescheduleType {
        Unspecified = 0,
        /// Reschedules maintenance to happen now (within 5 minutes).
        Immediate = 1,
        /// Reschedules maintenance to occur within one week from the originally
        /// scheduled day and time.
        NextAvailableWindow = 2,
        /// Reschedules maintenance to a specific time and day.
        SpecificTime = 3,
    }
    impl RescheduleType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RescheduleType::Unspecified => "RESCHEDULE_TYPE_UNSPECIFIED",
                RescheduleType::Immediate => "IMMEDIATE",
                RescheduleType::NextAvailableWindow => "NEXT_AVAILABLE_WINDOW",
                RescheduleType::SpecificTime => "SPECIFIC_TIME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESCHEDULE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "IMMEDIATE" => Some(Self::Immediate),
                "NEXT_AVAILABLE_WINDOW" => Some(Self::NextAvailableWindow),
                "SPECIFIC_TIME" => Some(Self::SpecificTime),
                _ => None,
            }
        }
    }
}
/// Database instance demote primary instance context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemoteMasterContext {
    /// This is always **sql#demoteMasterContext**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Verify GTID consistency for demote operation. Default value:
    /// **True**. Setting this flag to false enables you to bypass GTID consistency
    /// check between on-premises primary instance and Cloud SQL instance during
    /// the demotion operation but also exposes you to the risk of future
    /// replication failures. Change the value only if you know the reason for the
    /// GTID divergence and are confident that doing so will not cause any
    /// replication issues.
    #[prost(message, optional, tag = "2")]
    pub verify_gtid_consistency: ::core::option::Option<bool>,
    /// The name of the instance which will act as on-premises primary instance
    /// in the replication setup.
    #[prost(string, tag = "3")]
    pub master_instance_name: ::prost::alloc::string::String,
    /// Configuration specific to read-replicas replicating from the on-premises
    /// primary instance.
    #[prost(message, optional, tag = "4")]
    pub replica_configuration: ::core::option::Option<DemoteMasterConfiguration>,
    /// Flag to skip replication setup on the instance.
    #[prost(bool, tag = "5")]
    pub skip_replication_setup: bool,
}
/// Database instance failover context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailoverContext {
    /// The current settings version of this instance. Request will be rejected if
    /// this version doesn't match the current settings version.
    #[prost(int64, tag = "1")]
    pub settings_version: i64,
    /// This is always **sql#failoverContext**.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// Database instance restore from backup context.
/// Backup context contains source instance id and project id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreBackupContext {
    /// This is always **sql#restoreBackupContext**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The ID of the backup run to restore from.
    #[prost(int64, tag = "2")]
    pub backup_run_id: i64,
    /// The ID of the instance that the backup was taken from.
    #[prost(string, tag = "3")]
    pub instance_id: ::prost::alloc::string::String,
    /// The full project ID of the source instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
/// Instance rotate server CA context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RotateServerCaContext {
    /// This is always **sql#rotateServerCaContext**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The fingerprint of the next version to be rotated to. If left unspecified,
    /// will be rotated to the most recently added server CA version.
    #[prost(string, tag = "2")]
    pub next_version: ::prost::alloc::string::String,
}
/// Database Instance truncate log context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TruncateLogContext {
    /// This is always **sql#truncateLogContext**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The type of log to truncate. Valid values are **MYSQL_GENERAL_TABLE** and
    /// **MYSQL_SLOW_TABLE**.
    #[prost(string, tag = "2")]
    pub log_type: ::prost::alloc::string::String,
}
/// External primary instance migration setting error/warning.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlExternalSyncSettingError {
    /// Can be **sql#externalSyncSettingError** or
    /// **sql#externalSyncSettingWarning**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Identifies the specific error that occurred.
    #[prost(
        enumeration = "sql_external_sync_setting_error::SqlExternalSyncSettingErrorType",
        tag = "2"
    )]
    pub r#type: i32,
    /// Additional information about the error encountered.
    #[prost(string, tag = "3")]
    pub detail: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SqlExternalSyncSettingError`.
pub mod sql_external_sync_setting_error {
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
    pub enum SqlExternalSyncSettingErrorType {
        Unspecified = 0,
        ConnectionFailure = 1,
        BinlogNotEnabled = 2,
        IncompatibleDatabaseVersion = 3,
        ReplicaAlreadySetup = 4,
        InsufficientPrivilege = 5,
        /// Unsupported migration type.
        UnsupportedMigrationType = 6,
        /// No pglogical extension installed on databases, applicable for postgres.
        NoPglogicalInstalled = 7,
        /// pglogical node already exists on databases, applicable for postgres.
        PglogicalNodeAlreadyExists = 8,
        /// The value of parameter wal_level is not set to logical.
        InvalidWalLevel = 9,
        /// The value of parameter shared_preload_libraries does not include
        /// pglogical.
        InvalidSharedPreloadLibrary = 10,
        /// The value of parameter max_replication_slots is not sufficient.
        InsufficientMaxReplicationSlots = 11,
        /// The value of parameter max_wal_senders is not sufficient.
        InsufficientMaxWalSenders = 12,
        /// The value of parameter max_worker_processes is not sufficient.
        InsufficientMaxWorkerProcesses = 13,
        /// Extensions installed are either not supported or having unsupported
        /// versions
        UnsupportedExtensions = 14,
        /// The value of parameter rds.logical_replication is not set to 1.
        InvalidRdsLogicalReplication = 15,
        /// The primary instance logging setup doesn't allow EM sync.
        InvalidLoggingSetup = 16,
        /// The primary instance database parameter setup doesn't allow EM sync.
        InvalidDbParam = 17,
        /// The gtid_mode is not supported, applicable for MySQL.
        UnsupportedGtidMode = 18,
        /// SQL Server Agent is not running.
        SqlserverAgentNotRunning = 19,
        /// The table definition is not support due to missing primary key or replica
        /// identity, applicable for postgres.
        UnsupportedTableDefinition = 20,
        /// The customer has a definer that will break EM setup.
        UnsupportedDefiner = 21,
        /// SQL Server @@SERVERNAME does not match actual host name
        SqlserverServernameMismatch = 22,
        /// The primary instance has been setup and will fail the setup.
        PrimaryAlreadySetup = 23,
        /// The primary instance has unsupported binary log format.
        UnsupportedBinlogFormat = 24,
        /// The primary instance's binary log retention setting.
        BinlogRetentionSetting = 25,
    }
    impl SqlExternalSyncSettingErrorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlExternalSyncSettingErrorType::Unspecified => {
                    "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED"
                }
                SqlExternalSyncSettingErrorType::ConnectionFailure => {
                    "CONNECTION_FAILURE"
                }
                SqlExternalSyncSettingErrorType::BinlogNotEnabled => "BINLOG_NOT_ENABLED",
                SqlExternalSyncSettingErrorType::IncompatibleDatabaseVersion => {
                    "INCOMPATIBLE_DATABASE_VERSION"
                }
                SqlExternalSyncSettingErrorType::ReplicaAlreadySetup => {
                    "REPLICA_ALREADY_SETUP"
                }
                SqlExternalSyncSettingErrorType::InsufficientPrivilege => {
                    "INSUFFICIENT_PRIVILEGE"
                }
                SqlExternalSyncSettingErrorType::UnsupportedMigrationType => {
                    "UNSUPPORTED_MIGRATION_TYPE"
                }
                SqlExternalSyncSettingErrorType::NoPglogicalInstalled => {
                    "NO_PGLOGICAL_INSTALLED"
                }
                SqlExternalSyncSettingErrorType::PglogicalNodeAlreadyExists => {
                    "PGLOGICAL_NODE_ALREADY_EXISTS"
                }
                SqlExternalSyncSettingErrorType::InvalidWalLevel => "INVALID_WAL_LEVEL",
                SqlExternalSyncSettingErrorType::InvalidSharedPreloadLibrary => {
                    "INVALID_SHARED_PRELOAD_LIBRARY"
                }
                SqlExternalSyncSettingErrorType::InsufficientMaxReplicationSlots => {
                    "INSUFFICIENT_MAX_REPLICATION_SLOTS"
                }
                SqlExternalSyncSettingErrorType::InsufficientMaxWalSenders => {
                    "INSUFFICIENT_MAX_WAL_SENDERS"
                }
                SqlExternalSyncSettingErrorType::InsufficientMaxWorkerProcesses => {
                    "INSUFFICIENT_MAX_WORKER_PROCESSES"
                }
                SqlExternalSyncSettingErrorType::UnsupportedExtensions => {
                    "UNSUPPORTED_EXTENSIONS"
                }
                SqlExternalSyncSettingErrorType::InvalidRdsLogicalReplication => {
                    "INVALID_RDS_LOGICAL_REPLICATION"
                }
                SqlExternalSyncSettingErrorType::InvalidLoggingSetup => {
                    "INVALID_LOGGING_SETUP"
                }
                SqlExternalSyncSettingErrorType::InvalidDbParam => "INVALID_DB_PARAM",
                SqlExternalSyncSettingErrorType::UnsupportedGtidMode => {
                    "UNSUPPORTED_GTID_MODE"
                }
                SqlExternalSyncSettingErrorType::SqlserverAgentNotRunning => {
                    "SQLSERVER_AGENT_NOT_RUNNING"
                }
                SqlExternalSyncSettingErrorType::UnsupportedTableDefinition => {
                    "UNSUPPORTED_TABLE_DEFINITION"
                }
                SqlExternalSyncSettingErrorType::UnsupportedDefiner => {
                    "UNSUPPORTED_DEFINER"
                }
                SqlExternalSyncSettingErrorType::SqlserverServernameMismatch => {
                    "SQLSERVER_SERVERNAME_MISMATCH"
                }
                SqlExternalSyncSettingErrorType::PrimaryAlreadySetup => {
                    "PRIMARY_ALREADY_SETUP"
                }
                SqlExternalSyncSettingErrorType::UnsupportedBinlogFormat => {
                    "UNSUPPORTED_BINLOG_FORMAT"
                }
                SqlExternalSyncSettingErrorType::BinlogRetentionSetting => {
                    "BINLOG_RETENTION_SETTING"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED" => {
                    Some(Self::Unspecified)
                }
                "CONNECTION_FAILURE" => Some(Self::ConnectionFailure),
                "BINLOG_NOT_ENABLED" => Some(Self::BinlogNotEnabled),
                "INCOMPATIBLE_DATABASE_VERSION" => {
                    Some(Self::IncompatibleDatabaseVersion)
                }
                "REPLICA_ALREADY_SETUP" => Some(Self::ReplicaAlreadySetup),
                "INSUFFICIENT_PRIVILEGE" => Some(Self::InsufficientPrivilege),
                "UNSUPPORTED_MIGRATION_TYPE" => Some(Self::UnsupportedMigrationType),
                "NO_PGLOGICAL_INSTALLED" => Some(Self::NoPglogicalInstalled),
                "PGLOGICAL_NODE_ALREADY_EXISTS" => Some(Self::PglogicalNodeAlreadyExists),
                "INVALID_WAL_LEVEL" => Some(Self::InvalidWalLevel),
                "INVALID_SHARED_PRELOAD_LIBRARY" => {
                    Some(Self::InvalidSharedPreloadLibrary)
                }
                "INSUFFICIENT_MAX_REPLICATION_SLOTS" => {
                    Some(Self::InsufficientMaxReplicationSlots)
                }
                "INSUFFICIENT_MAX_WAL_SENDERS" => Some(Self::InsufficientMaxWalSenders),
                "INSUFFICIENT_MAX_WORKER_PROCESSES" => {
                    Some(Self::InsufficientMaxWorkerProcesses)
                }
                "UNSUPPORTED_EXTENSIONS" => Some(Self::UnsupportedExtensions),
                "INVALID_RDS_LOGICAL_REPLICATION" => {
                    Some(Self::InvalidRdsLogicalReplication)
                }
                "INVALID_LOGGING_SETUP" => Some(Self::InvalidLoggingSetup),
                "INVALID_DB_PARAM" => Some(Self::InvalidDbParam),
                "UNSUPPORTED_GTID_MODE" => Some(Self::UnsupportedGtidMode),
                "SQLSERVER_AGENT_NOT_RUNNING" => Some(Self::SqlserverAgentNotRunning),
                "UNSUPPORTED_TABLE_DEFINITION" => Some(Self::UnsupportedTableDefinition),
                "UNSUPPORTED_DEFINER" => Some(Self::UnsupportedDefiner),
                "SQLSERVER_SERVERNAME_MISMATCH" => {
                    Some(Self::SqlserverServernameMismatch)
                }
                "PRIMARY_ALREADY_SETUP" => Some(Self::PrimaryAlreadySetup),
                "UNSUPPORTED_BINLOG_FORMAT" => Some(Self::UnsupportedBinlogFormat),
                "BINLOG_RETENTION_SETTING" => Some(Self::BinlogRetentionSetting),
                _ => None,
            }
        }
    }
}
/// On-premises instance configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnPremisesConfiguration {
    /// The host and port of the on-premises instance in host:port format
    #[prost(string, tag = "1")]
    pub host_port: ::prost::alloc::string::String,
    /// This is always **sql#onPremisesConfiguration**.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
    /// The username for connecting to on-premises instance.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// The password for connecting to on-premises instance.
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    /// PEM representation of the trusted CA's x509 certificate.
    #[prost(string, tag = "5")]
    pub ca_certificate: ::prost::alloc::string::String,
    /// PEM representation of the replica's x509 certificate.
    #[prost(string, tag = "6")]
    pub client_certificate: ::prost::alloc::string::String,
    /// PEM representation of the replica's private key. The corresponsing public
    /// key is encoded in the client's certificate.
    #[prost(string, tag = "7")]
    pub client_key: ::prost::alloc::string::String,
    /// The dump file to create the Cloud SQL replica.
    #[prost(string, tag = "8")]
    pub dump_file_path: ::prost::alloc::string::String,
    /// The reference to Cloud SQL instance if the source is Cloud SQL.
    #[prost(message, optional, tag = "15")]
    pub source_instance: ::core::option::Option<InstanceReference>,
}
/// Read-replica configuration for connecting to the primary instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaConfiguration {
    /// This is always **sql#replicaConfiguration**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// MySQL specific configuration when replicating from a MySQL on-premises
    /// primary instance. Replication configuration information such as the
    /// username, password, certificates, and keys are not stored in the instance
    /// metadata. The configuration information is used only to set up the
    /// replication connection and is stored by MySQL in a file named
    /// **master.info** in the data directory.
    #[prost(message, optional, tag = "2")]
    pub mysql_replica_configuration: ::core::option::Option<MySqlReplicaConfiguration>,
    /// Specifies if the replica is the failover target. If the field is set to
    /// **true** the replica will be designated as a failover replica. In case the
    /// primary instance fails, the replica instance will be promoted as the new
    /// primary instance. Only one replica can be specified as failover target, and
    /// the replica has to be in different zone with the primary instance.
    #[prost(message, optional, tag = "3")]
    pub failover_target: ::core::option::Option<bool>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlInstanceType {
    /// This is an unknown Cloud SQL instance type.
    Unspecified = 0,
    /// A regular Cloud SQL instance.
    CloudSqlInstance = 1,
    /// An instance running on the customer's premises that is not managed by
    /// Cloud SQL.
    OnPremisesInstance = 2,
    /// A Cloud SQL instance acting as a read-replica.
    ReadReplicaInstance = 3,
}
impl SqlInstanceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlInstanceType::Unspecified => "SQL_INSTANCE_TYPE_UNSPECIFIED",
            SqlInstanceType::CloudSqlInstance => "CLOUD_SQL_INSTANCE",
            SqlInstanceType::OnPremisesInstance => "ON_PREMISES_INSTANCE",
            SqlInstanceType::ReadReplicaInstance => "READ_REPLICA_INSTANCE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_INSTANCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CLOUD_SQL_INSTANCE" => Some(Self::CloudSqlInstance),
            "ON_PREMISES_INSTANCE" => Some(Self::OnPremisesInstance),
            "READ_REPLICA_INSTANCE" => Some(Self::ReadReplicaInstance),
            _ => None,
        }
    }
}
/// The suspension reason of the database instance if the state is SUSPENDED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlSuspensionReason {
    /// This is an unknown suspension reason.
    Unspecified = 0,
    /// The instance is suspended due to billing issues (for example:, GCP account
    /// issue)
    BillingIssue = 2,
    /// The instance is suspended due to illegal content (for example:, child
    /// pornography, copyrighted material, etc.).
    LegalIssue = 3,
    /// The instance is causing operational issues (for example:, causing the
    /// database to crash).
    OperationalIssue = 4,
    /// The KMS key used by the instance is either revoked or denied access to
    KmsKeyIssue = 5,
}
impl SqlSuspensionReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SqlSuspensionReason::Unspecified => "SQL_SUSPENSION_REASON_UNSPECIFIED",
            SqlSuspensionReason::BillingIssue => "BILLING_ISSUE",
            SqlSuspensionReason::LegalIssue => "LEGAL_ISSUE",
            SqlSuspensionReason::OperationalIssue => "OPERATIONAL_ISSUE",
            SqlSuspensionReason::KmsKeyIssue => "KMS_KEY_ISSUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SQL_SUSPENSION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
            "BILLING_ISSUE" => Some(Self::BillingIssue),
            "LEGAL_ISSUE" => Some(Self::LegalIssue),
            "OPERATIONAL_ISSUE" => Some(Self::OperationalIssue),
            "KMS_KEY_ISSUE" => Some(Self::KmsKeyIssue),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod sql_instances_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage Cloud SQL instances.
    #[derive(Debug, Clone)]
    pub struct SqlInstancesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlInstancesServiceClient<T>
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
        ) -> SqlInstancesServiceClient<InterceptedService<T, F>>
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
            SqlInstancesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Adds a new trusted Certificate Authority (CA) version for the specified
        /// instance. Required to prepare for a certificate rotation. If a CA version
        /// was previously added but never used in a certificate rotation, this
        /// operation replaces that version. There cannot be more than one CA version
        /// waiting to be rotated in.
        pub async fn add_server_ca(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesAddServerCaRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/AddServerCa",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "AddServerCa",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Cloud SQL instance as a clone of the source instance. Using this
        /// operation might cause your instance to restart.
        pub async fn clone(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesCloneRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Clone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Clone"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Cloud SQL instance.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Delete"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Demotes the stand-alone instance to be a Cloud SQL read replica for an
        /// external database server.
        pub async fn demote_master(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesDemoteMasterRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/DemoteMaster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "DemoteMaster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports data from a Cloud SQL instance to a Cloud Storage bucket as a SQL
        /// dump or CSV file.
        pub async fn export(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesExportRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Export",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Export"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Initiates a manual failover of a high availability (HA) primary instance
        /// to a standby instance, which becomes the primary instance. Users are
        /// then rerouted to the new primary. For more information, see the
        /// [Overview of high
        /// availability](https://cloud.google.com/sql/docs/mysql/high-availability)
        /// page in the Cloud SQL documentation.
        /// If using Legacy HA (MySQL only), this causes the instance to failover to
        /// its failover replica instance.
        pub async fn failover(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesFailoverRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Failover",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "Failover",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a resource containing information about a Cloud SQL instance.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DatabaseInstance>,
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
                "/google.cloud.sql.v1.SqlInstancesService/Get",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Get"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports data into a Cloud SQL instance from a SQL dump  or CSV file in
        /// Cloud Storage.
        pub async fn import(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesImportRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Import",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Import"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Cloud SQL instance.
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesInsertRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Insert",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Insert"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists instances under a given project.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InstancesListResponse>,
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
                "/google.cloud.sql.v1.SqlInstancesService/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "List"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all of the trusted Certificate Authorities (CAs) for the specified
        /// instance. There can be up to three CAs listed: the CA that was used to sign
        /// the certificate that is currently in use, a CA that has been added but not
        /// yet used to sign a certificate, and a CA used to sign a certificate that
        /// has previously rotated out.
        pub async fn list_server_cas(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesListServerCasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InstancesListServerCasResponse>,
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
                "/google.cloud.sql.v1.SqlInstancesService/ListServerCas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "ListServerCas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates settings of a Cloud SQL instance.
        /// This method supports patch semantics.
        pub async fn patch(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesPatchRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Patch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Patch"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Promotes the read replica instance to be a stand-alone Cloud SQL instance.
        /// Using this operation might cause your instance to restart.
        pub async fn promote_replica(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesPromoteReplicaRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/PromoteReplica",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "PromoteReplica",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes all client certificates and generates a new server SSL certificate
        /// for the instance.
        pub async fn reset_ssl_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesResetSslConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/ResetSslConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "ResetSslConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restarts a Cloud SQL instance.
        pub async fn restart(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesRestartRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Restart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Restart"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restores a backup of a Cloud SQL instance. Using this operation might cause
        /// your instance to restart.
        pub async fn restore_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesRestoreBackupRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/RestoreBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "RestoreBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Rotates the server certificate to one signed by the Certificate Authority
        /// (CA) version previously added with the addServerCA method.
        pub async fn rotate_server_ca(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesRotateServerCaRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/RotateServerCa",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "RotateServerCa",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Starts the replication in the read replica instance.
        pub async fn start_replica(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesStartReplicaRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/StartReplica",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "StartReplica",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Stops the replication in the read replica instance.
        pub async fn stop_replica(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesStopReplicaRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/StopReplica",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "StopReplica",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Truncate MySQL general and slow query log tables
        /// MySQL only.
        pub async fn truncate_log(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesTruncateLogRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/TruncateLog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "TruncateLog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates settings of a Cloud SQL instance. Using this operation might cause
        /// your instance to restart.
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesUpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/Update",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlInstancesService", "Update"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates a short-lived X509 certificate containing the provided public key
        /// and signed by a private key specific to the target instance. Users may use
        /// the certificate to authenticate as themselves when connecting to the
        /// database.
        pub async fn create_ephemeral(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SqlInstancesCreateEphemeralCertRequest,
            >,
        ) -> std::result::Result<tonic::Response<super::SslCert>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/CreateEphemeral",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "CreateEphemeral",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Reschedules the maintenance on the given instance.
        pub async fn reschedule_maintenance(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SqlInstancesRescheduleMaintenanceRequest,
            >,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/RescheduleMaintenance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "RescheduleMaintenance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Verify External primary instance external sync settings.
        pub async fn verify_external_sync_settings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SqlInstancesVerifyExternalSyncSettingsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SqlInstancesVerifyExternalSyncSettingsResponse>,
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
                "/google.cloud.sql.v1.SqlInstancesService/VerifyExternalSyncSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "VerifyExternalSyncSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Start External primary instance migration.
        pub async fn start_external_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesStartExternalSyncRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlInstancesService/StartExternalSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.sql.v1.SqlInstancesService",
                        "StartExternalSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Operations get request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlOperationsGetRequest {
    /// Instance operation ID.
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// Operations list request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlOperationsListRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Maximum number of operations per response.
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
/// Operations list response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsListResponse {
    /// This is always **sql#operationsList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of operation resources.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Operation>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod sql_operations_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to fetch operations for database instances.
    #[derive(Debug, Clone)]
    pub struct SqlOperationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlOperationsServiceClient<T>
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
        ) -> SqlOperationsServiceClient<InterceptedService<T, F>>
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
            SqlOperationsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves an instance operation that has been performed on an instance.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlOperationsGetRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlOperationsService/Get",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlOperationsService", "Get"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all instance operations that have been performed on the given Cloud
        /// SQL instance in the reverse chronological order of the start time.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlOperationsListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OperationsListResponse>,
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
                "/google.cloud.sql.v1.SqlOperationsService/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlOperationsService", "List"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlSslCertsDeleteRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Sha1 FingerPrint.
    #[prost(string, tag = "3")]
    pub sha1_fingerprint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlSslCertsGetRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Sha1 FingerPrint.
    #[prost(string, tag = "3")]
    pub sha1_fingerprint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlSslCertsInsertRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<SslCertsInsertRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlSslCertsListRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// SslCerts insert request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertsInsertRequest {
    /// User supplied name.  Must be a distinct name from the other certificates
    /// for this instance.
    #[prost(string, tag = "1")]
    pub common_name: ::prost::alloc::string::String,
}
/// SslCert insert response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertsInsertResponse {
    /// This is always **sql#sslCertsInsert**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The operation to track the ssl certs insert request.
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<Operation>,
    /// The server Certificate Authority's certificate.  If this is missing you can
    /// force a new one to be generated by calling resetSslConfig method on
    /// instances resource.
    #[prost(message, optional, tag = "3")]
    pub server_ca_cert: ::core::option::Option<SslCert>,
    /// The new client certificate and private key.
    #[prost(message, optional, tag = "4")]
    pub client_cert: ::core::option::Option<SslCertDetail>,
}
/// SslCerts list response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertsListResponse {
    /// This is always **sql#sslCertsList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of client certificates for the instance.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<SslCert>,
}
/// Generated client implementations.
pub mod sql_ssl_certs_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage SSL certs for Cloud SQL instances.
    #[derive(Debug, Clone)]
    pub struct SqlSslCertsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlSslCertsServiceClient<T>
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
        ) -> SqlSslCertsServiceClient<InterceptedService<T, F>>
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
            SqlSslCertsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Deletes the SSL certificate. For First Generation instances, the
        /// certificate remains valid until the instance is restarted.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlSslCertsDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlSslCertsService/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlSslCertsService", "Delete"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a particular SSL certificate.  Does not include the private key
        /// (required for usage).  The private key must be saved from the response to
        /// initial creation.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlSslCertsGetRequest>,
        ) -> std::result::Result<tonic::Response<super::SslCert>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlSslCertsService/Get",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlSslCertsService", "Get"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an SSL certificate and returns it along with the private key and
        /// server certificate authority.  The new certificate will not be usable until
        /// the instance is restarted.
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlSslCertsInsertRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SslCertsInsertResponse>,
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
                "/google.cloud.sql.v1.SqlSslCertsService/Insert",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlSslCertsService", "Insert"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all of the current SSL certificates for the instance.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlSslCertsListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SslCertsListResponse>,
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
                "/google.cloud.sql.v1.SqlSslCertsService/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlSslCertsService", "List"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Tiers list request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlTiersListRequest {
    /// Project ID of the project for which to list tiers.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
}
/// Tiers list response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TiersListResponse {
    /// This is always **sql#tiersList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of tiers.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Tier>,
}
/// A Google Cloud SQL service tier resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tier {
    /// An identifier for the machine type, for example, db-custom-1-3840. For
    /// related information, see \[Pricing\](/sql/pricing).
    #[prost(string, tag = "1")]
    pub tier: ::prost::alloc::string::String,
    /// The maximum RAM usage of this tier in bytes.
    #[prost(int64, tag = "2")]
    pub ram: i64,
    /// This is always **sql#tier**.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    /// The maximum disk size of this tier in bytes.
    #[prost(int64, tag = "4")]
    pub disk_quota: i64,
    /// The applicable regions for this tier.
    #[prost(string, repeated, tag = "5")]
    pub region: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod sql_tiers_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for providing machine types (tiers) for Cloud SQL instances.
    #[derive(Debug, Clone)]
    pub struct SqlTiersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlTiersServiceClient<T>
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
        ) -> SqlTiersServiceClient<InterceptedService<T, F>>
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
            SqlTiersServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all available machine types (tiers) for Cloud SQL, for example,
        /// db-custom-1-3840. For more information, see
        /// https://cloud.google.com/sql/pricing.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlTiersListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TiersListResponse>,
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
                "/google.cloud.sql.v1.SqlTiersService/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.sql.v1.SqlTiersService", "List"));
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlUsersDeleteRequest {
    /// Host of the user in the instance.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Name of the user in the instance.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlUsersInsertRequest {
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlUsersListRequest {
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlUsersUpdateRequest {
    /// Optional. Host of the user in the instance.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Name of the user in the instance.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<User>,
}
/// A Cloud SQL user resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// This is always **sql#user**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The password for the user.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    /// This field is deprecated and will be removed from a future version of the
    /// API.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// The name of the user in the Cloud SQL instance. Can be omitted for
    /// **update** since it is already specified in the URL.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// The host name from which the user can connect. For **insert**
    /// operations, host defaults to an empty string. For **update**
    /// operations, host is specified as part of the request URL. The host name
    /// cannot be updated after insertion.
    #[prost(string, tag = "5")]
    pub host: ::prost::alloc::string::String,
    /// The name of the Cloud SQL instance. This does not include the project ID.
    /// Can be omitted for **update** since it is already specified on the
    /// URL.
    #[prost(string, tag = "6")]
    pub instance: ::prost::alloc::string::String,
    /// The project ID of the project containing the Cloud SQL database. The Google
    /// apps domain is prefixed if applicable. Can be omitted for **update** since
    /// it is already specified on the URL.
    #[prost(string, tag = "7")]
    pub project: ::prost::alloc::string::String,
    /// The user type. It determines the method to authenticate the user during
    /// login. The default is the database's built-in user type.
    #[prost(enumeration = "user::SqlUserType", tag = "8")]
    pub r#type: i32,
    /// User details for specific database type
    #[prost(oneof = "user::UserDetails", tags = "9")]
    pub user_details: ::core::option::Option<user::UserDetails>,
}
/// Nested message and enum types in `User`.
pub mod user {
    /// The user type.
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
    pub enum SqlUserType {
        /// The database's built-in user type.
        BuiltIn = 0,
        /// Cloud IAM user.
        CloudIamUser = 1,
        /// Cloud IAM service account.
        CloudIamServiceAccount = 2,
    }
    impl SqlUserType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlUserType::BuiltIn => "BUILT_IN",
                SqlUserType::CloudIamUser => "CLOUD_IAM_USER",
                SqlUserType::CloudIamServiceAccount => "CLOUD_IAM_SERVICE_ACCOUNT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUILT_IN" => Some(Self::BuiltIn),
                "CLOUD_IAM_USER" => Some(Self::CloudIamUser),
                "CLOUD_IAM_SERVICE_ACCOUNT" => Some(Self::CloudIamServiceAccount),
                _ => None,
            }
        }
    }
    /// User details for specific database type
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UserDetails {
        #[prost(message, tag = "9")]
        SqlserverUserDetails(super::SqlServerUserDetails),
    }
}
/// Represents a Sql Server user on the Cloud SQL instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlServerUserDetails {
    /// If the user has been disabled
    #[prost(bool, tag = "1")]
    pub disabled: bool,
    /// The server roles for this user
    #[prost(string, repeated, tag = "2")]
    pub server_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// User list response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsersListResponse {
    /// This is always **sql#usersList**.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of user resources in the instance.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<User>,
    /// An identifier that uniquely identifies the operation. You can use this
    /// identifier to retrieve the Operations resource that has information about
    /// the operation.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod sql_users_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud SQL users service.
    #[derive(Debug, Clone)]
    pub struct SqlUsersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlUsersServiceClient<T>
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
        ) -> SqlUsersServiceClient<InterceptedService<T, F>>
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
            SqlUsersServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Deletes a user from a Cloud SQL instance.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlUsersDeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlUsersService/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlUsersService", "Delete"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new user in a Cloud SQL instance.
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlUsersInsertRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlUsersService/Insert",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlUsersService", "Insert"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists users in the specified Cloud SQL instance.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlUsersListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UsersListResponse>,
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
                "/google.cloud.sql.v1.SqlUsersService/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.sql.v1.SqlUsersService", "List"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing user in a Cloud SQL instance.
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlUsersUpdateRequest>,
        ) -> std::result::Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.cloud.sql.v1.SqlUsersService/Update",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.sql.v1.SqlUsersService", "Update"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
