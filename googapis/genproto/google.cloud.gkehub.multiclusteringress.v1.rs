/// **Multi-cluster Ingress**: The configuration for the MultiClusterIngress
/// feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureSpec {
    /// Fully-qualified Membership name which hosts the MultiClusterIngress CRD.
    /// Example: `projects/foo-proj/locations/global/memberships/bar`
    #[prost(string, tag="1")]
    pub config_membership: ::prost::alloc::string::String,
}
