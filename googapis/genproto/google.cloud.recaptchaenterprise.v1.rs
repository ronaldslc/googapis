/// The create assessment request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssessmentRequest {
    /// Required. The name of the project in which the assessment will be created,
    /// in the format "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The assessment details.
    #[prost(message, optional, tag = "2")]
    pub assessment: ::core::option::Option<Assessment>,
}
/// The request message to annotate an Assessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentRequest {
    /// Required. The resource name of the Assessment, in the format
    /// "projects/{project}/assessments/{assessment}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The annotation that will be assigned to the Event. This field can be left
    /// empty to provide reasons that apply to an event without concluding whether
    /// the event is legitimate or fraudulent.
    #[prost(enumeration = "annotate_assessment_request::Annotation", tag = "2")]
    pub annotation: i32,
    /// Optional. Optional reasons for the annotation that will be assigned to the Event.
    #[prost(
        enumeration = "annotate_assessment_request::Reason",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Optional unique stable hashed user identifier to apply to the assessment.
    /// This is an alternative to setting the hashed_account_id in
    /// CreateAssessment, for example when the account identifier is not yet known
    /// in the initial request. It is recommended that the identifier is hashed
    /// using hmac-sha256 with stable secret.
    #[prost(bytes = "vec", tag = "4")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `AnnotateAssessmentRequest`.
pub mod annotate_assessment_request {
    /// Enum that represents the types of annotations.
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
    pub enum Annotation {
        /// Default unspecified type.
        Unspecified = 0,
        /// Provides information that the event turned out to be legitimate.
        Legitimate = 1,
        /// Provides information that the event turned out to be fraudulent.
        Fraudulent = 2,
        /// Provides information that the event was related to a login event in which
        /// the user typed the correct password. Deprecated, prefer indicating
        /// CORRECT_PASSWORD through the reasons field instead.
        PasswordCorrect = 3,
        /// Provides information that the event was related to a login event in which
        /// the user typed the incorrect password. Deprecated, prefer indicating
        /// INCORRECT_PASSWORD through the reasons field instead.
        PasswordIncorrect = 4,
    }
    impl Annotation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Annotation::Unspecified => "ANNOTATION_UNSPECIFIED",
                Annotation::Legitimate => "LEGITIMATE",
                Annotation::Fraudulent => "FRAUDULENT",
                Annotation::PasswordCorrect => "PASSWORD_CORRECT",
                Annotation::PasswordIncorrect => "PASSWORD_INCORRECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ANNOTATION_UNSPECIFIED" => Some(Self::Unspecified),
                "LEGITIMATE" => Some(Self::Legitimate),
                "FRAUDULENT" => Some(Self::Fraudulent),
                "PASSWORD_CORRECT" => Some(Self::PasswordCorrect),
                "PASSWORD_INCORRECT" => Some(Self::PasswordIncorrect),
                _ => None,
            }
        }
    }
    /// Enum that represents potential reasons for annotating an assessment.
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
    pub enum Reason {
        /// Default unspecified reason.
        Unspecified = 0,
        /// Indicates a chargeback for fraud was issued for the transaction
        /// associated with the assessment.
        Chargeback = 1,
        /// Indicates the transaction associated with the assessment is suspected of
        /// being fraudulent based on the payment method, billing details, shipping
        /// address or other transaction information.
        PaymentHeuristics = 2,
        /// Indicates that the user was served a 2FA challenge. An old assessment
        /// with `ENUM_VALUES.INITIATED_TWO_FACTOR` reason that has not been
        /// overwritten with `PASSED_TWO_FACTOR` is treated as an abandoned 2FA flow.
        /// This is equivalent to `FAILED_TWO_FACTOR`.
        InitiatedTwoFactor = 7,
        /// Indicates that the user passed a 2FA challenge.
        PassedTwoFactor = 3,
        /// Indicates that the user failed a 2FA challenge.
        FailedTwoFactor = 4,
        /// Indicates the user provided the correct password.
        CorrectPassword = 5,
        /// Indicates the user provided an incorrect password.
        IncorrectPassword = 6,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::Unspecified => "REASON_UNSPECIFIED",
                Reason::Chargeback => "CHARGEBACK",
                Reason::PaymentHeuristics => "PAYMENT_HEURISTICS",
                Reason::InitiatedTwoFactor => "INITIATED_TWO_FACTOR",
                Reason::PassedTwoFactor => "PASSED_TWO_FACTOR",
                Reason::FailedTwoFactor => "FAILED_TWO_FACTOR",
                Reason::CorrectPassword => "CORRECT_PASSWORD",
                Reason::IncorrectPassword => "INCORRECT_PASSWORD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "CHARGEBACK" => Some(Self::Chargeback),
                "PAYMENT_HEURISTICS" => Some(Self::PaymentHeuristics),
                "INITIATED_TWO_FACTOR" => Some(Self::InitiatedTwoFactor),
                "PASSED_TWO_FACTOR" => Some(Self::PassedTwoFactor),
                "FAILED_TWO_FACTOR" => Some(Self::FailedTwoFactor),
                "CORRECT_PASSWORD" => Some(Self::CorrectPassword),
                "INCORRECT_PASSWORD" => Some(Self::IncorrectPassword),
                _ => None,
            }
        }
    }
}
/// Empty response for AnnotateAssessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentResponse {}
/// A recaptcha assessment resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assessment {
    /// Output only. The resource name for the Assessment in the format
    /// "projects/{project}/assessments/{assessment}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The event being assessed.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
    /// Output only. The risk analysis result for the event being assessed.
    #[prost(message, optional, tag = "3")]
    pub risk_analysis: ::core::option::Option<RiskAnalysis>,
    /// Output only. Properties of the provided event token.
    #[prost(message, optional, tag = "4")]
    pub token_properties: ::core::option::Option<TokenProperties>,
    /// Assessment returned by Account Defender when a hashed_account_id is
    /// provided.
    #[prost(message, optional, tag = "6")]
    pub account_defender_assessment: ::core::option::Option<AccountDefenderAssessment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Optional. The user response token provided by the reCAPTCHA client-side integration
    /// on your site.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Optional. The site key that was used to invoke reCAPTCHA on your site and generate
    /// the token.
    #[prost(string, tag = "2")]
    pub site_key: ::prost::alloc::string::String,
    /// Optional. The user agent present in the request from the user's device related to
    /// this event.
    #[prost(string, tag = "3")]
    pub user_agent: ::prost::alloc::string::String,
    /// Optional. The IP address in the request from the user's device related to this event.
    #[prost(string, tag = "4")]
    pub user_ip_address: ::prost::alloc::string::String,
    /// Optional. The expected action for this type of event. This should be the same action
    /// provided at token generation time on client-side platforms already
    /// integrated with recaptcha enterprise.
    #[prost(string, tag = "5")]
    pub expected_action: ::prost::alloc::string::String,
    /// Optional. Optional unique stable hashed user identifier for the request. The
    /// identifier should ideally be hashed using sha256 with stable secret.
    #[prost(bytes = "vec", tag = "6")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
}
/// Risk analysis result for an event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RiskAnalysis {
    /// Legitimate event score from 0.0 to 1.0.
    /// (1.0 means very likely legitimate traffic while 0.0 means very likely
    /// non-legitimate traffic).
    #[prost(float, tag = "1")]
    pub score: f32,
    /// Reasons contributing to the risk analysis verdict.
    #[prost(enumeration = "risk_analysis::ClassificationReason", repeated, tag = "2")]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `RiskAnalysis`.
pub mod risk_analysis {
    /// Reasons contributing to the risk analysis verdict.
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
    pub enum ClassificationReason {
        /// Default unspecified type.
        Unspecified = 0,
        /// Interactions matched the behavior of an automated agent.
        Automation = 1,
        /// The event originated from an illegitimate environment.
        UnexpectedEnvironment = 2,
        /// Traffic volume from the event source is higher than normal.
        TooMuchTraffic = 3,
        /// Interactions with the site were significantly different than expected
        /// patterns.
        UnexpectedUsagePatterns = 4,
        /// Too little traffic has been received from this site thus far to generate
        /// quality risk analysis.
        LowConfidenceScore = 5,
    }
    impl ClassificationReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClassificationReason::Unspecified => "CLASSIFICATION_REASON_UNSPECIFIED",
                ClassificationReason::Automation => "AUTOMATION",
                ClassificationReason::UnexpectedEnvironment => "UNEXPECTED_ENVIRONMENT",
                ClassificationReason::TooMuchTraffic => "TOO_MUCH_TRAFFIC",
                ClassificationReason::UnexpectedUsagePatterns => {
                    "UNEXPECTED_USAGE_PATTERNS"
                }
                ClassificationReason::LowConfidenceScore => "LOW_CONFIDENCE_SCORE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLASSIFICATION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTOMATION" => Some(Self::Automation),
                "UNEXPECTED_ENVIRONMENT" => Some(Self::UnexpectedEnvironment),
                "TOO_MUCH_TRAFFIC" => Some(Self::TooMuchTraffic),
                "UNEXPECTED_USAGE_PATTERNS" => Some(Self::UnexpectedUsagePatterns),
                "LOW_CONFIDENCE_SCORE" => Some(Self::LowConfidenceScore),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenProperties {
    /// Whether the provided user response token is valid. When valid = false, the
    /// reason could be specified in invalid_reason or it could also be due to
    /// a user failing to solve a challenge or a sitekey mismatch (i.e the sitekey
    /// used to generate the token was different than the one specified in the
    /// assessment).
    #[prost(bool, tag = "1")]
    pub valid: bool,
    /// Reason associated with the response when valid = false.
    #[prost(enumeration = "token_properties::InvalidReason", tag = "2")]
    pub invalid_reason: i32,
    /// The timestamp corresponding to the generation of the token.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The hostname of the page on which the token was generated.
    #[prost(string, tag = "4")]
    pub hostname: ::prost::alloc::string::String,
    /// Action name provided at token generation.
    #[prost(string, tag = "5")]
    pub action: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TokenProperties`.
pub mod token_properties {
    /// Enum that represents the types of invalid token reasons.
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
    pub enum InvalidReason {
        /// Default unspecified type.
        Unspecified = 0,
        /// If the failure reason was not accounted for.
        UnknownInvalidReason = 1,
        /// The provided user verification token was malformed.
        Malformed = 2,
        /// The user verification token had expired.
        Expired = 3,
        /// The user verification had already been seen.
        Dupe = 4,
        /// The user verification token was not present.
        Missing = 5,
        /// A retriable error (such as network failure) occurred on the browser.
        /// Could easily be simulated by an attacker.
        BrowserError = 6,
    }
    impl InvalidReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InvalidReason::Unspecified => "INVALID_REASON_UNSPECIFIED",
                InvalidReason::UnknownInvalidReason => "UNKNOWN_INVALID_REASON",
                InvalidReason::Malformed => "MALFORMED",
                InvalidReason::Expired => "EXPIRED",
                InvalidReason::Dupe => "DUPE",
                InvalidReason::Missing => "MISSING",
                InvalidReason::BrowserError => "BROWSER_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN_INVALID_REASON" => Some(Self::UnknownInvalidReason),
                "MALFORMED" => Some(Self::Malformed),
                "EXPIRED" => Some(Self::Expired),
                "DUPE" => Some(Self::Dupe),
                "MISSING" => Some(Self::Missing),
                "BROWSER_ERROR" => Some(Self::BrowserError),
                _ => None,
            }
        }
    }
}
/// Account Defender risk assessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDefenderAssessment {
    /// Labels for this request.
    #[prost(
        enumeration = "account_defender_assessment::AccountDefenderLabel",
        repeated,
        tag = "1"
    )]
    pub labels: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `AccountDefenderAssessment`.
pub mod account_defender_assessment {
    /// Labels returned by Account Defender for this request.
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
    pub enum AccountDefenderLabel {
        /// Default unspecified type.
        Unspecified = 0,
        /// The request matches a known good profile for the user.
        ProfileMatch = 1,
        /// The request is potentially a suspicious login event and should be further
        /// verified either via multi-factor authentication or another system.
        SuspiciousLoginActivity = 2,
        /// The request matched a profile that previously had suspicious account
        /// creation behavior. This could mean this is a fake account.
        SuspiciousAccountCreation = 3,
        /// The account in the request has a high number of related accounts. It does
        /// not necessarily imply that the account is bad but could require
        /// investigating.
        RelatedAccountsNumberHigh = 4,
    }
    impl AccountDefenderLabel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountDefenderLabel::Unspecified => "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED",
                AccountDefenderLabel::ProfileMatch => "PROFILE_MATCH",
                AccountDefenderLabel::SuspiciousLoginActivity => {
                    "SUSPICIOUS_LOGIN_ACTIVITY"
                }
                AccountDefenderLabel::SuspiciousAccountCreation => {
                    "SUSPICIOUS_ACCOUNT_CREATION"
                }
                AccountDefenderLabel::RelatedAccountsNumberHigh => {
                    "RELATED_ACCOUNTS_NUMBER_HIGH"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED" => Some(Self::Unspecified),
                "PROFILE_MATCH" => Some(Self::ProfileMatch),
                "SUSPICIOUS_LOGIN_ACTIVITY" => Some(Self::SuspiciousLoginActivity),
                "SUSPICIOUS_ACCOUNT_CREATION" => Some(Self::SuspiciousAccountCreation),
                "RELATED_ACCOUNTS_NUMBER_HIGH" => Some(Self::RelatedAccountsNumberHigh),
                _ => None,
            }
        }
    }
}
/// The create key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyRequest {
    /// Required. The name of the project in which the key will be created, in the
    /// format "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Information to create a reCAPTCHA Enterprise key.
    #[prost(message, optional, tag = "2")]
    pub key: ::core::option::Option<Key>,
}
/// The list keys request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysRequest {
    /// Required. The name of the project that contains the keys that will be
    /// listed, in the format "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of keys to return. Default is 10. Max limit is
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous.
    /// ListKeysRequest, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response to request to list keys in a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysResponse {
    /// Key details.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
    /// Token to retrieve the next page of results. It is set to empty if no keys
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The get key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyRequest {
    /// Required. The name of the requested key, in the format
    /// "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The update key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateKeyRequest {
    /// Required. The key to update.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<Key>,
    /// Optional. The mask to control which fields of the key get updated. If the mask is not
    /// present, all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The delete key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyRequest {
    /// Required. The name of the key to be deleted, in the format
    /// "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The migrate key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateKeyRequest {
    /// Required. The name of the key to be migrated, in the format
    /// "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The get metrics request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricsRequest {
    /// Required. The name of the requested metrics, in the format
    /// "projects/{project}/keys/{key}/metrics".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metrics for a single Key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metrics {
    /// Output only. The name of the metrics, in the format
    /// "projects/{project}/keys/{key}/metrics".
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Inclusive start time aligned to a day (UTC).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Metrics will be continuous and in order by dates, and in the granularity
    /// of day. All Key types should have score-based data.
    #[prost(message, repeated, tag = "2")]
    pub score_metrics: ::prost::alloc::vec::Vec<ScoreMetrics>,
    /// Metrics will be continuous and in order by dates, and in the granularity
    /// of day. Only challenge-based keys (CHECKBOX, INVISIBLE), will have
    /// challenge-based data.
    #[prost(message, repeated, tag = "3")]
    pub challenge_metrics: ::prost::alloc::vec::Vec<ChallengeMetrics>,
}
/// A key used to identify and configure applications (web and/or mobile) that
/// use reCAPTCHA Enterprise.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    /// The resource name for the Key in the format
    /// "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-readable display name of this key. Modifiable by user.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// See <a href="<https://cloud.google.com/recaptcha-enterprise/docs/labels">>
    /// Creating and managing labels</a>.
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The timestamp corresponding to the creation of this Key.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Options for user acceptance testing.
    #[prost(message, optional, tag = "9")]
    pub testing_options: ::core::option::Option<TestingOptions>,
    /// Platform specific settings for this key. The key can only be used on one
    /// platform, the one it has settings for.
    #[prost(oneof = "key::PlatformSettings", tags = "3, 4, 5")]
    pub platform_settings: ::core::option::Option<key::PlatformSettings>,
}
/// Nested message and enum types in `Key`.
pub mod key {
    /// Platform specific settings for this key. The key can only be used on one
    /// platform, the one it has settings for.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PlatformSettings {
        /// Settings for keys that can be used by websites.
        #[prost(message, tag = "3")]
        WebSettings(super::WebKeySettings),
        /// Settings for keys that can be used by Android apps.
        #[prost(message, tag = "4")]
        AndroidSettings(super::AndroidKeySettings),
        /// Settings for keys that can be used by iOS apps.
        #[prost(message, tag = "5")]
        IosSettings(super::IosKeySettings),
    }
}
/// Options for user acceptance testing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestingOptions {
    /// All assessments for this Key will return this score. Must be between 0
    /// (likely not legitimate) and 1 (likely legitimate) inclusive.
    #[prost(float, tag = "1")]
    pub testing_score: f32,
    /// For challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests
    /// for this site will return nocaptcha if NOCAPTCHA, or an unsolvable
    /// challenge if CHALLENGE.
    #[prost(enumeration = "testing_options::TestingChallenge", tag = "2")]
    pub testing_challenge: i32,
}
/// Nested message and enum types in `TestingOptions`.
pub mod testing_options {
    /// Enum that represents the challenge option for challenge-based (CHECKBOX,
    /// INVISIBLE) testing keys.
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
    pub enum TestingChallenge {
        /// Perform the normal risk analysis and return either nocaptcha or a
        /// challenge depending on risk and trust factors.
        Unspecified = 0,
        /// Challenge requests for this key always return a nocaptcha, which
        /// does not require a solution.
        Nocaptcha = 1,
        /// Challenge requests for this key always return an unsolvable
        /// challenge.
        UnsolvableChallenge = 2,
    }
    impl TestingChallenge {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TestingChallenge::Unspecified => "TESTING_CHALLENGE_UNSPECIFIED",
                TestingChallenge::Nocaptcha => "NOCAPTCHA",
                TestingChallenge::UnsolvableChallenge => "UNSOLVABLE_CHALLENGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TESTING_CHALLENGE_UNSPECIFIED" => Some(Self::Unspecified),
                "NOCAPTCHA" => Some(Self::Nocaptcha),
                "UNSOLVABLE_CHALLENGE" => Some(Self::UnsolvableChallenge),
                _ => None,
            }
        }
    }
}
/// Settings specific to keys that can be used by websites.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebKeySettings {
    /// If set to true, it means allowed_domains will not be enforced.
    #[prost(bool, tag = "3")]
    pub allow_all_domains: bool,
    /// Domains or subdomains of websites allowed to use the key. All subdomains
    /// of an allowed domain are automatically allowed. A valid domain requires a
    /// host and must not include any path, port, query or fragment.
    /// Examples: 'example.com' or 'subdomain.example.com'
    #[prost(string, repeated, tag = "1")]
    pub allowed_domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If set to true, the key can be used on AMP (Accelerated Mobile Pages)
    /// websites. This is supported only for the SCORE integration type.
    #[prost(bool, tag = "2")]
    pub allow_amp_traffic: bool,
    /// Required. Describes how this key is integrated with the website.
    #[prost(enumeration = "web_key_settings::IntegrationType", tag = "4")]
    pub integration_type: i32,
    /// Settings for the frequency and difficulty at which this key triggers
    /// captcha challenges. This should only be specified for IntegrationTypes
    /// CHECKBOX and INVISIBLE.
    #[prost(enumeration = "web_key_settings::ChallengeSecurityPreference", tag = "5")]
    pub challenge_security_preference: i32,
}
/// Nested message and enum types in `WebKeySettings`.
pub mod web_key_settings {
    /// Enum that represents the integration types for web keys.
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
    pub enum IntegrationType {
        /// Default type that indicates this enum hasn't been specified. This is not
        /// a valid IntegrationType, one of the other types must be specified
        /// instead.
        Unspecified = 0,
        /// Only used to produce scores. It doesn't display the "I'm not a robot"
        /// checkbox and never shows captcha challenges.
        Score = 1,
        /// Displays the "I'm not a robot" checkbox and may show captcha challenges
        /// after it is checked.
        Checkbox = 2,
        /// Doesn't display the "I'm not a robot" checkbox, but may show captcha
        /// challenges after risk analysis.
        Invisible = 3,
    }
    impl IntegrationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IntegrationType::Unspecified => "INTEGRATION_TYPE_UNSPECIFIED",
                IntegrationType::Score => "SCORE",
                IntegrationType::Checkbox => "CHECKBOX",
                IntegrationType::Invisible => "INVISIBLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTEGRATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SCORE" => Some(Self::Score),
                "CHECKBOX" => Some(Self::Checkbox),
                "INVISIBLE" => Some(Self::Invisible),
                _ => None,
            }
        }
    }
    /// Enum that represents the possible challenge frequency and difficulty
    /// configurations for a web key.
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
    pub enum ChallengeSecurityPreference {
        /// Default type that indicates this enum hasn't been specified.
        Unspecified = 0,
        /// Key tends to show fewer and easier challenges.
        Usability = 1,
        /// Key tends to show balanced (in amount and difficulty) challenges.
        Balance = 2,
        /// Key tends to show more and harder challenges.
        Security = 3,
    }
    impl ChallengeSecurityPreference {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChallengeSecurityPreference::Unspecified => {
                    "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED"
                }
                ChallengeSecurityPreference::Usability => "USABILITY",
                ChallengeSecurityPreference::Balance => "BALANCE",
                ChallengeSecurityPreference::Security => "SECURITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED" => Some(Self::Unspecified),
                "USABILITY" => Some(Self::Usability),
                "BALANCE" => Some(Self::Balance),
                "SECURITY" => Some(Self::Security),
                _ => None,
            }
        }
    }
}
/// Settings specific to keys that can be used by Android apps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidKeySettings {
    /// If set to true, allowed_package_names are not enforced.
    #[prost(bool, tag = "2")]
    pub allow_all_package_names: bool,
    /// Android package names of apps allowed to use the key.
    /// Example: 'com.companyname.appname'
    #[prost(string, repeated, tag = "1")]
    pub allowed_package_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Settings specific to keys that can be used by iOS apps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosKeySettings {
    /// If set to true, allowed_bundle_ids are not enforced.
    #[prost(bool, tag = "2")]
    pub allow_all_bundle_ids: bool,
    /// iOS bundle ids of apps allowed to use the key.
    /// Example: 'com.companyname.productname.appname'
    #[prost(string, repeated, tag = "1")]
    pub allowed_bundle_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Score distribution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreDistribution {
    /// Map key is score value multiplied by 100. The scores are discrete values
    /// between [0, 1]. The maximum number of buckets is on order of a few dozen,
    /// but typically much lower (ie. 10).
    #[prost(map = "int32, int64", tag = "1")]
    pub score_buckets: ::std::collections::HashMap<i32, i64>,
}
/// Metrics related to scoring.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreMetrics {
    /// Aggregated score metrics for all traffic.
    #[prost(message, optional, tag = "1")]
    pub overall_metrics: ::core::option::Option<ScoreDistribution>,
    /// Action-based metrics. The map key is the action name which specified by the
    /// site owners at time of the "execute" client-side call.
    /// Populated only for SCORE keys.
    #[prost(map = "string, message", tag = "2")]
    pub action_metrics: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ScoreDistribution,
    >,
}
/// Metrics related to challenges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChallengeMetrics {
    /// Count of reCAPTCHA checkboxes or badges rendered. This is mostly equivalent
    /// to a count of pageloads for pages that include reCAPTCHA.
    #[prost(int64, tag = "1")]
    pub pageload_count: i64,
    /// Count of nocaptchas (successful verification without a challenge) issued.
    #[prost(int64, tag = "2")]
    pub nocaptcha_count: i64,
    /// Count of submitted challenge solutions that were incorrect or otherwise
    /// deemed suspicious such that a subsequent challenge was triggered.
    #[prost(int64, tag = "3")]
    pub failed_count: i64,
    /// Count of nocaptchas (successful verification without a challenge) plus
    /// submitted challenge solutions that were correct and resulted in
    /// verification.
    #[prost(int64, tag = "4")]
    pub passed_count: i64,
}
/// The request message to list memberships in a related account group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRelatedAccountGroupMembershipsRequest {
    /// Required. The resource name for the related account group in the format
    /// `projects/{project}/relatedaccountgroups/{relatedaccountgroup}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of accounts to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 50 accounts will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListRelatedAccountGroupMemberships`
    /// call.
    ///
    /// When paginating, all other parameters provided to
    /// `ListRelatedAccountGroupMemberships` must match the call that provided the
    /// page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response to a `ListRelatedAccountGroupMemberships` call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRelatedAccountGroupMembershipsResponse {
    /// The memberships listed by the query.
    #[prost(message, repeated, tag = "1")]
    pub related_account_group_memberships: ::prost::alloc::vec::Vec<
        RelatedAccountGroupMembership,
    >,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message to list related account groups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRelatedAccountGroupsRequest {
    /// Required. The name of the project to list related account groups from, in the format
    /// "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of groups to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 50 groups will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListRelatedAccountGroups` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListRelatedAccountGroups` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response to a `ListRelatedAccountGroups` call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRelatedAccountGroupsResponse {
    /// The groups of related accounts listed by the query.
    #[prost(message, repeated, tag = "1")]
    pub related_account_groups: ::prost::alloc::vec::Vec<RelatedAccountGroup>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message to search related account group memberships.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRelatedAccountGroupMembershipsRequest {
    /// Required. The name of the project to search related account group memberships from,
    /// in the format "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The unique stable hashed user identifier we should search connections to.
    /// The identifier should correspond to a `hashed_account_id` provided in a
    /// previous CreateAssessment or AnnotateAssessment call.
    #[prost(bytes = "vec", tag = "2")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
    /// Optional. The maximum number of groups to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 50 groups will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `SearchRelatedAccountGroupMemberships` call. Provide this to retrieve the
    /// subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `SearchRelatedAccountGroupMemberships` must match the call that provided
    /// the page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response to a `SearchRelatedAccountGroupMemberships` call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRelatedAccountGroupMembershipsResponse {
    /// The queried memberships.
    #[prost(message, repeated, tag = "1")]
    pub related_account_group_memberships: ::prost::alloc::vec::Vec<
        RelatedAccountGroupMembership,
    >,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A membership in a group of related accounts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedAccountGroupMembership {
    /// Required. The resource name for this membership in the format
    /// `projects/{project}/relatedaccountgroups/{relatedaccountgroup}/memberships/{membership}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The unique stable hashed user identifier of the member. The identifier
    /// corresponds to a `hashed_account_id` provided in a previous
    /// CreateAssessment or AnnotateAssessment call.
    #[prost(bytes = "vec", tag = "2")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
}
/// A group of related accounts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedAccountGroup {
    /// Required. The resource name for the related account group in the format
    /// `projects/{project}/relatedaccountgroups/{related_account_group}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod recaptcha_enterprise_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to determine the likelihood an event is legitimate.
    #[derive(Debug, Clone)]
    pub struct RecaptchaEnterpriseServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RecaptchaEnterpriseServiceClient<T>
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
        ) -> RecaptchaEnterpriseServiceClient<InterceptedService<T, F>>
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
            RecaptchaEnterpriseServiceClient::new(
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
        /// Creates an Assessment of the likelihood an event is legitimate.
        pub async fn create_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAssessmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Assessment>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/CreateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "CreateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Annotates a previously created Assessment to provide additional information
        /// on whether the event turned out to be authentic or fraudulent.
        pub async fn annotate_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateAssessmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnnotateAssessmentResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/AnnotateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "AnnotateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new reCAPTCHA Enterprise key.
        pub async fn create_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/CreateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "CreateKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the list of all keys that belong to a project.
        pub async fn list_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListKeysResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/ListKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "ListKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified key.
        pub async fn get_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/GetKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "GetKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified key.
        pub async fn update_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/UpdateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "UpdateKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified key.
        pub async fn delete_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteKeyRequest>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/DeleteKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "DeleteKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Migrates an existing key from reCAPTCHA to reCAPTCHA Enterprise.
        /// Once a key is migrated, it can be used from either product. SiteVerify
        /// requests are billed as CreateAssessment calls. You must be
        /// authenticated as one of the current owners of the reCAPTCHA Site Key, and
        /// your user must have the reCAPTCHA Enterprise Admin IAM role in the
        /// destination project.
        pub async fn migrate_key(
            &mut self,
            request: impl tonic::IntoRequest<super::MigrateKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/MigrateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "MigrateKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get some aggregated metrics for a Key. This data can be used to build
        /// dashboards.
        pub async fn get_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetricsRequest>,
        ) -> std::result::Result<tonic::Response<super::Metrics>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/GetMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "GetMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List groups of related accounts.
        pub async fn list_related_account_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRelatedAccountGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRelatedAccountGroupsResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/ListRelatedAccountGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "ListRelatedAccountGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the memberships in a group of related accounts.
        pub async fn list_related_account_group_memberships(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListRelatedAccountGroupMembershipsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListRelatedAccountGroupMembershipsResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/ListRelatedAccountGroupMemberships",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "ListRelatedAccountGroupMemberships",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Search group memberships related to a given account.
        pub async fn search_related_account_group_memberships(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SearchRelatedAccountGroupMembershipsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SearchRelatedAccountGroupMembershipsResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/SearchRelatedAccountGroupMemberships",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "SearchRelatedAccountGroupMemberships",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
