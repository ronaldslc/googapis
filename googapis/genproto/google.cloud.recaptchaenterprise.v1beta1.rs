/// The create assessment request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssessmentRequest {
    /// Required. The name of the project in which the assessment will be created,
    /// in the format "projects/{project_number}".
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
    /// "projects/{project_number}/assessments/{assessment_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The annotation that will be assigned to the Event.
    #[prost(enumeration = "annotate_assessment_request::Annotation", tag = "2")]
    pub annotation: i32,
}
/// Nested message and enum types in `AnnotateAssessmentRequest`.
pub mod annotate_assessment_request {
    /// Enum that reprensents the types of annotations.
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
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ANNOTATION_UNSPECIFIED" => Some(Self::Unspecified),
                "LEGITIMATE" => Some(Self::Legitimate),
                "FRAUDULENT" => Some(Self::Fraudulent),
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
    /// "projects/{project_number}/assessments/{assessment_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The event being assessed.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
    /// Output only. Legitimate event score from 0.0 to 1.0.
    /// (1.0 means very likely legitimate traffic while 0.0 means very likely
    /// non-legitimate traffic).
    #[prost(float, tag = "3")]
    pub score: f32,
    /// Output only. Properties of the provided event token.
    #[prost(message, optional, tag = "4")]
    pub token_properties: ::core::option::Option<TokenProperties>,
    /// Output only. Reasons contributing to the risk analysis verdict.
    #[prost(
        enumeration = "assessment::ClassificationReason",
        repeated,
        packed = "false",
        tag = "5"
    )]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `Assessment`.
pub mod assessment {
    /// LINT.IfChange(classification_reason)
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenProperties {
    /// Whether the provided user response token is valid.
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
    /// LINT.IfChange
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
        /// The user verification token did not match the provided site key.
        /// This may be a configuration error (e.g. development keys used in
        /// production) or end users trying to use verification tokens from other
        /// sites.
        SiteMismatch = 5,
        /// The user verification token was not present.  It is a required input.
        Missing = 6,
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
                InvalidReason::SiteMismatch => "SITE_MISMATCH",
                InvalidReason::Missing => "MISSING",
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
                "SITE_MISMATCH" => Some(Self::SiteMismatch),
                "MISSING" => Some(Self::Missing),
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
    /// format "projects/{project_number}".
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
    /// listed, in the format "projects/{project_number}".
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
    /// "projects/{project_number}/keys/{key_id}".
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
    /// Optional. The mask to control which field of the key get updated. If the mask is not
    /// present, all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The delete key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyRequest {
    /// Required. The name of the key to be deleted, in the format
    /// "projects/{project_number}/keys/{key_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A key used to identify and configure applications (web and/or mobile) that
/// use reCAPTCHA Enterprise.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    /// The resource name for the Key in the format
    /// "projects/{project_number}/keys/{key_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-readable display name of this key. Modifiable by user.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
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
/// Settings specific to keys that can be used by websites.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebKeySettings {
    /// Whether allowed_domains is enforced or not.
    #[prost(bool, tag = "3")]
    pub enforce_allowed_domains: bool,
    /// Domains or subdomains of websites allowed to use the key. All subdomains
    /// of an allowed domain are automatically allowed. A valid domain requires a
    /// host and must not include any path, port, query or fragment.
    /// Examples: 'example.com' or 'subdomain.example.com'
    #[prost(string, repeated, tag = "1")]
    pub allowed_domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether this key can be used on AMP (Accelerated Mobile Pages) websites.
    #[prost(bool, tag = "2")]
    pub allow_amp_traffic: bool,
    /// Required. Describes how this key is integrated with the website.
    #[prost(enumeration = "web_key_settings::IntegrationType", tag = "4")]
    pub integration_type: i32,
    /// Settings for the frequency and difficulty at which this key triggers
    /// captcha challenges. This should only be specified for IntegrationTypes
    /// CHECKBOX_CHALLENGE and INVISIBLE_CHALLENGE.
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
        ScoreOnly = 1,
        /// Displays the "I'm not a robot" checkbox and may show captcha challenges
        /// after it is checked.
        CheckboxChallenge = 2,
        /// Doesn't display the "I'm not a robot" checkbox, but may show captcha
        /// challenges after risk analysis.
        InvisibleChallenge = 3,
    }
    impl IntegrationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IntegrationType::Unspecified => "INTEGRATION_TYPE_UNSPECIFIED",
                IntegrationType::ScoreOnly => "SCORE_ONLY",
                IntegrationType::CheckboxChallenge => "CHECKBOX_CHALLENGE",
                IntegrationType::InvisibleChallenge => "INVISIBLE_CHALLENGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTEGRATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SCORE_ONLY" => Some(Self::ScoreOnly),
                "CHECKBOX_CHALLENGE" => Some(Self::CheckboxChallenge),
                "INVISIBLE_CHALLENGE" => Some(Self::InvisibleChallenge),
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
        Balanced = 2,
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
                ChallengeSecurityPreference::Balanced => "BALANCED",
                ChallengeSecurityPreference::Security => "SECURITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED" => Some(Self::Unspecified),
                "USABILITY" => Some(Self::Usability),
                "BALANCED" => Some(Self::Balanced),
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
    /// Android package names of apps allowed to use the key.
    /// Example: 'com.companyname.appname'
    #[prost(string, repeated, tag = "1")]
    pub allowed_package_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Settings specific to keys that can be used by iOS apps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosKeySettings {
    /// iOS bundle ids of apps allowed to use the key.
    /// Example: 'com.companyname.productname.appname'
    #[prost(string, repeated, tag = "1")]
    pub allowed_bundle_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod recaptcha_enterprise_service_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to determine the likelihood an event is legitimate.
    #[derive(Debug, Clone)]
    pub struct RecaptchaEnterpriseServiceV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RecaptchaEnterpriseServiceV1Beta1Client<T>
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
        ) -> RecaptchaEnterpriseServiceV1Beta1Client<InterceptedService<T, F>>
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
            RecaptchaEnterpriseServiceV1Beta1Client::new(
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/CreateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
                        "CreateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Annotates a previously created Assessment to provide additional information
        /// on whether the event turned out to be authentic or fradulent.
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/AnnotateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/CreateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/ListKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/GetKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/UpdateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/DeleteKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
                        "DeleteKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
