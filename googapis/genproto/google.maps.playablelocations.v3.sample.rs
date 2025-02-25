/// A geographical point suitable for placing game objects in location-based
/// games.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayableLocation {
    /// Required. The name of this playable location.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A collection of [Playable Location Types](/maps/tt/games/types) for this
    /// playable location. The first type in the collection is the primary type.
    ///
    /// Type information might not be available for all playable locations.
    #[prost(string, repeated, tag = "4")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The latitude and longitude associated with the center of the
    /// playable location.
    ///
    /// By default, the set of playable locations returned from
    /// \[SamplePlayableLocations][google.maps.playablelocations.v3.PlayableLocations.SamplePlayableLocations\]
    /// use center-point coordinates.
    #[prost(message, optional, tag = "5")]
    pub center_point: ::core::option::Option<super::super::super::super::r#type::LatLng>,
    /// The playable location's coordinates, snapped to the sidewalk of the
    /// nearest road, if a nearby road exists.
    #[prost(message, optional, tag = "6")]
    pub snapped_point: ::core::option::Option<
        super::super::super::super::r#type::LatLng,
    >,
    /// Required.
    /// Each location has one of the following identifiers:
    #[prost(oneof = "playable_location::LocationId", tags = "2, 3")]
    pub location_id: ::core::option::Option<playable_location::LocationId>,
}
/// Nested message and enum types in `PlayableLocation`.
pub mod playable_location {
    /// Required.
    /// Each location has one of the following identifiers:
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LocationId {
        /// A [place ID] (<https://developers.google.com/places/place-id>)
        #[prost(string, tag = "2")]
        PlaceId(::prost::alloc::string::String),
        /// A [plus code] (<http://openlocationcode.com>)
        #[prost(string, tag = "3")]
        PlusCode(::prost::alloc::string::String),
    }
}
/// A set of options that specifies the separation between playable locations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpacingOptions {
    /// Required. The minimum spacing between any two playable locations, measured
    /// in meters. The minimum value is 30. The maximum value is 1000.
    ///
    /// Inputs will be rounded up to the next 10 meter interval.
    ///
    /// The default value is 200m.
    ///
    /// Set this field to remove tight clusters of playable locations.
    ///
    /// Note:
    ///
    /// The spacing is a greedy algorithm. It optimizes for selecting the highest
    /// ranking locations first, not to maximize the number of locations selected.
    /// Consider the following scenario:
    ///
    ///    * Rank: A: 2, B: 1, C: 3.
    ///    * Distance: A--200m--B--200m--C
    ///
    /// If spacing=250, it will pick the highest ranked location \[B\], not [A, C].
    ///
    ///
    /// Note:
    ///
    /// Spacing works within the game object type itself, as well as the previous
    /// ones.
    /// Suppose three game object types, each with the following spacing:
    ///
    ///    * X: 400m, Y: undefined, Z: 200m.
    ///
    /// 1. Add locations for X, within 400m of each other.
    /// 2. Add locations for Y, without any spacing.
    /// 3. Finally, add locations for Z within 200m of each other as well X and Y.
    ///
    /// The distance diagram between those locations end up as:
    ///
    ///    * From->To.
    ///    * X->X: 400m
    ///    * Y->X, Y->Y: unspecified.
    ///    * Z->X, Z->Y, Z->Z: 200m.
    #[prost(double, tag = "1")]
    pub min_spacing_meters: f64,
    /// Specifies whether the minimum spacing constraint applies to the
    /// center-point or to the snapped point of playable locations. The default
    /// value is `CENTER_POINT`.
    ///
    /// If a snapped point is not available for a playable location, its
    /// center-point is used instead.
    ///
    /// Set this to the point type used in your game.
    #[prost(enumeration = "spacing_options::PointType", tag = "2")]
    pub point_type: i32,
}
/// Nested message and enum types in `SpacingOptions`.
pub mod spacing_options {
    /// Specifies whether the playable location's geographic coordinates (latitude
    /// and longitude) correspond to its center-point, or to its location snapped
    /// to the sidewalk of the nearest road.
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
    pub enum PointType {
        /// Unspecified point type. Do not use this value.
        Unspecified = 0,
        /// The geographic coordinates correspond to the center of the location.
        CenterPoint = 1,
        /// The geographic coordinates correspond to the location snapped to the
        /// sidewalk of the nearest road (when a nearby road exists).
        SnappedPoint = 2,
    }
    impl PointType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PointType::Unspecified => "POINT_TYPE_UNSPECIFIED",
                PointType::CenterPoint => "CENTER_POINT",
                PointType::SnappedPoint => "SNAPPED_POINT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POINT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CENTER_POINT" => Some(Self::CenterPoint),
                "SNAPPED_POINT" => Some(Self::SnappedPoint),
                _ => None,
            }
        }
    }
}
/// Specifies the filters to use when searching for playable locations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// Specifies the maximum number of playable locations to return. This value
    /// must not be greater than 1000. The default value is 100.
    ///
    /// Only the top-ranking playable locations are returned.
    #[prost(int32, tag = "1")]
    pub max_location_count: i32,
    /// A set of options that control the spacing between playable locations. By
    /// default the minimum distance between locations is 200m.
    #[prost(message, optional, tag = "2")]
    pub spacing: ::core::option::Option<SpacingOptions>,
    /// Restricts the set of playable locations to just the
    /// \[types\](/maps/tt/games/types) that you want.
    #[prost(string, repeated, tag = "3")]
    pub included_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Encapsulates a filter criterion for searching for a set of playable
/// locations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Criterion {
    /// Required. An arbitrary, developer-defined identifier of the type of game
    /// object that the playable location is used for. This field allows you to
    /// specify criteria per game object type when searching for playable
    /// locations.
    ///
    /// You should assign a unique `game_object_type` ID across all
    /// `request_criteria` to represent a distinct type of game object. For
    /// example, 1=monster location, 2=powerup location.
    ///
    /// The response contains a map<game_object_type, Response>.
    #[prost(int32, tag = "1")]
    pub game_object_type: i32,
    /// Specifies filtering options, and specifies what will be included in the
    /// result set.
    #[prost(message, optional, tag = "2")]
    pub filter: ::core::option::Option<Filter>,
    /// Specifies which `PlayableLocation` fields are returned.
    ///
    /// `name` (which is used for logging impressions), `center_point` and
    /// `place_id` (or `plus_code`) are always returned.
    ///
    /// The following fields are omitted unless you specify them here:
    ///
    ///    * snapped_point
    ///    * types
    ///
    /// Note: The more fields you include, the more expensive in terms of data and
    /// associated latency your query will be.
    #[prost(message, optional, tag = "3")]
    pub fields_to_return: ::core::option::Option<::prost_types::FieldMask>,
}
/// Specifies the area to search for playable locations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreaFilter {
    /// Required. The S2 cell ID of the area you want. This must be between cell
    /// level 11 and 14 (inclusive).
    ///
    /// S2 cells are 64-bit integers that identify areas on the Earth. They are
    /// hierarchical, and can therefore be used for spatial indexing.
    ///
    /// The S2 geometry library is available in a number of languages:
    ///
    ///    * \[C++\](<https://github.com/google/s2geometry>)
    ///    * \[Java\](<https://github.com/google/s2-geometry-library-java>)
    ///    * \[Go\](<https://github.com/golang/geo>)
    ///    * \[Python\](<https://github.com/google/s2geometry/tree/master/src/python>)
    #[prost(fixed64, tag = "1")]
    pub s2_cell_id: u64,
}
/// A list of PlayableLocation objects that satisfies a single Criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayableLocationList {
    /// A list of playable locations for this game object type.
    #[prost(message, repeated, tag = "1")]
    pub locations: ::prost::alloc::vec::Vec<PlayableLocation>,
}
