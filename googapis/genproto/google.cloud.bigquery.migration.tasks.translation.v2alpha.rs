#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetReference {
    /// Required. A unique ID for this dataset, without the project name. The ID
    /// must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
    /// The maximum length is 1,024 characters.
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Optional. The ID of the project containing this dataset.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Mapping between an input and output file to be translated in a subtask.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationFileMapping {
    /// The Cloud Storage path for a file to translation in a subtask.
    #[prost(string, tag = "1")]
    pub input_path: ::prost::alloc::string::String,
    /// The Cloud Storage path to write back the corresponding input file to.
    #[prost(string, tag = "2")]
    pub output_path: ::prost::alloc::string::String,
}
/// The translation task details to capture necessary settings for a translation
/// task and subtask.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationTaskDetails {
    /// The Cloud Storage path for translation input files.
    #[prost(string, tag = "1")]
    pub input_path: ::prost::alloc::string::String,
    /// The Cloud Storage path for translation output files.
    #[prost(string, tag = "2")]
    pub output_path: ::prost::alloc::string::String,
    /// Cloud Storage files to be processed for translation.
    #[prost(message, repeated, tag = "12")]
    pub file_paths: ::prost::alloc::vec::Vec<TranslationFileMapping>,
    /// The Cloud Storage path to DDL files as table schema to assist semantic
    /// translation.
    #[prost(string, tag = "3")]
    pub schema_path: ::prost::alloc::string::String,
    /// The file encoding type.
    #[prost(enumeration = "translation_task_details::FileEncoding", tag = "4")]
    pub file_encoding: i32,
    /// The settings for SQL identifiers.
    #[prost(message, optional, tag = "5")]
    pub identifier_settings: ::core::option::Option<IdentifierSettings>,
    /// The map capturing special tokens to be replaced during translation. The key
    /// is special token in string. The value is the token data type. This is used
    /// to translate SQL query template which contains special token as place
    /// holder. The special token makes a query invalid to parse. This map will be
    /// applied to annotate those special token with types to let parser understand
    /// how to parse them into proper structure with type information.
    #[prost(map = "string, enumeration(translation_task_details::TokenType)", tag = "6")]
    pub special_token_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        i32,
    >,
    /// The filter applied to translation details.
    #[prost(message, optional, tag = "7")]
    pub filter: ::core::option::Option<Filter>,
    /// Specifies the exact name of the bigquery table ("dataset.table") to be used
    /// for surfacing raw translation errors. If the table does not exist, we will
    /// create it. If it already exists and the schema is the same, we will re-use.
    /// If the table exists and the schema is different, we will throw an error.
    #[prost(string, tag = "13")]
    pub translation_exception_table: ::prost::alloc::string::String,
    /// The language specific settings for the translation task.
    #[prost(oneof = "translation_task_details::LanguageOptions", tags = "10, 11")]
    pub language_options: ::core::option::Option<
        translation_task_details::LanguageOptions,
    >,
}
/// Nested message and enum types in `TranslationTaskDetails`.
pub mod translation_task_details {
    /// The file encoding types.
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
    pub enum FileEncoding {
        /// File encoding setting is not specified.
        Unspecified = 0,
        /// File encoding is UTF_8.
        Utf8 = 1,
        /// File encoding is ISO_8859_1.
        Iso88591 = 2,
        /// File encoding is US_ASCII.
        UsAscii = 3,
        /// File encoding is UTF_16.
        Utf16 = 4,
        /// File encoding is UTF_16LE.
        Utf16le = 5,
        /// File encoding is UTF_16BE.
        Utf16be = 6,
    }
    impl FileEncoding {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FileEncoding::Unspecified => "FILE_ENCODING_UNSPECIFIED",
                FileEncoding::Utf8 => "UTF_8",
                FileEncoding::Iso88591 => "ISO_8859_1",
                FileEncoding::UsAscii => "US_ASCII",
                FileEncoding::Utf16 => "UTF_16",
                FileEncoding::Utf16le => "UTF_16LE",
                FileEncoding::Utf16be => "UTF_16BE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILE_ENCODING_UNSPECIFIED" => Some(Self::Unspecified),
                "UTF_8" => Some(Self::Utf8),
                "ISO_8859_1" => Some(Self::Iso88591),
                "US_ASCII" => Some(Self::UsAscii),
                "UTF_16" => Some(Self::Utf16),
                "UTF_16LE" => Some(Self::Utf16le),
                "UTF_16BE" => Some(Self::Utf16be),
                _ => None,
            }
        }
    }
    /// The special token data type.
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
    pub enum TokenType {
        /// Token type is not specified.
        Unspecified = 0,
        /// Token type as string.
        String = 1,
        /// Token type as integer.
        Int64 = 2,
        /// Token type as numeric.
        Numeric = 3,
        /// Token type as boolean.
        Bool = 4,
        /// Token type as float.
        Float64 = 5,
        /// Token type as date.
        Date = 6,
        /// Token type as timestamp.
        Timestamp = 7,
    }
    impl TokenType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TokenType::Unspecified => "TOKEN_TYPE_UNSPECIFIED",
                TokenType::String => "STRING",
                TokenType::Int64 => "INT64",
                TokenType::Numeric => "NUMERIC",
                TokenType::Bool => "BOOL",
                TokenType::Float64 => "FLOAT64",
                TokenType::Date => "DATE",
                TokenType::Timestamp => "TIMESTAMP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TOKEN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STRING" => Some(Self::String),
                "INT64" => Some(Self::Int64),
                "NUMERIC" => Some(Self::Numeric),
                "BOOL" => Some(Self::Bool),
                "FLOAT64" => Some(Self::Float64),
                "DATE" => Some(Self::Date),
                "TIMESTAMP" => Some(Self::Timestamp),
                _ => None,
            }
        }
    }
    /// The language specific settings for the translation task.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LanguageOptions {
        /// The Teradata SQL specific settings for the translation task.
        #[prost(message, tag = "10")]
        TeradataOptions(super::TeradataOptions),
        /// The BTEQ specific settings for the translation task.
        #[prost(message, tag = "11")]
        BteqOptions(super::BteqOptions),
    }
}
/// The filter applied to fields of translation details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The list of prefixes used to exclude processing for input files.
    #[prost(string, repeated, tag = "1")]
    pub input_file_exclusion_prefixes: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Settings related to SQL identifiers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifierSettings {
    /// The setting to control output queries' identifier case.
    #[prost(enumeration = "identifier_settings::IdentifierCase", tag = "1")]
    pub output_identifier_case: i32,
    /// Specifies the rewrite mode for SQL identifiers.
    #[prost(enumeration = "identifier_settings::IdentifierRewriteMode", tag = "2")]
    pub identifier_rewrite_mode: i32,
}
/// Nested message and enum types in `IdentifierSettings`.
pub mod identifier_settings {
    /// The identifier case type.
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
    pub enum IdentifierCase {
        /// The identifier case is not specified.
        Unspecified = 0,
        /// Identifiers' cases will be kept as the original cases.
        Original = 1,
        /// Identifiers will be in upper cases.
        Upper = 2,
        /// Identifiers will be in lower cases.
        Lower = 3,
    }
    impl IdentifierCase {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IdentifierCase::Unspecified => "IDENTIFIER_CASE_UNSPECIFIED",
                IdentifierCase::Original => "ORIGINAL",
                IdentifierCase::Upper => "UPPER",
                IdentifierCase::Lower => "LOWER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IDENTIFIER_CASE_UNSPECIFIED" => Some(Self::Unspecified),
                "ORIGINAL" => Some(Self::Original),
                "UPPER" => Some(Self::Upper),
                "LOWER" => Some(Self::Lower),
                _ => None,
            }
        }
    }
    /// The SQL identifier rewrite mode.
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
    pub enum IdentifierRewriteMode {
        /// SQL Identifier rewrite mode is unspecified.
        Unspecified = 0,
        /// SQL identifiers won't be rewrite.
        None = 1,
        /// All SQL identifiers will be rewrite.
        RewriteAll = 2,
    }
    impl IdentifierRewriteMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IdentifierRewriteMode::Unspecified => {
                    "IDENTIFIER_REWRITE_MODE_UNSPECIFIED"
                }
                IdentifierRewriteMode::None => "NONE",
                IdentifierRewriteMode::RewriteAll => "REWRITE_ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IDENTIFIER_REWRITE_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "NONE" => Some(Self::None),
                "REWRITE_ALL" => Some(Self::RewriteAll),
                _ => None,
            }
        }
    }
}
/// Teradata SQL specific translation task related settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeradataOptions {}
/// BTEQ translation task related settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BteqOptions {
    /// Specifies the project and dataset in BigQuery that will be used for
    /// external table creation during the translation.
    #[prost(message, optional, tag = "1")]
    pub project_dataset: ::core::option::Option<DatasetReference>,
    /// The Cloud Storage location to be used as the default path for files that
    /// are not otherwise specified in the file replacement map.
    #[prost(string, tag = "2")]
    pub default_path_uri: ::prost::alloc::string::String,
    /// Maps the local paths that are used in BTEQ scripts (the keys) to the paths
    /// in Cloud Storage that should be used in their stead in the translation (the
    /// value).
    #[prost(map = "string, string", tag = "3")]
    pub file_replacement_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
