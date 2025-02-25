/// Required Edu Attributes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EduData {
    /// Designated institute type of customer.
    #[prost(enumeration = "edu_data::InstituteType", tag = "1")]
    pub institute_type: i32,
    /// Size of the institute.
    #[prost(enumeration = "edu_data::InstituteSize", tag = "2")]
    pub institute_size: i32,
    /// Web address for the edu customer's institution.
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EduData`.
pub mod edu_data {
    /// Enum to specify the institute type.
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
    pub enum InstituteType {
        /// Default value.  This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Elementary/Secondary Schools & Districts
        K12 = 1,
        /// Higher Education Universities & Colleges
        University = 2,
    }
    impl InstituteType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InstituteType::Unspecified => "INSTITUTE_TYPE_UNSPECIFIED",
                InstituteType::K12 => "K12",
                InstituteType::University => "UNIVERSITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INSTITUTE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "K12" => Some(Self::K12),
                "UNIVERSITY" => Some(Self::University),
                _ => None,
            }
        }
    }
    /// Number of students and staff the institute has.
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
    pub enum InstituteSize {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// 1 - 100
        Size1100 = 1,
        /// 101 - 500
        Size101500 = 2,
        /// 501 - 1,000
        Size5011000 = 3,
        /// 1,001 - 2,000
        Size10012000 = 4,
        /// 2,001 - 5,000
        Size20015000 = 5,
        /// 5,001 - 10,000
        Size500110000 = 6,
        /// 10,001 +
        Size10001OrMore = 7,
    }
    impl InstituteSize {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InstituteSize::Unspecified => "INSTITUTE_SIZE_UNSPECIFIED",
                InstituteSize::Size1100 => "SIZE_1_100",
                InstituteSize::Size101500 => "SIZE_101_500",
                InstituteSize::Size5011000 => "SIZE_501_1000",
                InstituteSize::Size10012000 => "SIZE_1001_2000",
                InstituteSize::Size20015000 => "SIZE_2001_5000",
                InstituteSize::Size500110000 => "SIZE_5001_10000",
                InstituteSize::Size10001OrMore => "SIZE_10001_OR_MORE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INSTITUTE_SIZE_UNSPECIFIED" => Some(Self::Unspecified),
                "SIZE_1_100" => Some(Self::Size1100),
                "SIZE_101_500" => Some(Self::Size101500),
                "SIZE_501_1000" => Some(Self::Size5011000),
                "SIZE_1001_2000" => Some(Self::Size10012000),
                "SIZE_2001_5000" => Some(Self::Size20015000),
                "SIZE_5001_10000" => Some(Self::Size500110000),
                "SIZE_10001_OR_MORE" => Some(Self::Size10001OrMore),
                _ => None,
            }
        }
    }
}
/// Cloud Identity information for the Cloud Channel Customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudIdentityInfo {
    /// CustomerType indicates verification type needed for using services.
    #[prost(enumeration = "cloud_identity_info::CustomerType", tag = "1")]
    pub customer_type: i32,
    /// Output only. The primary domain name.
    #[prost(string, tag = "9")]
    pub primary_domain: ::prost::alloc::string::String,
    /// Output only. Whether the domain is verified.
    /// This field is not returned for a Customer's cloud_identity_info resource.
    /// Partners can use the domains.get() method of the Workspace SDK's
    /// Directory API, or listen to the PRIMARY_DOMAIN_VERIFIED Pub/Sub event in
    /// to track domain verification of their resolve Workspace customers.
    #[prost(bool, tag = "4")]
    pub is_domain_verified: bool,
    /// The alternate email.
    #[prost(string, tag = "6")]
    pub alternate_email: ::prost::alloc::string::String,
    /// Phone number associated with the Cloud Identity.
    #[prost(string, tag = "7")]
    pub phone_number: ::prost::alloc::string::String,
    /// Language code.
    #[prost(string, tag = "8")]
    pub language_code: ::prost::alloc::string::String,
    /// Output only. URI of Customer's Admin console dashboard.
    #[prost(string, tag = "10")]
    pub admin_console_uri: ::prost::alloc::string::String,
    /// Edu information about the customer.
    #[prost(message, optional, tag = "22")]
    pub edu_data: ::core::option::Option<EduData>,
}
/// Nested message and enum types in `CloudIdentityInfo`.
pub mod cloud_identity_info {
    /// CustomerType of the customer
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
    pub enum CustomerType {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Domain-owning customer which needs domain verification to use services.
        Domain = 1,
        /// Team customer which needs email verification to use services.
        Team = 2,
    }
    impl CustomerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomerType::Unspecified => "CUSTOMER_TYPE_UNSPECIFIED",
                CustomerType::Domain => "DOMAIN",
                CustomerType::Team => "TEAM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOMER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DOMAIN" => Some(Self::Domain),
                "TEAM" => Some(Self::Team),
                _ => None,
            }
        }
    }
}
/// Data type and value of a parameter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// The kind of value.
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Represents an int64 value.
        #[prost(int64, tag = "1")]
        Int64Value(i64),
        /// Represents a string value.
        #[prost(string, tag = "2")]
        StringValue(::prost::alloc::string::String),
        /// Represents a double value.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// Represents an 'Any' proto value.
        #[prost(message, tag = "4")]
        ProtoValue(::prost_types::Any),
        /// Represents a boolean value.
        #[prost(bool, tag = "5")]
        BoolValue(bool),
    }
}
/// Information needed to create an Admin User for Google Workspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminUser {
    /// Primary email of the admin user.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// Given name of the admin user.
    #[prost(string, tag = "2")]
    pub given_name: ::prost::alloc::string::String,
    /// Family name of the admin user.
    #[prost(string, tag = "3")]
    pub family_name: ::prost::alloc::string::String,
}
/// Entity representing a link between distributors and their indirect
/// resellers in an n-tier resale channel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPartnerLink {
    /// Output only. Resource name for the channel partner link, in the format
    /// accounts/{account_id}/channelPartnerLinks/{id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Cloud Identity ID of the linked reseller.
    #[prost(string, tag = "2")]
    pub reseller_cloud_identity_id: ::prost::alloc::string::String,
    /// Required. State of the channel partner link.
    #[prost(enumeration = "ChannelPartnerLinkState", tag = "3")]
    pub link_state: i32,
    /// Output only. URI of the web page where partner accepts the link invitation.
    #[prost(string, tag = "4")]
    pub invite_link_uri: ::prost::alloc::string::String,
    /// Output only. Timestamp of when the channel partner link is created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp of when the channel partner link is updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Public identifier that a customer must use to generate a transfer token
    /// to move to this distributor-reseller combination.
    #[prost(string, tag = "7")]
    pub public_id: ::prost::alloc::string::String,
    /// Output only. Cloud Identity info of the channel partner (IR).
    #[prost(message, optional, tag = "8")]
    pub channel_partner_cloud_identity_info: ::core::option::Option<CloudIdentityInfo>,
}
/// The level of granularity the \[ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink\] will display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelPartnerLinkView {
    /// The default / unset value.
    /// The API will default to the BASIC view.
    Unspecified = 0,
    /// Includes all fields except the
    /// \[ChannelPartnerLink.channel_partner_cloud_identity_info][google.cloud.channel.v1.ChannelPartnerLink.channel_partner_cloud_identity_info\].
    Basic = 1,
    /// Includes all fields.
    Full = 2,
}
impl ChannelPartnerLinkView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChannelPartnerLinkView::Unspecified => "UNSPECIFIED",
            ChannelPartnerLinkView::Basic => "BASIC",
            ChannelPartnerLinkView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// ChannelPartnerLinkState represents state of a channel partner link.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelPartnerLinkState {
    /// The state is not specified.
    Unspecified = 0,
    /// An invitation has been sent to the reseller to create a channel partner
    /// link.
    Invited = 1,
    /// Status when the reseller is active.
    Active = 2,
    /// Status when the reseller has been revoked by the distributor.
    Revoked = 3,
    /// Status when the reseller is suspended by Google or distributor.
    Suspended = 4,
}
impl ChannelPartnerLinkState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChannelPartnerLinkState::Unspecified => {
                "CHANNEL_PARTNER_LINK_STATE_UNSPECIFIED"
            }
            ChannelPartnerLinkState::Invited => "INVITED",
            ChannelPartnerLinkState::Active => "ACTIVE",
            ChannelPartnerLinkState::Revoked => "REVOKED",
            ChannelPartnerLinkState::Suspended => "SUSPENDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHANNEL_PARTNER_LINK_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "INVITED" => Some(Self::Invited),
            "ACTIVE" => Some(Self::Active),
            "REVOKED" => Some(Self::Revoked),
            "SUSPENDED" => Some(Self::Suspended),
            _ => None,
        }
    }
}
/// Entity representing a customer of a reseller or distributor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Customer {
    /// Output only. Resource name of the customer.
    /// Format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Name of the organization that the customer entity represents.
    #[prost(string, tag = "2")]
    pub org_display_name: ::prost::alloc::string::String,
    /// Required. The organization address for the customer. To enforce US laws and
    /// embargoes, we require a region and zip code. You must provide valid
    /// addresses for every customer. To set the customer's language, use the
    /// Customer-level language code.
    #[prost(message, optional, tag = "3")]
    pub org_postal_address: ::core::option::Option<
        super::super::super::r#type::PostalAddress,
    >,
    /// Primary contact info.
    #[prost(message, optional, tag = "4")]
    pub primary_contact_info: ::core::option::Option<ContactInfo>,
    /// Secondary contact email. You need to provide an alternate email to create
    /// different domains if a primary contact email already exists. Users will
    /// receive a notification with credentials when you create an admin.google.com
    /// account. Secondary emails are also recovery email addresses. Alternate
    /// emails are optional when you create Team customers.
    #[prost(string, tag = "5")]
    pub alternate_email: ::prost::alloc::string::String,
    /// Required. The customer's primary domain. Must match the primary contact
    /// email's domain.
    #[prost(string, tag = "6")]
    pub domain: ::prost::alloc::string::String,
    /// Output only. Time when the customer was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the customer was updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The customer's Cloud Identity ID if the customer has a Cloud
    /// Identity resource.
    #[prost(string, tag = "9")]
    pub cloud_identity_id: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <https://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    #[prost(string, tag = "10")]
    pub language_code: ::prost::alloc::string::String,
    /// Output only. Cloud Identity information for the customer.
    /// Populated only if a Cloud Identity account exists for this customer.
    #[prost(message, optional, tag = "12")]
    pub cloud_identity_info: ::core::option::Option<CloudIdentityInfo>,
    /// Cloud Identity ID of the customer's channel partner.
    /// Populated only if a channel partner exists for this customer.
    #[prost(string, tag = "13")]
    pub channel_partner_id: ::prost::alloc::string::String,
}
/// Contact information for a customer account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactInfo {
    /// The customer account contact's first name. Optional for Team customers.
    #[prost(string, tag = "1")]
    pub first_name: ::prost::alloc::string::String,
    /// The customer account contact's last name. Optional for Team customers.
    #[prost(string, tag = "2")]
    pub last_name: ::prost::alloc::string::String,
    /// Output only. The customer account contact's display name, formatted as a
    /// combination of the customer's first and last name.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// The customer account's contact email. Required for entitlements that create
    /// admin.google.com accounts, and serves as the customer's username for those
    /// accounts. Use this email to invite Team customers.
    #[prost(string, tag = "5")]
    pub email: ::prost::alloc::string::String,
    /// Optional. The customer account contact's job title.
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    /// The customer account's contact phone number.
    #[prost(string, tag = "7")]
    pub phone: ::prost::alloc::string::String,
}
/// A Product is the entity a customer uses when placing an order. For example,
/// Google Workspace, Google Voice, etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// Resource Name of the Product.
    /// Format: products/{product_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Marketing information for the product.
    #[prost(message, optional, tag = "2")]
    pub marketing_info: ::core::option::Option<MarketingInfo>,
}
/// Represents a product's purchasable Stock Keeping Unit (SKU).
/// SKUs represent the different variations of the product. For example, Google
/// Workspace Business Standard and Google Workspace Business Plus are Google
/// Workspace product SKUs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sku {
    /// Resource Name of the SKU.
    /// Format: products/{product_id}/skus/{sku_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Marketing information for the SKU.
    #[prost(message, optional, tag = "2")]
    pub marketing_info: ::core::option::Option<MarketingInfo>,
    /// Product the SKU is associated with.
    #[prost(message, optional, tag = "3")]
    pub product: ::core::option::Option<Product>,
}
/// Represents the marketing information for a Product, SKU or Offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketingInfo {
    /// Human readable name.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Human readable description. Description can contain HTML.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Default logo.
    #[prost(message, optional, tag = "3")]
    pub default_logo: ::core::option::Option<Media>,
}
/// Represents media information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Media {
    /// Title of the media.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// URL of the media.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// Type of the media.
    #[prost(enumeration = "MediaType", tag = "3")]
    pub r#type: i32,
}
/// Type of media used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediaType {
    /// Not used.
    Unspecified = 0,
    /// Type of image.
    Image = 1,
}
impl MediaType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MediaType::Unspecified => "MEDIA_TYPE_UNSPECIFIED",
            MediaType::Image => "MEDIA_TYPE_IMAGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEDIA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MEDIA_TYPE_IMAGE" => Some(Self::Image),
            _ => None,
        }
    }
}
/// Represents an offer made to resellers for purchase.
/// An offer is associated with a \[Sku][google.cloud.channel.v1.Sku\], has a plan for payment, a price, and
/// defines the constraints for buying.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Offer {
    /// Resource Name of the Offer.
    /// Format: accounts/{account_id}/offers/{offer_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Marketing information for the Offer.
    #[prost(message, optional, tag = "2")]
    pub marketing_info: ::core::option::Option<MarketingInfo>,
    /// SKU the offer is associated with.
    #[prost(message, optional, tag = "3")]
    pub sku: ::core::option::Option<Sku>,
    /// Describes the payment plan for the Offer.
    #[prost(message, optional, tag = "4")]
    pub plan: ::core::option::Option<Plan>,
    /// Constraints on transacting the Offer.
    #[prost(message, optional, tag = "5")]
    pub constraints: ::core::option::Option<Constraints>,
    /// Price for each monetizable resource type.
    #[prost(message, repeated, tag = "6")]
    pub price_by_resources: ::prost::alloc::vec::Vec<PriceByResource>,
    /// Start of the Offer validity time.
    #[prost(message, optional, tag = "7")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. End of the Offer validity time.
    #[prost(message, optional, tag = "8")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Parameters required to use current Offer to purchase.
    #[prost(message, repeated, tag = "9")]
    pub parameter_definitions: ::prost::alloc::vec::Vec<ParameterDefinition>,
}
/// Parameter's definition. Specifies what parameter is required to use the
/// current Offer to purchase.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterDefinition {
    /// Name of the parameter.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Data type of the parameter. Minimal value, Maximum value and allowed values
    /// will use specified data type here.
    #[prost(enumeration = "parameter_definition::ParameterType", tag = "2")]
    pub parameter_type: i32,
    /// Minimal value of the parameter, if applicable. Inclusive. For example,
    /// minimal commitment when purchasing Anthos is 0.01.
    /// Applicable to INT64 and DOUBLE parameter types.
    #[prost(message, optional, tag = "3")]
    pub min_value: ::core::option::Option<Value>,
    /// Maximum value of the parameter, if applicable. Inclusive. For example,
    /// maximum seats when purchasing Google Workspace Business Standard.
    /// Applicable to INT64 and DOUBLE parameter types.
    #[prost(message, optional, tag = "4")]
    pub max_value: ::core::option::Option<Value>,
    /// If not empty, parameter values must be drawn from this list.
    /// For example, [us-west1, us-west2, ...]
    /// Applicable to STRING parameter type.
    #[prost(message, repeated, tag = "5")]
    pub allowed_values: ::prost::alloc::vec::Vec<Value>,
    /// If set to true, parameter is optional to purchase this Offer.
    #[prost(bool, tag = "6")]
    pub optional: bool,
}
/// Nested message and enum types in `ParameterDefinition`.
pub mod parameter_definition {
    /// Data type of the parameter.
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
    pub enum ParameterType {
        /// Not used.
        Unspecified = 0,
        /// Int64 type.
        Int64 = 1,
        /// String type.
        String = 2,
        /// Double type.
        Double = 3,
    }
    impl ParameterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ParameterType::Unspecified => "PARAMETER_TYPE_UNSPECIFIED",
                ParameterType::Int64 => "INT64",
                ParameterType::String => "STRING",
                ParameterType::Double => "DOUBLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PARAMETER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "INT64" => Some(Self::Int64),
                "STRING" => Some(Self::String),
                "DOUBLE" => Some(Self::Double),
                _ => None,
            }
        }
    }
}
/// Represents the constraints for buying the Offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraints {
    /// Represents constraints required to purchase the Offer for a customer.
    #[prost(message, optional, tag = "1")]
    pub customer_constraints: ::core::option::Option<CustomerConstraints>,
}
/// Represents constraints required to purchase the Offer for a customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerConstraints {
    /// Allowed geographical regions of the customer.
    #[prost(string, repeated, tag = "1")]
    pub allowed_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Allowed Customer Type.
    #[prost(enumeration = "cloud_identity_info::CustomerType", repeated, tag = "2")]
    pub allowed_customer_types: ::prost::alloc::vec::Vec<i32>,
    /// Allowed Promotional Order Type. Present for Promotional offers.
    #[prost(enumeration = "PromotionalOrderType", repeated, tag = "3")]
    pub promotional_order_types: ::prost::alloc::vec::Vec<i32>,
}
/// The payment plan for the Offer. Describes how to make a payment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// Describes how a reseller will be billed.
    #[prost(enumeration = "PaymentPlan", tag = "1")]
    pub payment_plan: i32,
    /// Specifies when the payment needs to happen.
    #[prost(enumeration = "PaymentType", tag = "2")]
    pub payment_type: i32,
    /// Describes how frequently the reseller will be billed, such as
    /// once per month.
    #[prost(message, optional, tag = "3")]
    pub payment_cycle: ::core::option::Option<Period>,
    /// Present for Offers with a trial period.
    /// For trial-only Offers, a paid service needs to start before the trial
    /// period ends for continued service.
    /// For Regular Offers with a trial period, the regular pricing goes into
    /// effect when trial period ends, or if paid service is started before the end
    /// of the trial period.
    #[prost(message, optional, tag = "4")]
    pub trial_period: ::core::option::Option<Period>,
    /// Reseller Billing account to charge after an offer transaction.
    /// Only present for Google Cloud Platform offers.
    #[prost(string, tag = "5")]
    pub billing_account: ::prost::alloc::string::String,
}
/// Represents price by resource type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceByResource {
    /// Resource Type. Example: SEAT
    #[prost(enumeration = "ResourceType", tag = "1")]
    pub resource_type: i32,
    /// Price of the Offer. Present if there are no price phases.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Price>,
    /// Specifies the price by time range.
    #[prost(message, repeated, tag = "3")]
    pub price_phases: ::prost::alloc::vec::Vec<PricePhase>,
}
/// Represents the price of the Offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// Base price.
    #[prost(message, optional, tag = "1")]
    pub base_price: ::core::option::Option<super::super::super::r#type::Money>,
    /// Discount percentage, represented as decimal.
    /// For example, a 20% discount will be represent as 0.2.
    #[prost(double, tag = "2")]
    pub discount: f64,
    /// Effective Price after applying the discounts.
    #[prost(message, optional, tag = "3")]
    pub effective_price: ::core::option::Option<super::super::super::r#type::Money>,
    /// Link to external price list, such as link to Google Voice rate card.
    #[prost(string, tag = "4")]
    pub external_price_uri: ::prost::alloc::string::String,
}
/// Specifies the price by the duration of months.
/// For example, a 20% discount for the first six months, then a 10% discount
/// starting on the seventh month.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricePhase {
    /// Defines the phase period type.
    #[prost(enumeration = "PeriodType", tag = "1")]
    pub period_type: i32,
    /// Defines first period for the phase.
    #[prost(int32, tag = "2")]
    pub first_period: i32,
    /// Defines first period for the phase.
    #[prost(int32, tag = "3")]
    pub last_period: i32,
    /// Price of the phase. Present if there are no price tiers.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<Price>,
    /// Price by the resource tiers.
    #[prost(message, repeated, tag = "5")]
    pub price_tiers: ::prost::alloc::vec::Vec<PriceTier>,
}
/// Defines price at resource tier level.
/// For example, an offer with following definition :
///
/// * Tier 1: Provide 25% discount for all seats between 1 and 25.
/// * Tier 2: Provide 10% discount for all seats between 26 and 100.
/// * Tier 3: Provide flat 15% discount for all seats above 100.
///
/// Each of these tiers is represented as a PriceTier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceTier {
    /// First resource for which the tier price applies.
    #[prost(int32, tag = "1")]
    pub first_resource: i32,
    /// Last resource for which the tier price applies.
    #[prost(int32, tag = "2")]
    pub last_resource: i32,
    /// Price of the tier.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Price>,
}
/// Represents period in days/months/years.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Period {
    /// Total duration of Period Type defined.
    #[prost(int32, tag = "1")]
    pub duration: i32,
    /// Period Type.
    #[prost(enumeration = "PeriodType", tag = "2")]
    pub period_type: i32,
}
/// Constraints type for Promotional offers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PromotionalOrderType {
    /// Not used.
    PromotionalTypeUnspecified = 0,
    /// Order used for new customers, trial conversions and upgrades.
    NewUpgrade = 1,
    /// All orders for transferring an existing customer.
    Transfer = 2,
    /// Orders for modifying an existing customer's promotion on the same SKU.
    PromotionSwitch = 3,
}
impl PromotionalOrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PromotionalOrderType::PromotionalTypeUnspecified => {
                "PROMOTIONAL_TYPE_UNSPECIFIED"
            }
            PromotionalOrderType::NewUpgrade => "NEW_UPGRADE",
            PromotionalOrderType::Transfer => "TRANSFER",
            PromotionalOrderType::PromotionSwitch => "PROMOTION_SWITCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROMOTIONAL_TYPE_UNSPECIFIED" => Some(Self::PromotionalTypeUnspecified),
            "NEW_UPGRADE" => Some(Self::NewUpgrade),
            "TRANSFER" => Some(Self::Transfer),
            "PROMOTION_SWITCH" => Some(Self::PromotionSwitch),
            _ => None,
        }
    }
}
/// Describes how the reseller will be billed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentPlan {
    /// Not used.
    Unspecified = 0,
    /// Commitment.
    Commitment = 1,
    /// No commitment.
    Flexible = 2,
    /// Free.
    Free = 3,
    /// Trial.
    Trial = 4,
    /// Price and ordering not available through API.
    Offline = 5,
}
impl PaymentPlan {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentPlan::Unspecified => "PAYMENT_PLAN_UNSPECIFIED",
            PaymentPlan::Commitment => "COMMITMENT",
            PaymentPlan::Flexible => "FLEXIBLE",
            PaymentPlan::Free => "FREE",
            PaymentPlan::Trial => "TRIAL",
            PaymentPlan::Offline => "OFFLINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PAYMENT_PLAN_UNSPECIFIED" => Some(Self::Unspecified),
            "COMMITMENT" => Some(Self::Commitment),
            "FLEXIBLE" => Some(Self::Flexible),
            "FREE" => Some(Self::Free),
            "TRIAL" => Some(Self::Trial),
            "OFFLINE" => Some(Self::Offline),
            _ => None,
        }
    }
}
/// Specifies when the payment needs to happen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentType {
    /// Not used.
    Unspecified = 0,
    /// Prepay. Amount has to be paid before service is rendered.
    Prepay = 1,
    /// Postpay. Reseller is charged at the end of the Payment cycle.
    Postpay = 2,
}
impl PaymentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentType::Unspecified => "PAYMENT_TYPE_UNSPECIFIED",
            PaymentType::Prepay => "PREPAY",
            PaymentType::Postpay => "POSTPAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PAYMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PREPAY" => Some(Self::Prepay),
            "POSTPAY" => Some(Self::Postpay),
            _ => None,
        }
    }
}
/// Represents the type for a monetizable resource(any entity on which billing
/// happens). For example, this could be MINUTES for Google Voice and GB for
/// Google Drive. One SKU can map to multiple monetizable resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceType {
    /// Not used.
    Unspecified = 0,
    /// Seat.
    Seat = 1,
    /// Monthly active user.
    Mau = 2,
    /// GB (used for storage SKUs).
    Gb = 3,
    /// Active licensed users(for Voice SKUs).
    LicensedUser = 4,
    /// Voice usage.
    Minutes = 5,
    /// For IaaS SKUs like Google Cloud Platform, monetization is based on usage
    /// accrued on your billing account irrespective of the type of monetizable
    /// resource. This enum represents an aggregated resource/container for all
    /// usage SKUs on a billing account. Currently, only applicable to Google Cloud
    /// Platform.
    IaasUsage = 6,
    /// For Google Cloud Platform subscriptions like Anthos or SAP.
    Subscription = 7,
}
impl ResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceType::Unspecified => "RESOURCE_TYPE_UNSPECIFIED",
            ResourceType::Seat => "SEAT",
            ResourceType::Mau => "MAU",
            ResourceType::Gb => "GB",
            ResourceType::LicensedUser => "LICENSED_USER",
            ResourceType::Minutes => "MINUTES",
            ResourceType::IaasUsage => "IAAS_USAGE",
            ResourceType::Subscription => "SUBSCRIPTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SEAT" => Some(Self::Seat),
            "MAU" => Some(Self::Mau),
            "GB" => Some(Self::Gb),
            "LICENSED_USER" => Some(Self::LicensedUser),
            "MINUTES" => Some(Self::Minutes),
            "IAAS_USAGE" => Some(Self::IaasUsage),
            "SUBSCRIPTION" => Some(Self::Subscription),
            _ => None,
        }
    }
}
/// Period Type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeriodType {
    /// Not used.
    Unspecified = 0,
    /// Day.
    Day = 1,
    /// Month.
    Month = 2,
    /// Year.
    Year = 3,
}
impl PeriodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PeriodType::Unspecified => "PERIOD_TYPE_UNSPECIFIED",
            PeriodType::Day => "DAY",
            PeriodType::Month => "MONTH",
            PeriodType::Year => "YEAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERIOD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "DAY" => Some(Self::Day),
            "MONTH" => Some(Self::Month),
            "YEAR" => Some(Self::Year),
            _ => None,
        }
    }
}
/// An entitlement is a representation of a customer's ability to use a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entitlement {
    /// Output only. Resource name of an entitlement in the form:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which the entitlement is created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the entitlement is updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The offer resource name for which the entitlement is to be
    /// created. Takes the form: accounts/{account_id}/offers/{offer_id}.
    #[prost(string, tag = "8")]
    pub offer: ::prost::alloc::string::String,
    /// Commitment settings for a commitment-based Offer.
    /// Required for commitment based offers.
    #[prost(message, optional, tag = "12")]
    pub commitment_settings: ::core::option::Option<CommitmentSettings>,
    /// Output only. Current provisioning state of the entitlement.
    #[prost(enumeration = "entitlement::ProvisioningState", tag = "13")]
    pub provisioning_state: i32,
    /// Output only. Service provisioning details for the entitlement.
    #[prost(message, optional, tag = "16")]
    pub provisioned_service: ::core::option::Option<ProvisionedService>,
    /// Output only. Enumerable of all current suspension reasons for an entitlement.
    #[prost(
        enumeration = "entitlement::SuspensionReason",
        repeated,
        packed = "false",
        tag = "18"
    )]
    pub suspension_reasons: ::prost::alloc::vec::Vec<i32>,
    /// Optional. This purchase order (PO) information is for resellers to use for their
    /// company tracking usage. If a purchaseOrderId value is given, it appears in
    /// the API responses and shows up in the invoice. The property accepts up to
    /// 80 plain text characters.
    #[prost(string, tag = "19")]
    pub purchase_order_id: ::prost::alloc::string::String,
    /// Output only. Settings for trial offers.
    #[prost(message, optional, tag = "21")]
    pub trial_settings: ::core::option::Option<TrialSettings>,
    /// Association information to other entitlements.
    #[prost(message, optional, tag = "23")]
    pub association_info: ::core::option::Option<AssociationInfo>,
    /// Extended entitlement parameters. When creating an entitlement, valid
    /// parameter names and values are defined in the
    /// \[Offer.parameter_definitions][google.cloud.channel.v1.Offer.parameter_definitions\].
    ///
    /// The response may include the following output-only Parameters:
    ///
    /// - assigned_units: The number of licenses assigned to users.
    ///
    /// - max_units: The maximum assignable units for a flexible offer.
    ///
    /// - num_units: The total commitment for commitment-based offers.
    #[prost(message, repeated, tag = "26")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
}
/// Nested message and enum types in `Entitlement`.
pub mod entitlement {
    /// Indicates the current provisioning state of the entitlement.
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
    pub enum ProvisioningState {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// The entitlement is currently active.
        Active = 1,
        /// The entitlement is currently suspended.
        Suspended = 5,
    }
    impl ProvisioningState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProvisioningState::Unspecified => "PROVISIONING_STATE_UNSPECIFIED",
                ProvisioningState::Active => "ACTIVE",
                ProvisioningState::Suspended => "SUSPENDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROVISIONING_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "SUSPENDED" => Some(Self::Suspended),
                _ => None,
            }
        }
    }
    /// Suspension reason for an entitlement if \[provisioning_state][google.cloud.channel.v1.Entitlement.provisioning_state\] = SUSPENDED.
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
    pub enum SuspensionReason {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Entitlement was manually suspended by the Reseller.
        ResellerInitiated = 1,
        /// Trial ended.
        TrialEnded = 2,
        /// Entitlement renewal was canceled.
        RenewalWithTypeCancel = 3,
        /// Entitlement was automatically suspended on creation for pending ToS
        /// acceptance on customer.
        PendingTosAcceptance = 4,
        /// Other reasons (internal reasons, abuse, etc.).
        Other = 100,
    }
    impl SuspensionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SuspensionReason::Unspecified => "SUSPENSION_REASON_UNSPECIFIED",
                SuspensionReason::ResellerInitiated => "RESELLER_INITIATED",
                SuspensionReason::TrialEnded => "TRIAL_ENDED",
                SuspensionReason::RenewalWithTypeCancel => "RENEWAL_WITH_TYPE_CANCEL",
                SuspensionReason::PendingTosAcceptance => "PENDING_TOS_ACCEPTANCE",
                SuspensionReason::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUSPENSION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "RESELLER_INITIATED" => Some(Self::ResellerInitiated),
                "TRIAL_ENDED" => Some(Self::TrialEnded),
                "RENEWAL_WITH_TYPE_CANCEL" => Some(Self::RenewalWithTypeCancel),
                "PENDING_TOS_ACCEPTANCE" => Some(Self::PendingTosAcceptance),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
/// Definition for extended entitlement parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    /// Name of the parameter.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the parameter.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
    /// Output only. Specifies whether this parameter is allowed to be changed. For example, for
    /// a Google Workspace Business Starter entitlement in commitment plan,
    /// num_units is editable when entitlement is active.
    #[prost(bool, tag = "3")]
    pub editable: bool,
}
/// Association links that an entitlement has to other entitlements.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociationInfo {
    /// The name of the base entitlement, for which this entitlement is an add-on.
    #[prost(string, tag = "1")]
    pub base_entitlement: ::prost::alloc::string::String,
}
/// Service provisioned for an entitlement.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionedService {
    /// Output only. Provisioning ID of the entitlement. For Google Workspace, this would be the
    /// underlying Subscription ID.
    #[prost(string, tag = "1")]
    pub provisioning_id: ::prost::alloc::string::String,
    /// Output only. The product pertaining to the provisioning resource as specified in the
    /// Offer.
    #[prost(string, tag = "2")]
    pub product_id: ::prost::alloc::string::String,
    /// Output only. The SKU pertaining to the provisioning resource as specified in the Offer.
    #[prost(string, tag = "3")]
    pub sku_id: ::prost::alloc::string::String,
}
/// Commitment settings for commitment-based offers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitmentSettings {
    /// Output only. Commitment start timestamp.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Commitment end timestamp.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Renewal settings applicable for a commitment-based Offer.
    #[prost(message, optional, tag = "4")]
    pub renewal_settings: ::core::option::Option<RenewalSettings>,
}
/// Renewal settings for renewable Offers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenewalSettings {
    /// If false, the plan will be completed at the end date.
    #[prost(bool, tag = "1")]
    pub enable_renewal: bool,
    /// If true and enable_renewal = true, the unit (for example seats or licenses)
    /// will be set to the number of active units at renewal time.
    #[prost(bool, tag = "2")]
    pub resize_unit_count: bool,
    /// Describes how a reseller will be billed.
    #[prost(enumeration = "PaymentPlan", tag = "5")]
    pub payment_plan: i32,
    /// Describes how frequently the reseller will be billed, such as
    /// once per month.
    #[prost(message, optional, tag = "6")]
    pub payment_cycle: ::core::option::Option<Period>,
}
/// Settings for trial offers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrialSettings {
    /// Determines if the entitlement is in a trial or not:
    ///
    /// * `true` - The entitlement is in trial.
    /// * `false` - The entitlement is not in trial.
    #[prost(bool, tag = "1")]
    pub trial: bool,
    /// Date when the trial ends. The value is in milliseconds
    /// using the UNIX Epoch format. See an example [Epoch
    /// converter](<https://www.epochconverter.com>).
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// TransferableSku represents information a reseller needs to view existing
/// provisioned services for a customer that they do not own.
/// Read-only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferableSku {
    /// Describes the transfer eligibility of a SKU.
    #[prost(message, optional, tag = "9")]
    pub transfer_eligibility: ::core::option::Option<TransferEligibility>,
    /// The SKU pertaining to the provisioning resource as specified in the Offer.
    #[prost(message, optional, tag = "11")]
    pub sku: ::core::option::Option<Sku>,
    /// Optional. The customer to transfer has an entitlement with the populated legacy SKU.
    #[prost(message, optional, tag = "12")]
    pub legacy_sku: ::core::option::Option<Sku>,
}
/// Specifies transfer eligibility of a SKU.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEligibility {
    /// Whether reseller is eligible to transfer the SKU.
    #[prost(bool, tag = "1")]
    pub is_eligible: bool,
    /// Localized description if reseller is not eligible to transfer the SKU.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Specified the reason for ineligibility.
    #[prost(enumeration = "transfer_eligibility::Reason", tag = "3")]
    pub ineligibility_reason: i32,
}
/// Nested message and enum types in `TransferEligibility`.
pub mod transfer_eligibility {
    /// Reason of ineligibility.
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
        /// Reason is not available.
        Unspecified = 0,
        /// Reseller needs to accept TOS before transferring the SKU.
        PendingTosAcceptance = 1,
        /// Reseller not eligible to sell the SKU.
        SkuNotEligible = 2,
        /// SKU subscription is suspended
        SkuSuspended = 3,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::Unspecified => "REASON_UNSPECIFIED",
                Reason::PendingTosAcceptance => "PENDING_TOS_ACCEPTANCE",
                Reason::SkuNotEligible => "SKU_NOT_ELIGIBLE",
                Reason::SkuSuspended => "SKU_SUSPENDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING_TOS_ACCEPTANCE" => Some(Self::PendingTosAcceptance),
                "SKU_NOT_ELIGIBLE" => Some(Self::SkuNotEligible),
                "SKU_SUSPENDED" => Some(Self::SkuSuspended),
                _ => None,
            }
        }
    }
}
/// Provides contextual information about a \[google.longrunning.Operation][google.longrunning.Operation\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The RPC that initiated this Long Running Operation.
    #[prost(enumeration = "operation_metadata::OperationType", tag = "1")]
    pub operation_type: i32,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// RPCs that return a Long Running Operation.
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
    pub enum OperationType {
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Long Running Operation was triggered by CreateEntitlement.
        CreateEntitlement = 1,
        /// Long Running Operation was triggered by ChangeRenewalSettings.
        ChangeRenewalSettings = 3,
        /// Long Running Operation was triggered by StartPaidService.
        StartPaidService = 5,
        /// Long Running Operation was triggered by ActivateEntitlement.
        ActivateEntitlement = 7,
        /// Long Running Operation was triggered by SuspendEntitlement.
        SuspendEntitlement = 8,
        /// Long Running Operation was triggered by CancelEntitlement.
        CancelEntitlement = 9,
        /// Long Running Operation was triggered by TransferEntitlements.
        TransferEntitlements = 10,
        /// Long Running Operation was triggered by TransferEntitlementsToGoogle.
        TransferEntitlementsToGoogle = 11,
        /// Long Running Operation was triggered by ChangeOffer.
        ChangeOffer = 14,
        /// Long Running Operation was triggered by ChangeParameters.
        ChangeParameters = 15,
        /// Long Running Operation was triggered by ProvisionCloudIdentity.
        ProvisionCloudIdentity = 16,
    }
    impl OperationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
                OperationType::CreateEntitlement => "CREATE_ENTITLEMENT",
                OperationType::ChangeRenewalSettings => "CHANGE_RENEWAL_SETTINGS",
                OperationType::StartPaidService => "START_PAID_SERVICE",
                OperationType::ActivateEntitlement => "ACTIVATE_ENTITLEMENT",
                OperationType::SuspendEntitlement => "SUSPEND_ENTITLEMENT",
                OperationType::CancelEntitlement => "CANCEL_ENTITLEMENT",
                OperationType::TransferEntitlements => "TRANSFER_ENTITLEMENTS",
                OperationType::TransferEntitlementsToGoogle => {
                    "TRANSFER_ENTITLEMENTS_TO_GOOGLE"
                }
                OperationType::ChangeOffer => "CHANGE_OFFER",
                OperationType::ChangeParameters => "CHANGE_PARAMETERS",
                OperationType::ProvisionCloudIdentity => "PROVISION_CLOUD_IDENTITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE_ENTITLEMENT" => Some(Self::CreateEntitlement),
                "CHANGE_RENEWAL_SETTINGS" => Some(Self::ChangeRenewalSettings),
                "START_PAID_SERVICE" => Some(Self::StartPaidService),
                "ACTIVATE_ENTITLEMENT" => Some(Self::ActivateEntitlement),
                "SUSPEND_ENTITLEMENT" => Some(Self::SuspendEntitlement),
                "CANCEL_ENTITLEMENT" => Some(Self::CancelEntitlement),
                "TRANSFER_ENTITLEMENTS" => Some(Self::TransferEntitlements),
                "TRANSFER_ENTITLEMENTS_TO_GOOGLE" => {
                    Some(Self::TransferEntitlementsToGoogle)
                }
                "CHANGE_OFFER" => Some(Self::ChangeOffer),
                "CHANGE_PARAMETERS" => Some(Self::ChangeParameters),
                "PROVISION_CLOUD_IDENTITY" => Some(Self::ProvisionCloudIdentity),
                _ => None,
            }
        }
    }
}
/// Request message for \[CloudChannelService.CheckCloudIdentityAccountsExist][google.cloud.channel.v1.CloudChannelService.CheckCloudIdentityAccountsExist\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCloudIdentityAccountsExistRequest {
    /// Required. The reseller account's resource name.
    /// Parent uses the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Domain to fetch for Cloud Identity account customer.
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
}
/// Entity representing a Cloud Identity account that may be
/// associated with a Channel Services API partner.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudIdentityCustomerAccount {
    /// Returns true if a Cloud Identity account exists for a specific domain.
    #[prost(bool, tag = "1")]
    pub existing: bool,
    /// Returns true if the Cloud Identity account is associated with a customer
    /// of the Channel Services partner.
    #[prost(bool, tag = "2")]
    pub owned: bool,
    /// If owned = true, the name of the customer that owns the Cloud Identity
    /// account.
    /// Customer_name uses the format:
    /// accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "3")]
    pub customer_name: ::prost::alloc::string::String,
    /// If existing = true, the Cloud Identity ID of the customer.
    #[prost(string, tag = "4")]
    pub customer_cloud_identity_id: ::prost::alloc::string::String,
}
/// Response message for
/// \[CloudChannelService.CheckCloudIdentityAccountsExist][google.cloud.channel.v1.CloudChannelService.CheckCloudIdentityAccountsExist\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCloudIdentityAccountsExistResponse {
    /// The Cloud Identity accounts associated with the domain.
    #[prost(message, repeated, tag = "1")]
    pub cloud_identity_accounts: ::prost::alloc::vec::Vec<CloudIdentityCustomerAccount>,
}
/// Request message for \[CloudChannelService.ListCustomers][google.cloud.channel.v1.CloudChannelService.ListCustomers\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomersRequest {
    /// Required. The resource name of the reseller account to list customers from.
    /// Parent uses the format: accounts/{account_id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of customers to return. The service may return fewer
    /// than this value. If unspecified, returns at most 10 customers. The
    /// maximum value is 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results other than the first page.
    /// Obtained through
    /// \[ListCustomersResponse.next_page_token][google.cloud.channel.v1.ListCustomersResponse.next_page_token\] of the previous
    /// \[CloudChannelService.ListCustomers][google.cloud.channel.v1.CloudChannelService.ListCustomers\] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[CloudChannelService.ListCustomers][google.cloud.channel.v1.CloudChannelService.ListCustomers\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomersResponse {
    /// The customers belonging to a reseller or distributor.
    #[prost(message, repeated, tag = "1")]
    pub customers: ::prost::alloc::vec::Vec<Customer>,
    /// A token to retrieve the next page of results.
    /// Pass to \[ListCustomersRequest.page_token][google.cloud.channel.v1.ListCustomersRequest.page_token\] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.GetCustomer][google.cloud.channel.v1.CloudChannelService.GetCustomer\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerRequest {
    /// Required. The resource name of the customer to retrieve.
    /// Name uses the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.CreateCustomer][google.cloud.channel.v1.CloudChannelService.CreateCustomer\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomerRequest {
    /// Required. The resource name of reseller account in which to create the customer.
    /// Parent uses the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The customer to create.
    #[prost(message, optional, tag = "2")]
    pub customer: ::core::option::Option<Customer>,
}
/// Request message for \[CloudChannelService.UpdateCustomer][google.cloud.channel.v1.CloudChannelService.UpdateCustomer\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomerRequest {
    /// Required. New contents of the customer.
    #[prost(message, optional, tag = "2")]
    pub customer: ::core::option::Option<Customer>,
    /// The update mask that applies to the resource.
    /// Optional.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for \[CloudChannelService.DeleteCustomer][google.cloud.channel.v1.CloudChannelService.DeleteCustomer\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCustomerRequest {
    /// Required. The resource name of the customer to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.ImportCustomer][google.cloud.channel.v1.CloudChannelService.ImportCustomer\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCustomerRequest {
    /// Required. The resource name of the reseller's account.
    /// Parent takes the format: accounts/{account_id} or
    /// accounts/{account_id}/channelPartnerLinks/{channel_partner_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The super admin of the resold customer generates this token to
    /// authorize a reseller to access their Cloud Identity and purchase
    /// entitlements on their behalf. You can omit this token after authorization.
    /// See <https://support.google.com/a/answer/7643790> for more details.
    #[prost(string, tag = "4")]
    pub auth_token: ::prost::alloc::string::String,
    /// Required. Choose to overwrite an existing customer if found.
    /// This must be set to true if there is an existing customer with a
    /// conflicting region code or domain.
    #[prost(bool, tag = "5")]
    pub overwrite_if_exists: bool,
    /// Optional. Cloud Identity ID of a channel partner who will be the direct reseller for
    /// the customer's order. This field is required for 2-tier transfer scenarios
    /// and can be provided via the request Parent binding as well.
    #[prost(string, tag = "6")]
    pub channel_partner_id: ::prost::alloc::string::String,
    /// Optional. Specifies the customer that will receive imported Cloud Identity
    /// information.
    /// Format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "7")]
    pub customer: ::prost::alloc::string::String,
    /// Specifies the identity of the transfer customer.
    /// A customer's cloud_identity_id or domain is required to look up the
    /// customer's Cloud Identity. For Team customers, only the cloud_identity_id
    /// option is valid.
    #[prost(oneof = "import_customer_request::CustomerIdentity", tags = "2, 3")]
    pub customer_identity: ::core::option::Option<
        import_customer_request::CustomerIdentity,
    >,
}
/// Nested message and enum types in `ImportCustomerRequest`.
pub mod import_customer_request {
    /// Specifies the identity of the transfer customer.
    /// A customer's cloud_identity_id or domain is required to look up the
    /// customer's Cloud Identity. For Team customers, only the cloud_identity_id
    /// option is valid.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CustomerIdentity {
        /// Required. Customer domain.
        #[prost(string, tag = "2")]
        Domain(::prost::alloc::string::String),
        /// Required. Customer's Cloud Identity ID
        #[prost(string, tag = "3")]
        CloudIdentityId(::prost::alloc::string::String),
    }
}
/// Request message for \[CloudChannelService.ProvisionCloudIdentity][google.cloud.channel.v1.CloudChannelService.ProvisionCloudIdentity\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionCloudIdentityRequest {
    /// Required. Resource name of the customer.
    /// Format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub customer: ::prost::alloc::string::String,
    /// CloudIdentity-specific customer information.
    #[prost(message, optional, tag = "2")]
    pub cloud_identity_info: ::core::option::Option<CloudIdentityInfo>,
    /// Admin user information.
    #[prost(message, optional, tag = "3")]
    pub user: ::core::option::Option<AdminUser>,
    /// Validate the request and preview the review, but do not post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for \[CloudChannelService.ListEntitlements][google.cloud.channel.v1.CloudChannelService.ListEntitlements\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntitlementsRequest {
    /// Required. The resource name of the reseller's customer account to list
    /// entitlements for.
    /// Parent uses the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, return at most 50 entitlements.
    /// The maximum value is 100; the server will coerce values above 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token for a page of results other than the first page.
    /// Obtained using
    /// \[ListEntitlementsResponse.next_page_token][google.cloud.channel.v1.ListEntitlementsResponse.next_page_token\] of the previous
    /// \[CloudChannelService.ListEntitlements][google.cloud.channel.v1.CloudChannelService.ListEntitlements\] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[CloudChannelService.ListEntitlements][google.cloud.channel.v1.CloudChannelService.ListEntitlements\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntitlementsResponse {
    /// The reseller customer's entitlements.
    #[prost(message, repeated, tag = "1")]
    pub entitlements: ::prost::alloc::vec::Vec<Entitlement>,
    /// A token to list the next page of results.
    /// Pass to \[ListEntitlementsRequest.page_token][google.cloud.channel.v1.ListEntitlementsRequest.page_token\] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.ListTransferableSkus][google.cloud.channel.v1.CloudChannelService.ListTransferableSkus\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferableSkusRequest {
    /// Required. The reseller account's resource name.
    /// Parent uses the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The requested page size. Server might return fewer results than requested.
    /// If unspecified, returns at most 100 SKUs.
    /// The maximum value is 1000; the server will coerce values above 1000.
    /// Optional.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token for a page of results other than the first page.
    /// Obtained using
    /// \[ListTransferableSkusResponse.next_page_token][google.cloud.channel.v1.ListTransferableSkusResponse.next_page_token\] of the previous
    /// \[CloudChannelService.ListTransferableSkus][google.cloud.channel.v1.CloudChannelService.ListTransferableSkus\] call.
    /// Optional.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The super admin of the resold customer generates this token to
    /// authorize a reseller to access their Cloud Identity and purchase
    /// entitlements on their behalf. You can omit this token after authorization.
    /// See <https://support.google.com/a/answer/7643790> for more details.
    #[prost(string, tag = "5")]
    pub auth_token: ::prost::alloc::string::String,
    /// The BCP-47 language code. For example, "en-US". The
    /// response will localize in the corresponding language code, if specified.
    /// The default value is "en-US".
    /// Optional.
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Specifies the identity of transferred customer.
    /// Either a cloud_identity_id of the customer or the customer name is
    /// required to look up transferable SKUs.
    #[prost(
        oneof = "list_transferable_skus_request::TransferredCustomerIdentity",
        tags = "4, 7"
    )]
    pub transferred_customer_identity: ::core::option::Option<
        list_transferable_skus_request::TransferredCustomerIdentity,
    >,
}
/// Nested message and enum types in `ListTransferableSkusRequest`.
pub mod list_transferable_skus_request {
    /// Specifies the identity of transferred customer.
    /// Either a cloud_identity_id of the customer or the customer name is
    /// required to look up transferable SKUs.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TransferredCustomerIdentity {
        /// Customer's Cloud Identity ID
        #[prost(string, tag = "4")]
        CloudIdentityId(::prost::alloc::string::String),
        /// A reseller is required to create a customer and use the resource name of
        /// the created customer here.
        /// Customer_name uses the format:
        /// accounts/{account_id}/customers/{customer_id}
        #[prost(string, tag = "7")]
        CustomerName(::prost::alloc::string::String),
    }
}
/// Response message for \[CloudChannelService.ListTransferableSkus][google.cloud.channel.v1.CloudChannelService.ListTransferableSkus\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferableSkusResponse {
    /// Information about existing SKUs for a customer that needs a transfer.
    #[prost(message, repeated, tag = "1")]
    pub transferable_skus: ::prost::alloc::vec::Vec<TransferableSku>,
    /// A token to retrieve the next page of results.
    /// Pass to \[ListTransferableSkusRequest.page_token][google.cloud.channel.v1.ListTransferableSkusRequest.page_token\] to obtain
    /// that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.ListTransferableOffers][google.cloud.channel.v1.CloudChannelService.ListTransferableOffers\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferableOffersRequest {
    /// Required. The resource name of the reseller's account.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server might return fewer results than requested.
    /// If unspecified, returns at most 100 offers.
    /// The maximum value is 1000; the server will coerce values above 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token for a page of results other than the first page.
    /// Obtained using
    /// \[ListTransferableOffersResponse.next_page_token][google.cloud.channel.v1.ListTransferableOffersResponse.next_page_token\] of the previous
    /// \[CloudChannelService.ListTransferableOffers][google.cloud.channel.v1.CloudChannelService.ListTransferableOffers\] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Required. The SKU to look up Offers for.
    #[prost(string, tag = "6")]
    pub sku: ::prost::alloc::string::String,
    /// The BCP-47 language code. For example, "en-US". The
    /// response will localize in the corresponding language code, if specified.
    /// The default value is "en-US".
    #[prost(string, tag = "7")]
    pub language_code: ::prost::alloc::string::String,
    /// Specifies the identity of transferred customer.
    /// Either a cloud_identity_id of the customer or the customer name is
    /// required to look up transferrable Offers.
    #[prost(
        oneof = "list_transferable_offers_request::TransferredCustomerIdentity",
        tags = "4, 5"
    )]
    pub transferred_customer_identity: ::core::option::Option<
        list_transferable_offers_request::TransferredCustomerIdentity,
    >,
}
/// Nested message and enum types in `ListTransferableOffersRequest`.
pub mod list_transferable_offers_request {
    /// Specifies the identity of transferred customer.
    /// Either a cloud_identity_id of the customer or the customer name is
    /// required to look up transferrable Offers.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TransferredCustomerIdentity {
        /// Customer's Cloud Identity ID
        #[prost(string, tag = "4")]
        CloudIdentityId(::prost::alloc::string::String),
        /// A reseller should create a customer and use the resource name of
        /// that customer here.
        #[prost(string, tag = "5")]
        CustomerName(::prost::alloc::string::String),
    }
}
/// Response message for \[CloudChannelService.ListTransferableOffers][google.cloud.channel.v1.CloudChannelService.ListTransferableOffers\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferableOffersResponse {
    /// Information about Offers for a customer that can be used for
    /// transfer.
    #[prost(message, repeated, tag = "1")]
    pub transferable_offers: ::prost::alloc::vec::Vec<TransferableOffer>,
    /// A token to retrieve the next page of results.
    /// Pass to \[ListTransferableOffersRequest.page_token][google.cloud.channel.v1.ListTransferableOffersRequest.page_token\] to obtain
    /// that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// TransferableOffer represents an Offer that can be used in Transfer.
/// Read-only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferableOffer {
    /// Offer with parameter constraints updated to allow the Transfer.
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<Offer>,
}
/// Request message for \[CloudChannelService.GetEntitlement][google.cloud.channel.v1.CloudChannelService.GetEntitlement\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitlementRequest {
    /// Required. The resource name of the entitlement to retrieve.
    /// Name uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.ListChannelPartnerLinks][google.cloud.channel.v1.CloudChannelService.ListChannelPartnerLinks\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelPartnerLinksRequest {
    /// Required. The resource name of the reseller account for listing channel partner
    /// links.
    /// Parent uses the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, server will pick a default size (25).
    /// The maximum value is 200; the server will coerce values above 200.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token for a page of results other than the first page.
    /// Obtained using
    /// \[ListChannelPartnerLinksResponse.next_page_token][google.cloud.channel.v1.ListChannelPartnerLinksResponse.next_page_token\] of the previous
    /// \[CloudChannelService.ListChannelPartnerLinks][google.cloud.channel.v1.CloudChannelService.ListChannelPartnerLinks\] call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The level of granularity the ChannelPartnerLink will display.
    #[prost(enumeration = "ChannelPartnerLinkView", tag = "4")]
    pub view: i32,
}
/// Response message for \[CloudChannelService.ListChannelPartnerLinks][google.cloud.channel.v1.CloudChannelService.ListChannelPartnerLinks\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelPartnerLinksResponse {
    /// The Channel partner links for a reseller.
    #[prost(message, repeated, tag = "1")]
    pub channel_partner_links: ::prost::alloc::vec::Vec<ChannelPartnerLink>,
    /// A token to retrieve the next page of results.
    /// Pass to \[ListChannelPartnerLinksRequest.page_token][google.cloud.channel.v1.ListChannelPartnerLinksRequest.page_token\] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.GetChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.GetChannelPartnerLink\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelPartnerLinkRequest {
    /// Required. The resource name of the channel partner link to retrieve.
    /// Name uses the format: accounts/{account_id}/channelPartnerLinks/{id}
    /// where {id} is the Cloud Identity ID of the partner.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The level of granularity the ChannelPartnerLink will display.
    #[prost(enumeration = "ChannelPartnerLinkView", tag = "2")]
    pub view: i32,
}
/// Request message for \[CloudChannelService.CreateChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.CreateChannelPartnerLink\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelPartnerLinkRequest {
    /// Required. Create a channel partner link for the provided reseller account's
    /// resource name.
    /// Parent uses the format: accounts/{account_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The channel partner link to create.
    /// Either channel_partner_link.reseller_cloud_identity_id or domain can be
    /// used to create a link.
    #[prost(message, optional, tag = "2")]
    pub channel_partner_link: ::core::option::Option<ChannelPartnerLink>,
}
/// Request message for \[CloudChannelService.UpdateChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.UpdateChannelPartnerLink\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannelPartnerLinkRequest {
    /// Required. The resource name of the channel partner link to cancel.
    /// Name uses the format: accounts/{account_id}/channelPartnerLinks/{id}
    /// where {id} is the Cloud Identity ID of the partner.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The channel partner link to update. Only channel_partner_link.link_state
    /// is allowed for updates.
    #[prost(message, optional, tag = "2")]
    pub channel_partner_link: ::core::option::Option<ChannelPartnerLink>,
    /// Required. The update mask that applies to the resource.
    /// The only allowable value for an update mask is
    /// channel_partner_link.link_state.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for \[CloudChannelService.CreateEntitlement][google.cloud.channel.v1.CloudChannelService.CreateEntitlement\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntitlementRequest {
    /// Required. The resource name of the reseller's customer account in which to create the
    /// entitlement.
    /// Parent uses the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The entitlement to create.
    #[prost(message, optional, tag = "2")]
    pub entitlement: ::core::option::Option<Entitlement>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.TransferEntitlements][google.cloud.channel.v1.CloudChannelService.TransferEntitlements\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEntitlementsRequest {
    /// Required. The resource name of the reseller's customer account that will receive
    /// transferred entitlements.
    /// Parent uses the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The new entitlements to create or transfer.
    #[prost(message, repeated, tag = "2")]
    pub entitlements: ::prost::alloc::vec::Vec<Entitlement>,
    /// The super admin of the resold customer generates this token to
    /// authorize a reseller to access their Cloud Identity and purchase
    /// entitlements on their behalf. You can omit this token after authorization.
    /// See <https://support.google.com/a/answer/7643790> for more details.
    #[prost(string, tag = "4")]
    pub auth_token: ::prost::alloc::string::String,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for \[CloudChannelService.TransferEntitlements][google.cloud.channel.v1.CloudChannelService.TransferEntitlements\].
/// This is put in the response field of google.longrunning.Operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEntitlementsResponse {
    /// The transferred entitlements.
    #[prost(message, repeated, tag = "1")]
    pub entitlements: ::prost::alloc::vec::Vec<Entitlement>,
}
/// Request message for \[CloudChannelService.TransferEntitlementsToGoogle][google.cloud.channel.v1.CloudChannelService.TransferEntitlementsToGoogle\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEntitlementsToGoogleRequest {
    /// Required. The resource name of the reseller's customer account where the entitlements
    /// transfer from.
    /// Parent uses the format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The entitlements to transfer to Google.
    #[prost(message, repeated, tag = "2")]
    pub entitlements: ::prost::alloc::vec::Vec<Entitlement>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.ChangeParametersRequest][\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeParametersRequest {
    /// Required. The name of the entitlement to update.
    /// Name uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Entitlement parameters to update. You can only change editable parameters.
    ///
    /// To view the available Parameters for a request, refer to the
    /// \[Offer.parameter_definitions][google.cloud.channel.v1.Offer.parameter_definitions\] from the desired offer.
    #[prost(message, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Purchase order ID provided by the reseller.
    #[prost(string, tag = "5")]
    pub purchase_order_id: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.ChangeRenewalSettings][google.cloud.channel.v1.CloudChannelService.ChangeRenewalSettings\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeRenewalSettingsRequest {
    /// Required. The name of the entitlement to update.
    /// Name uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. New renewal settings.
    #[prost(message, optional, tag = "4")]
    pub renewal_settings: ::core::option::Option<RenewalSettings>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.ChangeOffer][google.cloud.channel.v1.CloudChannelService.ChangeOffer\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOfferRequest {
    /// Required. The resource name of the entitlement to update.
    /// Name uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. New Offer.
    /// Format: accounts/{account_id}/offers/{offer_id}.
    #[prost(string, tag = "2")]
    pub offer: ::prost::alloc::string::String,
    /// Optional. Parameters needed to purchase the Offer. To view the available Parameters
    /// refer to the \[Offer.parameter_definitions][google.cloud.channel.v1.Offer.parameter_definitions\] from the desired offer.
    #[prost(message, repeated, tag = "3")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
    /// Optional. Purchase order id provided by the reseller.
    #[prost(string, tag = "5")]
    pub purchase_order_id: ::prost::alloc::string::String,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.StartPaidService][google.cloud.channel.v1.CloudChannelService.StartPaidService\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPaidServiceRequest {
    /// Required. The name of the entitlement to start a paid service for.
    /// Name uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.CancelEntitlement][google.cloud.channel.v1.CloudChannelService.CancelEntitlement\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelEntitlementRequest {
    /// Required. The resource name of the entitlement to cancel.
    /// Name uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.SuspendEntitlement][google.cloud.channel.v1.CloudChannelService.SuspendEntitlement\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuspendEntitlementRequest {
    /// Required. The resource name of the entitlement to suspend.
    /// Name uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for \[CloudChannelService.ActivateEntitlement][google.cloud.channel.v1.CloudChannelService.ActivateEntitlement\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateEntitlementRequest {
    /// Required. The resource name of the entitlement to activate.
    /// Name uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. You can specify an optional unique request ID, and if you need to retry
    /// your request, the server will know to ignore the request if it's complete.
    ///
    /// For example, you make an initial request and the request times out. If you
    /// make the request again with the same request ID, the server can check if
    /// it received the original operation with the same request ID. If it did, it
    /// will ignore the second request.
    ///
    /// The request ID must be a valid \[UUID\](<https://tools.ietf.org/html/rfc4122>)
    /// with the exception that zero UUID is not supported
    /// (`00000000-0000-0000-0000-000000000000`).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for LookupOffer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupOfferRequest {
    /// Required. The resource name of the entitlement to retrieve the Offer.
    /// Entitlement uses the format:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub entitlement: ::prost::alloc::string::String,
}
/// Request message for ListProducts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsRequest {
    /// Required. The resource name of the reseller account.
    /// Format: accounts/{account_id}.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, returns at most 100 Products.
    /// The maximum value is 1000; the server will coerce values above 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token for a page of results other than the first page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code. For example, "en-US". The
    /// response will localize in the corresponding language code, if specified.
    /// The default value is "en-US".
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response message for ListProducts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsResponse {
    /// List of Products requested.
    #[prost(message, repeated, tag = "1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ListSkus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkusRequest {
    /// Required. The resource name of the Product to list SKUs for.
    /// Parent uses the format: products/{product_id}.
    /// Supports products/- to retrieve SKUs for all products.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Resource name of the reseller.
    /// Format: accounts/{account_id}.
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, returns at most 100 SKUs.
    /// The maximum value is 1000; the server will coerce values above 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. A token for a page of results other than the first page.
    /// Optional.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code. For example, "en-US". The
    /// response will localize in the corresponding language code, if specified.
    /// The default value is "en-US".
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response message for ListSkus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkusResponse {
    /// The list of SKUs requested.
    #[prost(message, repeated, tag = "1")]
    pub skus: ::prost::alloc::vec::Vec<Sku>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ListOffers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersRequest {
    /// Required. The resource name of the reseller account from which to list Offers.
    /// Parent uses the format: accounts/{account_id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, returns at most 500 Offers.
    /// The maximum value is 1000; the server will coerce values above 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token for a page of results other than the first page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The expression to filter results by name (name of
    /// the Offer), sku.name (name of the SKU), or sku.product.name (name of the
    /// Product).
    /// Example 1: sku.product.name=products/p1 AND sku.name!=products/p1/skus/s1
    /// Example 2: name=accounts/a1/offers/o1
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code. For example, "en-US". The
    /// response will localize in the corresponding language code, if specified.
    /// The default value is "en-US".
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response message for ListOffers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersResponse {
    /// The list of Offers requested.
    #[prost(message, repeated, tag = "1")]
    pub offers: ::prost::alloc::vec::Vec<Offer>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ListPurchasableSkus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPurchasableSkusRequest {
    /// Required. The resource name of the customer to list SKUs for.
    /// Format: accounts/{account_id}/customers/{customer_id}.
    #[prost(string, tag = "1")]
    pub customer: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, returns at most 100 SKUs.
    /// The maximum value is 1000; the server will coerce values above 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A token for a page of results other than the first page.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code. For example, "en-US". The
    /// response will localize in the corresponding language code, if specified.
    /// The default value is "en-US".
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Defines the intended purchase.
    #[prost(oneof = "list_purchasable_skus_request::PurchaseOption", tags = "2, 3")]
    pub purchase_option: ::core::option::Option<
        list_purchasable_skus_request::PurchaseOption,
    >,
}
/// Nested message and enum types in `ListPurchasableSkusRequest`.
pub mod list_purchasable_skus_request {
    /// List SKUs for a new entitlement. Make the purchase using
    /// \[CloudChannelService.CreateEntitlement][google.cloud.channel.v1.CloudChannelService.CreateEntitlement\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateEntitlementPurchase {
        /// Required. List SKUs belonging to this Product.
        /// Format: products/{product_id}.
        /// Supports products/- to retrieve SKUs for all products.
        #[prost(string, tag = "1")]
        pub product: ::prost::alloc::string::String,
    }
    /// List SKUs for upgrading or downgrading an entitlement. Make the purchase
    /// using \[CloudChannelService.ChangeOffer][google.cloud.channel.v1.CloudChannelService.ChangeOffer\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangeOfferPurchase {
        /// Required. Resource name of the entitlement.
        /// Format:
        /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
        #[prost(string, tag = "1")]
        pub entitlement: ::prost::alloc::string::String,
        /// Required. Change Type for the entitlement.
        #[prost(enumeration = "change_offer_purchase::ChangeType", tag = "2")]
        pub change_type: i32,
    }
    /// Nested message and enum types in `ChangeOfferPurchase`.
    pub mod change_offer_purchase {
        /// Change Type enum.
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
        pub enum ChangeType {
            /// Not used.
            Unspecified = 0,
            /// SKU is an upgrade on the current entitlement.
            Upgrade = 1,
            /// SKU is a downgrade on the current entitlement.
            Downgrade = 2,
        }
        impl ChangeType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ChangeType::Unspecified => "CHANGE_TYPE_UNSPECIFIED",
                    ChangeType::Upgrade => "UPGRADE",
                    ChangeType::Downgrade => "DOWNGRADE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "UPGRADE" => Some(Self::Upgrade),
                    "DOWNGRADE" => Some(Self::Downgrade),
                    _ => None,
                }
            }
        }
    }
    /// Defines the intended purchase.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PurchaseOption {
        /// List SKUs for CreateEntitlement purchase.
        #[prost(message, tag = "2")]
        CreateEntitlementPurchase(CreateEntitlementPurchase),
        /// List SKUs for ChangeOffer purchase with a new SKU.
        #[prost(message, tag = "3")]
        ChangeOfferPurchase(ChangeOfferPurchase),
    }
}
/// Response message for ListPurchasableSkus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPurchasableSkusResponse {
    /// The list of SKUs requested.
    #[prost(message, repeated, tag = "1")]
    pub purchasable_skus: ::prost::alloc::vec::Vec<PurchasableSku>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// SKU that you can purchase. This is used in ListPurchasableSku API
/// response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchasableSku {
    /// SKU
    #[prost(message, optional, tag = "1")]
    pub sku: ::core::option::Option<Sku>,
}
/// Request message for ListPurchasableOffers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPurchasableOffersRequest {
    /// Required. The resource name of the customer to list Offers for.
    /// Format: accounts/{account_id}/customers/{customer_id}.
    #[prost(string, tag = "1")]
    pub customer: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server might return fewer results than requested.
    /// If unspecified, returns at most 100 Offers.
    /// The maximum value is 1000; the server will coerce values above 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A token for a page of results other than the first page.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code. For example, "en-US". The
    /// response will localize in the corresponding language code, if specified.
    /// The default value is "en-US".
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Defines the intended purchase.
    #[prost(oneof = "list_purchasable_offers_request::PurchaseOption", tags = "2, 3")]
    pub purchase_option: ::core::option::Option<
        list_purchasable_offers_request::PurchaseOption,
    >,
}
/// Nested message and enum types in `ListPurchasableOffersRequest`.
pub mod list_purchasable_offers_request {
    /// List Offers for CreateEntitlement purchase.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateEntitlementPurchase {
        /// Required. SKU that the result should be restricted to.
        /// Format: products/{product_id}/skus/{sku_id}.
        #[prost(string, tag = "1")]
        pub sku: ::prost::alloc::string::String,
    }
    /// List Offers for ChangeOffer purchase.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangeOfferPurchase {
        /// Required. Resource name of the entitlement.
        /// Format:
        /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
        #[prost(string, tag = "1")]
        pub entitlement: ::prost::alloc::string::String,
        /// Optional. Resource name of the new target SKU. Provide this SKU when
        /// upgrading or downgrading an entitlement. Format:
        /// products/{product_id}/skus/{sku_id}
        #[prost(string, tag = "2")]
        pub new_sku: ::prost::alloc::string::String,
    }
    /// Defines the intended purchase.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PurchaseOption {
        /// List Offers for CreateEntitlement purchase.
        #[prost(message, tag = "2")]
        CreateEntitlementPurchase(CreateEntitlementPurchase),
        /// List Offers for ChangeOffer purchase.
        #[prost(message, tag = "3")]
        ChangeOfferPurchase(ChangeOfferPurchase),
    }
}
/// Response message for ListPurchasableOffers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPurchasableOffersResponse {
    /// The list of Offers requested.
    #[prost(message, repeated, tag = "1")]
    pub purchasable_offers: ::prost::alloc::vec::Vec<PurchasableOffer>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Offer that you can purchase for a customer. This is used in the
/// ListPurchasableOffer API response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchasableOffer {
    /// Offer.
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<Offer>,
}
/// Request Message for RegisterSubscriber.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterSubscriberRequest {
    /// Required. Resource name of the account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Required. Service account that provides subscriber access to the registered topic.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
}
/// Response Message for RegisterSubscriber.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterSubscriberResponse {
    /// Name of the topic the subscriber will listen to.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// Request Message for UnregisterSubscriber.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnregisterSubscriberRequest {
    /// Required. Resource name of the account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Required. Service account to unregister from subscriber access to the topic.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
}
/// Response Message for UnregisterSubscriber.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnregisterSubscriberResponse {
    /// Name of the topic the service account subscriber access was removed from.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// Request Message for ListSubscribers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscribersRequest {
    /// Required. Resource name of the account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Optional. The maximum number of service accounts to return. The service may return
    /// fewer than this value.
    /// If unspecified, returns at most 100 service accounts.
    /// The maximum value is 1000; the server will coerce values above 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListSubscribers` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListSubscribers` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response Message for ListSubscribers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscribersResponse {
    /// Name of the topic registered with the reseller.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// List of service accounts which have subscriber access to the topic.
    #[prost(string, repeated, tag = "2")]
    pub service_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod cloud_channel_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// CloudChannelService lets Google cloud resellers and distributors manage
    /// their customers, channel partners, entitlements, and reports.
    ///
    /// Using this service:
    /// 1. Resellers and distributors can manage a customer entity.
    /// 2. Distributors can register an authorized reseller in their channel and
    ///    provide them with delegated admin access.
    /// 3. Resellers and distributors can manage customer entitlements.
    ///
    /// CloudChannelService exposes the following resources:
    /// - [Customer][google.cloud.channel.v1.Customer]s: An entity—usually an enterprise—managed by a reseller or
    /// distributor.
    ///
    /// - [Entitlement][google.cloud.channel.v1.Entitlement]s: An entity that provides a customer with the means to use
    /// a service. Entitlements are created or updated as a result of a successful
    /// fulfillment.
    ///
    /// - [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink]s: An entity that identifies links between
    /// distributors and their indirect resellers in a channel.
    #[derive(Debug, Clone)]
    pub struct CloudChannelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudChannelServiceClient<T>
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
        ) -> CloudChannelServiceClient<InterceptedService<T, F>>
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
            CloudChannelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// List [Customer][google.cloud.channel.v1.Customer]s.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        ///
        /// Return value:
        /// List of [Customer][google.cloud.channel.v1.Customer]s, or an empty list if there are no customers.
        pub async fn list_customers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomersResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListCustomers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListCustomers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested [Customer][google.cloud.channel.v1.Customer] resource.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: The customer resource doesn't exist. Usually the result of an
        /// invalid name parameter.
        ///
        /// Return value:
        /// The [Customer][google.cloud.channel.v1.Customer] resource.
        pub async fn get_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerRequest>,
        ) -> std::result::Result<tonic::Response<super::Customer>, tonic::Status> {
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
                "/google.cloud.channel.v1.CloudChannelService/GetCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "GetCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Confirms the existence of Cloud Identity accounts based on the domain and
        /// if the Cloud Identity accounts are owned by the reseller.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * INVALID_VALUE: Invalid domain value in the request.
        ///
        /// Return value:
        /// A list of [CloudIdentityCustomerAccount][google.cloud.channel.v1.CloudIdentityCustomerAccount] resources for the domain (may be
        /// empty)
        ///
        /// Note: in the v1alpha1 version of the API, a NOT_FOUND error returns if
        /// no [CloudIdentityCustomerAccount][google.cloud.channel.v1.CloudIdentityCustomerAccount] resources match the domain.
        pub async fn check_cloud_identity_accounts_exist(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CheckCloudIdentityAccountsExistRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CheckCloudIdentityAccountsExistResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/CheckCloudIdentityAccountsExist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "CheckCloudIdentityAccountsExist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new [Customer][google.cloud.channel.v1.Customer] resource under the reseller or distributor
        /// account.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT:
        ///     * Required request parameters are missing or invalid.
        ///     * Domain field value doesn't match the primary email domain.
        ///
        /// Return value:
        /// The newly created [Customer][google.cloud.channel.v1.Customer] resource.
        pub async fn create_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomerRequest>,
        ) -> std::result::Result<tonic::Response<super::Customer>, tonic::Status> {
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
                "/google.cloud.channel.v1.CloudChannelService/CreateCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "CreateCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing [Customer][google.cloud.channel.v1.Customer] resource for the reseller or
        /// distributor.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: No [Customer][google.cloud.channel.v1.Customer] resource found for the name in the request.
        ///
        /// Return value:
        /// The updated [Customer][google.cloud.channel.v1.Customer] resource.
        pub async fn update_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomerRequest>,
        ) -> std::result::Result<tonic::Response<super::Customer>, tonic::Status> {
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
                "/google.cloud.channel.v1.CloudChannelService/UpdateCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "UpdateCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the given [Customer][google.cloud.channel.v1.Customer] permanently.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The account making the request does not own
        /// this customer.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * FAILED_PRECONDITION: The customer has existing entitlements.
        /// * NOT_FOUND: No [Customer][google.cloud.channel.v1.Customer] resource found for the name in the request.
        pub async fn delete_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCustomerRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/DeleteCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "DeleteCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports a [Customer][google.cloud.channel.v1.Customer] from the Cloud Identity associated with the provided
        /// Cloud Identity ID or domain before a TransferEntitlements call. If a
        /// linked Customer already exists and overwrite_if_exists is true, it will
        /// update that Customer's data.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * NOT_FOUND: Cloud Identity doesn't exist or was deleted.
        /// * INVALID_ARGUMENT: Required parameters are missing, or the auth_token is
        /// expired or invalid.
        /// * ALREADY_EXISTS: A customer already exists and has conflicting critical
        /// fields. Requires an overwrite.
        ///
        /// Return value:
        /// The [Customer][google.cloud.channel.v1.Customer].
        pub async fn import_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportCustomerRequest>,
        ) -> std::result::Result<tonic::Response<super::Customer>, tonic::Status> {
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
                "/google.cloud.channel.v1.CloudChannelService/ImportCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ImportCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Cloud Identity for the given customer using the customer's
        /// information, or the information provided here.
        ///
        /// Possible error codes:
        ///
        /// *  PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// *  INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// *  NOT_FOUND: The customer was not found.
        /// *  ALREADY_EXISTS: The customer's primary email already exists. Retry
        ///    after changing the customer's primary contact email.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata contains an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn provision_cloud_identity(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvisionCloudIdentityRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/ProvisionCloudIdentity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ProvisionCloudIdentity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists [Entitlement][google.cloud.channel.v1.Entitlement]s belonging to a customer.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        ///
        /// Return value:
        /// A list of the customer's [Entitlement][google.cloud.channel.v1.Entitlement]s.
        pub async fn list_entitlements(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntitlementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEntitlementsResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListEntitlements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListEntitlements",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List [TransferableSku][google.cloud.channel.v1.TransferableSku]s of a customer based on the Cloud Identity ID or
        /// Customer Name in the request.
        ///
        /// Use this method to list the entitlements information of an
        /// unowned customer. You should provide the customer's
        /// Cloud Identity ID or Customer Name.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED:
        ///     * The customer doesn't belong to the reseller and has no auth token.
        ///     * The supplied auth token is invalid.
        ///     * The reseller account making the request is different
        ///     from the reseller account in the query.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        ///
        /// Return value:
        /// A list of the customer's [TransferableSku][google.cloud.channel.v1.TransferableSku].
        pub async fn list_transferable_skus(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferableSkusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTransferableSkusResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListTransferableSkus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListTransferableSkus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List [TransferableOffer][google.cloud.channel.v1.TransferableOffer]s of a customer based on Cloud Identity ID or
        /// Customer Name in the request.
        ///
        /// Use this method when a reseller gets the entitlement information of an
        /// unowned customer. The reseller should provide the customer's
        /// Cloud Identity ID or Customer Name.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED:
        ///     * The customer doesn't belong to the reseller and has no auth token.
        ///     * The supplied auth token is invalid.
        ///     * The reseller account making the request is different
        ///     from the reseller account in the query.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        ///
        /// Return value:
        /// List of [TransferableOffer][google.cloud.channel.v1.TransferableOffer] for the given customer and SKU.
        pub async fn list_transferable_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferableOffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTransferableOffersResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListTransferableOffers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListTransferableOffers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested [Entitlement][google.cloud.channel.v1.Entitlement] resource.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: The customer entitlement was not found.
        ///
        /// Return value:
        /// The requested [Entitlement][google.cloud.channel.v1.Entitlement] resource.
        pub async fn get_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntitlementRequest>,
        ) -> std::result::Result<tonic::Response<super::Entitlement>, tonic::Status> {
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
                "/google.cloud.channel.v1.CloudChannelService/GetEntitlement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "GetEntitlement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an entitlement for a customer.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT:
        ///     * Required request parameters are missing or invalid.
        ///     * There is already a customer entitlement for a SKU from the same
        ///     product family.
        /// * INVALID_VALUE: Make sure the OfferId is valid. If it is, contact
        /// Google Channel support for further troubleshooting.
        /// * NOT_FOUND: The customer or offer resource was not found.
        /// * ALREADY_EXISTS:
        ///     * The SKU was already purchased for the customer.
        ///     * The customer's primary email already exists. Retry
        ///     after changing the customer's primary contact email.
        /// * CONDITION_NOT_MET or FAILED_PRECONDITION:
        ///     * The domain required for purchasing a SKU has not been verified.
        ///     * A pre-requisite SKU required to purchase an Add-On SKU is missing.
        ///     For example, Google Workspace Business Starter is required to purchase
        ///     Vault or Drive.
        ///     * (Developer accounts only) Reseller and resold domain must meet the
        ///     following naming requirements:
        ///         * Domain names must start with goog-test.
        ///         * Domain names must include the reseller domain.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn create_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntitlementRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/CreateEntitlement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "CreateEntitlement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Change parameters of the entitlement.
        ///
        /// An entitlement update is a long-running operation and it updates the
        /// entitlement as a result of fulfillment.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// For example, the number of seats being changed is greater than the allowed
        /// number of max seats, or decreasing seats for a commitment based plan.
        /// * NOT_FOUND: Entitlement resource not found.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn change_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeParametersRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/ChangeParameters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ChangeParameters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the renewal settings for an existing customer entitlement.
        ///
        /// An entitlement update is a long-running operation and it updates the
        /// entitlement as a result of fulfillment.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: Entitlement resource not found.
        /// * NOT_COMMITMENT_PLAN: Renewal Settings are only applicable for a
        /// commitment plan. Can't enable or disable renewals for non-commitment plans.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        ///   Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn change_renewal_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeRenewalSettingsRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/ChangeRenewalSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ChangeRenewalSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the Offer for an existing customer entitlement.
        ///
        /// An entitlement update is a long-running operation and it updates the
        /// entitlement as a result of fulfillment.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: Offer or Entitlement resource not found.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn change_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeOfferRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/ChangeOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ChangeOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Starts paid service for a trial entitlement.
        ///
        /// Starts paid service for a trial entitlement immediately. This method is
        /// only applicable if a plan is set up for a trial entitlement but has some
        /// trial days remaining.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: Entitlement resource not found.
        /// * FAILED_PRECONDITION/NOT_IN_TRIAL: This method only works for
        /// entitlement on trial plans.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn start_paid_service(
            &mut self,
            request: impl tonic::IntoRequest<super::StartPaidServiceRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/StartPaidService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "StartPaidService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Suspends a previously fulfilled entitlement.
        ///
        /// An entitlement suspension is a long-running operation.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: Entitlement resource not found.
        /// * NOT_ACTIVE: Entitlement is not active.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn suspend_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::SuspendEntitlementRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/SuspendEntitlement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "SuspendEntitlement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels a previously fulfilled entitlement.
        ///
        /// An entitlement cancellation is a long-running operation.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * FAILED_PRECONDITION: There are Google Cloud projects linked to the
        /// Google Cloud entitlement's Cloud Billing subaccount.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: Entitlement resource not found.
        /// * DELETION_TYPE_NOT_ALLOWED: Cancel is only allowed for Google Workspace
        /// add-ons, or entitlements for Google Cloud's development platform.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The response will contain
        /// google.protobuf.Empty on success. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn cancel_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelEntitlementRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/CancelEntitlement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "CancelEntitlement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Activates a previously suspended entitlement. Entitlements suspended for
        /// pending ToS acceptance can't be activated using this method.
        ///
        /// An entitlement activation is a long-running operation and it updates
        /// the state of the customer entitlement.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: Entitlement resource not found.
        /// * SUSPENSION_NOT_RESELLER_INITIATED: Can only activate reseller-initiated
        /// suspensions and entitlements that have accepted the TOS.
        /// * NOT_SUSPENDED: Can only activate suspended entitlements not in an ACTIVE
        /// state.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn activate_entitlement(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateEntitlementRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/ActivateEntitlement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ActivateEntitlement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Transfers customer entitlements to new reseller.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: The customer or offer resource was not found.
        /// * ALREADY_EXISTS: The SKU was already transferred for the customer.
        /// * CONDITION_NOT_MET or FAILED_PRECONDITION:
        ///     * The SKU requires domain verification to transfer, but the domain is
        ///     not verified.
        ///     * An Add-On SKU (example, Vault or Drive) is missing the
        ///     pre-requisite SKU (example, G Suite Basic).
        ///     * (Developer accounts only) Reseller and resold domain must meet the
        ///     following naming requirements:
        ///         * Domain names must start with goog-test.
        ///         * Domain names must include the reseller domain.
        ///     * Specify all transferring entitlements.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn transfer_entitlements(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferEntitlementsRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/TransferEntitlements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "TransferEntitlements",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Transfers customer entitlements from their current reseller to Google.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: The customer or offer resource was not found.
        /// * ALREADY_EXISTS: The SKU was already transferred for the customer.
        /// * CONDITION_NOT_MET or FAILED_PRECONDITION:
        ///     * The SKU requires domain verification to transfer, but the domain is
        ///     not verified.
        ///     * An Add-On SKU (example, Vault or Drive) is missing the
        ///     pre-requisite SKU (example, G Suite Basic).
        ///     * (Developer accounts only) Reseller and resold domain must meet the
        ///     following naming requirements:
        ///         * Domain names must start with goog-test.
        ///         * Domain names must include the reseller domain.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The ID of a long-running operation.
        ///
        /// To get the results of the operation, call the GetOperation method of
        /// CloudChannelOperationsService. The response will contain
        /// google.protobuf.Empty on success. The Operation metadata will contain an
        /// instance of [OperationMetadata][google.cloud.channel.v1.OperationMetadata].
        pub async fn transfer_entitlements_to_google(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferEntitlementsToGoogleRequest>,
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
                "/google.cloud.channel.v1.CloudChannelService/TransferEntitlementsToGoogle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "TransferEntitlementsToGoogle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink]s belonging to a distributor.
        /// You must be a distributor to call this method.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        ///
        /// Return value:
        /// The list of the distributor account's [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resources.
        pub async fn list_channel_partner_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelPartnerLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListChannelPartnerLinksResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListChannelPartnerLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListChannelPartnerLinks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resource.
        /// You must be a distributor to call this method.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: ChannelPartnerLink resource not found because of an
        /// invalid channel partner link name.
        ///
        /// Return value:
        /// The [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resource.
        pub async fn get_channel_partner_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChannelPartnerLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChannelPartnerLink>,
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
                "/google.cloud.channel.v1.CloudChannelService/GetChannelPartnerLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "GetChannelPartnerLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Initiates a channel partner link between a distributor and a reseller, or
        /// between resellers in an n-tier reseller channel.
        /// Invited partners need to follow the invite_link_uri provided in the
        /// response to accept. After accepting the invitation, a link is set up
        /// between the two parties.
        /// You must be a distributor to call this method.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * ALREADY_EXISTS: The ChannelPartnerLink sent in the request already
        /// exists.
        /// * NOT_FOUND: No Cloud Identity customer exists for provided domain.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The new [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resource.
        pub async fn create_channel_partner_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChannelPartnerLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChannelPartnerLink>,
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
                "/google.cloud.channel.v1.CloudChannelService/CreateChannelPartnerLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "CreateChannelPartnerLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a channel partner link. Distributors call this method to change a
        /// link's status. For example, to suspend a partner link.
        /// You must be a distributor to call this method.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request is different
        /// from the reseller account in the API request.
        /// * INVALID_ARGUMENT:
        ///     * Required request parameters are missing or invalid.
        ///     * Link state cannot change from invited to active or suspended.
        ///     * Cannot send reseller_cloud_identity_id, invite_url, or name in update
        ///     mask.
        /// * NOT_FOUND: ChannelPartnerLink resource not found.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The updated [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] resource.
        pub async fn update_channel_partner_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateChannelPartnerLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChannelPartnerLink>,
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
                "/google.cloud.channel.v1.CloudChannelService/UpdateChannelPartnerLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "UpdateChannelPartnerLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested [Offer][google.cloud.channel.v1.Offer] resource.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The entitlement doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: Entitlement or offer was not found.
        ///
        /// Return value:
        /// The [Offer][google.cloud.channel.v1.Offer] resource.
        pub async fn lookup_offer(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupOfferRequest>,
        ) -> std::result::Result<tonic::Response<super::Offer>, tonic::Status> {
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
                "/google.cloud.channel.v1.CloudChannelService/LookupOffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "LookupOffer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the Products the reseller is authorized to sell.
        ///
        /// Possible error codes:
        ///
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        pub async fn list_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProductsResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListProducts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListProducts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the SKUs for a product the reseller is authorized to sell.
        ///
        /// Possible error codes:
        ///
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        pub async fn list_skus(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSkusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSkusResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListSkus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListSkus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the Offers the reseller can sell.
        ///
        /// Possible error codes:
        ///
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        pub async fn list_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOffersResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListOffers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListOffers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the following:
        ///
        /// * SKUs that you can purchase for a customer
        /// * SKUs that you can upgrade or downgrade for an entitlement.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        pub async fn list_purchasable_skus(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPurchasableSkusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPurchasableSkusResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListPurchasableSkus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListPurchasableSkus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the following:
        ///
        /// * Offers that you can purchase for a customer.
        /// * Offers that you can change for an entitlement.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The customer doesn't belong to the reseller
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        pub async fn list_purchasable_offers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPurchasableOffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPurchasableOffersResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListPurchasableOffers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListPurchasableOffers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Registers a service account with subscriber privileges on the Cloud Pub/Sub
        /// topic for this Channel Services account. After you create a
        /// subscriber, you get the events through [SubscriberEvent][google.cloud.channel.v1.SubscriberEvent]
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request and the
        /// provided reseller account are different, or the impersonated user
        /// is not a super admin.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The topic name with the registered service email address.
        pub async fn register_subscriber(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterSubscriberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterSubscriberResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/RegisterSubscriber",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "RegisterSubscriber",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Unregisters a service account with subscriber privileges on the Cloud
        /// Pub/Sub topic created for this Channel Services account. If there are no
        /// service accounts left with subscriber privileges, this deletes the topic.
        /// You can call ListSubscribers to check for these accounts.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request and the
        /// provided reseller account are different, or the impersonated user
        /// is not a super admin.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: The topic resource doesn't exist.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// The topic name that unregistered the service email address.
        /// Returns a success response if the service email address wasn't registered
        /// with the topic.
        pub async fn unregister_subscriber(
            &mut self,
            request: impl tonic::IntoRequest<super::UnregisterSubscriberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnregisterSubscriberResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/UnregisterSubscriber",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "UnregisterSubscriber",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists service accounts with subscriber privileges on the Cloud Pub/Sub
        /// topic created for this Channel Services account.
        ///
        /// Possible error codes:
        ///
        /// * PERMISSION_DENIED: The reseller account making the request and the
        /// provided reseller account are different, or the impersonated user
        /// is not a super admin.
        /// * INVALID_ARGUMENT: Required request parameters are missing or invalid.
        /// * NOT_FOUND: The topic resource doesn't exist.
        /// * INTERNAL: Any non-user error related to a technical issue in the
        /// backend. Contact Cloud Channel support.
        /// * UNKNOWN: Any non-user error related to a technical issue in the backend.
        /// Contact Cloud Channel support.
        ///
        /// Return value:
        /// A list of service email addresses.
        pub async fn list_subscribers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubscribersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSubscribersResponse>,
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
                "/google.cloud.channel.v1.CloudChannelService/ListSubscribers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.channel.v1.CloudChannelService",
                        "ListSubscribers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Represents Pub/Sub message content describing customer update.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerEvent {
    /// Resource name of the customer.
    /// Format: accounts/{account_id}/customers/{customer_id}
    #[prost(string, tag = "1")]
    pub customer: ::prost::alloc::string::String,
    /// Type of event which happened on the customer.
    #[prost(enumeration = "customer_event::Type", tag = "2")]
    pub event_type: i32,
}
/// Nested message and enum types in `CustomerEvent`.
pub mod customer_event {
    /// Type of customer event.
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
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// Primary domain for customer was changed.
        PrimaryDomainChanged = 1,
        /// Primary domain of the customer has been verified.
        PrimaryDomainVerified = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::PrimaryDomainChanged => "PRIMARY_DOMAIN_CHANGED",
                Type::PrimaryDomainVerified => "PRIMARY_DOMAIN_VERIFIED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIMARY_DOMAIN_CHANGED" => Some(Self::PrimaryDomainChanged),
                "PRIMARY_DOMAIN_VERIFIED" => Some(Self::PrimaryDomainVerified),
                _ => None,
            }
        }
    }
}
/// Represents Pub/Sub message content describing entitlement update.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntitlementEvent {
    /// Resource name of an entitlement of the form:
    /// accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    #[prost(string, tag = "1")]
    pub entitlement: ::prost::alloc::string::String,
    /// Type of event which happened on the entitlement.
    #[prost(enumeration = "entitlement_event::Type", tag = "2")]
    pub event_type: i32,
}
/// Nested message and enum types in `EntitlementEvent`.
pub mod entitlement_event {
    /// Type of entitlement event.
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
        /// Default value. This state doesn't show unless an error occurs.
        Unspecified = 0,
        /// A new entitlement was created.
        Created = 1,
        /// The offer type associated with an entitlement was changed.
        /// This is not triggered if an entitlement converts from a commit offer to a
        /// flexible offer as part of a renewal.
        PricePlanSwitched = 3,
        /// Annual commitment for a commit plan was changed.
        CommitmentChanged = 4,
        /// An annual entitlement was renewed.
        Renewed = 5,
        /// Entitlement was suspended.
        Suspended = 6,
        /// Entitlement was unsuspended.
        Activated = 7,
        /// Entitlement was cancelled.
        Cancelled = 8,
        /// Entitlement was upgraded or downgraded (e.g. from Google Workspace
        /// Business Standard to Google Workspace Business Plus).
        SkuChanged = 9,
        /// The renewal settings of an entitlement has changed.
        RenewalSettingChanged = 10,
        /// Paid service has started on trial entitlement.
        PaidServiceStarted = 11,
        /// License was assigned to or revoked from a user.
        LicenseAssignmentChanged = 12,
        /// License cap was changed for the entitlement.
        LicenseCapChanged = 13,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Created => "CREATED",
                Type::PricePlanSwitched => "PRICE_PLAN_SWITCHED",
                Type::CommitmentChanged => "COMMITMENT_CHANGED",
                Type::Renewed => "RENEWED",
                Type::Suspended => "SUSPENDED",
                Type::Activated => "ACTIVATED",
                Type::Cancelled => "CANCELLED",
                Type::SkuChanged => "SKU_CHANGED",
                Type::RenewalSettingChanged => "RENEWAL_SETTING_CHANGED",
                Type::PaidServiceStarted => "PAID_SERVICE_STARTED",
                Type::LicenseAssignmentChanged => "LICENSE_ASSIGNMENT_CHANGED",
                Type::LicenseCapChanged => "LICENSE_CAP_CHANGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATED" => Some(Self::Created),
                "PRICE_PLAN_SWITCHED" => Some(Self::PricePlanSwitched),
                "COMMITMENT_CHANGED" => Some(Self::CommitmentChanged),
                "RENEWED" => Some(Self::Renewed),
                "SUSPENDED" => Some(Self::Suspended),
                "ACTIVATED" => Some(Self::Activated),
                "CANCELLED" => Some(Self::Cancelled),
                "SKU_CHANGED" => Some(Self::SkuChanged),
                "RENEWAL_SETTING_CHANGED" => Some(Self::RenewalSettingChanged),
                "PAID_SERVICE_STARTED" => Some(Self::PaidServiceStarted),
                "LICENSE_ASSIGNMENT_CHANGED" => Some(Self::LicenseAssignmentChanged),
                "LICENSE_CAP_CHANGED" => Some(Self::LicenseCapChanged),
                _ => None,
            }
        }
    }
}
/// Represents information which resellers will get as part of notification from
/// Cloud Pub/Sub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriberEvent {
    /// Specifies the Pub/Sub event provided to the partners.
    /// This is a required field.
    #[prost(oneof = "subscriber_event::Event", tags = "1, 2")]
    pub event: ::core::option::Option<subscriber_event::Event>,
}
/// Nested message and enum types in `SubscriberEvent`.
pub mod subscriber_event {
    /// Specifies the Pub/Sub event provided to the partners.
    /// This is a required field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// Customer event send as part of Pub/Sub event to partners.
        #[prost(message, tag = "1")]
        CustomerEvent(super::CustomerEvent),
        /// Entitlement event send as part of Pub/Sub event to partners.
        #[prost(message, tag = "2")]
        EntitlementEvent(super::EntitlementEvent),
    }
}
