/// The device resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// The user-defined device identifier. The device ID must be unique
    /// within a device registry.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// The resource path name. For example,
    /// `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or
    /// `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`.
    /// When `name` is populated as a response from the service, it always ends
    /// in the device numeric ID.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// [Output only] A server-defined unique numeric ID for the device. This is a
    /// more compact way to identify devices, and it is globally unique.
    #[prost(uint64, tag = "3")]
    pub num_id: u64,
    /// The credentials used to authenticate this device. To allow credential
    /// rotation without interruption, multiple device credentials can be bound to
    /// this device. No more than 3 credentials can be bound to a single device at
    /// a time. When new credentials are added to a device, they are verified
    /// against the registry credentials. For details, see the description of the
    /// `DeviceRegistry.credentials` field.
    #[prost(message, repeated, tag = "12")]
    pub credentials: ::std::vec::Vec<DeviceCredential>,
    /// [Output only] The last time an MQTT `PINGREQ` was received. This field
    /// applies only to devices connecting through MQTT. MQTT clients usually only
    /// send `PINGREQ` messages if the connection is idle, and no other messages
    /// have been sent. Timestamps are periodically collected and written to
    /// storage; they may be stale by a few minutes.
    #[prost(message, optional, tag = "7")]
    pub last_heartbeat_time: ::std::option::Option<::prost_types::Timestamp>,
    /// [Output only] The last time a telemetry event was received. Timestamps are
    /// periodically collected and written to storage; they may be stale by a few
    /// minutes.
    #[prost(message, optional, tag = "8")]
    pub last_event_time: ::std::option::Option<::prost_types::Timestamp>,
    /// [Output only] The last time a state event was received. Timestamps are
    /// periodically collected and written to storage; they may be stale by a few
    /// minutes.
    #[prost(message, optional, tag = "20")]
    pub last_state_time: ::std::option::Option<::prost_types::Timestamp>,
    /// [Output only] The last time a cloud-to-device config version acknowledgment
    /// was received from the device. This field is only for configurations
    /// sent through MQTT.
    #[prost(message, optional, tag = "14")]
    pub last_config_ack_time: ::std::option::Option<::prost_types::Timestamp>,
    /// [Output only] The last time a cloud-to-device config version was sent to
    /// the device.
    #[prost(message, optional, tag = "18")]
    pub last_config_send_time: ::std::option::Option<::prost_types::Timestamp>,
    /// If a device is blocked, connections or requests from this device will fail.
    /// Can be used to temporarily prevent the device from connecting if, for
    /// example, the sensor is generating bad data and needs maintenance.
    #[prost(bool, tag = "19")]
    pub blocked: bool,
    /// [Output only] The time the most recent error occurred, such as a failure to
    /// publish to Cloud Pub/Sub. This field is the timestamp of
    /// 'last_error_status'.
    #[prost(message, optional, tag = "10")]
    pub last_error_time: ::std::option::Option<::prost_types::Timestamp>,
    /// [Output only] The error message of the most recent error, such as a failure
    /// to publish to Cloud Pub/Sub. 'last_error_time' is the timestamp of this
    /// field. If no errors have occurred, this field has an empty message
    /// and the status code 0 == OK. Otherwise, this field is expected to have a
    /// status code other than OK.
    #[prost(message, optional, tag = "11")]
    pub last_error_status: ::std::option::Option<super::super::super::rpc::Status>,
    /// The most recent device configuration, which is eventually sent from
    /// Cloud IoT Core to the device. If not present on creation, the
    /// configuration will be initialized with an empty payload and version value
    /// of `1`. To update this field after creation, use the
    /// `DeviceManager.ModifyCloudToDeviceConfig` method.
    #[prost(message, optional, tag = "13")]
    pub config: ::std::option::Option<DeviceConfig>,
    /// [Output only] The state most recently received from the device. If no state
    /// has been reported, this field is not present.
    #[prost(message, optional, tag = "16")]
    pub state: ::std::option::Option<DeviceState>,
    /// **Beta Feature**
    ///
    /// The logging verbosity for device activity. If unspecified,
    /// DeviceRegistry.log_level will be used.
    #[prost(enumeration = "LogLevel", tag = "21")]
    pub log_level: i32,
    /// The metadata key-value pairs assigned to the device. This metadata is not
    /// interpreted or indexed by Cloud IoT Core. It can be used to add contextual
    /// information for the device.
    ///
    /// Keys must conform to the regular expression [a-zA-Z][a-zA-Z0-9-_.+~%]+ and
    /// be less than 128 bytes in length.
    ///
    /// Values are free-form strings. Each value must be less than or equal to 32
    /// KB in size.
    ///
    /// The total size of all keys and values must be less than 256 KB, and the
    /// maximum number of key-value pairs is 500.
    #[prost(map = "string, string", tag = "17")]
    pub metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Gateway-related configuration and state.
    #[prost(message, optional, tag = "24")]
    pub gateway_config: ::std::option::Option<GatewayConfig>,
}
/// Gateway-related configuration and state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayConfig {
    /// Indicates whether the device is a gateway.
    #[prost(enumeration = "GatewayType", tag = "1")]
    pub gateway_type: i32,
    /// Indicates how to authorize and/or authenticate devices to access the
    /// gateway.
    #[prost(enumeration = "GatewayAuthMethod", tag = "2")]
    pub gateway_auth_method: i32,
    /// [Output only] The ID of the gateway the device accessed most recently.
    #[prost(string, tag = "3")]
    pub last_accessed_gateway_id: std::string::String,
    /// [Output only] The most recent time at which the device accessed the gateway
    /// specified in `last_accessed_gateway`.
    #[prost(message, optional, tag = "4")]
    pub last_accessed_gateway_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// A container for a group of devices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceRegistry {
    /// The identifier of this device registry. For example, `myRegistry`.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// The resource path name. For example,
    /// `projects/example-project/locations/us-central1/registries/my-registry`.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// The configuration for notification of telemetry events received from the
    /// device. All telemetry events that were successfully published by the
    /// device and acknowledged by Cloud IoT Core are guaranteed to be
    /// delivered to Cloud Pub/Sub. If multiple configurations match a message,
    /// only the first matching configuration is used. If you try to publish a
    /// device telemetry event using MQTT without specifying a Cloud Pub/Sub topic
    /// for the device's registry, the connection closes automatically. If you try
    /// to do so using an HTTP connection, an error is returned. Up to 10
    /// configurations may be provided.
    #[prost(message, repeated, tag = "10")]
    pub event_notification_configs: ::std::vec::Vec<EventNotificationConfig>,
    /// The configuration for notification of new states received from the device.
    /// State updates are guaranteed to be stored in the state history, but
    /// notifications to Cloud Pub/Sub are not guaranteed. For example, if
    /// permissions are misconfigured or the specified topic doesn't exist, no
    /// notification will be published but the state will still be stored in Cloud
    /// IoT Core.
    #[prost(message, optional, tag = "7")]
    pub state_notification_config: ::std::option::Option<StateNotificationConfig>,
    /// The MQTT configuration for this device registry.
    #[prost(message, optional, tag = "4")]
    pub mqtt_config: ::std::option::Option<MqttConfig>,
    /// The DeviceService (HTTP) configuration for this device registry.
    #[prost(message, optional, tag = "9")]
    pub http_config: ::std::option::Option<HttpConfig>,
    /// **Beta Feature**
    ///
    /// The default logging verbosity for activity from devices in this registry.
    /// The verbosity level can be overridden by Device.log_level.
    #[prost(enumeration = "LogLevel", tag = "11")]
    pub log_level: i32,
    /// The credentials used to verify the device credentials. No more than 10
    /// credentials can be bound to a single registry at a time. The verification
    /// process occurs at the time of device creation or update. If this field is
    /// empty, no verification is performed. Otherwise, the credentials of a newly
    /// created device or added credentials of an updated device should be signed
    /// with one of these registry credentials.
    ///
    /// Note, however, that existing devices will never be affected by
    /// modifications to this list of credentials: after a device has been
    /// successfully created in a registry, it should be able to connect even if
    /// its registry credentials are revoked, deleted, or modified.
    #[prost(message, repeated, tag = "8")]
    pub credentials: ::std::vec::Vec<RegistryCredential>,
}
/// The configuration of MQTT for a device registry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MqttConfig {
    /// If enabled, allows connections using the MQTT protocol. Otherwise, MQTT
    /// connections to this registry will fail.
    #[prost(enumeration = "MqttState", tag = "1")]
    pub mqtt_enabled_state: i32,
}
/// The configuration of the HTTP bridge for a device registry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpConfig {
    /// If enabled, allows devices to use DeviceService via the HTTP protocol.
    /// Otherwise, any requests to DeviceService will fail for this registry.
    #[prost(enumeration = "HttpState", tag = "1")]
    pub http_enabled_state: i32,
}
/// The configuration for forwarding telemetry events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNotificationConfig {
    /// If the subfolder name matches this string exactly, this configuration will
    /// be used. The string must not include the leading '/' character. If empty,
    /// all strings are matched. This field is used only for telemetry events;
    /// subfolders are not supported for state changes.
    #[prost(string, tag = "2")]
    pub subfolder_matches: std::string::String,
    /// A Cloud Pub/Sub topic name. For example,
    /// `projects/myProject/topics/deviceEvents`.
    #[prost(string, tag = "1")]
    pub pubsub_topic_name: std::string::String,
}
/// The configuration for notification of new states received from the device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateNotificationConfig {
    /// A Cloud Pub/Sub topic name. For example,
    /// `projects/myProject/topics/deviceEvents`.
    #[prost(string, tag = "1")]
    pub pubsub_topic_name: std::string::String,
}
/// A server-stored registry credential used to validate device credentials.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryCredential {
    /// The credential data. Reserved for expansion in the future.
    #[prost(oneof = "registry_credential::Credential", tags = "1")]
    pub credential: ::std::option::Option<registry_credential::Credential>,
}
pub mod registry_credential {
    /// The credential data. Reserved for expansion in the future.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        /// A public key certificate used to verify the device credentials.
        #[prost(message, tag = "1")]
        PublicKeyCertificate(super::PublicKeyCertificate),
    }
}
/// Details of an X.509 certificate. For informational purposes only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509CertificateDetails {
    /// The entity that signed the certificate.
    #[prost(string, tag = "1")]
    pub issuer: std::string::String,
    /// The entity the certificate and public key belong to.
    #[prost(string, tag = "2")]
    pub subject: std::string::String,
    /// The time the certificate becomes valid.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the certificate becomes invalid.
    #[prost(message, optional, tag = "4")]
    pub expiry_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The algorithm used to sign the certificate.
    #[prost(string, tag = "5")]
    pub signature_algorithm: std::string::String,
    /// The type of public key in the certificate.
    #[prost(string, tag = "6")]
    pub public_key_type: std::string::String,
}
/// A public key certificate format and data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyCertificate {
    /// The certificate format.
    #[prost(enumeration = "PublicKeyCertificateFormat", tag = "1")]
    pub format: i32,
    /// The certificate data.
    #[prost(string, tag = "2")]
    pub certificate: std::string::String,
    /// [Output only] The certificate details. Used only for X.509 certificates.
    #[prost(message, optional, tag = "3")]
    pub x509_details: ::std::option::Option<X509CertificateDetails>,
}
/// A server-stored device credential used for authentication.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceCredential {
    /// [Optional] The time at which this credential becomes invalid. This
    /// credential will be ignored for new client authentication requests after
    /// this timestamp; however, it will not be automatically deleted.
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The credential data. Reserved for expansion in the future.
    #[prost(oneof = "device_credential::Credential", tags = "2")]
    pub credential: ::std::option::Option<device_credential::Credential>,
}
pub mod device_credential {
    /// The credential data. Reserved for expansion in the future.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        /// A public key used to verify the signature of JSON Web Tokens (JWTs).
        /// When adding a new device credential, either via device creation or via
        /// modifications, this public key credential may be required to be signed by
        /// one of the registry level certificates. More specifically, if the
        /// registry contains at least one certificate, any new device credential
        /// must be signed by one of the registry certificates. As a result,
        /// when the registry contains certificates, only X.509 certificates are
        /// accepted as device credentials. However, if the registry does
        /// not contain a certificate, self-signed certificates and public keys will
        /// be accepted. New device credentials must be different from every
        /// registry-level certificate.
        #[prost(message, tag = "2")]
        PublicKey(super::PublicKeyCredential),
    }
}
/// A public key format and data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyCredential {
    /// The format of the key.
    #[prost(enumeration = "PublicKeyFormat", tag = "1")]
    pub format: i32,
    /// The key data.
    #[prost(string, tag = "2")]
    pub key: std::string::String,
}
/// The device configuration. Eventually delivered to devices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceConfig {
    /// [Output only] The version of this update. The version number is assigned by
    /// the server, and is always greater than 0 after device creation. The
    /// version must be 0 on the `CreateDevice` request if a `config` is
    /// specified; the response of `CreateDevice` will always have a value of 1.
    #[prost(int64, tag = "1")]
    pub version: i64,
    /// [Output only] The time at which this configuration version was updated in
    /// Cloud IoT Core. This timestamp is set by the server.
    #[prost(message, optional, tag = "2")]
    pub cloud_update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// [Output only] The time at which Cloud IoT Core received the
    /// acknowledgment from the device, indicating that the device has received
    /// this configuration version. If this field is not present, the device has
    /// not yet acknowledged that it received this version. Note that when
    /// the config was sent to the device, many config versions may have been
    /// available in Cloud IoT Core while the device was disconnected, and on
    /// connection, only the latest version is sent to the device. Some
    /// versions may never be sent to the device, and therefore are never
    /// acknowledged. This timestamp is set by Cloud IoT Core.
    #[prost(message, optional, tag = "3")]
    pub device_ack_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The device configuration data.
    #[prost(bytes, tag = "4")]
    pub binary_data: std::vec::Vec<u8>,
}
/// The device state, as reported by the device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceState {
    /// [Output only] The time at which this state version was updated in Cloud
    /// IoT Core.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The device state data.
    #[prost(bytes, tag = "2")]
    pub binary_data: std::vec::Vec<u8>,
}
/// Indicates whether an MQTT connection is enabled or disabled. See the field
/// description for details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MqttState {
    /// No MQTT state specified. If not specified, MQTT will be enabled by default.
    Unspecified = 0,
    /// Enables a MQTT connection.
    MqttEnabled = 1,
    /// Disables a MQTT connection.
    MqttDisabled = 2,
}
/// Indicates whether DeviceService (HTTP) is enabled or disabled for the
/// registry. See the field description for details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HttpState {
    /// No HTTP state specified. If not specified, DeviceService will be
    /// enabled by default.
    Unspecified = 0,
    /// Enables DeviceService (HTTP) service for the registry.
    HttpEnabled = 1,
    /// Disables DeviceService (HTTP) service for the registry.
    HttpDisabled = 2,
}
/// **Beta Feature**
///
/// The logging verbosity for device activity. Specifies which events should be
/// written to logs. For example, if the LogLevel is ERROR, only events that
/// terminate in errors will be logged. LogLevel is inclusive; enabling INFO
/// logging will also enable ERROR logging.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogLevel {
    /// No logging specified. If not specified, logging will be disabled.
    Unspecified = 0,
    /// Disables logging.
    None = 10,
    /// Error events will be logged.
    Error = 20,
    /// Informational events will be logged, such as connections and
    /// disconnections.
    Info = 30,
    /// All events will be logged.
    Debug = 40,
}
/// Gateway type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GatewayType {
    /// If unspecified, the device is considered a non-gateway device.
    Unspecified = 0,
    /// The device is a gateway.
    Gateway = 1,
    /// The device is not a gateway.
    NonGateway = 2,
}
/// The gateway authorization/authentication method. This setting determines how
/// Cloud IoT Core authorizes/authenticate devices to access the gateway.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GatewayAuthMethod {
    /// No authentication/authorization method specified. No devices are allowed to
    /// access the gateway.
    Unspecified = 0,
    /// The device is authenticated through the gateway association only. Device
    /// credentials are ignored even if provided.
    AssociationOnly = 1,
    /// The device is authenticated through its own credentials. Gateway
    /// association is not checked.
    DeviceAuthTokenOnly = 2,
    /// The device is authenticated through both device credentials and gateway
    /// association. The device must be bound to the gateway and must provide its
    /// own credentials.
    AssociationAndDeviceAuthToken = 3,
}
/// The supported formats for the public key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PublicKeyCertificateFormat {
    /// The format has not been specified. This is an invalid default value and
    /// must not be used.
    UnspecifiedPublicKeyCertificateFormat = 0,
    /// An X.509v3 certificate ([RFC5280](https://www.ietf.org/rfc/rfc5280.txt)),
    /// encoded in base64, and wrapped by `-----BEGIN CERTIFICATE-----` and
    /// `-----END CERTIFICATE-----`.
    X509CertificatePem = 1,
}
/// The supported formats for the public key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PublicKeyFormat {
    /// The format has not been specified. This is an invalid default value and
    /// must not be used.
    UnspecifiedPublicKeyFormat = 0,
    /// An RSA public key encoded in base64, and wrapped by
    /// `-----BEGIN PUBLIC KEY-----` and `-----END PUBLIC KEY-----`. This can be
    /// used to verify `RS256` signatures in JWT tokens ([RFC7518](
    /// https://www.ietf.org/rfc/rfc7518.txt)).
    RsaPem = 3,
    /// As RSA_PEM, but wrapped in an X.509v3 certificate ([RFC5280](
    /// https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped by
    /// `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`.
    RsaX509Pem = 1,
    /// Public key for the ECDSA algorithm using P-256 and SHA-256, encoded in
    /// base64, and wrapped by `-----BEGIN PUBLIC KEY-----` and `-----END
    /// PUBLIC KEY-----`. This can be used to verify JWT tokens with the `ES256`
    /// algorithm ([RFC7518](https://www.ietf.org/rfc/rfc7518.txt)). This curve is
    /// defined in [OpenSSL](https://www.openssl.org/) as the `prime256v1` curve.
    Es256Pem = 2,
    /// As ES256_PEM, but wrapped in an X.509v3 certificate ([RFC5280](
    /// https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped by
    /// `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`.
    Es256X509Pem = 4,
}
/// Request for `CreateDeviceRegistry`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeviceRegistryRequest {
    /// Required. The project and cloud region where this device registry must be created.
    /// For example, `projects/example-project/locations/us-central1`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The device registry. The field `name` must be empty. The server will
    /// generate that field from the device registry `id` provided and the
    /// `parent` field.
    #[prost(message, optional, tag = "2")]
    pub device_registry: ::std::option::Option<DeviceRegistry>,
}
/// Request for `GetDeviceRegistry`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeviceRegistryRequest {
    /// Required. The name of the device registry. For example,
    /// `projects/example-project/locations/us-central1/registries/my-registry`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for `DeleteDeviceRegistry`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeviceRegistryRequest {
    /// Required. The name of the device registry. For example,
    /// `projects/example-project/locations/us-central1/registries/my-registry`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for `UpdateDeviceRegistry`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeviceRegistryRequest {
    /// Required. The new values for the device registry. The `id` field must be empty, and
    /// the `name` field must indicate the path of the resource. For example,
    /// `projects/example-project/locations/us-central1/registries/my-registry`.
    #[prost(message, optional, tag = "1")]
    pub device_registry: ::std::option::Option<DeviceRegistry>,
    /// Required. Only updates the `device_registry` fields indicated by this mask.
    /// The field mask must not be empty, and it must not contain fields that
    /// are immutable or only set by the server.
    /// Mutable top-level fields: `event_notification_config`, `http_config`,
    /// `mqtt_config`, and `state_notification_config`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for `ListDeviceRegistries`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeviceRegistriesRequest {
    /// Required. The project and cloud region path. For example,
    /// `projects/example-project/locations/us-central1`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of registries to return in the response. If this value
    /// is zero, the service will select a default size. A call may return fewer
    /// objects than requested. A non-empty `next_page_token` in the response
    /// indicates that more data is available.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListDeviceRegistriesResponse`; indicates
    /// that this is a continuation of a prior `ListDeviceRegistries` call and
    /// the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for `ListDeviceRegistries`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeviceRegistriesResponse {
    /// The registries that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub device_registries: ::std::vec::Vec<DeviceRegistry>,
    /// If not empty, indicates that there may be more registries that match the
    /// request; this value should be passed in a new
    /// `ListDeviceRegistriesRequest`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for `CreateDevice`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeviceRequest {
    /// Required. The name of the device registry where this device should be created.
    /// For example,
    /// `projects/example-project/locations/us-central1/registries/my-registry`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The device registration details. The field `name` must be empty. The server
    /// generates `name` from the device registry `id` and the
    /// `parent` field.
    #[prost(message, optional, tag = "2")]
    pub device: ::std::option::Option<Device>,
}
/// Request for `GetDevice`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeviceRequest {
    /// Required. The name of the device. For example,
    /// `projects/p0/locations/us-central1/registries/registry0/devices/device0` or
    /// `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The fields of the `Device` resource to be returned in the response. If the
    /// field mask is unset or empty, all fields are returned.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for `UpdateDevice`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeviceRequest {
    /// Required. The new values for the device. The `id` and `num_id` fields must
    /// be empty, and the field `name` must specify the name path. For example,
    /// `projects/p0/locations/us-central1/registries/registry0/devices/device0`or
    /// `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    #[prost(message, optional, tag = "2")]
    pub device: ::std::option::Option<Device>,
    /// Required. Only updates the `device` fields indicated by this mask.
    /// The field mask must not be empty, and it must not contain fields that
    /// are immutable or only set by the server.
    /// Mutable top-level fields: `credentials`, `blocked`, and `metadata`
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for `DeleteDevice`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeviceRequest {
    /// Required. The name of the device. For example,
    /// `projects/p0/locations/us-central1/registries/registry0/devices/device0` or
    /// `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for `ListDevices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDevicesRequest {
    /// Required. The device registry path. Required. For example,
    /// `projects/my-project/locations/us-central1/registries/my-registry`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// A list of device numeric IDs. If empty, this field is ignored. Maximum
    /// IDs: 10,000.
    #[prost(uint64, repeated, tag = "2")]
    pub device_num_ids: ::std::vec::Vec<u64>,
    /// A list of device string IDs. For example, `['device0', 'device12']`.
    /// If empty, this field is ignored. Maximum IDs: 10,000
    #[prost(string, repeated, tag = "3")]
    pub device_ids: ::std::vec::Vec<std::string::String>,
    /// The fields of the `Device` resource to be returned in the response. The
    /// fields `id` and `num_id` are always returned, along with any
    /// other fields specified.
    #[prost(message, optional, tag = "4")]
    pub field_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Options related to gateways.
    #[prost(message, optional, tag = "6")]
    pub gateway_list_options: ::std::option::Option<GatewayListOptions>,
    /// The maximum number of devices to return in the response. If this value
    /// is zero, the service will select a default size. A call may return fewer
    /// objects than requested. A non-empty `next_page_token` in the response
    /// indicates that more data is available.
    #[prost(int32, tag = "100")]
    pub page_size: i32,
    /// The value returned by the last `ListDevicesResponse`; indicates
    /// that this is a continuation of a prior `ListDevices` call and
    /// the system should return the next page of data.
    #[prost(string, tag = "101")]
    pub page_token: std::string::String,
}
/// Options for limiting the list based on gateway type and associations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayListOptions {
    /// If not set, all devices and gateways are returned. If set, the list is
    /// filtered based on gateway type and associations.
    #[prost(oneof = "gateway_list_options::Filter", tags = "1, 2, 3")]
    pub filter: ::std::option::Option<gateway_list_options::Filter>,
}
pub mod gateway_list_options {
    /// If not set, all devices and gateways are returned. If set, the list is
    /// filtered based on gateway type and associations.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// If `GATEWAY` is specified, only gateways are returned. If `NON_GATEWAY`
        /// is specified, only non-gateway devices are returned. If
        /// `GATEWAY_TYPE_UNSPECIFIED` is specified, all devices are returned.
        #[prost(enumeration = "super::GatewayType", tag = "1")]
        GatewayType(i32),
        /// If set, only devices associated with the specified gateway are returned.
        /// The gateway ID can be numeric (`num_id`) or the user-defined string
        /// (`id`). For example, if `123` is specified, only devices bound to the
        /// gateway with `num_id` 123 are returned.
        #[prost(string, tag = "2")]
        AssociationsGatewayId(std::string::String),
        /// If set, returns only the gateways with which the specified device is
        /// associated. The device ID can be numeric (`num_id`) or the user-defined
        /// string (`id`). For example, if `456` is specified, returns only the
        /// gateways to which the device with `num_id` 456 is bound.
        #[prost(string, tag = "3")]
        AssociationsDeviceId(std::string::String),
    }
}
/// Response for `ListDevices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDevicesResponse {
    /// The devices that match the request.
    #[prost(message, repeated, tag = "1")]
    pub devices: ::std::vec::Vec<Device>,
    /// If not empty, indicates that there may be more devices that match the
    /// request; this value should be passed in a new `ListDevicesRequest`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for `ModifyCloudToDeviceConfig`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyCloudToDeviceConfigRequest {
    /// Required. The name of the device. For example,
    /// `projects/p0/locations/us-central1/registries/registry0/devices/device0` or
    /// `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The version number to update. If this value is zero, it will not check the
    /// version number of the server and will always update the current version;
    /// otherwise, this update will fail if the version number found on the server
    /// does not match this version number. This is used to support multiple
    /// simultaneous updates without losing data.
    #[prost(int64, tag = "2")]
    pub version_to_update: i64,
    /// Required. The configuration data for the device.
    #[prost(bytes, tag = "3")]
    pub binary_data: std::vec::Vec<u8>,
}
/// Request for `ListDeviceConfigVersions`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeviceConfigVersionsRequest {
    /// Required. The name of the device. For example,
    /// `projects/p0/locations/us-central1/registries/registry0/devices/device0` or
    /// `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The number of versions to list. Versions are listed in decreasing order of
    /// the version number. The maximum number of versions retained is 10. If this
    /// value is zero, it will return all the versions available.
    #[prost(int32, tag = "2")]
    pub num_versions: i32,
}
/// Response for `ListDeviceConfigVersions`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeviceConfigVersionsResponse {
    /// The device configuration for the last few versions. Versions are listed
    /// in decreasing order, starting from the most recent one.
    #[prost(message, repeated, tag = "1")]
    pub device_configs: ::std::vec::Vec<DeviceConfig>,
}
/// Request for `ListDeviceStates`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeviceStatesRequest {
    /// Required. The name of the device. For example,
    /// `projects/p0/locations/us-central1/registries/registry0/devices/device0` or
    /// `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The number of states to list. States are listed in descending order of
    /// update time. The maximum number of states retained is 10. If this
    /// value is zero, it will return all the states available.
    #[prost(int32, tag = "2")]
    pub num_states: i32,
}
/// Response for `ListDeviceStates`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeviceStatesResponse {
    /// The last few device states. States are listed in descending order of server
    /// update time, starting from the most recent one.
    #[prost(message, repeated, tag = "1")]
    pub device_states: ::std::vec::Vec<DeviceState>,
}
/// Request for `SendCommandToDevice`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCommandToDeviceRequest {
    /// Required. The name of the device. For example,
    /// `projects/p0/locations/us-central1/registries/registry0/devices/device0` or
    /// `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The command data to send to the device.
    #[prost(bytes, tag = "2")]
    pub binary_data: std::vec::Vec<u8>,
    /// Optional subfolder for the command. If empty, the command will be delivered
    /// to the /devices/{device-id}/commands topic, otherwise it will be delivered
    /// to the /devices/{device-id}/commands/{subfolder} topic. Multi-level
    /// subfolders are allowed. This field must not have more than 256 characters,
    /// and must not contain any MQTT wildcards ("+" or "#") or null characters.
    #[prost(string, tag = "3")]
    pub subfolder: std::string::String,
}
/// Response for `SendCommandToDevice`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCommandToDeviceResponse {}
/// Request for `BindDeviceToGateway`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindDeviceToGatewayRequest {
    /// Required. The name of the registry. For example,
    /// `projects/example-project/locations/us-central1/registries/my-registry`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The value of `gateway_id` can be either the device numeric ID or the
    /// user-defined device identifier.
    #[prost(string, tag = "2")]
    pub gateway_id: std::string::String,
    /// Required. The device to associate with the specified gateway. The value of
    /// `device_id` can be either the device numeric ID or the user-defined device
    /// identifier.
    #[prost(string, tag = "3")]
    pub device_id: std::string::String,
}
/// Response for `BindDeviceToGateway`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindDeviceToGatewayResponse {}
/// Request for `UnbindDeviceFromGateway`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbindDeviceFromGatewayRequest {
    /// Required. The name of the registry. For example,
    /// `projects/example-project/locations/us-central1/registries/my-registry`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The value of `gateway_id` can be either the device numeric ID or the
    /// user-defined device identifier.
    #[prost(string, tag = "2")]
    pub gateway_id: std::string::String,
    /// Required. The device to disassociate from the specified gateway. The value of
    /// `device_id` can be either the device numeric ID or the user-defined device
    /// identifier.
    #[prost(string, tag = "3")]
    pub device_id: std::string::String,
}
/// Response for `UnbindDeviceFromGateway`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbindDeviceFromGatewayResponse {}
#[doc = r" Generated client implementations."]
pub mod device_manager_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Internet of Things (IoT) service. Securely connect and manage IoT devices."]
    pub struct DeviceManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DeviceManagerClient<T>
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
        #[doc = " Creates a device registry that contains devices."]
        pub async fn create_device_registry(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeviceRegistryRequest>,
        ) -> Result<tonic::Response<super::DeviceRegistry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/CreateDeviceRegistry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a device registry configuration."]
        pub async fn get_device_registry(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeviceRegistryRequest>,
        ) -> Result<tonic::Response<super::DeviceRegistry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/GetDeviceRegistry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a device registry configuration."]
        pub async fn update_device_registry(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeviceRegistryRequest>,
        ) -> Result<tonic::Response<super::DeviceRegistry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/UpdateDeviceRegistry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a device registry configuration."]
        pub async fn delete_device_registry(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDeviceRegistryRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/DeleteDeviceRegistry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists device registries."]
        pub async fn list_device_registries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeviceRegistriesRequest>,
        ) -> Result<tonic::Response<super::ListDeviceRegistriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/ListDeviceRegistries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a device in a device registry."]
        pub async fn create_device(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeviceRequest>,
        ) -> Result<tonic::Response<super::Device>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/CreateDevice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details about a device."]
        pub async fn get_device(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeviceRequest>,
        ) -> Result<tonic::Response<super::Device>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/GetDevice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a device."]
        pub async fn update_device(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeviceRequest>,
        ) -> Result<tonic::Response<super::Device>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/UpdateDevice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a device."]
        pub async fn delete_device(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDeviceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/DeleteDevice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List devices in a device registry."]
        pub async fn list_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDevicesRequest>,
        ) -> Result<tonic::Response<super::ListDevicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/ListDevices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Modifies the configuration for the device, which is eventually sent from"]
        #[doc = " the Cloud IoT Core servers. Returns the modified configuration version and"]
        #[doc = " its metadata."]
        pub async fn modify_cloud_to_device_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyCloudToDeviceConfigRequest>,
        ) -> Result<tonic::Response<super::DeviceConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/ModifyCloudToDeviceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the last few versions of the device configuration in descending"]
        #[doc = " order (i.e.: newest first)."]
        pub async fn list_device_config_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeviceConfigVersionsRequest>,
        ) -> Result<tonic::Response<super::ListDeviceConfigVersionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/ListDeviceConfigVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the last few versions of the device state in descending order (i.e.:"]
        #[doc = " newest first)."]
        pub async fn list_device_states(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeviceStatesRequest>,
        ) -> Result<tonic::Response<super::ListDeviceStatesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/ListDeviceStates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on the specified resource. Replaces any"]
        #[doc = " existing policy."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a resource."]
        #[doc = " Returns an empty policy if the resource exists and does not have a policy"]
        #[doc = " set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has on the specified resource."]
        #[doc = " If the resource does not exist, this will return an empty set of"]
        #[doc = " permissions, not a NOT_FOUND error."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.cloud.iot.v1.DeviceManager/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sends a command to the specified device. In order for a device to be able"]
        #[doc = " to receive commands, it must:"]
        #[doc = " 1) be connected to Cloud IoT Core using the MQTT protocol, and"]
        #[doc = " 2) be subscribed to the group of MQTT topics specified by"]
        #[doc = "    /devices/{device-id}/commands/#. This subscription will receive commands"]
        #[doc = "    at the top-level topic /devices/{device-id}/commands as well as commands"]
        #[doc = "    for subfolders, like /devices/{device-id}/commands/subfolder."]
        #[doc = "    Note that subscribing to specific subfolders is not supported."]
        #[doc = " If the command could not be delivered to the device, this method will"]
        #[doc = " return an error; in particular, if the device is not subscribed, this"]
        #[doc = " method will return FAILED_PRECONDITION. Otherwise, this method will"]
        #[doc = " return OK. If the subscription is QoS 1, at least once delivery will be"]
        #[doc = " guaranteed; for QoS 0, no acknowledgment will be expected from the device."]
        pub async fn send_command_to_device(
            &mut self,
            request: impl tonic::IntoRequest<super::SendCommandToDeviceRequest>,
        ) -> Result<tonic::Response<super::SendCommandToDeviceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/SendCommandToDevice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Associates the device with the gateway."]
        pub async fn bind_device_to_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::BindDeviceToGatewayRequest>,
        ) -> Result<tonic::Response<super::BindDeviceToGatewayResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/BindDeviceToGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the association between the device and the gateway."]
        pub async fn unbind_device_from_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::UnbindDeviceFromGatewayRequest>,
        ) -> Result<tonic::Response<super::UnbindDeviceFromGatewayResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iot.v1.DeviceManager/UnbindDeviceFromGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DeviceManagerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DeviceManagerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DeviceManagerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod device_manager_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DeviceManagerServer."]
    #[async_trait]
    pub trait DeviceManager: Send + Sync + 'static {
        #[doc = " Creates a device registry that contains devices."]
        async fn create_device_registry(
            &self,
            request: tonic::Request<super::CreateDeviceRegistryRequest>,
        ) -> Result<tonic::Response<super::DeviceRegistry>, tonic::Status>;
        #[doc = " Gets a device registry configuration."]
        async fn get_device_registry(
            &self,
            request: tonic::Request<super::GetDeviceRegistryRequest>,
        ) -> Result<tonic::Response<super::DeviceRegistry>, tonic::Status>;
        #[doc = " Updates a device registry configuration."]
        async fn update_device_registry(
            &self,
            request: tonic::Request<super::UpdateDeviceRegistryRequest>,
        ) -> Result<tonic::Response<super::DeviceRegistry>, tonic::Status>;
        #[doc = " Deletes a device registry configuration."]
        async fn delete_device_registry(
            &self,
            request: tonic::Request<super::DeleteDeviceRegistryRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists device registries."]
        async fn list_device_registries(
            &self,
            request: tonic::Request<super::ListDeviceRegistriesRequest>,
        ) -> Result<tonic::Response<super::ListDeviceRegistriesResponse>, tonic::Status>;
        #[doc = " Creates a device in a device registry."]
        async fn create_device(
            &self,
            request: tonic::Request<super::CreateDeviceRequest>,
        ) -> Result<tonic::Response<super::Device>, tonic::Status>;
        #[doc = " Gets details about a device."]
        async fn get_device(
            &self,
            request: tonic::Request<super::GetDeviceRequest>,
        ) -> Result<tonic::Response<super::Device>, tonic::Status>;
        #[doc = " Updates a device."]
        async fn update_device(
            &self,
            request: tonic::Request<super::UpdateDeviceRequest>,
        ) -> Result<tonic::Response<super::Device>, tonic::Status>;
        #[doc = " Deletes a device."]
        async fn delete_device(
            &self,
            request: tonic::Request<super::DeleteDeviceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " List devices in a device registry."]
        async fn list_devices(
            &self,
            request: tonic::Request<super::ListDevicesRequest>,
        ) -> Result<tonic::Response<super::ListDevicesResponse>, tonic::Status>;
        #[doc = " Modifies the configuration for the device, which is eventually sent from"]
        #[doc = " the Cloud IoT Core servers. Returns the modified configuration version and"]
        #[doc = " its metadata."]
        async fn modify_cloud_to_device_config(
            &self,
            request: tonic::Request<super::ModifyCloudToDeviceConfigRequest>,
        ) -> Result<tonic::Response<super::DeviceConfig>, tonic::Status>;
        #[doc = " Lists the last few versions of the device configuration in descending"]
        #[doc = " order (i.e.: newest first)."]
        async fn list_device_config_versions(
            &self,
            request: tonic::Request<super::ListDeviceConfigVersionsRequest>,
        ) -> Result<tonic::Response<super::ListDeviceConfigVersionsResponse>, tonic::Status>;
        #[doc = " Lists the last few versions of the device state in descending order (i.e.:"]
        #[doc = " newest first)."]
        async fn list_device_states(
            &self,
            request: tonic::Request<super::ListDeviceStatesRequest>,
        ) -> Result<tonic::Response<super::ListDeviceStatesResponse>, tonic::Status>;
        #[doc = " Sets the access control policy on the specified resource. Replaces any"]
        #[doc = " existing policy."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Gets the access control policy for a resource."]
        #[doc = " Returns an empty policy if the resource exists and does not have a policy"]
        #[doc = " set."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Returns permissions that a caller has on the specified resource."]
        #[doc = " If the resource does not exist, this will return an empty set of"]
        #[doc = " permissions, not a NOT_FOUND error."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
        #[doc = " Sends a command to the specified device. In order for a device to be able"]
        #[doc = " to receive commands, it must:"]
        #[doc = " 1) be connected to Cloud IoT Core using the MQTT protocol, and"]
        #[doc = " 2) be subscribed to the group of MQTT topics specified by"]
        #[doc = "    /devices/{device-id}/commands/#. This subscription will receive commands"]
        #[doc = "    at the top-level topic /devices/{device-id}/commands as well as commands"]
        #[doc = "    for subfolders, like /devices/{device-id}/commands/subfolder."]
        #[doc = "    Note that subscribing to specific subfolders is not supported."]
        #[doc = " If the command could not be delivered to the device, this method will"]
        #[doc = " return an error; in particular, if the device is not subscribed, this"]
        #[doc = " method will return FAILED_PRECONDITION. Otherwise, this method will"]
        #[doc = " return OK. If the subscription is QoS 1, at least once delivery will be"]
        #[doc = " guaranteed; for QoS 0, no acknowledgment will be expected from the device."]
        async fn send_command_to_device(
            &self,
            request: tonic::Request<super::SendCommandToDeviceRequest>,
        ) -> Result<tonic::Response<super::SendCommandToDeviceResponse>, tonic::Status>;
        #[doc = " Associates the device with the gateway."]
        async fn bind_device_to_gateway(
            &self,
            request: tonic::Request<super::BindDeviceToGatewayRequest>,
        ) -> Result<tonic::Response<super::BindDeviceToGatewayResponse>, tonic::Status>;
        #[doc = " Deletes the association between the device and the gateway."]
        async fn unbind_device_from_gateway(
            &self,
            request: tonic::Request<super::UnbindDeviceFromGatewayRequest>,
        ) -> Result<tonic::Response<super::UnbindDeviceFromGatewayResponse>, tonic::Status>;
    }
    #[doc = " Internet of Things (IoT) service. Securely connect and manage IoT devices."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct DeviceManagerServer<T: DeviceManager> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DeviceManager> DeviceManagerServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DeviceManagerServer<T>
    where
        T: DeviceManager,
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
                "/google.cloud.iot.v1.DeviceManager/CreateDeviceRegistry" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDeviceRegistrySvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::CreateDeviceRegistryRequest>
                        for CreateDeviceRegistrySvc<T>
                    {
                        type Response = super::DeviceRegistry;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDeviceRegistryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_device_registry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDeviceRegistrySvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/GetDeviceRegistry" => {
                    #[allow(non_camel_case_types)]
                    struct GetDeviceRegistrySvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::GetDeviceRegistryRequest>
                        for GetDeviceRegistrySvc<T>
                    {
                        type Response = super::DeviceRegistry;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDeviceRegistryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_device_registry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetDeviceRegistrySvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/UpdateDeviceRegistry" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDeviceRegistrySvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::UpdateDeviceRegistryRequest>
                        for UpdateDeviceRegistrySvc<T>
                    {
                        type Response = super::DeviceRegistry;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDeviceRegistryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_device_registry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateDeviceRegistrySvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/DeleteDeviceRegistry" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDeviceRegistrySvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::DeleteDeviceRegistryRequest>
                        for DeleteDeviceRegistrySvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDeviceRegistryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_device_registry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteDeviceRegistrySvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/ListDeviceRegistries" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceRegistriesSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::ListDeviceRegistriesRequest>
                        for ListDeviceRegistriesSvc<T>
                    {
                        type Response = super::ListDeviceRegistriesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDeviceRegistriesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_device_registries(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListDeviceRegistriesSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/CreateDevice" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDeviceSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager> tonic::server::UnaryService<super::CreateDeviceRequest>
                        for CreateDeviceSvc<T>
                    {
                        type Response = super::Device;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDeviceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_device(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDeviceSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/GetDevice" => {
                    #[allow(non_camel_case_types)]
                    struct GetDeviceSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager> tonic::server::UnaryService<super::GetDeviceRequest> for GetDeviceSvc<T> {
                        type Response = super::Device;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDeviceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_device(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetDeviceSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/UpdateDevice" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDeviceSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager> tonic::server::UnaryService<super::UpdateDeviceRequest>
                        for UpdateDeviceSvc<T>
                    {
                        type Response = super::Device;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDeviceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_device(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateDeviceSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/DeleteDevice" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDeviceSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager> tonic::server::UnaryService<super::DeleteDeviceRequest>
                        for DeleteDeviceSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDeviceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_device(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteDeviceSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/ListDevices" => {
                    #[allow(non_camel_case_types)]
                    struct ListDevicesSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager> tonic::server::UnaryService<super::ListDevicesRequest>
                        for ListDevicesSvc<T>
                    {
                        type Response = super::ListDevicesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDevicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_devices(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListDevicesSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/ModifyCloudToDeviceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyCloudToDeviceConfigSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::ModifyCloudToDeviceConfigRequest>
                        for ModifyCloudToDeviceConfigSvc<T>
                    {
                        type Response = super::DeviceConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModifyCloudToDeviceConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.modify_cloud_to_device_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyCloudToDeviceConfigSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/ListDeviceConfigVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceConfigVersionsSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::ListDeviceConfigVersionsRequest>
                        for ListDeviceConfigVersionsSvc<T>
                    {
                        type Response = super::ListDeviceConfigVersionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDeviceConfigVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.list_device_config_versions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListDeviceConfigVersionsSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/ListDeviceStates" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceStatesSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::ListDeviceStatesRequest>
                        for ListDeviceStatesSvc<T>
                    {
                        type Response = super::ListDeviceStatesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDeviceStatesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_device_states(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListDeviceStatesSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::SetIamPolicyRequest,
                        > for SetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::SetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetIamPolicySvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::GetIamPolicyRequest,
                        > for GetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::GetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIamPolicySvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::TestIamPermissionsRequest,
                        > for TestIamPermissionsSvc<T>
                    {
                        type Response =
                            super::super::super::super::iam::v1::TestIamPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::TestIamPermissionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.test_iam_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TestIamPermissionsSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/SendCommandToDevice" => {
                    #[allow(non_camel_case_types)]
                    struct SendCommandToDeviceSvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::SendCommandToDeviceRequest>
                        for SendCommandToDeviceSvc<T>
                    {
                        type Response = super::SendCommandToDeviceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendCommandToDeviceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.send_command_to_device(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendCommandToDeviceSvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/BindDeviceToGateway" => {
                    #[allow(non_camel_case_types)]
                    struct BindDeviceToGatewaySvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::BindDeviceToGatewayRequest>
                        for BindDeviceToGatewaySvc<T>
                    {
                        type Response = super::BindDeviceToGatewayResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BindDeviceToGatewayRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.bind_device_to_gateway(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BindDeviceToGatewaySvc(inner);
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
                "/google.cloud.iot.v1.DeviceManager/UnbindDeviceFromGateway" => {
                    #[allow(non_camel_case_types)]
                    struct UnbindDeviceFromGatewaySvc<T: DeviceManager>(pub Arc<T>);
                    impl<T: DeviceManager>
                        tonic::server::UnaryService<super::UnbindDeviceFromGatewayRequest>
                        for UnbindDeviceFromGatewaySvc<T>
                    {
                        type Response = super::UnbindDeviceFromGatewayResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnbindDeviceFromGatewayRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.unbind_device_from_gateway(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UnbindDeviceFromGatewaySvc(inner);
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
    impl<T: DeviceManager> Clone for DeviceManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DeviceManager> Clone for _Inner<T> {
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
