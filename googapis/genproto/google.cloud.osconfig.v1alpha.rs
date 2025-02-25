/// Step performed by the OS Config agent for configuring an `OSPolicyResource`
/// to its desired state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyResourceConfigStep {
    /// Configuration step type.
    #[prost(enumeration = "os_policy_resource_config_step::Type", tag = "1")]
    pub r#type: i32,
    /// Outcome of the configuration step.
    #[prost(enumeration = "os_policy_resource_config_step::Outcome", tag = "2")]
    pub outcome: i32,
    /// An error message recorded during the execution of this step.
    /// Only populated when outcome is FAILED.
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OSPolicyResourceConfigStep`.
pub mod os_policy_resource_config_step {
    /// Supported configuration step types
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
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Validation to detect resource conflicts, schema errors, etc.
        Validation = 1,
        /// Check the current desired state status of the resource.
        DesiredStateCheck = 2,
        /// Enforce the desired state for a resource that is not in desired state.
        DesiredStateEnforcement = 3,
        /// Re-check desired state status for a resource after enforcement of all
        /// resources in the current configuration run.
        ///
        /// This step is used to determine the final desired state status for the
        /// resource. It accounts for any resources that might have drifted from
        /// their desired state due to side effects from configuring other resources
        /// during the current configuration run.
        DesiredStateCheckPostEnforcement = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Validation => "VALIDATION",
                Type::DesiredStateCheck => "DESIRED_STATE_CHECK",
                Type::DesiredStateEnforcement => "DESIRED_STATE_ENFORCEMENT",
                Type::DesiredStateCheckPostEnforcement => {
                    "DESIRED_STATE_CHECK_POST_ENFORCEMENT"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "VALIDATION" => Some(Self::Validation),
                "DESIRED_STATE_CHECK" => Some(Self::DesiredStateCheck),
                "DESIRED_STATE_ENFORCEMENT" => Some(Self::DesiredStateEnforcement),
                "DESIRED_STATE_CHECK_POST_ENFORCEMENT" => {
                    Some(Self::DesiredStateCheckPostEnforcement)
                }
                _ => None,
            }
        }
    }
    /// Supported outcomes for a configuration step.
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
    pub enum Outcome {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The step succeeded.
        Succeeded = 1,
        /// The step failed.
        Failed = 2,
    }
    impl Outcome {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Outcome::Unspecified => "OUTCOME_UNSPECIFIED",
                Outcome::Succeeded => "SUCCEEDED",
                Outcome::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OUTCOME_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Compliance data for an OS policy resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyResourceCompliance {
    /// The id of the OS policy resource.
    #[prost(string, tag = "1")]
    pub os_policy_resource_id: ::prost::alloc::string::String,
    /// Ordered list of configuration steps taken by the agent for the OS policy
    /// resource.
    #[prost(message, repeated, tag = "2")]
    pub config_steps: ::prost::alloc::vec::Vec<OsPolicyResourceConfigStep>,
    /// Compliance state of the OS policy resource.
    #[prost(enumeration = "OsPolicyComplianceState", tag = "3")]
    pub state: i32,
    /// Resource specific output.
    #[prost(oneof = "os_policy_resource_compliance::Output", tags = "4")]
    pub output: ::core::option::Option<os_policy_resource_compliance::Output>,
}
/// Nested message and enum types in `OSPolicyResourceCompliance`.
pub mod os_policy_resource_compliance {
    /// ExecResource specific output.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecResourceOutput {
        /// Output from Enforcement phase output file (if run).
        /// Output size is limited to 100K bytes.
        #[prost(bytes = "vec", tag = "2")]
        pub enforcement_output: ::prost::alloc::vec::Vec<u8>,
    }
    /// Resource specific output.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        /// ExecResource specific output.
        #[prost(message, tag = "4")]
        ExecResourceOutput(ExecResourceOutput),
    }
}
/// Supported OSPolicy compliance states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OsPolicyComplianceState {
    /// Default value. This value is unused.
    Unspecified = 0,
    /// Compliant state.
    Compliant = 1,
    /// Non-compliant state
    NonCompliant = 2,
    /// Unknown compliance state.
    Unknown = 3,
    /// No applicable OS policies were found for the instance.
    /// This state is only applicable to the instance.
    NoOsPoliciesApplicable = 4,
}
impl OsPolicyComplianceState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OsPolicyComplianceState::Unspecified => {
                "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED"
            }
            OsPolicyComplianceState::Compliant => "COMPLIANT",
            OsPolicyComplianceState::NonCompliant => "NON_COMPLIANT",
            OsPolicyComplianceState::Unknown => "UNKNOWN",
            OsPolicyComplianceState::NoOsPoliciesApplicable => {
                "NO_OS_POLICIES_APPLICABLE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OS_POLICY_COMPLIANCE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "COMPLIANT" => Some(Self::Compliant),
            "NON_COMPLIANT" => Some(Self::NonCompliant),
            "UNKNOWN" => Some(Self::Unknown),
            "NO_OS_POLICIES_APPLICABLE" => Some(Self::NoOsPoliciesApplicable),
            _ => None,
        }
    }
}
/// This API resource represents the OS policies compliance data for a Compute
/// Engine virtual machine (VM) instance at a given point in time.
///
/// A Compute Engine VM can have multiple OS policy assignments, and each
/// assignment can have multiple OS policies. As a result, multiple OS policies
/// could be applied to a single VM.
///
/// You can use this API resource to determine both the compliance state of your
/// VM as well as the compliance state of an individual OS policy.
///
/// For more information, see [View
/// compliance](<https://cloud.google.com/compute/docs/os-configuration-management/view-compliance>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceOsPoliciesCompliance {
    /// Output only. The `InstanceOSPoliciesCompliance` API resource name.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/instanceOSPoliciesCompliances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The Compute Engine VM instance name.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Output only. Compliance state of the VM.
    #[prost(enumeration = "OsPolicyComplianceState", tag = "3")]
    pub state: i32,
    /// Output only. Detailed compliance state of the VM.
    /// This field is populated only when compliance state is `UNKNOWN`.
    ///
    /// It may contain one of the following values:
    ///
    /// * `no-compliance-data`: Compliance data is not available for this VM.
    /// * `no-agent-detected`: OS Config agent is not detected for this VM.
    /// * `config-not-supported-by-agent`: The version of the OS Config agent
    /// running on this VM does not support configuration management.
    /// * `inactive`: VM is not running.
    /// * `internal-service-errors`: There were internal service errors encountered
    /// while enforcing compliance.
    /// * `agent-errors`: OS config agent encountered errors while enforcing
    /// compliance.
    #[prost(string, tag = "4")]
    pub detailed_state: ::prost::alloc::string::String,
    /// Output only. The reason for the `detailed_state` of the VM (if any).
    #[prost(string, tag = "5")]
    pub detailed_state_reason: ::prost::alloc::string::String,
    /// Output only. Compliance data for each `OSPolicy` that is applied to the VM.
    #[prost(message, repeated, tag = "6")]
    pub os_policy_compliances: ::prost::alloc::vec::Vec<
        instance_os_policies_compliance::OsPolicyCompliance,
    >,
    /// Output only. Timestamp of the last compliance check for the VM.
    #[prost(message, optional, tag = "7")]
    pub last_compliance_check_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Unique identifier for the last compliance run.
    /// This id will be logged by the OS config agent during a compliance run and
    /// can be used for debugging and tracing purpose.
    #[prost(string, tag = "8")]
    pub last_compliance_run_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InstanceOSPoliciesCompliance`.
pub mod instance_os_policies_compliance {
    /// Compliance data for an OS policy
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsPolicyCompliance {
        /// The OS policy id
        #[prost(string, tag = "1")]
        pub os_policy_id: ::prost::alloc::string::String,
        /// Reference to the `OSPolicyAssignment` API resource that the `OSPolicy`
        /// belongs to.
        ///
        /// Format:
        /// `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}`
        #[prost(string, tag = "2")]
        pub os_policy_assignment: ::prost::alloc::string::String,
        /// Compliance state of the OS policy.
        #[prost(enumeration = "super::OsPolicyComplianceState", tag = "4")]
        pub state: i32,
        /// Compliance data for each `OSPolicyResource` that is applied to the
        /// VM.
        #[prost(message, repeated, tag = "5")]
        pub os_policy_resource_compliances: ::prost::alloc::vec::Vec<
            super::OsPolicyResourceCompliance,
        >,
    }
}
/// A request message for getting OS policies compliance data for the given
/// Compute Engine VM instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceOsPoliciesComplianceRequest {
    /// Required. API resource name for instance OS policies compliance resource.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/instanceOSPoliciesCompliances/{instance}`
    ///
    /// For `{project}`, either Compute Engine project-number or project-id can be
    /// provided.
    /// For `{instance}`, either Compute Engine VM instance-id or instance-name can
    /// be provided.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for listing OS policies compliance data for all Compute
/// Engine VMs in the given location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceOsPoliciesCompliancesRequest {
    /// Required. The parent resource name.
    ///
    /// Format: `projects/{project}/locations/{location}`
    ///
    /// For `{project}`, either Compute Engine project-number or project-id can be
    /// provided.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListInstanceOSPoliciesCompliances` that indicates where this listing
    /// should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by a
    /// `InstanceOSPoliciesCompliance` API resource to be included in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing OS policies compliance data for all Compute
/// Engine VMs in the given location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceOsPoliciesCompliancesResponse {
    /// List of instance OS policies compliance objects.
    #[prost(message, repeated, tag = "1")]
    pub instance_os_policies_compliances: ::prost::alloc::vec::Vec<
        InstanceOsPoliciesCompliance,
    >,
    /// The pagination token to retrieve the next page of instance OS policies
    /// compliance objects.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// This API resource represents the available inventory data for a
/// Compute Engine virtual machine (VM) instance at a given point in time.
///
/// You can use this API resource to determine the inventory data of your VM.
///
/// For more information, see [Information provided by OS inventory
/// management](<https://cloud.google.com/compute/docs/instances/os-inventory-management#data-collected>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inventory {
    /// Output only. The `Inventory` API resource name.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/instances/{instance_id}/inventory`
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Base level operating system information for the VM.
    #[prost(message, optional, tag = "1")]
    pub os_info: ::core::option::Option<inventory::OsInfo>,
    /// Output only. Inventory items related to the VM keyed by an opaque unique
    /// identifier for each inventory item. The identifier is unique to each
    /// distinct and addressable inventory item and will change, when there is a
    /// new package version.
    #[prost(map = "string, message", tag = "2")]
    pub items: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        inventory::Item,
    >,
    /// Output only. Timestamp of the last reported inventory for the VM.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Inventory`.
pub mod inventory {
    /// Operating system information for the VM.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsInfo {
        /// The VM hostname.
        #[prost(string, tag = "9")]
        pub hostname: ::prost::alloc::string::String,
        /// The operating system long name.
        /// For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019
        /// Datacenter'.
        #[prost(string, tag = "2")]
        pub long_name: ::prost::alloc::string::String,
        /// The operating system short name.
        /// For example, 'windows' or 'debian'.
        #[prost(string, tag = "3")]
        pub short_name: ::prost::alloc::string::String,
        /// The version of the operating system.
        #[prost(string, tag = "4")]
        pub version: ::prost::alloc::string::String,
        /// The system architecture of the operating system.
        #[prost(string, tag = "5")]
        pub architecture: ::prost::alloc::string::String,
        /// The kernel version of the operating system.
        #[prost(string, tag = "6")]
        pub kernel_version: ::prost::alloc::string::String,
        /// The kernel release of the operating system.
        #[prost(string, tag = "7")]
        pub kernel_release: ::prost::alloc::string::String,
        /// The current version of the OS Config agent running on the VM.
        #[prost(string, tag = "8")]
        pub osconfig_agent_version: ::prost::alloc::string::String,
    }
    /// A single piece of inventory on a VM.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        /// Identifier for this item, unique across items for this VM.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The origin of this inventory item.
        #[prost(enumeration = "item::OriginType", tag = "2")]
        pub origin_type: i32,
        /// When this inventory item was first detected.
        #[prost(message, optional, tag = "8")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// When this inventory item was last modified.
        #[prost(message, optional, tag = "9")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The specific type of inventory, correlating to its specific details.
        #[prost(enumeration = "item::Type", tag = "5")]
        pub r#type: i32,
        /// Specific details of this inventory item based on its type.
        #[prost(oneof = "item::Details", tags = "6, 7")]
        pub details: ::core::option::Option<item::Details>,
    }
    /// Nested message and enum types in `Item`.
    pub mod item {
        /// The origin of a specific inventory item.
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
        pub enum OriginType {
            /// Invalid. An origin type must be specified.
            Unspecified = 0,
            /// This inventory item was discovered as the result of the agent
            /// reporting inventory via the reporting API.
            InventoryReport = 1,
        }
        impl OriginType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    OriginType::Unspecified => "ORIGIN_TYPE_UNSPECIFIED",
                    OriginType::InventoryReport => "INVENTORY_REPORT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "ORIGIN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "INVENTORY_REPORT" => Some(Self::InventoryReport),
                    _ => None,
                }
            }
        }
        /// The different types of inventory that are tracked on a VM.
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
            /// Invalid. An type must be specified.
            Unspecified = 0,
            /// This represents a package that is installed on the VM.
            InstalledPackage = 1,
            /// This represents an update that is available for a package.
            AvailablePackage = 2,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::InstalledPackage => "INSTALLED_PACKAGE",
                    Type::AvailablePackage => "AVAILABLE_PACKAGE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "INSTALLED_PACKAGE" => Some(Self::InstalledPackage),
                    "AVAILABLE_PACKAGE" => Some(Self::AvailablePackage),
                    _ => None,
                }
            }
        }
        /// Specific details of this inventory item based on its type.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Details {
            /// Software package present on the VM instance.
            #[prost(message, tag = "6")]
            InstalledPackage(super::SoftwarePackage),
            /// Software package available to be installed on the VM instance.
            #[prost(message, tag = "7")]
            AvailablePackage(super::SoftwarePackage),
        }
    }
    /// Software package information of the operating system.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SoftwarePackage {
        /// Information about the different types of software packages.
        #[prost(oneof = "software_package::Details", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
        pub details: ::core::option::Option<software_package::Details>,
    }
    /// Nested message and enum types in `SoftwarePackage`.
    pub mod software_package {
        /// Information about the different types of software packages.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Details {
            /// Yum package info.
            /// For details about the yum package manager, see
            /// <https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum.>
            #[prost(message, tag = "1")]
            YumPackage(super::VersionedPackage),
            /// Details of an APT package.
            /// For details about the apt package manager, see
            /// <https://wiki.debian.org/Apt.>
            #[prost(message, tag = "2")]
            AptPackage(super::VersionedPackage),
            /// Details of a Zypper package.
            /// For details about the Zypper package manager, see
            /// <https://en.opensuse.org/SDB:Zypper_manual.>
            #[prost(message, tag = "3")]
            ZypperPackage(super::VersionedPackage),
            /// Details of a Googet package.
            ///   For details about the googet package manager, see
            ///   <https://github.com/google/googet.>
            #[prost(message, tag = "4")]
            GoogetPackage(super::VersionedPackage),
            /// Details of a Zypper patch.
            /// For details about the Zypper package manager, see
            /// <https://en.opensuse.org/SDB:Zypper_manual.>
            #[prost(message, tag = "5")]
            ZypperPatch(super::ZypperPatch),
            /// Details of a Windows Update package.
            /// See <https://docs.microsoft.com/en-us/windows/win32/api/_wua/> for
            /// information about Windows Update.
            #[prost(message, tag = "6")]
            WuaPackage(super::WindowsUpdatePackage),
            /// Details of a Windows Quick Fix engineering package.
            /// See
            /// <https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering>
            /// for info in Windows Quick Fix Engineering.
            #[prost(message, tag = "7")]
            QfePackage(super::WindowsQuickFixEngineeringPackage),
            /// Details of a COS package.
            #[prost(message, tag = "8")]
            CosPackage(super::VersionedPackage),
            /// Details of Windows Application.
            #[prost(message, tag = "9")]
            WindowsApplication(super::WindowsApplication),
        }
    }
    /// Information related to the a standard versioned package.  This includes
    /// package info for APT, Yum, Zypper, and Googet package managers.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VersionedPackage {
        /// The name of the package.
        #[prost(string, tag = "4")]
        pub package_name: ::prost::alloc::string::String,
        /// The system architecture this package is intended for.
        #[prost(string, tag = "2")]
        pub architecture: ::prost::alloc::string::String,
        /// The version of the package.
        #[prost(string, tag = "3")]
        pub version: ::prost::alloc::string::String,
    }
    /// Details related to a Zypper Patch.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ZypperPatch {
        /// The name of the patch.
        #[prost(string, tag = "5")]
        pub patch_name: ::prost::alloc::string::String,
        /// The category of the patch.
        #[prost(string, tag = "2")]
        pub category: ::prost::alloc::string::String,
        /// The severity specified for this patch
        #[prost(string, tag = "3")]
        pub severity: ::prost::alloc::string::String,
        /// Any summary information provided about this patch.
        #[prost(string, tag = "4")]
        pub summary: ::prost::alloc::string::String,
    }
    /// Details related to a Windows Update package.
    /// Field data and names are taken from Windows Update API IUpdate Interface:
    /// <https://docs.microsoft.com/en-us/windows/win32/api/_wua/>
    /// Descriptive fields like title, and description are localized based on
    /// the locale of the VM being updated.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsUpdatePackage {
        /// The localized title of the update package.
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        /// The localized description of the update package.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// The categories that are associated with this update package.
        #[prost(message, repeated, tag = "3")]
        pub categories: ::prost::alloc::vec::Vec<
            windows_update_package::WindowsUpdateCategory,
        >,
        /// A collection of Microsoft Knowledge Base article IDs that are associated
        /// with the update package.
        #[prost(string, repeated, tag = "4")]
        pub kb_article_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A hyperlink to the language-specific support information for the update.
        #[prost(string, tag = "11")]
        pub support_url: ::prost::alloc::string::String,
        /// A collection of URLs that provide more information about the update
        /// package.
        #[prost(string, repeated, tag = "5")]
        pub more_info_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Gets the identifier of an update package.  Stays the same across
        /// revisions.
        #[prost(string, tag = "6")]
        pub update_id: ::prost::alloc::string::String,
        /// The revision number of this update package.
        #[prost(int32, tag = "7")]
        pub revision_number: i32,
        /// The last published date of the update, in (UTC) date and time.
        #[prost(message, optional, tag = "10")]
        pub last_deployment_change_time: ::core::option::Option<
            ::prost_types::Timestamp,
        >,
    }
    /// Nested message and enum types in `WindowsUpdatePackage`.
    pub mod windows_update_package {
        /// Categories specified by the Windows Update.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WindowsUpdateCategory {
            /// The identifier of the windows update category.
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
            /// The name of the windows update category.
            #[prost(string, tag = "2")]
            pub name: ::prost::alloc::string::String,
        }
    }
    /// Information related to a Quick Fix Engineering package.
    /// Fields are taken from Windows QuickFixEngineering Interface and match
    /// the source names:
    /// <https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsQuickFixEngineeringPackage {
        /// A short textual description of the QFE update.
        #[prost(string, tag = "1")]
        pub caption: ::prost::alloc::string::String,
        /// A textual description of the QFE update.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Unique identifier associated with a particular QFE update.
        #[prost(string, tag = "3")]
        pub hot_fix_id: ::prost::alloc::string::String,
        /// Date that the QFE update was installed.  Mapped from installed_on field.
        #[prost(message, optional, tag = "5")]
        pub install_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Contains information about a Windows application as retrieved from the
    /// Windows Registry. For more information about these fields, see
    ///
    /// [Windows Installer Properties for the Uninstall
    /// Registry](<https://docs.microsoft.com/en-us/windows/win32/msi/uninstall-registry-key>){:
    /// class="external" }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsApplication {
        /// The name of the application or product.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// The version of the product or application in string format.
        #[prost(string, tag = "2")]
        pub display_version: ::prost::alloc::string::String,
        /// The name of the manufacturer for the product or application.
        #[prost(string, tag = "3")]
        pub publisher: ::prost::alloc::string::String,
        /// The last time this product received service. The value of this property
        /// is replaced each time a patch is applied or removed from the product or
        /// the command-line option is used to repair the product.
        #[prost(message, optional, tag = "4")]
        pub install_date: ::core::option::Option<
            super::super::super::super::r#type::Date,
        >,
        /// The internet address for technical support.
        #[prost(string, tag = "5")]
        pub help_link: ::prost::alloc::string::String,
    }
}
/// A request message for getting inventory data for the specified VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInventoryRequest {
    /// Required. API resource name for inventory resource.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/instances/{instance}/inventory`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    /// For `{instance}`, either Compute Engine  `instance-id` or `instance-name`
    /// can be provided.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Inventory view indicating what information should be included in the
    /// inventory resource. If unspecified, the default view is BASIC.
    #[prost(enumeration = "InventoryView", tag = "2")]
    pub view: i32,
}
/// A request message for listing inventory data for all VMs in the specified
/// location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInventoriesRequest {
    /// Required. The parent resource name.
    ///
    /// Format: `projects/{project}/locations/{location}/instances/{instance}`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be
    /// provided. For `{instance}`, only hyphen or dash character is supported to
    /// list inventories across VMs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Inventory view indicating what information should be included in the
    /// inventory resource. If unspecified, the default view is BASIC.
    #[prost(enumeration = "InventoryView", tag = "2")]
    pub view: i32,
    /// The maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListInventories` that indicates where this listing
    /// should continue from.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by a
    /// `Inventory` API resource to be included in the response.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing inventory data for all VMs in a specified
/// location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInventoriesResponse {
    /// List of inventory objects.
    #[prost(message, repeated, tag = "1")]
    pub inventories: ::prost::alloc::vec::Vec<Inventory>,
    /// The pagination token to retrieve the next page of inventory objects.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The view for inventory objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InventoryView {
    /// The default value.
    /// The API defaults to the BASIC view.
    Unspecified = 0,
    /// Returns the basic inventory information that includes `os_info`.
    Basic = 1,
    /// Returns all fields.
    Full = 2,
}
impl InventoryView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InventoryView::Unspecified => "INVENTORY_VIEW_UNSPECIFIED",
            InventoryView::Basic => "BASIC",
            InventoryView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVENTORY_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// An OS policy defines the desired state configuration for a VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicy {
    /// Required. The id of the OS policy with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the assignment.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Policy description.
    /// Length of the description is limited to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Policy mode
    #[prost(enumeration = "os_policy::Mode", tag = "3")]
    pub mode: i32,
    /// Required. List of resource groups for the policy.
    /// For a particular VM, resource groups are evaluated in the order specified
    /// and the first resource group that is applicable is selected and the rest
    /// are ignored.
    ///
    /// If none of the resource groups are applicable for a VM, the VM is
    /// considered to be non-compliant w.r.t this policy. This behavior can be
    /// toggled by the flag `allow_no_resource_group_match`
    #[prost(message, repeated, tag = "4")]
    pub resource_groups: ::prost::alloc::vec::Vec<os_policy::ResourceGroup>,
    /// This flag determines the OS policy compliance status when none of the
    /// resource groups within the policy are applicable for a VM. Set this value
    /// to `true` if the policy needs to be reported as compliant even if the
    /// policy has nothing to validate or enforce.
    #[prost(bool, tag = "5")]
    pub allow_no_resource_group_match: bool,
}
/// Nested message and enum types in `OSPolicy`.
pub mod os_policy {
    /// The `OSFilter` is used to specify the OS filtering criteria for the
    /// resource group.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsFilter {
        /// This should match OS short name emitted by the OS inventory agent.
        /// An empty value matches any OS.
        #[prost(string, tag = "1")]
        pub os_short_name: ::prost::alloc::string::String,
        /// This value should match the version emitted by the OS inventory
        /// agent.
        /// Prefix matches are supported if asterisk(*) is provided as the
        /// last character. For example, to match all versions with a major
        /// version of `7`, specify the following value for this field `7.*`
        #[prost(string, tag = "2")]
        pub os_version: ::prost::alloc::string::String,
    }
    /// An OS policy resource is used to define the desired state configuration
    /// and provides a specific functionality like installing/removing packages,
    /// executing a script etc.
    ///
    /// The system ensures that resources are always in their desired state by
    /// taking necessary actions if they have drifted from their desired state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        /// Required. The id of the resource with the following restrictions:
        ///
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the OS policy.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Resource type.
        #[prost(oneof = "resource::ResourceType", tags = "2, 3, 4, 5")]
        pub resource_type: ::core::option::Option<resource::ResourceType>,
    }
    /// Nested message and enum types in `Resource`.
    pub mod resource {
        /// A remote or local file.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct File {
            /// Defaults to false. When false, files are subject to validations
            /// based on the file type:
            ///
            /// Remote: A checksum must be specified.
            /// Cloud Storage: An object generation number must be specified.
            #[prost(bool, tag = "4")]
            pub allow_insecure: bool,
            /// A specific type of file.
            #[prost(oneof = "file::Type", tags = "1, 2, 3")]
            pub r#type: ::core::option::Option<file::Type>,
        }
        /// Nested message and enum types in `File`.
        pub mod file {
            /// Specifies a file available via some URI.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Remote {
                /// Required. URI from which to fetch the object. It should contain both the
                /// protocol and path following the format `{protocol}://{location}`.
                #[prost(string, tag = "1")]
                pub uri: ::prost::alloc::string::String,
                /// SHA256 checksum of the remote file.
                #[prost(string, tag = "2")]
                pub sha256_checksum: ::prost::alloc::string::String,
            }
            /// Specifies a file available as a Cloud Storage Object.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Gcs {
                /// Required. Bucket of the Cloud Storage object.
                #[prost(string, tag = "1")]
                pub bucket: ::prost::alloc::string::String,
                /// Required. Name of the Cloud Storage object.
                #[prost(string, tag = "2")]
                pub object: ::prost::alloc::string::String,
                /// Generation number of the Cloud Storage object.
                #[prost(int64, tag = "3")]
                pub generation: i64,
            }
            /// A specific type of file.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                /// A generic remote file.
                #[prost(message, tag = "1")]
                Remote(Remote),
                /// A Cloud Storage object.
                #[prost(message, tag = "2")]
                Gcs(Gcs),
                /// A local path within the VM to use.
                #[prost(string, tag = "3")]
                LocalPath(::prost::alloc::string::String),
            }
        }
        /// A resource that manages a system package.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PackageResource {
            /// Required. The desired state the agent should maintain for this package.
            #[prost(enumeration = "package_resource::DesiredState", tag = "1")]
            pub desired_state: i32,
            /// A system package.
            #[prost(
                oneof = "package_resource::SystemPackage",
                tags = "2, 3, 4, 5, 6, 7, 8"
            )]
            pub system_package: ::core::option::Option<package_resource::SystemPackage>,
        }
        /// Nested message and enum types in `PackageResource`.
        pub mod package_resource {
            /// A deb package file. dpkg packages only support INSTALLED state.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Deb {
                /// Required. A deb package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Whether dependencies should also be installed.
                /// - install when false: `dpkg -i package`
                /// - install when true: `apt-get update && apt-get -y install
                /// package.deb`
                #[prost(bool, tag = "2")]
                pub pull_deps: bool,
            }
            /// A package managed by APT.
            /// - install: `apt-get update && apt-get -y install \[name\]`
            /// - remove: `apt-get -y remove \[name\]`
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Apt {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// An RPM package file. RPM packages only support INSTALLED state.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Rpm {
                /// Required. An rpm package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Whether dependencies should also be installed.
                /// - install when false: `rpm --upgrade --replacepkgs package.rpm`
                /// - install when true: `yum -y install package.rpm` or
                /// `zypper -y install package.rpm`
                #[prost(bool, tag = "2")]
                pub pull_deps: bool,
            }
            /// A package managed by YUM.
            /// - install: `yum -y install package`
            /// - remove: `yum -y remove package`
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Yum {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// A package managed by Zypper.
            /// - install: `zypper -y install package`
            /// - remove: `zypper -y rm package`
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Zypper {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// A package managed by GooGet.
            /// - install: `googet -noconfirm install package`
            /// - remove: `googet -noconfirm remove package`
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GooGet {
                /// Required. Package name.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
            }
            /// An MSI package. MSI packages only support INSTALLED state.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Msi {
                /// Required. The MSI package.
                #[prost(message, optional, tag = "1")]
                pub source: ::core::option::Option<super::File>,
                /// Additional properties to use during installation.
                /// This should be in the format of Property=Setting.
                /// Appended to the defaults of `ACTION=INSTALL
                /// REBOOT=ReallySuppress`.
                #[prost(string, repeated, tag = "2")]
                pub properties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// The desired state that the OS Config agent maintains on the VM.
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
            pub enum DesiredState {
                /// Unspecified is invalid.
                Unspecified = 0,
                /// Ensure that the package is installed.
                Installed = 1,
                /// The agent ensures that the package is not installed and
                /// uninstalls it if detected.
                Removed = 2,
            }
            impl DesiredState {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        DesiredState::Unspecified => "DESIRED_STATE_UNSPECIFIED",
                        DesiredState::Installed => "INSTALLED",
                        DesiredState::Removed => "REMOVED",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "DESIRED_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                        "INSTALLED" => Some(Self::Installed),
                        "REMOVED" => Some(Self::Removed),
                        _ => None,
                    }
                }
            }
            /// A system package.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum SystemPackage {
                /// A package managed by Apt.
                #[prost(message, tag = "2")]
                Apt(Apt),
                /// A deb package file.
                #[prost(message, tag = "3")]
                Deb(Deb),
                /// A package managed by YUM.
                #[prost(message, tag = "4")]
                Yum(Yum),
                /// A package managed by Zypper.
                #[prost(message, tag = "5")]
                Zypper(Zypper),
                /// An rpm package file.
                #[prost(message, tag = "6")]
                Rpm(Rpm),
                /// A package managed by GooGet.
                #[prost(message, tag = "7")]
                Googet(GooGet),
                /// An MSI package.
                #[prost(message, tag = "8")]
                Msi(Msi),
            }
        }
        /// A resource that manages a package repository.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RepositoryResource {
            /// A specific type of repository.
            #[prost(oneof = "repository_resource::Repository", tags = "1, 2, 3, 4")]
            pub repository: ::core::option::Option<repository_resource::Repository>,
        }
        /// Nested message and enum types in `RepositoryResource`.
        pub mod repository_resource {
            /// Represents a single apt package repository. These will be added to
            /// a repo file that will be managed at
            /// `/etc/apt/sources.list.d/google_osconfig.list`.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct AptRepository {
                /// Required. Type of archive files in this repository.
                #[prost(enumeration = "apt_repository::ArchiveType", tag = "1")]
                pub archive_type: i32,
                /// Required. URI for this repository.
                #[prost(string, tag = "2")]
                pub uri: ::prost::alloc::string::String,
                /// Required. Distribution of this repository.
                #[prost(string, tag = "3")]
                pub distribution: ::prost::alloc::string::String,
                /// Required. List of components for this repository. Must contain at least one
                /// item.
                #[prost(string, repeated, tag = "4")]
                pub components: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// URI of the key file for this repository. The agent maintains a
                /// keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg`.
                #[prost(string, tag = "5")]
                pub gpg_key: ::prost::alloc::string::String,
            }
            /// Nested message and enum types in `AptRepository`.
            pub mod apt_repository {
                /// Type of archive.
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
                pub enum ArchiveType {
                    /// Unspecified is invalid.
                    Unspecified = 0,
                    /// Deb indicates that the archive contains binary files.
                    Deb = 1,
                    /// Deb-src indicates that the archive contains source files.
                    DebSrc = 2,
                }
                impl ArchiveType {
                    /// String value of the enum field names used in the ProtoBuf definition.
                    ///
                    /// The values are not transformed in any way and thus are considered stable
                    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                    pub fn as_str_name(&self) -> &'static str {
                        match self {
                            ArchiveType::Unspecified => "ARCHIVE_TYPE_UNSPECIFIED",
                            ArchiveType::Deb => "DEB",
                            ArchiveType::DebSrc => "DEB_SRC",
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "ARCHIVE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                            "DEB" => Some(Self::Deb),
                            "DEB_SRC" => Some(Self::DebSrc),
                            _ => None,
                        }
                    }
                }
            }
            /// Represents a single yum package repository. These are added to a
            /// repo file that is managed at
            /// `/etc/yum.repos.d/google_osconfig.repo`.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct YumRepository {
                /// Required. A one word, unique name for this repository. This is  the `repo
                /// id` in the yum config file and also the `display_name` if
                /// `display_name` is omitted. This id is also used as the unique
                /// identifier when checking for resource conflicts.
                #[prost(string, tag = "1")]
                pub id: ::prost::alloc::string::String,
                /// The display name of the repository.
                #[prost(string, tag = "2")]
                pub display_name: ::prost::alloc::string::String,
                /// Required. The location of the repository directory.
                #[prost(string, tag = "3")]
                pub base_url: ::prost::alloc::string::String,
                /// URIs of GPG keys.
                #[prost(string, repeated, tag = "4")]
                pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// Represents a single zypper package repository. These are added to a
            /// repo file that is managed at
            /// `/etc/zypp/repos.d/google_osconfig.repo`.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ZypperRepository {
                /// Required. A one word, unique name for this repository. This is the `repo
                /// id` in the zypper config file and also the `display_name` if
                /// `display_name` is omitted. This id is also used as the unique
                /// identifier when checking for GuestPolicy conflicts.
                #[prost(string, tag = "1")]
                pub id: ::prost::alloc::string::String,
                /// The display name of the repository.
                #[prost(string, tag = "2")]
                pub display_name: ::prost::alloc::string::String,
                /// Required. The location of the repository directory.
                #[prost(string, tag = "3")]
                pub base_url: ::prost::alloc::string::String,
                /// URIs of GPG keys.
                #[prost(string, repeated, tag = "4")]
                pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// Represents a Goo package repository. These are added to a repo file
            /// that is managed at
            /// `C:/ProgramData/GooGet/repos/google_osconfig.repo`.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GooRepository {
                /// Required. The name of the repository.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
                /// Required. The url of the repository.
                #[prost(string, tag = "2")]
                pub url: ::prost::alloc::string::String,
            }
            /// A specific type of repository.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Repository {
                /// An Apt Repository.
                #[prost(message, tag = "1")]
                Apt(AptRepository),
                /// A Yum Repository.
                #[prost(message, tag = "2")]
                Yum(YumRepository),
                /// A Zypper Repository.
                #[prost(message, tag = "3")]
                Zypper(ZypperRepository),
                /// A Goo Repository.
                #[prost(message, tag = "4")]
                Goo(GooRepository),
            }
        }
        /// A resource that allows executing scripts on the VM.
        ///
        /// The `ExecResource` has 2 stages: `validate` and `enforce` and both stages
        /// accept a script as an argument to execute.
        ///
        /// When the `ExecResource` is applied by the agent, it first executes the
        /// script in the `validate` stage. The `validate` stage can signal that the
        /// `ExecResource` is already in the desired state by returning an exit code
        /// of `100`. If the `ExecResource` is not in the desired state, it should
        /// return an exit code of `101`. Any other exit code returned by this stage
        /// is considered an error.
        ///
        /// If the `ExecResource` is not in the desired state based on the exit code
        /// from the `validate` stage, the agent proceeds to execute the script from
        /// the `enforce` stage. If the `ExecResource` is already in the desired
        /// state, the `enforce` stage will not be run.
        /// Similar to `validate` stage, the `enforce` stage should return an exit
        /// code of `100` to indicate that the resource in now in its desired state.
        /// Any other exit code is considered an error.
        ///
        /// NOTE: An exit code of `100` was chosen over `0` (and `101` vs `1`) to
        /// have an explicit indicator of `in desired state`, `not in desired state`
        /// and errors. Because, for example, Powershell will always return an exit
        /// code of `0` unless an `exit` statement is provided in the script. So, for
        /// reasons of consistency and being explicit, exit codes `100` and `101`
        /// were chosen.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExecResource {
            /// Required. What to run to validate this resource is in the desired state.
            /// An exit code of 100 indicates "in desired state", and exit code of 101
            /// indicates "not in desired state". Any other exit code indicates a
            /// failure running validate.
            #[prost(message, optional, tag = "1")]
            pub validate: ::core::option::Option<exec_resource::Exec>,
            /// What to run to bring this resource into the desired state.
            /// An exit code of 100 indicates "success", any other exit code indicates
            /// a failure running enforce.
            #[prost(message, optional, tag = "2")]
            pub enforce: ::core::option::Option<exec_resource::Exec>,
        }
        /// Nested message and enum types in `ExecResource`.
        pub mod exec_resource {
            /// A file or script to execute.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Exec {
                /// Optional arguments to pass to the source during execution.
                #[prost(string, repeated, tag = "3")]
                pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Required. The script interpreter to use.
                #[prost(enumeration = "exec::Interpreter", tag = "4")]
                pub interpreter: i32,
                /// Only recorded for enforce Exec.
                /// Path to an output file (that is created by this Exec) whose
                /// content will be recorded in OSPolicyResourceCompliance after a
                /// successful run. Absence or failure to read this file will result in
                /// this ExecResource being non-compliant. Output file size is limited to
                /// 100K bytes.
                #[prost(string, tag = "5")]
                pub output_file_path: ::prost::alloc::string::String,
                /// What to execute.
                #[prost(oneof = "exec::Source", tags = "1, 2")]
                pub source: ::core::option::Option<exec::Source>,
            }
            /// Nested message and enum types in `Exec`.
            pub mod exec {
                /// The interpreter to use.
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
                pub enum Interpreter {
                    /// Defaults to NONE.
                    Unspecified = 0,
                    /// If no interpreter is specified the
                    /// source will be executed directly, which will likely only
                    /// succeed for executables and scripts with shebang lines.
                    /// [Wikipedia
                    /// shebang](<https://en.wikipedia.org/wiki/Shebang_(Unix>)).
                    None = 1,
                    /// Indicates that the script will be run with /bin/sh on Linux and
                    /// cmd.exe on windows.
                    Shell = 2,
                    /// Indicates that the script will be run with powershell.
                    Powershell = 3,
                }
                impl Interpreter {
                    /// String value of the enum field names used in the ProtoBuf definition.
                    ///
                    /// The values are not transformed in any way and thus are considered stable
                    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                    pub fn as_str_name(&self) -> &'static str {
                        match self {
                            Interpreter::Unspecified => "INTERPRETER_UNSPECIFIED",
                            Interpreter::None => "NONE",
                            Interpreter::Shell => "SHELL",
                            Interpreter::Powershell => "POWERSHELL",
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "INTERPRETER_UNSPECIFIED" => Some(Self::Unspecified),
                            "NONE" => Some(Self::None),
                            "SHELL" => Some(Self::Shell),
                            "POWERSHELL" => Some(Self::Powershell),
                            _ => None,
                        }
                    }
                }
                /// What to execute.
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Source {
                    /// A remote or local file.
                    #[prost(message, tag = "1")]
                    File(super::super::File),
                    /// An inline script.
                    /// The size of the script is limited to 1024 characters.
                    #[prost(string, tag = "2")]
                    Script(::prost::alloc::string::String),
                }
            }
        }
        /// A resource that manages the state of a file.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FileResource {
            /// Required. The absolute path of the file within the VM.
            #[prost(string, tag = "3")]
            pub path: ::prost::alloc::string::String,
            /// Required. Desired state of the file.
            #[prost(enumeration = "file_resource::DesiredState", tag = "4")]
            pub state: i32,
            /// Consists of three octal digits which represent, in
            /// order, the permissions of the owner, group, and other users for the
            /// file (similarly to the numeric mode used in the linux chmod
            /// utility). Each digit represents a three bit number with the 4 bit
            /// corresponding to the read permissions, the 2 bit corresponds to the
            /// write bit, and the one bit corresponds to the execute permission.
            /// Default behavior is 755.
            ///
            /// Below are some examples of permissions and their associated values:
            /// read, write, and execute: 7
            /// read and execute: 5
            /// read and write: 6
            /// read only: 4
            #[prost(string, tag = "5")]
            pub permissions: ::prost::alloc::string::String,
            /// The source for the contents of the file.
            #[prost(oneof = "file_resource::Source", tags = "1, 2")]
            pub source: ::core::option::Option<file_resource::Source>,
        }
        /// Nested message and enum types in `FileResource`.
        pub mod file_resource {
            /// Desired state of the file.
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
            pub enum DesiredState {
                /// Unspecified is invalid.
                Unspecified = 0,
                /// Ensure file at path is present.
                Present = 1,
                /// Ensure file at path is absent.
                Absent = 2,
                /// Ensure the contents of the file at path matches. If the file does
                /// not exist it will be created.
                ContentsMatch = 3,
            }
            impl DesiredState {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        DesiredState::Unspecified => "DESIRED_STATE_UNSPECIFIED",
                        DesiredState::Present => "PRESENT",
                        DesiredState::Absent => "ABSENT",
                        DesiredState::ContentsMatch => "CONTENTS_MATCH",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "DESIRED_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                        "PRESENT" => Some(Self::Present),
                        "ABSENT" => Some(Self::Absent),
                        "CONTENTS_MATCH" => Some(Self::ContentsMatch),
                        _ => None,
                    }
                }
            }
            /// The source for the contents of the file.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Source {
                /// A remote or local source.
                #[prost(message, tag = "1")]
                File(super::File),
                /// A a file with this content.
                /// The size of the content is limited to 1024 characters.
                #[prost(string, tag = "2")]
                Content(::prost::alloc::string::String),
            }
        }
        /// Resource type.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ResourceType {
            /// Package resource
            #[prost(message, tag = "2")]
            Pkg(PackageResource),
            /// Package repository resource
            #[prost(message, tag = "3")]
            Repository(RepositoryResource),
            /// Exec resource
            #[prost(message, tag = "4")]
            Exec(ExecResource),
            /// File resource
            #[prost(message, tag = "5")]
            File(FileResource),
        }
    }
    /// Resource groups provide a mechanism to group OS policy resources.
    ///
    /// Resource groups enable OS policy authors to create a single OS policy
    /// to be applied to VMs running different operating Systems.
    ///
    /// When the OS policy is applied to a target VM, the appropriate resource
    /// group within the OS policy is selected based on the `OSFilter` specified
    /// within the resource group.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceGroup {
        /// Used to specify the OS filter for a resource group
        #[prost(message, optional, tag = "1")]
        pub os_filter: ::core::option::Option<OsFilter>,
        /// Required. List of resources configured for this resource group.
        /// The resources are executed in the exact order specified here.
        #[prost(message, repeated, tag = "2")]
        pub resources: ::prost::alloc::vec::Vec<Resource>,
    }
    /// Policy mode
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
    pub enum Mode {
        /// Invalid mode
        Unspecified = 0,
        /// This mode checks if the configuration resources in the policy are in
        /// their desired state. No actions are performed if they are not in the
        /// desired state. This mode is used for reporting purposes.
        Validation = 1,
        /// This mode checks if the configuration resources in the policy are in
        /// their desired state, and if not, enforces the desired state.
        Enforcement = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::Validation => "VALIDATION",
                Mode::Enforcement => "ENFORCEMENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "VALIDATION" => Some(Self::Validation),
                "ENFORCEMENT" => Some(Self::Enforcement),
                _ => None,
            }
        }
    }
}
/// Message encapsulating a value that can be either absolute ("fixed") or
/// relative ("percent") to a value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedOrPercent {
    /// Type of the value.
    #[prost(oneof = "fixed_or_percent::Mode", tags = "1, 2")]
    pub mode: ::core::option::Option<fixed_or_percent::Mode>,
}
/// Nested message and enum types in `FixedOrPercent`.
pub mod fixed_or_percent {
    /// Type of the value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// Specifies a fixed value.
        #[prost(int32, tag = "1")]
        Fixed(i32),
        /// Specifies the relative value defined as a percentage, which will be
        /// multiplied by a reference value.
        #[prost(int32, tag = "2")]
        Percent(i32),
    }
}
/// OS policy assignment is an API resource that is used to
/// apply a set of OS policies to a dynamically targeted group of Compute Engine
/// VM instances.
///
/// An OS policy is used to define the desired state configuration for a
/// Compute Engine VM instance through a set of configuration resources that
/// provide capabilities such as installing or removing software packages, or
/// executing a script.
///
/// For more information, see [OS policy and OS policy
/// assignment](<https://cloud.google.com/compute/docs/os-configuration-management/working-with-os-policies>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyAssignment {
    /// Resource name.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id}`
    ///
    /// This field is ignored when you create an OS policy assignment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// OS policy assignment description.
    /// Length of the description is limited to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. List of OS policies to be applied to the VMs.
    #[prost(message, repeated, tag = "3")]
    pub os_policies: ::prost::alloc::vec::Vec<OsPolicy>,
    /// Required. Filter to select VMs.
    #[prost(message, optional, tag = "4")]
    pub instance_filter: ::core::option::Option<os_policy_assignment::InstanceFilter>,
    /// Required. Rollout to deploy the OS policy assignment.
    /// A rollout is triggered in the following situations:
    /// 1) OSPolicyAssignment is created.
    /// 2) OSPolicyAssignment is updated and the update contains changes to one of
    /// the following fields:
    ///     - instance_filter
    ///     - os_policies
    /// 3) OSPolicyAssignment is deleted.
    #[prost(message, optional, tag = "5")]
    pub rollout: ::core::option::Option<os_policy_assignment::Rollout>,
    /// Output only. The assignment revision ID
    /// A new revision is committed whenever a rollout is triggered for a OS policy
    /// assignment
    #[prost(string, tag = "6")]
    pub revision_id: ::prost::alloc::string::String,
    /// Output only. The timestamp that the revision was created.
    #[prost(message, optional, tag = "7")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. OS policy assignment rollout state
    #[prost(enumeration = "os_policy_assignment::RolloutState", tag = "9")]
    pub rollout_state: i32,
    /// Output only. Indicates that this revision has been successfully rolled out in this zone
    /// and new VMs will be assigned OS policies from this revision.
    ///
    /// For a given OS policy assignment, there is only one revision with a value
    /// of `true` for this field.
    #[prost(bool, tag = "10")]
    pub baseline: bool,
    /// Output only. Indicates that this revision deletes the OS policy assignment.
    #[prost(bool, tag = "11")]
    pub deleted: bool,
    /// Output only. Indicates that reconciliation is in progress for the revision.
    /// This value is `true` when the `rollout_state` is one of:
    /// * IN_PROGRESS
    /// * CANCELLING
    #[prost(bool, tag = "12")]
    pub reconciling: bool,
    /// Output only. Server generated unique id for the OS policy assignment resource.
    #[prost(string, tag = "13")]
    pub uid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OSPolicyAssignment`.
pub mod os_policy_assignment {
    /// Message representing label set.
    /// * A label is a key value pair set for a VM.
    /// * A LabelSet is a set of labels.
    /// * Labels within a LabelSet are ANDed. In other words, a LabelSet is
    ///    applicable for a VM only if it matches all the labels in the
    ///    LabelSet.
    /// * Example: A LabelSet with 2 labels: `env=prod` and `type=webserver` will
    ///             only be applicable for those VMs with both labels
    ///             present.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LabelSet {
        /// Labels are identified by key/value pairs in this map.
        /// A VM should contain all the key/value pairs specified in this
        /// map to be selected.
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    /// Message to represent the filters to select VMs for an assignment
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceFilter {
        /// Target all VMs in the project. If true, no other criteria is
        /// permitted.
        #[prost(bool, tag = "1")]
        pub all: bool,
        /// A VM is included if it's OS short name matches with any of the
        /// values provided in this list.
        #[prost(string, repeated, tag = "2")]
        pub os_short_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// List of label sets used for VM inclusion.
        ///
        /// If the list has more than one `LabelSet`, the VM is included if any
        /// of the label sets are applicable for the VM.
        #[prost(message, repeated, tag = "3")]
        pub inclusion_labels: ::prost::alloc::vec::Vec<LabelSet>,
        /// List of label sets used for VM exclusion.
        ///
        /// If the list has more than one label set, the VM is excluded if any
        /// of the label sets are applicable for the VM.
        ///
        /// This filter is applied last in the filtering chain and therefore a
        /// VM is guaranteed to be excluded if it satisfies one of the below
        /// label sets.
        #[prost(message, repeated, tag = "4")]
        pub exclusion_labels: ::prost::alloc::vec::Vec<LabelSet>,
    }
    /// Message to configure the rollout at the zonal level for the OS policy
    /// assignment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rollout {
        /// Required. The maximum number (or percentage) of VMs per zone to disrupt at
        /// any given moment.
        #[prost(message, optional, tag = "1")]
        pub disruption_budget: ::core::option::Option<super::FixedOrPercent>,
        /// Required. This determines the minimum duration of time to wait after the
        /// configuration changes are applied through the current rollout. A
        /// VM continues to count towards the `disruption_budget` at least
        /// until this duration of time has passed after configuration changes are
        /// applied.
        #[prost(message, optional, tag = "2")]
        pub min_wait_duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// OS policy assignment rollout state
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
    pub enum RolloutState {
        /// Invalid value
        Unspecified = 0,
        /// The rollout is in progress.
        InProgress = 1,
        /// The rollout is being cancelled.
        Cancelling = 2,
        /// The rollout is cancelled.
        Cancelled = 3,
        /// The rollout has completed successfully.
        Succeeded = 4,
    }
    impl RolloutState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolloutState::Unspecified => "ROLLOUT_STATE_UNSPECIFIED",
                RolloutState::InProgress => "IN_PROGRESS",
                RolloutState::Cancelling => "CANCELLING",
                RolloutState::Cancelled => "CANCELLED",
                RolloutState::Succeeded => "SUCCEEDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLLOUT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "CANCELLING" => Some(Self::Cancelling),
                "CANCELLED" => Some(Self::Cancelled),
                "SUCCEEDED" => Some(Self::Succeeded),
                _ => None,
            }
        }
    }
}
/// OS policy assignment operation metadata provided by OS policy assignment API
/// methods that return long running operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsPolicyAssignmentOperationMetadata {
    /// Reference to the `OSPolicyAssignment` API resource.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/osPolicyAssignments/{os_policy_assignment_id@revision_id}`
    #[prost(string, tag = "1")]
    pub os_policy_assignment: ::prost::alloc::string::String,
    /// The OS policy assignment API method.
    #[prost(
        enumeration = "os_policy_assignment_operation_metadata::ApiMethod",
        tag = "2"
    )]
    pub api_method: i32,
    /// State of the rollout
    #[prost(
        enumeration = "os_policy_assignment_operation_metadata::RolloutState",
        tag = "3"
    )]
    pub rollout_state: i32,
    /// Rollout start time
    #[prost(message, optional, tag = "4")]
    pub rollout_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Rollout update time
    #[prost(message, optional, tag = "5")]
    pub rollout_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OSPolicyAssignmentOperationMetadata`.
pub mod os_policy_assignment_operation_metadata {
    /// The OS policy assignment API method.
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
    pub enum ApiMethod {
        /// Invalid value
        Unspecified = 0,
        /// Create OS policy assignment API method
        Create = 1,
        /// Update OS policy assignment API method
        Update = 2,
        /// Delete OS policy assignment API method
        Delete = 3,
    }
    impl ApiMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApiMethod::Unspecified => "API_METHOD_UNSPECIFIED",
                ApiMethod::Create => "CREATE",
                ApiMethod::Update => "UPDATE",
                ApiMethod::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "API_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE" => Some(Self::Create),
                "UPDATE" => Some(Self::Update),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
    /// State of the rollout
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
    pub enum RolloutState {
        /// Invalid value
        Unspecified = 0,
        /// The rollout is in progress.
        InProgress = 1,
        /// The rollout is being cancelled.
        Cancelling = 2,
        /// The rollout is cancelled.
        Cancelled = 3,
        /// The rollout has completed successfully.
        Succeeded = 4,
    }
    impl RolloutState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolloutState::Unspecified => "ROLLOUT_STATE_UNSPECIFIED",
                RolloutState::InProgress => "IN_PROGRESS",
                RolloutState::Cancelling => "CANCELLING",
                RolloutState::Cancelled => "CANCELLED",
                RolloutState::Succeeded => "SUCCEEDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLLOUT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "CANCELLING" => Some(Self::Cancelling),
                "CANCELLED" => Some(Self::Cancelled),
                "SUCCEEDED" => Some(Self::Succeeded),
                _ => None,
            }
        }
    }
}
/// A request message to create an OS policy assignment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOsPolicyAssignmentRequest {
    /// Required. The parent resource name in the form:
    /// projects/{project}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The OS policy assignment to be created.
    #[prost(message, optional, tag = "2")]
    pub os_policy_assignment: ::core::option::Option<OsPolicyAssignment>,
    /// Required. The logical name of the OS policy assignment in the project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the project.
    #[prost(string, tag = "3")]
    pub os_policy_assignment_id: ::prost::alloc::string::String,
}
/// A request message to update an OS policy assignment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOsPolicyAssignmentRequest {
    /// Required. The updated OS policy assignment.
    #[prost(message, optional, tag = "1")]
    pub os_policy_assignment: ::core::option::Option<OsPolicyAssignment>,
    /// Optional. Field mask that controls which fields of the assignment should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request message to get an OS policy assignment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOsPolicyAssignmentRequest {
    /// Required. The resource name of OS policy assignment.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/osPolicyAssignments/{os_policy_assignment}@{revisionId}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message to list OS policy assignments for a parent resource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentsRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of assignments to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListOSPolicyAssignments` that indicates where this listing should continue
    /// from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing all assignments under given parent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentsResponse {
    /// The list of assignments
    #[prost(message, repeated, tag = "1")]
    pub os_policy_assignments: ::prost::alloc::vec::Vec<OsPolicyAssignment>,
    /// The pagination token to retrieve the next page of OS policy assignments.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request message to list revisions for a OS policy assignment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentRevisionsRequest {
    /// Required. The name of the OS policy assignment to list revisions for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of revisions to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListOSPolicyAssignmentRevisions` that indicates where this listing should
    /// continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing all revisions for a OS policy assignment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsPolicyAssignmentRevisionsResponse {
    /// The OS policy assignment revisions
    #[prost(message, repeated, tag = "1")]
    pub os_policy_assignments: ::prost::alloc::vec::Vec<OsPolicyAssignment>,
    /// The pagination token to retrieve the next page of OS policy assignment
    /// revisions.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request message for deleting a OS policy assignment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOsPolicyAssignmentRequest {
    /// Required. The name of the OS policy assignment to be deleted
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// This API resource represents the vulnerability report for a specified
/// Compute Engine virtual machine (VM) instance at a given point in time.
///
/// For more information, see [Vulnerability
/// reports](<https://cloud.google.com/compute/docs/instances/os-inventory-management#vulnerability-reports>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityReport {
    /// Output only. The `vulnerabilityReport` API resource name.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/instances/{instance_id}/vulnerabilityReport`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. List of vulnerabilities affecting the VM.
    #[prost(message, repeated, tag = "2")]
    pub vulnerabilities: ::prost::alloc::vec::Vec<vulnerability_report::Vulnerability>,
    /// Output only. The timestamp for when the last vulnerability report was
    /// generated for the VM.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `VulnerabilityReport`.
pub mod vulnerability_report {
    /// A vulnerability affecting the VM instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Vulnerability {
        /// Contains metadata as per the upstream feed of the operating system and
        /// NVD.
        #[prost(message, optional, tag = "1")]
        pub details: ::core::option::Option<vulnerability::Details>,
        /// Corresponds to the `INSTALLED_PACKAGE` inventory item on the VM.
        /// This field displays the inventory items affected by this vulnerability.
        /// If the vulnerability report was not updated after the VM inventory
        /// update, these values might not display in VM inventory. For some distros,
        /// this field may be empty.
        #[prost(string, repeated, tag = "2")]
        pub installed_inventory_item_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Corresponds to the `AVAILABLE_PACKAGE` inventory item on the VM.
        /// If the vulnerability report was not updated after the VM inventory
        /// update, these values might not display in VM inventory. If there is no
        /// available fix, the field is empty. The `inventory_item` value specifies
        /// the latest `SoftwarePackage` available to the VM that fixes the
        /// vulnerability.
        #[prost(string, repeated, tag = "3")]
        pub available_inventory_item_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// The timestamp for when the vulnerability was first detected.
        #[prost(message, optional, tag = "4")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The timestamp for when the vulnerability was last modified.
        #[prost(message, optional, tag = "5")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Nested message and enum types in `Vulnerability`.
    pub mod vulnerability {
        /// Contains metadata information for the vulnerability. This information is
        /// collected from the upstream feed of the operating system.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Details {
            /// The CVE of the vulnerability. CVE cannot be
            /// empty and the combination of <cve, classification> should be unique
            /// across vulnerabilities for a VM.
            #[prost(string, tag = "1")]
            pub cve: ::prost::alloc::string::String,
            /// The CVSS V2 score of this vulnerability. CVSS V2 score is on a scale of
            /// 0 - 10 where 0 indicates low severity and 10 indicates high severity.
            #[prost(float, tag = "2")]
            pub cvss_v2_score: f32,
            /// The full description of the CVSSv3 for this vulnerability from NVD.
            #[prost(message, optional, tag = "3")]
            pub cvss_v3: ::core::option::Option<super::super::CvsSv3>,
            /// Assigned severity/impact ranking from the distro.
            #[prost(string, tag = "4")]
            pub severity: ::prost::alloc::string::String,
            /// The note or description describing the vulnerability from the distro.
            #[prost(string, tag = "5")]
            pub description: ::prost::alloc::string::String,
            /// Corresponds to the references attached to the `VulnerabilityDetails`.
            #[prost(message, repeated, tag = "6")]
            pub references: ::prost::alloc::vec::Vec<details::Reference>,
        }
        /// Nested message and enum types in `Details`.
        pub mod details {
            /// A reference for this vulnerability.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Reference {
                /// The url of the reference.
                #[prost(string, tag = "1")]
                pub url: ::prost::alloc::string::String,
            }
        }
    }
}
/// A request message for getting the vulnerability report for the specified VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVulnerabilityReportRequest {
    /// Required. API resource name for vulnerability resource.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/instances/{instance}/vulnerabilityReport`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    /// For `{instance}`, either Compute Engine `instance-id` or `instance-name`
    /// can be provided.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for listing vulnerability reports for all VM instances in
/// the specified location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVulnerabilityReportsRequest {
    /// Required. The parent resource name.
    ///
    /// Format: `projects/{project}/locations/{location}/instances/{instance}`
    ///
    /// For `{project}`, either `project-number` or `project-id` can be provided.
    /// For `{instance}`, only `-` character is supported to list vulnerability
    /// reports across VMs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to
    /// `ListVulnerabilityReports` that indicates where this listing
    /// should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by a
    /// `vulnerabilityReport` API resource to be included in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing vulnerability reports for all VM instances in
/// the specified location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVulnerabilityReportsResponse {
    /// List of vulnerabilityReport objects.
    #[prost(message, repeated, tag = "1")]
    pub vulnerability_reports: ::prost::alloc::vec::Vec<VulnerabilityReport>,
    /// The pagination token to retrieve the next page of vulnerabilityReports
    /// object.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Common Vulnerability Scoring System version 3.
/// For details, see <https://www.first.org/cvss/specification-document>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CvsSv3 {
    /// The base score is a function of the base metric scores.
    /// <https://www.first.org/cvss/specification-document#Base-Metrics>
    #[prost(float, tag = "1")]
    pub base_score: f32,
    /// The Exploitability sub-score equation is derived from the Base
    /// Exploitability metrics.
    /// <https://www.first.org/cvss/specification-document#2-1-Exploitability-Metrics>
    #[prost(float, tag = "2")]
    pub exploitability_score: f32,
    /// The Impact sub-score equation is derived from the Base Impact metrics.
    #[prost(float, tag = "3")]
    pub impact_score: f32,
    /// This metric reflects the context by which vulnerability exploitation is
    /// possible.
    #[prost(enumeration = "cvs_sv3::AttackVector", tag = "5")]
    pub attack_vector: i32,
    /// This metric describes the conditions beyond the attacker's control that
    /// must exist in order to exploit the vulnerability.
    #[prost(enumeration = "cvs_sv3::AttackComplexity", tag = "6")]
    pub attack_complexity: i32,
    /// This metric describes the level of privileges an attacker must possess
    /// before successfully exploiting the vulnerability.
    #[prost(enumeration = "cvs_sv3::PrivilegesRequired", tag = "7")]
    pub privileges_required: i32,
    /// This metric captures the requirement for a human user, other than the
    /// attacker, to participate in the successful compromise of the vulnerable
    /// component.
    #[prost(enumeration = "cvs_sv3::UserInteraction", tag = "8")]
    pub user_interaction: i32,
    /// The Scope metric captures whether a vulnerability in one vulnerable
    /// component impacts resources in components beyond its security scope.
    #[prost(enumeration = "cvs_sv3::Scope", tag = "9")]
    pub scope: i32,
    /// This metric measures the impact to the confidentiality of the information
    /// resources managed by a software component due to a successfully exploited
    /// vulnerability.
    #[prost(enumeration = "cvs_sv3::Impact", tag = "10")]
    pub confidentiality_impact: i32,
    /// This metric measures the impact to integrity of a successfully exploited
    /// vulnerability.
    #[prost(enumeration = "cvs_sv3::Impact", tag = "11")]
    pub integrity_impact: i32,
    /// This metric measures the impact to the availability of the impacted
    /// component resulting from a successfully exploited vulnerability.
    #[prost(enumeration = "cvs_sv3::Impact", tag = "12")]
    pub availability_impact: i32,
}
/// Nested message and enum types in `CVSSv3`.
pub mod cvs_sv3 {
    /// This metric reflects the context by which vulnerability exploitation is
    /// possible.
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
    pub enum AttackVector {
        /// Invalid value.
        Unspecified = 0,
        /// The vulnerable component is bound to the network stack and the set of
        /// possible attackers extends beyond the other options listed below, up to
        /// and including the entire Internet.
        Network = 1,
        /// The vulnerable component is bound to the network stack, but the attack is
        /// limited at the protocol level to a logically adjacent topology.
        Adjacent = 2,
        /// The vulnerable component is not bound to the network stack and the
        /// attacker's path is via read/write/execute capabilities.
        Local = 3,
        /// The attack requires the attacker to physically touch or manipulate the
        /// vulnerable component.
        Physical = 4,
    }
    impl AttackVector {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttackVector::Unspecified => "ATTACK_VECTOR_UNSPECIFIED",
                AttackVector::Network => "ATTACK_VECTOR_NETWORK",
                AttackVector::Adjacent => "ATTACK_VECTOR_ADJACENT",
                AttackVector::Local => "ATTACK_VECTOR_LOCAL",
                AttackVector::Physical => "ATTACK_VECTOR_PHYSICAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ATTACK_VECTOR_UNSPECIFIED" => Some(Self::Unspecified),
                "ATTACK_VECTOR_NETWORK" => Some(Self::Network),
                "ATTACK_VECTOR_ADJACENT" => Some(Self::Adjacent),
                "ATTACK_VECTOR_LOCAL" => Some(Self::Local),
                "ATTACK_VECTOR_PHYSICAL" => Some(Self::Physical),
                _ => None,
            }
        }
    }
    /// This metric describes the conditions beyond the attacker's control that
    /// must exist in order to exploit the vulnerability.
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
    pub enum AttackComplexity {
        /// Invalid value.
        Unspecified = 0,
        /// Specialized access conditions or extenuating circumstances do not exist.
        /// An attacker can expect repeatable success when attacking the vulnerable
        /// component.
        Low = 1,
        /// A successful attack depends on conditions beyond the attacker's control.
        /// That is, a successful attack cannot be accomplished at will, but requires
        /// the attacker to invest in some measurable amount of effort in preparation
        /// or execution against the vulnerable component before a successful attack
        /// can be expected.
        High = 2,
    }
    impl AttackComplexity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttackComplexity::Unspecified => "ATTACK_COMPLEXITY_UNSPECIFIED",
                AttackComplexity::Low => "ATTACK_COMPLEXITY_LOW",
                AttackComplexity::High => "ATTACK_COMPLEXITY_HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ATTACK_COMPLEXITY_UNSPECIFIED" => Some(Self::Unspecified),
                "ATTACK_COMPLEXITY_LOW" => Some(Self::Low),
                "ATTACK_COMPLEXITY_HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
    /// This metric describes the level of privileges an attacker must possess
    /// before successfully exploiting the vulnerability.
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
    pub enum PrivilegesRequired {
        /// Invalid value.
        Unspecified = 0,
        /// The attacker is unauthorized prior to attack, and therefore does not
        /// require any access to settings or files of the vulnerable system to
        /// carry out an attack.
        None = 1,
        /// The attacker requires privileges that provide basic user capabilities
        /// that could normally affect only settings and files owned by a user.
        /// Alternatively, an attacker with Low privileges has the ability to access
        /// only non-sensitive resources.
        Low = 2,
        /// The attacker requires privileges that provide significant (e.g.,
        /// administrative) control over the vulnerable component allowing access to
        /// component-wide settings and files.
        High = 3,
    }
    impl PrivilegesRequired {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PrivilegesRequired::Unspecified => "PRIVILEGES_REQUIRED_UNSPECIFIED",
                PrivilegesRequired::None => "PRIVILEGES_REQUIRED_NONE",
                PrivilegesRequired::Low => "PRIVILEGES_REQUIRED_LOW",
                PrivilegesRequired::High => "PRIVILEGES_REQUIRED_HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIVILEGES_REQUIRED_NONE" => Some(Self::None),
                "PRIVILEGES_REQUIRED_LOW" => Some(Self::Low),
                "PRIVILEGES_REQUIRED_HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
    /// This metric captures the requirement for a human user, other than the
    /// attacker, to participate in the successful compromise of the vulnerable
    /// component.
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
    pub enum UserInteraction {
        /// Invalid value.
        Unspecified = 0,
        /// The vulnerable system can be exploited without interaction from any user.
        None = 1,
        /// Successful exploitation of this vulnerability requires a user to take
        /// some action before the vulnerability can be exploited.
        Required = 2,
    }
    impl UserInteraction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserInteraction::Unspecified => "USER_INTERACTION_UNSPECIFIED",
                UserInteraction::None => "USER_INTERACTION_NONE",
                UserInteraction::Required => "USER_INTERACTION_REQUIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_INTERACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "USER_INTERACTION_NONE" => Some(Self::None),
                "USER_INTERACTION_REQUIRED" => Some(Self::Required),
                _ => None,
            }
        }
    }
    /// The Scope metric captures whether a vulnerability in one vulnerable
    /// component impacts resources in components beyond its security scope.
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
    pub enum Scope {
        /// Invalid value.
        Unspecified = 0,
        /// An exploited vulnerability can only affect resources managed by the same
        /// security authority.
        Unchanged = 1,
        /// An exploited vulnerability can affect resources beyond the security scope
        /// managed by the security authority of the vulnerable component.
        Changed = 2,
    }
    impl Scope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Scope::Unspecified => "SCOPE_UNSPECIFIED",
                Scope::Unchanged => "SCOPE_UNCHANGED",
                Scope::Changed => "SCOPE_CHANGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SCOPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SCOPE_UNCHANGED" => Some(Self::Unchanged),
                "SCOPE_CHANGED" => Some(Self::Changed),
                _ => None,
            }
        }
    }
    /// The Impact metrics capture the effects of a successfully exploited
    /// vulnerability on the component that suffers the worst outcome that is most
    /// directly and predictably associated with the attack.
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
    pub enum Impact {
        /// Invalid value.
        Unspecified = 0,
        /// High impact.
        High = 1,
        /// Low impact.
        Low = 2,
        /// No impact.
        None = 3,
    }
    impl Impact {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Impact::Unspecified => "IMPACT_UNSPECIFIED",
                Impact::High => "IMPACT_HIGH",
                Impact::Low => "IMPACT_LOW",
                Impact::None => "IMPACT_NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMPACT_UNSPECIFIED" => Some(Self::Unspecified),
                "IMPACT_HIGH" => Some(Self::High),
                "IMPACT_LOW" => Some(Self::Low),
                "IMPACT_NONE" => Some(Self::None),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod os_config_zonal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Zonal OS Config API
    ///
    /// The OS Config service is the server-side component that allows users to
    /// manage package installations and patch jobs for Compute Engine VM instances.
    #[derive(Debug, Clone)]
    pub struct OsConfigZonalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OsConfigZonalServiceClient<T>
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
        ) -> OsConfigZonalServiceClient<InterceptedService<T, F>>
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
            OsConfigZonalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Create an OS policy assignment.
        ///
        /// This method also creates the first revision of the OS policy assignment.
        ///
        /// This method returns a long running operation (LRO) that contains the
        /// rollout details. The rollout can be cancelled by cancelling the LRO.
        ///
        /// For more information, see [Method:
        /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1alpha/projects.locations.osPolicyAssignments.operations/cancel).
        pub async fn create_os_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOsPolicyAssignmentRequest>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/CreateOSPolicyAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "CreateOSPolicyAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update an existing OS policy assignment.
        ///
        /// This method creates a new revision of the OS policy assignment.
        ///
        /// This method returns a long running operation (LRO) that contains the
        /// rollout details. The rollout can be cancelled by cancelling the LRO.
        ///
        /// For more information, see [Method:
        /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1alpha/projects.locations.osPolicyAssignments.operations/cancel).
        pub async fn update_os_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOsPolicyAssignmentRequest>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/UpdateOSPolicyAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "UpdateOSPolicyAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieve an existing OS policy assignment.
        ///
        /// This method always returns the latest revision. In order to retrieve a
        /// previous revision of the assignment, also provide the revision ID in the
        /// `name` parameter.
        pub async fn get_os_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOsPolicyAssignmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OsPolicyAssignment>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetOSPolicyAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "GetOSPolicyAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List the OS policy assignments under the parent resource.
        ///
        /// For each OS policy assignment, the latest revision is returned.
        pub async fn list_os_policy_assignments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOsPolicyAssignmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOsPolicyAssignmentsResponse>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListOSPolicyAssignments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "ListOSPolicyAssignments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List the OS policy assignment revisions for a given OS policy assignment.
        pub async fn list_os_policy_assignment_revisions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListOsPolicyAssignmentRevisionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListOsPolicyAssignmentRevisionsResponse>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListOSPolicyAssignmentRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "ListOSPolicyAssignmentRevisions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete the OS policy assignment.
        ///
        /// This method creates a new revision of the OS policy assignment.
        ///
        /// This method returns a long running operation (LRO) that contains the
        /// rollout details. The rollout can be cancelled by cancelling the LRO.
        ///
        /// If the LRO completes and is not cancelled, all revisions associated with
        /// the OS policy assignment are deleted.
        ///
        /// For more information, see [Method:
        /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1alpha/projects.locations.osPolicyAssignments.operations/cancel).
        pub async fn delete_os_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOsPolicyAssignmentRequest>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/DeleteOSPolicyAssignment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "DeleteOSPolicyAssignment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get OS policies compliance data for the specified Compute Engine VM
        /// instance.
        pub async fn get_instance_os_policies_compliance(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetInstanceOsPoliciesComplianceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::InstanceOsPoliciesCompliance>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetInstanceOSPoliciesCompliance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "GetInstanceOSPoliciesCompliance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List OS policies compliance data for all Compute Engine VM instances in the
        /// specified zone.
        pub async fn list_instance_os_policies_compliances(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListInstanceOsPoliciesCompliancesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListInstanceOsPoliciesCompliancesResponse>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListInstanceOSPoliciesCompliances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "ListInstanceOSPoliciesCompliances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get inventory data for the specified VM instance. If the VM has no
        /// associated inventory, the message `NOT_FOUND` is returned.
        pub async fn get_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInventoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Inventory>, tonic::Status> {
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "GetInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List inventory data for all VM instances in the specified zone.
        pub async fn list_inventories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInventoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInventoriesResponse>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListInventories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "ListInventories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the vulnerability report for the specified VM instance. Only VMs with
        /// inventory data have vulnerability reports associated with them.
        pub async fn get_vulnerability_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVulnerabilityReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VulnerabilityReport>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/GetVulnerabilityReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "GetVulnerabilityReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List vulnerability reports for all VM instances in the specified zone.
        pub async fn list_vulnerability_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVulnerabilityReportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVulnerabilityReportsResponse>,
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
                "/google.cloud.osconfig.v1alpha.OsConfigZonalService/ListVulnerabilityReports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1alpha.OsConfigZonalService",
                        "ListVulnerabilityReports",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
