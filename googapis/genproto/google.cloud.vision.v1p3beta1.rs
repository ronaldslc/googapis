/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Y coordinate.
    #[prost(int32, tag = "2")]
    pub y: i32,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
}
/// A bounding polygon for the detected image annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    #[prost(message, repeated, tag = "1")]
    pub vertices: ::std::vec::Vec<Vertex>,
    /// The bounding polygon normalized vertices.
    #[prost(message, repeated, tag = "2")]
    pub normalized_vertices: ::std::vec::Vec<NormalizedVertex>,
}
/// A normalized bounding polygon around a portion of an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedBoundingPoly {
    /// Normalized vertices of the bounding polygon.
    #[prost(message, repeated, tag = "1")]
    pub vertices: ::std::vec::Vec<NormalizedVertex>,
}
/// A 3D position in the image, used primarily for Face detection landmarks.
/// A valid Position must have both x and y coordinates.
/// The position coordinates are in the same scale as the original image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
    /// Z coordinate (or depth).
    #[prost(float, tag = "3")]
    pub z: f32,
}
/// A Product contains ReferenceImages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// The resource name of the product.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.
    ///
    /// This field is ignored when creating a product.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The user-provided name for this Product. Must not be empty. Must be at most
    /// 4096 characters long.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// User-provided metadata to be stored with this product. Must be at most 4096
    /// characters long.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// Immutable. The category for the product identified by the reference image. This should
    /// be either "homegoods-v2", "apparel-v2", or "toys-v2". The legacy categories
    /// "homegoods", "apparel", and "toys" are still supported, but these should
    /// not be used for new products.
    #[prost(string, tag = "4")]
    pub product_category: std::string::String,
    /// Key-value pairs that can be attached to a product. At query time,
    /// constraints can be specified based on the product_labels.
    ///
    /// Note that integer values can be provided as strings, e.g. "1199". Only
    /// strings with integer values can match a range-based restriction which is
    /// to be supported soon.
    ///
    /// Multiple values can be assigned to the same key. One product may have up to
    /// 100 product_labels.
    #[prost(message, repeated, tag = "5")]
    pub product_labels: ::std::vec::Vec<product::KeyValue>,
}
pub mod product {
    /// A product label represented as a key-value pair.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyValue {
        /// The key of the label attached to the product. Cannot be empty and cannot
        /// exceed 128 bytes.
        #[prost(string, tag = "1")]
        pub key: std::string::String,
        /// The value of the label attached to the product. Cannot be empty and
        /// cannot exceed 128 bytes.
        #[prost(string, tag = "2")]
        pub value: std::string::String,
    }
}
/// A ProductSet contains Products. A ProductSet can contain a maximum of 1
/// million reference images. If the limit is exceeded, periodic indexing will
/// fail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductSet {
    /// The resource name of the ProductSet.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`.
    ///
    /// This field is ignored when creating a ProductSet.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The user-provided name for this ProductSet. Must not be empty. Must be at
    /// most 4096 characters long.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Output only. The time at which this ProductSet was last indexed. Query
    /// results will reflect all updates before this time. If this ProductSet has
    /// never been indexed, this field is 0.
    ///
    /// This field is ignored when creating a ProductSet.
    #[prost(message, optional, tag = "3")]
    pub index_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. If there was an error with indexing the product set, the field
    /// is populated.
    ///
    /// This field is ignored when creating a ProductSet.
    #[prost(message, optional, tag = "4")]
    pub index_error: ::std::option::Option<super::super::super::rpc::Status>,
}
/// A `ReferenceImage` represents a product image and its associated metadata,
/// such as bounding boxes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceImage {
    /// The resource name of the reference image.
    ///
    /// Format is:
    ///
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`.
    ///
    /// This field is ignored when creating a reference image.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The Google Cloud Storage URI of the reference image.
    ///
    /// The URI must start with `gs://`.
    #[prost(string, tag = "2")]
    pub uri: std::string::String,
    /// Optional. Bounding polygons around the areas of interest in the reference image.
    /// If this field is empty, the system will try to detect regions of
    /// interest. At most 10 bounding polygons will be used.
    ///
    /// The provided shape is converted into a non-rotated rectangle. Once
    /// converted, the small edge of the rectangle must be greater than or equal
    /// to 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5
    /// is not).
    #[prost(message, repeated, tag = "3")]
    pub bounding_polys: ::std::vec::Vec<BoundingPoly>,
}
/// Request message for the `CreateProduct` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductRequest {
    /// Required. The project in which the Product should be created.
    ///
    /// Format is
    /// `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The product to create.
    #[prost(message, optional, tag = "2")]
    pub product: ::std::option::Option<Product>,
    /// A user-supplied resource id for this Product. If set, the server will
    /// attempt to use this value as the resource id. If it is already in use, an
    /// error is returned with code ALREADY_EXISTS. Must be at most 128 characters
    /// long. It cannot contain the character `/`.
    #[prost(string, tag = "3")]
    pub product_id: std::string::String,
}
/// Request message for the `ListProducts` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsRequest {
    /// Required. The project OR ProductSet from which Products should be listed.
    ///
    /// Format:
    /// `projects/PROJECT_ID/locations/LOC_ID`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for the `ListProducts` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsResponse {
    /// List of products.
    #[prost(message, repeated, tag = "1")]
    pub products: ::std::vec::Vec<Product>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for the `GetProduct` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductRequest {
    /// Required. Resource name of the Product to get.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the `UpdateProduct` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProductRequest {
    /// Required. The Product resource which replaces the one on the server.
    /// product.name is immutable.
    #[prost(message, optional, tag = "1")]
    pub product: ::std::option::Option<Product>,
    /// The [FieldMask][google.protobuf.FieldMask] that specifies which fields
    /// to update.
    /// If update_mask isn't specified, all mutable fields are to be updated.
    /// Valid mask paths include `product_labels`, `display_name`, and
    /// `description`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for the `DeleteProduct` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProductRequest {
    /// Required. Resource name of product to delete.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the `CreateProductSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductSetRequest {
    /// Required. The project in which the ProductSet should be created.
    ///
    /// Format is `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ProductSet to create.
    #[prost(message, optional, tag = "2")]
    pub product_set: ::std::option::Option<ProductSet>,
    /// A user-supplied resource id for this ProductSet. If set, the server will
    /// attempt to use this value as the resource id. If it is already in use, an
    /// error is returned with code ALREADY_EXISTS. Must be at most 128 characters
    /// long. It cannot contain the character `/`.
    #[prost(string, tag = "3")]
    pub product_set_id: std::string::String,
}
/// Request message for the `ListProductSets` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductSetsRequest {
    /// Required. The project from which ProductSets should be listed.
    ///
    /// Format is `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for the `ListProductSets` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductSetsResponse {
    /// List of ProductSets.
    #[prost(message, repeated, tag = "1")]
    pub product_sets: ::std::vec::Vec<ProductSet>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for the `GetProductSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductSetRequest {
    /// Required. Resource name of the ProductSet to get.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the `UpdateProductSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProductSetRequest {
    /// Required. The ProductSet resource which replaces the one on the server.
    #[prost(message, optional, tag = "1")]
    pub product_set: ::std::option::Option<ProductSet>,
    /// The [FieldMask][google.protobuf.FieldMask] that specifies which fields to
    /// update.
    /// If update_mask isn't specified, all mutable fields are to be updated.
    /// Valid mask path is `display_name`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for the `DeleteProductSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProductSetRequest {
    /// Required. Resource name of the ProductSet to delete.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the `CreateReferenceImage` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReferenceImageRequest {
    /// Required. Resource name of the product in which to create the reference image.
    ///
    /// Format is
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The reference image to create.
    /// If an image ID is specified, it is ignored.
    #[prost(message, optional, tag = "2")]
    pub reference_image: ::std::option::Option<ReferenceImage>,
    /// A user-supplied resource id for the ReferenceImage to be added. If set,
    /// the server will attempt to use this value as the resource id. If it is
    /// already in use, an error is returned with code ALREADY_EXISTS. Must be at
    /// most 128 characters long. It cannot contain the character `/`.
    #[prost(string, tag = "3")]
    pub reference_image_id: std::string::String,
}
/// Request message for the `ListReferenceImages` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReferenceImagesRequest {
    /// Required. Resource name of the product containing the reference images.
    ///
    /// Format is
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results to be returned. This is the value
    /// of `nextPageToken` returned in a previous reference image list request.
    ///
    /// Defaults to the first page if not specified.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for the `ListReferenceImages` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReferenceImagesResponse {
    /// The list of reference images.
    #[prost(message, repeated, tag = "1")]
    pub reference_images: ::std::vec::Vec<ReferenceImage>,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
}
/// Request message for the `GetReferenceImage` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferenceImageRequest {
    /// Required. The resource name of the ReferenceImage to get.
    ///
    /// Format is:
    ///
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the `DeleteReferenceImage` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReferenceImageRequest {
    /// Required. The resource name of the reference image to delete.
    ///
    /// Format is:
    ///
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the `AddProductToProductSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProductToProductSetRequest {
    /// Required. The resource name for the ProductSet to modify.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The resource name for the Product to be added to this ProductSet.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    #[prost(string, tag = "2")]
    pub product: std::string::String,
}
/// Request message for the `RemoveProductFromProductSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProductFromProductSetRequest {
    /// Required. The resource name for the ProductSet to modify.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The resource name for the Product to be removed from this ProductSet.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    #[prost(string, tag = "2")]
    pub product: std::string::String,
}
/// Request message for the `ListProductsInProductSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsInProductSetRequest {
    /// Required. The ProductSet resource for which to retrieve Products.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for the `ListProductsInProductSet` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsInProductSetResponse {
    /// The list of Products.
    #[prost(message, repeated, tag = "1")]
    pub products: ::std::vec::Vec<Product>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The Google Cloud Storage location for a csv file which preserves a list of
/// ImportProductSetRequests in each line.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductSetsGcsSource {
    /// The Google Cloud Storage URI of the input csv file.
    ///
    /// The URI must start with `gs://`.
    ///
    /// The format of the input csv file should be one image per line.
    /// In each line, there are 6 columns.
    /// 1. image_uri
    /// 2, image_id
    /// 3. product_set_id
    /// 4. product_id
    /// 5, product_category
    /// 6, product_display_name
    /// 7, labels
    /// 8. bounding_poly
    ///
    /// Columns 1, 3, 4, and 5 are required, other columns are optional. A new
    /// ProductSet/Product with the same id will be created on the fly
    /// if the ProductSet/Product specified by product_set_id/product_id does not
    /// exist.
    ///
    /// The image_id field is optional but has to be unique if provided. If it is
    /// empty, we will automatically assign an unique id to the image.
    ///
    /// The product_display_name field is optional. If it is empty, a space (" ")
    /// is used as the place holder for the product display_name, which can
    /// be updated later through the realtime API.
    ///
    /// If the Product with product_id already exists, the fields
    /// product_display_name, product_category and labels are ignored.
    ///
    /// If a Product doesn't exist and needs to be created on the fly, the
    /// product_display_name field refers to
    /// [Product.display_name][google.cloud.vision.v1p3beta1.Product.display_name],
    /// the product_category field refers to
    /// [Product.product_category][google.cloud.vision.v1p3beta1.Product.product_category],
    /// and the labels field refers to [Product.labels][].
    ///
    /// Labels (optional) should be a line containing a list of comma-separated
    /// key-value pairs, with the format
    ///     "key_1=value_1,key_2=value_2,...,key_n=value_n".
    ///
    /// The bounding_poly (optional) field is used to identify one region of
    /// interest from the image in the same manner as CreateReferenceImage. If no
    /// bounding_poly is specified, the system will try to detect regions of
    /// interest automatically.
    ///
    /// Note that the pipeline will resize the image if the image resolution is too
    /// large to process (above 20MP).
    ///
    /// Also note that at most one bounding_poly is allowed per line. If the image
    /// contains multiple regions of interest, the csv should contain one line per
    /// region of interest.
    ///
    /// The bounding_poly column should contain an even number of comma-separated
    /// numbers, with the format "p1_x,p1_y,p2_x,p2_y,...,pn_x,pn_y". Nonnegative
    /// integers should be used for absolute bounding polygons, and float values
    /// in [0, 1] should be used for normalized bounding polygons.
    #[prost(string, tag = "1")]
    pub csv_file_uri: std::string::String,
}
/// The input content for the `ImportProductSets` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductSetsInputConfig {
    /// The source of the input.
    #[prost(oneof = "import_product_sets_input_config::Source", tags = "1")]
    pub source: ::std::option::Option<import_product_sets_input_config::Source>,
}
pub mod import_product_sets_input_config {
    /// The source of the input.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Google Cloud Storage location for a csv file which preserves a list
        /// of ImportProductSetRequests in each line.
        #[prost(message, tag = "1")]
        GcsSource(super::ImportProductSetsGcsSource),
    }
}
/// Request message for the `ImportProductSets` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductSetsRequest {
    /// Required. The project in which the ProductSets should be imported.
    ///
    /// Format is `projects/PROJECT_ID/locations/LOC_ID`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The input content for the list of requests.
    #[prost(message, optional, tag = "2")]
    pub input_config: ::std::option::Option<ImportProductSetsInputConfig>,
}
/// Response message for the `ImportProductSets` method.
///
/// This message is returned by the
/// [google.longrunning.Operations.GetOperation][google.longrunning.Operations.GetOperation]
/// method in the returned
/// [google.longrunning.Operation.response][google.longrunning.Operation.response]
/// field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductSetsResponse {
    /// The list of reference_images that are imported successfully.
    #[prost(message, repeated, tag = "1")]
    pub reference_images: ::std::vec::Vec<ReferenceImage>,
    /// The rpc status for each ImportProductSet request, including both successes
    /// and errors.
    ///
    /// The number of statuses here matches the number of lines in the csv file,
    /// and statuses[i] stores the success or failure status of processing the i-th
    /// line of the csv, starting from line 0.
    #[prost(message, repeated, tag = "2")]
    pub statuses: ::std::vec::Vec<super::super::super::rpc::Status>,
}
/// Metadata for the batch operations such as the current state.
///
/// This is included in the `metadata` field of the `Operation` returned by the
/// `GetOperation` call of the `google::longrunning::Operations` service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperationMetadata {
    /// The current state of the batch operation.
    #[prost(enumeration = "batch_operation_metadata::State", tag = "1")]
    pub state: i32,
    /// The time when the batch request was submitted to the server.
    #[prost(message, optional, tag = "2")]
    pub submit_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time when the batch request is finished and
    /// [google.longrunning.Operation.done][google.longrunning.Operation.done] is
    /// set to true.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
pub mod batch_operation_metadata {
    /// Enumerates the possible states that the batch request can be in.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid.
        Unspecified = 0,
        /// Request is actively being processed.
        Processing = 1,
        /// The request is done and at least one item has been successfully
        /// processed.
        Successful = 2,
        /// The request is done and no item has been successfully processed.
        Failed = 3,
        /// The request is done after the longrunning.Operations.CancelOperation has
        /// been called by the user.  Any records that were processed before the
        /// cancel command are output as specified in the request.
        Cancelled = 4,
    }
}
#[doc = r" Generated client implementations."]
pub mod product_search_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages Products and ProductSets of reference images for use in product"]
    #[doc = " search. It uses the following resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of [ProductSet][google.cloud.vision.v1p3beta1.ProductSet] resources, named"]
    #[doc = " `projects/*/locations/*/productSets/*`, which acts as a way to put different"]
    #[doc = " products into groups to limit identification."]
    #[doc = ""]
    #[doc = " In parallel,"]
    #[doc = ""]
    #[doc = " - The API has a collection of [Product][google.cloud.vision.v1p3beta1.Product] resources, named"]
    #[doc = "   `projects/*/locations/*/products/*`"]
    #[doc = ""]
    #[doc = " - Each [Product][google.cloud.vision.v1p3beta1.Product] has a collection of [ReferenceImage][google.cloud.vision.v1p3beta1.ReferenceImage] resources, named"]
    #[doc = "   `projects/*/locations/*/products/*/referenceImages/*`"]
    pub struct ProductSearchClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ProductSearchClient<T>
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
        #[doc = " Creates and returns a new ProductSet resource."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if display_name is missing, or is longer than"]
        #[doc = "   4096 characters."]
        pub async fn create_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/CreateProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists ProductSets in an unspecified order."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if page_size is greater than 100, or less"]
        #[doc = "   than 1."]
        pub async fn list_product_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductSetsRequest>,
        ) -> Result<tonic::Response<super::ListProductSetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/ListProductSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information associated with a ProductSet."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the ProductSet does not exist."]
        pub async fn get_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/GetProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Makes changes to a ProductSet resource."]
        #[doc = " Only display_name can be updated currently."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the ProductSet does not exist."]
        #[doc = " * Returns INVALID_ARGUMENT if display_name is present in update_mask but"]
        #[doc = "   missing from the request or longer than 4096 characters."]
        pub async fn update_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/UpdateProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes a ProductSet. All Products and ReferenceImages in the"]
        #[doc = " ProductSet will be deleted."]
        #[doc = ""]
        #[doc = " The actual image files are not deleted from Google Cloud Storage."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the ProductSet does not exist."]
        pub async fn delete_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/DeleteProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates and returns a new product resource."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if display_name is missing or longer than 4096"]
        #[doc = "   characters."]
        #[doc = " * Returns INVALID_ARGUMENT if description is longer than 4096 characters."]
        #[doc = " * Returns INVALID_ARGUMENT if product_category is missing or invalid."]
        pub async fn create_product(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/CreateProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists products in an unspecified order."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1."]
        pub async fn list_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsRequest>,
        ) -> Result<tonic::Response<super::ListProductsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/ListProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information associated with a Product."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the Product does not exist."]
        pub async fn get_product(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/GetProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Makes changes to a Product resource."]
        #[doc = " Only display_name, description and labels can be updated right now."]
        #[doc = ""]
        #[doc = " If labels are updated, the change will not be reflected in queries until"]
        #[doc = " the next index time."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the Product does not exist."]
        #[doc = " * Returns INVALID_ARGUMENT if display_name is present in update_mask but is"]
        #[doc = "   missing from the request or longer than 4096 characters."]
        #[doc = " * Returns INVALID_ARGUMENT if description is present in update_mask but is"]
        #[doc = "   longer than 4096 characters."]
        #[doc = " * Returns INVALID_ARGUMENT if product_category is present in update_mask."]
        pub async fn update_product(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/UpdateProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes a product and its reference images."]
        #[doc = ""]
        #[doc = " Metadata of the product and all its images will be deleted right away, but"]
        #[doc = " search queries against ProductSets containing the product may still work"]
        #[doc = " until all related caches are refreshed."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the product does not exist."]
        pub async fn delete_product(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProductRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/DeleteProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates and returns a new ReferenceImage resource."]
        #[doc = ""]
        #[doc = " The `bounding_poly` field is optional. If `bounding_poly` is not specified,"]
        #[doc = " the system will try to detect regions of interest in the image that are"]
        #[doc = " compatible with the product_category on the parent product. If it is"]
        #[doc = " specified, detection is ALWAYS skipped. The system converts polygons into"]
        #[doc = " non-rotated rectangles."]
        #[doc = ""]
        #[doc = " Note that the pipeline will resize the image if the image resolution is too"]
        #[doc = " large to process (above 50MP)."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if the image_uri is missing or longer than 4096"]
        #[doc = "   characters."]
        #[doc = " * Returns INVALID_ARGUMENT if the product does not exist."]
        #[doc = " * Returns INVALID_ARGUMENT if bounding_poly is not provided, and nothing"]
        #[doc = "   compatible with the parent product's product_category is detected."]
        #[doc = " * Returns INVALID_ARGUMENT if bounding_poly contains more than 10 polygons."]
        pub async fn create_reference_image(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReferenceImageRequest>,
        ) -> Result<tonic::Response<super::ReferenceImage>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/CreateReferenceImage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes a reference image."]
        #[doc = ""]
        #[doc = " The image metadata will be deleted right away, but search queries"]
        #[doc = " against ProductSets containing the image may still work until all related"]
        #[doc = " caches are refreshed."]
        #[doc = ""]
        #[doc = " The actual image files are not deleted from Google Cloud Storage."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the reference image does not exist."]
        pub async fn delete_reference_image(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReferenceImageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/DeleteReferenceImage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists reference images."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the parent product does not exist."]
        #[doc = " * Returns INVALID_ARGUMENT if the page_size is greater than 100, or less"]
        #[doc = "   than 1."]
        pub async fn list_reference_images(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReferenceImagesRequest>,
        ) -> Result<tonic::Response<super::ListReferenceImagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/ListReferenceImages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information associated with a ReferenceImage."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the specified image does not exist."]
        pub async fn get_reference_image(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReferenceImageRequest>,
        ) -> Result<tonic::Response<super::ReferenceImage>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/GetReferenceImage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds a Product to the specified ProductSet. If the Product is already"]
        #[doc = " present, no change is made."]
        #[doc = ""]
        #[doc = " One Product can be added to at most 100 ProductSets."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the Product or the ProductSet doesn't exist."]
        pub async fn add_product_to_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProductToProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/AddProductToProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Removes a Product from the specified ProductSet."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND If the Product is not found under the ProductSet."]
        pub async fn remove_product_from_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveProductFromProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/RemoveProductFromProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the Products in a ProductSet, in an unspecified order. If the"]
        #[doc = " ProductSet does not exist, the products field of the response will be"]
        #[doc = " empty."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1."]
        pub async fn list_products_in_product_set(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsInProductSetRequest>,
        ) -> Result<tonic::Response<super::ListProductsInProductSetResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ProductSearch/ListProductsInProductSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Asynchronous API that imports a list of reference images to specified"]
        #[doc = " product sets based on a list of image information."]
        #[doc = ""]
        #[doc = " The [google.longrunning.Operation][google.longrunning.Operation] API can be"]
        #[doc = " used to keep track of the progress and results of the request."]
        #[doc = " `Operation.metadata` contains `BatchOperationMetadata`. (progress)"]
        #[doc = " `Operation.response` contains `ImportProductSetsResponse`. (results)"]
        #[doc = ""]
        #[doc = " The input source of this method is a csv file on Google Cloud Storage."]
        #[doc = " For the format of the csv file please see"]
        #[doc = " [ImportProductSetsGcsSource.csv_file_uri][google.cloud.vision.v1p3beta1.ImportProductSetsGcsSource.csv_file_uri]."]
        pub async fn import_product_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportProductSetsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/ImportProductSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProductSearchClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProductSearchClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProductSearchClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod product_search_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProductSearchServer."]
    #[async_trait]
    pub trait ProductSearch: Send + Sync + 'static {
        #[doc = " Creates and returns a new ProductSet resource."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if display_name is missing, or is longer than"]
        #[doc = "   4096 characters."]
        async fn create_product_set(
            &self,
            request: tonic::Request<super::CreateProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status>;
        #[doc = " Lists ProductSets in an unspecified order."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if page_size is greater than 100, or less"]
        #[doc = "   than 1."]
        async fn list_product_sets(
            &self,
            request: tonic::Request<super::ListProductSetsRequest>,
        ) -> Result<tonic::Response<super::ListProductSetsResponse>, tonic::Status>;
        #[doc = " Gets information associated with a ProductSet."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the ProductSet does not exist."]
        async fn get_product_set(
            &self,
            request: tonic::Request<super::GetProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status>;
        #[doc = " Makes changes to a ProductSet resource."]
        #[doc = " Only display_name can be updated currently."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the ProductSet does not exist."]
        #[doc = " * Returns INVALID_ARGUMENT if display_name is present in update_mask but"]
        #[doc = "   missing from the request or longer than 4096 characters."]
        async fn update_product_set(
            &self,
            request: tonic::Request<super::UpdateProductSetRequest>,
        ) -> Result<tonic::Response<super::ProductSet>, tonic::Status>;
        #[doc = " Permanently deletes a ProductSet. All Products and ReferenceImages in the"]
        #[doc = " ProductSet will be deleted."]
        #[doc = ""]
        #[doc = " The actual image files are not deleted from Google Cloud Storage."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the ProductSet does not exist."]
        async fn delete_product_set(
            &self,
            request: tonic::Request<super::DeleteProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Creates and returns a new product resource."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if display_name is missing or longer than 4096"]
        #[doc = "   characters."]
        #[doc = " * Returns INVALID_ARGUMENT if description is longer than 4096 characters."]
        #[doc = " * Returns INVALID_ARGUMENT if product_category is missing or invalid."]
        async fn create_product(
            &self,
            request: tonic::Request<super::CreateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status>;
        #[doc = " Lists products in an unspecified order."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1."]
        async fn list_products(
            &self,
            request: tonic::Request<super::ListProductsRequest>,
        ) -> Result<tonic::Response<super::ListProductsResponse>, tonic::Status>;
        #[doc = " Gets information associated with a Product."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the Product does not exist."]
        async fn get_product(
            &self,
            request: tonic::Request<super::GetProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status>;
        #[doc = " Makes changes to a Product resource."]
        #[doc = " Only display_name, description and labels can be updated right now."]
        #[doc = ""]
        #[doc = " If labels are updated, the change will not be reflected in queries until"]
        #[doc = " the next index time."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the Product does not exist."]
        #[doc = " * Returns INVALID_ARGUMENT if display_name is present in update_mask but is"]
        #[doc = "   missing from the request or longer than 4096 characters."]
        #[doc = " * Returns INVALID_ARGUMENT if description is present in update_mask but is"]
        #[doc = "   longer than 4096 characters."]
        #[doc = " * Returns INVALID_ARGUMENT if product_category is present in update_mask."]
        async fn update_product(
            &self,
            request: tonic::Request<super::UpdateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status>;
        #[doc = " Permanently deletes a product and its reference images."]
        #[doc = ""]
        #[doc = " Metadata of the product and all its images will be deleted right away, but"]
        #[doc = " search queries against ProductSets containing the product may still work"]
        #[doc = " until all related caches are refreshed."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the product does not exist."]
        async fn delete_product(
            &self,
            request: tonic::Request<super::DeleteProductRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Creates and returns a new ReferenceImage resource."]
        #[doc = ""]
        #[doc = " The `bounding_poly` field is optional. If `bounding_poly` is not specified,"]
        #[doc = " the system will try to detect regions of interest in the image that are"]
        #[doc = " compatible with the product_category on the parent product. If it is"]
        #[doc = " specified, detection is ALWAYS skipped. The system converts polygons into"]
        #[doc = " non-rotated rectangles."]
        #[doc = ""]
        #[doc = " Note that the pipeline will resize the image if the image resolution is too"]
        #[doc = " large to process (above 50MP)."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if the image_uri is missing or longer than 4096"]
        #[doc = "   characters."]
        #[doc = " * Returns INVALID_ARGUMENT if the product does not exist."]
        #[doc = " * Returns INVALID_ARGUMENT if bounding_poly is not provided, and nothing"]
        #[doc = "   compatible with the parent product's product_category is detected."]
        #[doc = " * Returns INVALID_ARGUMENT if bounding_poly contains more than 10 polygons."]
        async fn create_reference_image(
            &self,
            request: tonic::Request<super::CreateReferenceImageRequest>,
        ) -> Result<tonic::Response<super::ReferenceImage>, tonic::Status>;
        #[doc = " Permanently deletes a reference image."]
        #[doc = ""]
        #[doc = " The image metadata will be deleted right away, but search queries"]
        #[doc = " against ProductSets containing the image may still work until all related"]
        #[doc = " caches are refreshed."]
        #[doc = ""]
        #[doc = " The actual image files are not deleted from Google Cloud Storage."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the reference image does not exist."]
        async fn delete_reference_image(
            &self,
            request: tonic::Request<super::DeleteReferenceImageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists reference images."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the parent product does not exist."]
        #[doc = " * Returns INVALID_ARGUMENT if the page_size is greater than 100, or less"]
        #[doc = "   than 1."]
        async fn list_reference_images(
            &self,
            request: tonic::Request<super::ListReferenceImagesRequest>,
        ) -> Result<tonic::Response<super::ListReferenceImagesResponse>, tonic::Status>;
        #[doc = " Gets information associated with a ReferenceImage."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the specified image does not exist."]
        async fn get_reference_image(
            &self,
            request: tonic::Request<super::GetReferenceImageRequest>,
        ) -> Result<tonic::Response<super::ReferenceImage>, tonic::Status>;
        #[doc = " Adds a Product to the specified ProductSet. If the Product is already"]
        #[doc = " present, no change is made."]
        #[doc = ""]
        #[doc = " One Product can be added to at most 100 ProductSets."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND if the Product or the ProductSet doesn't exist."]
        async fn add_product_to_product_set(
            &self,
            request: tonic::Request<super::AddProductToProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Removes a Product from the specified ProductSet."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns NOT_FOUND If the Product is not found under the ProductSet."]
        async fn remove_product_from_product_set(
            &self,
            request: tonic::Request<super::RemoveProductFromProductSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists the Products in a ProductSet, in an unspecified order. If the"]
        #[doc = " ProductSet does not exist, the products field of the response will be"]
        #[doc = " empty."]
        #[doc = ""]
        #[doc = " Possible errors:"]
        #[doc = ""]
        #[doc = " * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1."]
        async fn list_products_in_product_set(
            &self,
            request: tonic::Request<super::ListProductsInProductSetRequest>,
        ) -> Result<tonic::Response<super::ListProductsInProductSetResponse>, tonic::Status>;
        #[doc = " Asynchronous API that imports a list of reference images to specified"]
        #[doc = " product sets based on a list of image information."]
        #[doc = ""]
        #[doc = " The [google.longrunning.Operation][google.longrunning.Operation] API can be"]
        #[doc = " used to keep track of the progress and results of the request."]
        #[doc = " `Operation.metadata` contains `BatchOperationMetadata`. (progress)"]
        #[doc = " `Operation.response` contains `ImportProductSetsResponse`. (results)"]
        #[doc = ""]
        #[doc = " The input source of this method is a csv file on Google Cloud Storage."]
        #[doc = " For the format of the csv file please see"]
        #[doc = " [ImportProductSetsGcsSource.csv_file_uri][google.cloud.vision.v1p3beta1.ImportProductSetsGcsSource.csv_file_uri]."]
        async fn import_product_sets(
            &self,
            request: tonic::Request<super::ImportProductSetsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Manages Products and ProductSets of reference images for use in product"]
    #[doc = " search. It uses the following resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of [ProductSet][google.cloud.vision.v1p3beta1.ProductSet] resources, named"]
    #[doc = " `projects/*/locations/*/productSets/*`, which acts as a way to put different"]
    #[doc = " products into groups to limit identification."]
    #[doc = ""]
    #[doc = " In parallel,"]
    #[doc = ""]
    #[doc = " - The API has a collection of [Product][google.cloud.vision.v1p3beta1.Product] resources, named"]
    #[doc = "   `projects/*/locations/*/products/*`"]
    #[doc = ""]
    #[doc = " - Each [Product][google.cloud.vision.v1p3beta1.Product] has a collection of [ReferenceImage][google.cloud.vision.v1p3beta1.ReferenceImage] resources, named"]
    #[doc = "   `projects/*/locations/*/products/*/referenceImages/*`"]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ProductSearchServer<T: ProductSearch> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ProductSearch> ProductSearchServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ProductSearchServer<T>
    where
        T: ProductSearch,
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/CreateProductSet" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProductSetSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::CreateProductSetRequest>
                        for CreateProductSetSvc<T>
                    {
                        type Response = super::ProductSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProductSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_product_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateProductSetSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/ListProductSets" => {
                    #[allow(non_camel_case_types)]
                    struct ListProductSetsSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::ListProductSetsRequest>
                        for ListProductSetsSvc<T>
                    {
                        type Response = super::ListProductSetsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProductSetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_product_sets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListProductSetsSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/GetProductSet" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductSetSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch> tonic::server::UnaryService<super::GetProductSetRequest>
                        for GetProductSetSvc<T>
                    {
                        type Response = super::ProductSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProductSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_product_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetProductSetSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/UpdateProductSet" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProductSetSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::UpdateProductSetRequest>
                        for UpdateProductSetSvc<T>
                    {
                        type Response = super::ProductSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProductSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_product_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateProductSetSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/DeleteProductSet" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProductSetSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::DeleteProductSetRequest>
                        for DeleteProductSetSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteProductSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_product_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteProductSetSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/CreateProduct" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProductSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch> tonic::server::UnaryService<super::CreateProductRequest>
                        for CreateProductSvc<T>
                    {
                        type Response = super::Product;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_product(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateProductSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/ListProducts" => {
                    #[allow(non_camel_case_types)]
                    struct ListProductsSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch> tonic::server::UnaryService<super::ListProductsRequest>
                        for ListProductsSvc<T>
                    {
                        type Response = super::ListProductsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProductsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_products(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListProductsSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/GetProduct" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch> tonic::server::UnaryService<super::GetProductRequest> for GetProductSvc<T> {
                        type Response = super::Product;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_product(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetProductSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/UpdateProduct" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProductSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch> tonic::server::UnaryService<super::UpdateProductRequest>
                        for UpdateProductSvc<T>
                    {
                        type Response = super::Product;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_product(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateProductSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/DeleteProduct" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProductSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch> tonic::server::UnaryService<super::DeleteProductRequest>
                        for DeleteProductSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_product(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteProductSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/CreateReferenceImage" => {
                    #[allow(non_camel_case_types)]
                    struct CreateReferenceImageSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::CreateReferenceImageRequest>
                        for CreateReferenceImageSvc<T>
                    {
                        type Response = super::ReferenceImage;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateReferenceImageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_reference_image(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateReferenceImageSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/DeleteReferenceImage" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteReferenceImageSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::DeleteReferenceImageRequest>
                        for DeleteReferenceImageSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteReferenceImageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_reference_image(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteReferenceImageSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/ListReferenceImages" => {
                    #[allow(non_camel_case_types)]
                    struct ListReferenceImagesSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::ListReferenceImagesRequest>
                        for ListReferenceImagesSvc<T>
                    {
                        type Response = super::ListReferenceImagesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListReferenceImagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_reference_images(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListReferenceImagesSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/GetReferenceImage" => {
                    #[allow(non_camel_case_types)]
                    struct GetReferenceImageSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::GetReferenceImageRequest>
                        for GetReferenceImageSvc<T>
                    {
                        type Response = super::ReferenceImage;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReferenceImageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_reference_image(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetReferenceImageSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/AddProductToProductSet" => {
                    #[allow(non_camel_case_types)]
                    struct AddProductToProductSetSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::AddProductToProductSetRequest>
                        for AddProductToProductSetSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddProductToProductSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.add_product_to_product_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AddProductToProductSetSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/RemoveProductFromProductSet" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveProductFromProductSetSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::RemoveProductFromProductSetRequest>
                        for RemoveProductFromProductSetSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveProductFromProductSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.remove_product_from_product_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveProductFromProductSetSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/ListProductsInProductSet" => {
                    #[allow(non_camel_case_types)]
                    struct ListProductsInProductSetSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::ListProductsInProductSetRequest>
                        for ListProductsInProductSetSvc<T>
                    {
                        type Response = super::ListProductsInProductSetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProductsInProductSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.list_products_in_product_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListProductsInProductSetSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ProductSearch/ImportProductSets" => {
                    #[allow(non_camel_case_types)]
                    struct ImportProductSetsSvc<T: ProductSearch>(pub Arc<T>);
                    impl<T: ProductSearch>
                        tonic::server::UnaryService<super::ImportProductSetsRequest>
                        for ImportProductSetsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportProductSetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.import_product_sets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportProductSetsSvc(inner);
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
    impl<T: ProductSearch> Clone for ProductSearchServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ProductSearch> Clone for _Inner<T> {
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
/// Parameters for a product search request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductSearchParams {
    /// The resource name of the catalog to search.
    ///
    /// Format is: `productSearch/catalogs/CATALOG_NAME`.
    #[prost(string, tag = "1")]
    pub catalog_name: std::string::String,
    /// The category to search in.
    /// Optional. It is inferred by the system if it is not specified.
    /// [Deprecated] Use `product_category`.
    #[prost(enumeration = "ProductSearchCategory", tag = "2")]
    pub category: i32,
    /// The product category to search in.
    /// Optional. It is inferred by the system if it is not specified.
    /// Supported values are `bag`, `shoe`, `sunglasses`, `dress`, `outerwear`,
    /// `skirt`, `top`, `shorts`, and `pants`.
    #[prost(string, tag = "5")]
    pub product_category: std::string::String,
    /// The bounding polygon around the area of interest in the image.
    /// Optional. If it is not specified, system discretion will be applied.
    /// [Deprecated] Use `bounding_poly`.
    #[prost(message, optional, tag = "3")]
    pub normalized_bounding_poly: ::std::option::Option<NormalizedBoundingPoly>,
    /// The bounding polygon around the area of interest in the image.
    /// Optional. If it is not specified, system discretion will be applied.
    #[prost(message, optional, tag = "9")]
    pub bounding_poly: ::std::option::Option<BoundingPoly>,
    /// Specifies the verbosity of the  product search results.
    /// Optional. Defaults to `BASIC`.
    #[prost(enumeration = "ProductSearchResultsView", tag = "4")]
    pub view: i32,
    /// The resource name of a
    /// [ProductSet][google.cloud.vision.v1p3beta1.ProductSet] to be searched for
    /// similar images.
    ///
    /// Format is:
    /// `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`.
    #[prost(string, tag = "6")]
    pub product_set: std::string::String,
    /// The list of product categories to search in. Currently, we only consider
    /// the first category, and either "homegoods" or "apparel" should be
    /// specified.
    #[prost(string, repeated, tag = "7")]
    pub product_categories: ::std::vec::Vec<std::string::String>,
    /// The filtering expression. This can be used to restrict search results based
    /// on Product labels. We currently support an AND of OR of key-value
    /// expressions, where each expression within an OR must have the same key.
    ///
    /// For example, "(color = red OR color = blue) AND brand = Google" is
    /// acceptable, but not "(color = red OR brand = Google)" or "color: red".
    #[prost(string, tag = "8")]
    pub filter: std::string::String,
}
/// Results for a product search request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductSearchResults {
    /// Product category.
    /// [Deprecated] Use `product_category`.
    #[prost(enumeration = "ProductSearchCategory", tag = "1")]
    pub category: i32,
    /// Product category.
    /// Supported values are `bag` and `shoe`.
    /// [Deprecated] `product_category` is provided in each Product.
    #[prost(string, tag = "4")]
    pub product_category: std::string::String,
    /// Timestamp of the index which provided these results. Changes made after
    /// this time are not reflected in the current results.
    #[prost(message, optional, tag = "2")]
    pub index_time: ::std::option::Option<::prost_types::Timestamp>,
    /// List of detected products.
    #[prost(message, repeated, tag = "3")]
    pub products: ::std::vec::Vec<product_search_results::ProductInfo>,
    /// List of results, one for each product match.
    #[prost(message, repeated, tag = "5")]
    pub results: ::std::vec::Vec<product_search_results::Result>,
}
pub mod product_search_results {
    /// Information about a product.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProductInfo {
        /// Product ID.
        #[prost(string, tag = "1")]
        pub product_id: std::string::String,
        /// The URI of the image which matched the query image.
        ///
        /// This field is returned only if `view` is set to `FULL` in
        /// the request.
        #[prost(string, tag = "2")]
        pub image_uri: std::string::String,
        /// A confidence level on the match, ranging from 0 (no confidence) to
        /// 1 (full confidence).
        ///
        /// This field is returned only if `view` is set to `FULL` in
        /// the request.
        #[prost(float, tag = "3")]
        pub score: f32,
    }
    /// Information about a product.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        /// The Product.
        #[prost(message, optional, tag = "1")]
        pub product: ::std::option::Option<super::Product>,
        /// A confidence level on the match, ranging from 0 (no confidence) to
        /// 1 (full confidence).
        ///
        /// This field is returned only if `view` is set to `FULL` in
        /// the request.
        #[prost(float, tag = "2")]
        pub score: f32,
        /// The resource name of the image from the product that is the closest match
        /// to the query.
        #[prost(string, tag = "3")]
        pub image: std::string::String,
    }
}
/// Supported product search categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProductSearchCategory {
    /// Default value used when a category is not specified.
    Unspecified = 0,
    /// Shoes category.
    Shoes = 1,
    /// Bags category.
    Bags = 2,
}
/// Specifies the fields to include in product search results.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProductSearchResultsView {
    /// Product search results contain only `product_category` and `product_id`.
    /// Default value.
    Basic = 0,
    /// Product search results contain `product_category`, `product_id`,
    /// `image_uri`, and `score`.
    Full = 1,
}
/// TextAnnotation contains a structured representation of OCR extracted text.
/// The hierarchy of an OCR extracted text structure is like this:
///     TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol
/// Each structural component, starting from Page, may further have their own
/// properties. Properties describe detected languages, breaks etc.. Please refer
/// to the
/// [TextAnnotation.TextProperty][google.cloud.vision.v1p3beta1.TextAnnotation.TextProperty]
/// message definition below for more detail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAnnotation {
    /// List of pages detected by OCR.
    #[prost(message, repeated, tag = "1")]
    pub pages: ::std::vec::Vec<Page>,
    /// UTF-8 text detected on the pages.
    #[prost(string, tag = "2")]
    pub text: std::string::String,
}
pub mod text_annotation {
    /// Detected language for a structural component.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectedLanguage {
        /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
        /// information, see
        /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
        #[prost(string, tag = "1")]
        pub language_code: std::string::String,
        /// Confidence of detected language. Range [0, 1].
        #[prost(float, tag = "2")]
        pub confidence: f32,
    }
    /// Detected start or end of a structural component.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectedBreak {
        /// Detected break type.
        #[prost(enumeration = "detected_break::BreakType", tag = "1")]
        pub r#type: i32,
        /// True if break prepends the element.
        #[prost(bool, tag = "2")]
        pub is_prefix: bool,
    }
    pub mod detected_break {
        /// Enum to denote the type of break found. New line, space etc.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum BreakType {
            /// Unknown break label type.
            Unknown = 0,
            /// Regular space.
            Space = 1,
            /// Sure space (very wide).
            SureSpace = 2,
            /// Line-wrapping break.
            EolSureSpace = 3,
            /// End-line hyphen that is not present in text; does not co-occur with
            /// `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`.
            Hyphen = 4,
            /// Line break that ends a paragraph.
            LineBreak = 5,
        }
    }
    /// Additional information detected on the structural component.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextProperty {
        /// A list of detected languages together with confidence.
        #[prost(message, repeated, tag = "1")]
        pub detected_languages: ::std::vec::Vec<DetectedLanguage>,
        /// Detected start or end of a text segment.
        #[prost(message, optional, tag = "2")]
        pub detected_break: ::std::option::Option<DetectedBreak>,
    }
}
/// Detected page from OCR.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// Additional information detected on the page.
    #[prost(message, optional, tag = "1")]
    pub property: ::std::option::Option<text_annotation::TextProperty>,
    /// Page width. For PDFs the unit is points. For images (including
    /// TIFFs) the unit is pixels.
    #[prost(int32, tag = "2")]
    pub width: i32,
    /// Page height. For PDFs the unit is points. For images (including
    /// TIFFs) the unit is pixels.
    #[prost(int32, tag = "3")]
    pub height: i32,
    /// List of blocks of text, images etc on this page.
    #[prost(message, repeated, tag = "4")]
    pub blocks: ::std::vec::Vec<Block>,
    /// Confidence of the OCR results on the page. Range [0, 1].
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
/// Logical element on the page.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Additional information detected for the block.
    #[prost(message, optional, tag = "1")]
    pub property: ::std::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the block.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///
    /// * when the text is horizontal it might look like:
    ///
    ///         0----1
    ///         |    |
    ///         3----2
    ///
    /// * when it's rotated 180 degrees around the top-left corner it becomes:
    ///
    ///         2----3
    ///         |    |
    ///         1----0
    ///
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::std::option::Option<BoundingPoly>,
    /// List of paragraphs in this block (if this blocks is of type text).
    #[prost(message, repeated, tag = "3")]
    pub paragraphs: ::std::vec::Vec<Paragraph>,
    /// Detected block type (text, image etc) for this block.
    #[prost(enumeration = "block::BlockType", tag = "4")]
    pub block_type: i32,
    /// Confidence of the OCR results on the block. Range [0, 1].
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
pub mod block {
    /// Type of a block (text, image etc) as identified by OCR.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BlockType {
        /// Unknown block type.
        Unknown = 0,
        /// Regular text block.
        Text = 1,
        /// Table block.
        Table = 2,
        /// Image block.
        Picture = 3,
        /// Horizontal/vertical line box.
        Ruler = 4,
        /// Barcode block.
        Barcode = 5,
    }
}
/// Structural unit of text representing a number of words in certain order.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paragraph {
    /// Additional information detected for the paragraph.
    #[prost(message, optional, tag = "1")]
    pub property: ::std::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the paragraph.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::std::option::Option<BoundingPoly>,
    /// List of words in this paragraph.
    #[prost(message, repeated, tag = "3")]
    pub words: ::std::vec::Vec<Word>,
    /// Confidence of the OCR results for the paragraph. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// A word representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Word {
    /// Additional information detected for the word.
    #[prost(message, optional, tag = "1")]
    pub property: ::std::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the word.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::std::option::Option<BoundingPoly>,
    /// List of symbols in the word.
    /// The order of the symbols follows the natural reading order.
    #[prost(message, repeated, tag = "3")]
    pub symbols: ::std::vec::Vec<Symbol>,
    /// Confidence of the OCR results for the word. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// A single symbol representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symbol {
    /// Additional information detected for the symbol.
    #[prost(message, optional, tag = "1")]
    pub property: ::std::option::Option<text_annotation::TextProperty>,
    /// The bounding box for the symbol.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::std::option::Option<BoundingPoly>,
    /// The actual UTF-8 representation of the symbol.
    #[prost(string, tag = "3")]
    pub text: std::string::String,
    /// Confidence of the OCR results for the symbol. Range [0, 1].
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// Relevant information for the image from the Internet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDetection {
    /// Deduced entities from similar images on the Internet.
    #[prost(message, repeated, tag = "1")]
    pub web_entities: ::std::vec::Vec<web_detection::WebEntity>,
    /// Fully matching images from the Internet.
    /// Can include resized copies of the query image.
    #[prost(message, repeated, tag = "2")]
    pub full_matching_images: ::std::vec::Vec<web_detection::WebImage>,
    /// Partial matching images from the Internet.
    /// Those images are similar enough to share some key-point features. For
    /// example an original image will likely have partial matching for its crops.
    #[prost(message, repeated, tag = "3")]
    pub partial_matching_images: ::std::vec::Vec<web_detection::WebImage>,
    /// Web pages containing the matching images from the Internet.
    #[prost(message, repeated, tag = "4")]
    pub pages_with_matching_images: ::std::vec::Vec<web_detection::WebPage>,
    /// The visually similar image results.
    #[prost(message, repeated, tag = "6")]
    pub visually_similar_images: ::std::vec::Vec<web_detection::WebImage>,
    /// Best guess text labels for the request image.
    #[prost(message, repeated, tag = "8")]
    pub best_guess_labels: ::std::vec::Vec<web_detection::WebLabel>,
}
pub mod web_detection {
    /// Entity deduced from similar images on the Internet.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebEntity {
        /// Opaque entity ID.
        #[prost(string, tag = "1")]
        pub entity_id: std::string::String,
        /// Overall relevancy score for the entity.
        /// Not normalized and not comparable across different image queries.
        #[prost(float, tag = "2")]
        pub score: f32,
        /// Canonical description of the entity, in English.
        #[prost(string, tag = "3")]
        pub description: std::string::String,
    }
    /// Metadata for online images.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebImage {
        /// The result image URL.
        #[prost(string, tag = "1")]
        pub url: std::string::String,
        /// (Deprecated) Overall relevancy score for the image.
        #[prost(float, tag = "2")]
        pub score: f32,
    }
    /// Metadata for web pages.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebPage {
        /// The result web page URL.
        #[prost(string, tag = "1")]
        pub url: std::string::String,
        /// (Deprecated) Overall relevancy score for the web page.
        #[prost(float, tag = "2")]
        pub score: f32,
        /// Title for the web page, may contain HTML markups.
        #[prost(string, tag = "3")]
        pub page_title: std::string::String,
        /// Fully matching images on the page.
        /// Can include resized copies of the query image.
        #[prost(message, repeated, tag = "4")]
        pub full_matching_images: ::std::vec::Vec<WebImage>,
        /// Partial matching images on the page.
        /// Those images are similar enough to share some key-point features. For
        /// example an original image will likely have partial matching for its
        /// crops.
        #[prost(message, repeated, tag = "5")]
        pub partial_matching_images: ::std::vec::Vec<WebImage>,
    }
    /// Label to provide extra metadata for the web detection.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebLabel {
        /// Label for extra metadata.
        #[prost(string, tag = "1")]
        pub label: std::string::String,
        /// The BCP-47 language code for `label`, such as "en-US" or "sr-Latn".
        /// For more information, see
        /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
        #[prost(string, tag = "2")]
        pub language_code: std::string::String,
    }
}
/// The type of Google Cloud Vision API detection to perform, and the maximum
/// number of results to return for that type. Multiple `Feature` objects can
/// be specified in the `features` list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    /// The feature type.
    #[prost(enumeration = "feature::Type", tag = "1")]
    pub r#type: i32,
    /// Maximum number of results of this type. Does not apply to
    /// `TEXT_DETECTION`, `DOCUMENT_TEXT_DETECTION`, or `CROP_HINTS`.
    #[prost(int32, tag = "2")]
    pub max_results: i32,
    /// Model to use for the feature.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "3")]
    pub model: std::string::String,
}
pub mod feature {
    /// Type of Google Cloud Vision API feature to be extracted.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified feature type.
        Unspecified = 0,
        /// Run face detection.
        FaceDetection = 1,
        /// Run landmark detection.
        LandmarkDetection = 2,
        /// Run logo detection.
        LogoDetection = 3,
        /// Run label detection.
        LabelDetection = 4,
        /// Run text detection / optical character recognition (OCR). Text detection
        /// is optimized for areas of text within a larger image; if the image is
        /// a document, use `DOCUMENT_TEXT_DETECTION` instead.
        TextDetection = 5,
        /// Run dense text document OCR. Takes precedence when both
        /// `DOCUMENT_TEXT_DETECTION` and `TEXT_DETECTION` are present.
        DocumentTextDetection = 11,
        /// Run Safe Search to detect potentially unsafe
        /// or undesirable content.
        SafeSearchDetection = 6,
        /// Compute a set of image properties, such as the
        /// image's dominant colors.
        ImageProperties = 7,
        /// Run crop hints.
        CropHints = 9,
        /// Run web detection.
        WebDetection = 10,
        /// Run Product Search.
        ProductSearch = 12,
        /// Run localizer for object detection.
        ObjectLocalization = 19,
    }
}
/// External image source (Google Cloud Storage or web URL image location).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSource {
    /// **Use `image_uri` instead.**
    ///
    /// The Google Cloud Storage  URI of the form
    /// `gs://bucket_name/object_name`. Object versioning is not supported. See
    /// [Google Cloud Storage Request
    /// URIs](https://cloud.google.com/storage/docs/reference-uris) for more info.
    #[prost(string, tag = "1")]
    pub gcs_image_uri: std::string::String,
    /// The URI of the source image. Can be either:
    ///
    /// 1. A Google Cloud Storage URI of the form
    ///    `gs://bucket_name/object_name`. Object versioning is not supported. See
    ///    [Google Cloud Storage Request
    ///    URIs](https://cloud.google.com/storage/docs/reference-uris) for more
    ///    info.
    ///
    /// 2. A publicly-accessible image HTTP/HTTPS URL. When fetching images from
    ///    HTTP/HTTPS URLs, Google cannot guarantee that the request will be
    ///    completed. Your request may fail if the specified host denies the
    ///    request (e.g. due to request throttling or DOS prevention), or if Google
    ///    throttles requests to the site for abuse prevention. You should not
    ///    depend on externally-hosted images for production applications.
    ///
    /// When both `gcs_image_uri` and `image_uri` are specified, `image_uri` takes
    /// precedence.
    #[prost(string, tag = "2")]
    pub image_uri: std::string::String,
}
/// Client image to perform Google Cloud Vision API tasks over.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Image content, represented as a stream of bytes.
    /// Note: As with all `bytes` fields, protobuffers use a pure binary
    /// representation, whereas JSON representations use base64.
    #[prost(bytes, tag = "1")]
    pub content: std::vec::Vec<u8>,
    /// Google Cloud Storage image location, or publicly-accessible image
    /// URL. If both `content` and `source` are provided for an image, `content`
    /// takes precedence and is used to perform the image annotation request.
    #[prost(message, optional, tag = "2")]
    pub source: ::std::option::Option<ImageSource>,
}
/// A face annotation object contains the results of face detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceAnnotation {
    /// The bounding polygon around the face. The coordinates of the bounding box
    /// are in the original image's scale, as returned in `ImageParams`.
    /// The bounding box is computed to "frame" the face in accordance with human
    /// expectations. It is based on the landmarker results.
    /// Note that one or more x and/or y coordinates may not be generated in the
    /// `BoundingPoly` (the polygon will be unbounded) if only a partial face
    /// appears in the image to be annotated.
    #[prost(message, optional, tag = "1")]
    pub bounding_poly: ::std::option::Option<BoundingPoly>,
    /// The `fd_bounding_poly` bounding polygon is tighter than the
    /// `boundingPoly`, and encloses only the skin part of the face. Typically, it
    /// is used to eliminate the face from any image analysis that detects the
    /// "amount of skin" visible in an image. It is not based on the
    /// landmarker results, only on the initial face detection, hence
    /// the <code>fd</code> (face detection) prefix.
    #[prost(message, optional, tag = "2")]
    pub fd_bounding_poly: ::std::option::Option<BoundingPoly>,
    /// Detected face landmarks.
    #[prost(message, repeated, tag = "3")]
    pub landmarks: ::std::vec::Vec<face_annotation::Landmark>,
    /// Roll angle, which indicates the amount of clockwise/anti-clockwise rotation
    /// of the face relative to the image vertical about the axis perpendicular to
    /// the face. Range [-180,180].
    #[prost(float, tag = "4")]
    pub roll_angle: f32,
    /// Yaw angle, which indicates the leftward/rightward angle that the face is
    /// pointing relative to the vertical plane perpendicular to the image. Range
    /// [-180,180].
    #[prost(float, tag = "5")]
    pub pan_angle: f32,
    /// Pitch angle, which indicates the upwards/downwards angle that the face is
    /// pointing relative to the image's horizontal plane. Range [-180,180].
    #[prost(float, tag = "6")]
    pub tilt_angle: f32,
    /// Detection confidence. Range [0, 1].
    #[prost(float, tag = "7")]
    pub detection_confidence: f32,
    /// Face landmarking confidence. Range [0, 1].
    #[prost(float, tag = "8")]
    pub landmarking_confidence: f32,
    /// Joy likelihood.
    #[prost(enumeration = "Likelihood", tag = "9")]
    pub joy_likelihood: i32,
    /// Sorrow likelihood.
    #[prost(enumeration = "Likelihood", tag = "10")]
    pub sorrow_likelihood: i32,
    /// Anger likelihood.
    #[prost(enumeration = "Likelihood", tag = "11")]
    pub anger_likelihood: i32,
    /// Surprise likelihood.
    #[prost(enumeration = "Likelihood", tag = "12")]
    pub surprise_likelihood: i32,
    /// Under-exposed likelihood.
    #[prost(enumeration = "Likelihood", tag = "13")]
    pub under_exposed_likelihood: i32,
    /// Blurred likelihood.
    #[prost(enumeration = "Likelihood", tag = "14")]
    pub blurred_likelihood: i32,
    /// Headwear likelihood.
    #[prost(enumeration = "Likelihood", tag = "15")]
    pub headwear_likelihood: i32,
}
pub mod face_annotation {
    /// A face-specific landmark (for example, a face feature).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Landmark {
        /// Face landmark type.
        #[prost(enumeration = "landmark::Type", tag = "3")]
        pub r#type: i32,
        /// Face landmark position.
        #[prost(message, optional, tag = "4")]
        pub position: ::std::option::Option<super::Position>,
    }
    pub mod landmark {
        /// Face landmark (feature) type.
        /// Left and right are defined from the vantage of the viewer of the image
        /// without considering mirror projections typical of photos. So, `LEFT_EYE`,
        /// typically, is the person's right eye.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// Unknown face landmark detected. Should not be filled.
            UnknownLandmark = 0,
            /// Left eye.
            LeftEye = 1,
            /// Right eye.
            RightEye = 2,
            /// Left of left eyebrow.
            LeftOfLeftEyebrow = 3,
            /// Right of left eyebrow.
            RightOfLeftEyebrow = 4,
            /// Left of right eyebrow.
            LeftOfRightEyebrow = 5,
            /// Right of right eyebrow.
            RightOfRightEyebrow = 6,
            /// Midpoint between eyes.
            MidpointBetweenEyes = 7,
            /// Nose tip.
            NoseTip = 8,
            /// Upper lip.
            UpperLip = 9,
            /// Lower lip.
            LowerLip = 10,
            /// Mouth left.
            MouthLeft = 11,
            /// Mouth right.
            MouthRight = 12,
            /// Mouth center.
            MouthCenter = 13,
            /// Nose, bottom right.
            NoseBottomRight = 14,
            /// Nose, bottom left.
            NoseBottomLeft = 15,
            /// Nose, bottom center.
            NoseBottomCenter = 16,
            /// Left eye, top boundary.
            LeftEyeTopBoundary = 17,
            /// Left eye, right corner.
            LeftEyeRightCorner = 18,
            /// Left eye, bottom boundary.
            LeftEyeBottomBoundary = 19,
            /// Left eye, left corner.
            LeftEyeLeftCorner = 20,
            /// Right eye, top boundary.
            RightEyeTopBoundary = 21,
            /// Right eye, right corner.
            RightEyeRightCorner = 22,
            /// Right eye, bottom boundary.
            RightEyeBottomBoundary = 23,
            /// Right eye, left corner.
            RightEyeLeftCorner = 24,
            /// Left eyebrow, upper midpoint.
            LeftEyebrowUpperMidpoint = 25,
            /// Right eyebrow, upper midpoint.
            RightEyebrowUpperMidpoint = 26,
            /// Left ear tragion.
            LeftEarTragion = 27,
            /// Right ear tragion.
            RightEarTragion = 28,
            /// Left eye pupil.
            LeftEyePupil = 29,
            /// Right eye pupil.
            RightEyePupil = 30,
            /// Forehead glabella.
            ForeheadGlabella = 31,
            /// Chin gnathion.
            ChinGnathion = 32,
            /// Chin left gonion.
            ChinLeftGonion = 33,
            /// Chin right gonion.
            ChinRightGonion = 34,
        }
    }
}
/// Detected entity location information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationInfo {
    /// lat/long location coordinates.
    #[prost(message, optional, tag = "1")]
    pub lat_lng: ::std::option::Option<super::super::super::r#type::LatLng>,
}
/// A `Property` consists of a user-supplied name/value pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Name of the property.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Value of the property.
    #[prost(string, tag = "2")]
    pub value: std::string::String,
    /// Value of numeric properties.
    #[prost(uint64, tag = "3")]
    pub uint64_value: u64,
}
/// Set of detected entity features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityAnnotation {
    /// Opaque entity ID. Some IDs may be available in
    /// [Google Knowledge Graph Search
    /// API](https://developers.google.com/knowledge-graph/).
    #[prost(string, tag = "1")]
    pub mid: std::string::String,
    /// The language code for the locale in which the entity textual
    /// `description` is expressed.
    #[prost(string, tag = "2")]
    pub locale: std::string::String,
    /// Entity textual description, expressed in its `locale` language.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// Overall score of the result. Range [0, 1].
    #[prost(float, tag = "4")]
    pub score: f32,
    /// **Deprecated. Use `score` instead.**
    /// The accuracy of the entity detection in an image.
    /// For example, for an image in which the "Eiffel Tower" entity is detected,
    /// this field represents the confidence that there is a tower in the query
    /// image. Range [0, 1].
    #[prost(float, tag = "5")]
    pub confidence: f32,
    /// The relevancy of the ICA (Image Content Annotation) label to the
    /// image. For example, the relevancy of "tower" is likely higher to an image
    /// containing the detected "Eiffel Tower" than to an image containing a
    /// detected distant towering building, even though the confidence that
    /// there is a tower in each image may be the same. Range [0, 1].
    #[prost(float, tag = "6")]
    pub topicality: f32,
    /// Image region to which this entity belongs. Not produced
    /// for `LABEL_DETECTION` features.
    #[prost(message, optional, tag = "7")]
    pub bounding_poly: ::std::option::Option<BoundingPoly>,
    /// The location information for the detected entity. Multiple
    /// `LocationInfo` elements can be present because one location may
    /// indicate the location of the scene in the image, and another location
    /// may indicate the location of the place where the image was taken.
    /// Location information is usually present for landmarks.
    #[prost(message, repeated, tag = "8")]
    pub locations: ::std::vec::Vec<LocationInfo>,
    /// Some entities may have optional user-supplied `Property` (name/value)
    /// fields, such a score or string that qualifies the entity.
    #[prost(message, repeated, tag = "9")]
    pub properties: ::std::vec::Vec<Property>,
}
/// Set of detected objects with bounding boxes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalizedObjectAnnotation {
    /// Object ID that should align with EntityAnnotation mid.
    #[prost(string, tag = "1")]
    pub mid: std::string::String,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// Object name, expressed in its `language_code` language.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Score of the result. Range [0, 1].
    #[prost(float, tag = "4")]
    pub score: f32,
    /// Image region to which this object belongs. This must be populated.
    #[prost(message, optional, tag = "5")]
    pub bounding_poly: ::std::option::Option<BoundingPoly>,
}
/// Set of features pertaining to the image, computed by computer vision
/// methods over safe-search verticals (for example, adult, spoof, medical,
/// violence).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeSearchAnnotation {
    /// Represents the adult content likelihood for the image. Adult content may
    /// contain elements such as nudity, pornographic images or cartoons, or
    /// sexual activities.
    #[prost(enumeration = "Likelihood", tag = "1")]
    pub adult: i32,
    /// Spoof likelihood. The likelihood that an modification
    /// was made to the image's canonical version to make it appear
    /// funny or offensive.
    #[prost(enumeration = "Likelihood", tag = "2")]
    pub spoof: i32,
    /// Likelihood that this is a medical image.
    #[prost(enumeration = "Likelihood", tag = "3")]
    pub medical: i32,
    /// Likelihood that this image contains violent content.
    #[prost(enumeration = "Likelihood", tag = "4")]
    pub violence: i32,
    /// Likelihood that the request image contains racy content. Racy content may
    /// include (but is not limited to) skimpy or sheer clothing, strategically
    /// covered nudity, lewd or provocative poses, or close-ups of sensitive
    /// body areas.
    #[prost(enumeration = "Likelihood", tag = "9")]
    pub racy: i32,
}
/// Rectangle determined by min and max `LatLng` pairs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLongRect {
    /// Min lat/long pair.
    #[prost(message, optional, tag = "1")]
    pub min_lat_lng: ::std::option::Option<super::super::super::r#type::LatLng>,
    /// Max lat/long pair.
    #[prost(message, optional, tag = "2")]
    pub max_lat_lng: ::std::option::Option<super::super::super::r#type::LatLng>,
}
/// Color information consists of RGB channels, score, and the fraction of
/// the image that the color occupies in the image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorInfo {
    /// RGB components of the color.
    #[prost(message, optional, tag = "1")]
    pub color: ::std::option::Option<super::super::super::r#type::Color>,
    /// Image-specific score for this color. Value in range [0, 1].
    #[prost(float, tag = "2")]
    pub score: f32,
    /// The fraction of pixels the color occupies in the image.
    /// Value in range [0, 1].
    #[prost(float, tag = "3")]
    pub pixel_fraction: f32,
}
/// Set of dominant colors and their corresponding scores.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DominantColorsAnnotation {
    /// RGB color values with their score and pixel fraction.
    #[prost(message, repeated, tag = "1")]
    pub colors: ::std::vec::Vec<ColorInfo>,
}
/// Stores image properties, such as dominant colors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageProperties {
    /// If present, dominant colors completed successfully.
    #[prost(message, optional, tag = "1")]
    pub dominant_colors: ::std::option::Option<DominantColorsAnnotation>,
}
/// Single crop hint that is used to generate a new crop when serving an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHint {
    /// The bounding polygon for the crop region. The coordinates of the bounding
    /// box are in the original image's scale, as returned in `ImageParams`.
    #[prost(message, optional, tag = "1")]
    pub bounding_poly: ::std::option::Option<BoundingPoly>,
    /// Confidence of this being a salient region.  Range [0, 1].
    #[prost(float, tag = "2")]
    pub confidence: f32,
    /// Fraction of importance of this salient region with respect to the original
    /// image.
    #[prost(float, tag = "3")]
    pub importance_fraction: f32,
}
/// Set of crop hints that are used to generate new crops when serving images.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHintsAnnotation {
    /// Crop hint results.
    #[prost(message, repeated, tag = "1")]
    pub crop_hints: ::std::vec::Vec<CropHint>,
}
/// Parameters for crop hints annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropHintsParams {
    /// Aspect ratios in floats, representing the ratio of the width to the height
    /// of the image. For example, if the desired aspect ratio is 4/3, the
    /// corresponding float value should be 1.33333.  If not specified, the
    /// best possible crop is returned. The number of provided aspect ratios is
    /// limited to a maximum of 16; any aspect ratios provided after the 16th are
    /// ignored.
    #[prost(float, repeated, tag = "1")]
    pub aspect_ratios: ::std::vec::Vec<f32>,
}
/// Parameters for web detection request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebDetectionParams {
    /// Whether to include results derived from the geo information in the image.
    #[prost(bool, tag = "2")]
    pub include_geo_results: bool,
}
/// Image context and/or feature-specific parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageContext {
    /// Not used.
    #[prost(message, optional, tag = "1")]
    pub lat_long_rect: ::std::option::Option<LatLongRect>,
    /// List of languages to use for TEXT_DETECTION. In most cases, an empty value
    /// yields the best results since it enables automatic language detection. For
    /// languages based on the Latin alphabet, setting `language_hints` is not
    /// needed. In rare cases, when the language of the text in the image is known,
    /// setting a hint will help get better results (although it will be a
    /// significant hindrance if the hint is wrong). Text detection returns an
    /// error if one or more of the specified languages is not one of the
    /// [supported languages](https://cloud.google.com/vision/docs/languages).
    #[prost(string, repeated, tag = "2")]
    pub language_hints: ::std::vec::Vec<std::string::String>,
    /// Parameters for crop hints annotation request.
    #[prost(message, optional, tag = "4")]
    pub crop_hints_params: ::std::option::Option<CropHintsParams>,
    /// Parameters for product search.
    #[prost(message, optional, tag = "5")]
    pub product_search_params: ::std::option::Option<ProductSearchParams>,
    /// Parameters for web detection.
    #[prost(message, optional, tag = "6")]
    pub web_detection_params: ::std::option::Option<WebDetectionParams>,
}
/// Request for performing Google Cloud Vision API tasks over a user-provided
/// image, with user-requested features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateImageRequest {
    /// The image to be processed.
    #[prost(message, optional, tag = "1")]
    pub image: ::std::option::Option<Image>,
    /// Requested features.
    #[prost(message, repeated, tag = "2")]
    pub features: ::std::vec::Vec<Feature>,
    /// Additional context that may accompany the image.
    #[prost(message, optional, tag = "3")]
    pub image_context: ::std::option::Option<ImageContext>,
}
/// If an image was produced from a file (e.g. a PDF), this message gives
/// information about the source of that image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageAnnotationContext {
    /// The URI of the file used to produce the image.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
    /// If the file was a PDF or TIFF, this field gives the page number within
    /// the file used to produce the image.
    #[prost(int32, tag = "2")]
    pub page_number: i32,
}
/// Response to an image annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateImageResponse {
    /// If present, face detection has completed successfully.
    #[prost(message, repeated, tag = "1")]
    pub face_annotations: ::std::vec::Vec<FaceAnnotation>,
    /// If present, landmark detection has completed successfully.
    #[prost(message, repeated, tag = "2")]
    pub landmark_annotations: ::std::vec::Vec<EntityAnnotation>,
    /// If present, logo detection has completed successfully.
    #[prost(message, repeated, tag = "3")]
    pub logo_annotations: ::std::vec::Vec<EntityAnnotation>,
    /// If present, label detection has completed successfully.
    #[prost(message, repeated, tag = "4")]
    pub label_annotations: ::std::vec::Vec<EntityAnnotation>,
    /// If present, localized object detection has completed successfully.
    /// This will be sorted descending by confidence score.
    #[prost(message, repeated, tag = "22")]
    pub localized_object_annotations: ::std::vec::Vec<LocalizedObjectAnnotation>,
    /// If present, text (OCR) detection has completed successfully.
    #[prost(message, repeated, tag = "5")]
    pub text_annotations: ::std::vec::Vec<EntityAnnotation>,
    /// If present, text (OCR) detection or document (OCR) text detection has
    /// completed successfully.
    /// This annotation provides the structural hierarchy for the OCR detected
    /// text.
    #[prost(message, optional, tag = "12")]
    pub full_text_annotation: ::std::option::Option<TextAnnotation>,
    /// If present, safe-search annotation has completed successfully.
    #[prost(message, optional, tag = "6")]
    pub safe_search_annotation: ::std::option::Option<SafeSearchAnnotation>,
    /// If present, image properties were extracted successfully.
    #[prost(message, optional, tag = "8")]
    pub image_properties_annotation: ::std::option::Option<ImageProperties>,
    /// If present, crop hints have completed successfully.
    #[prost(message, optional, tag = "11")]
    pub crop_hints_annotation: ::std::option::Option<CropHintsAnnotation>,
    /// If present, web detection has completed successfully.
    #[prost(message, optional, tag = "13")]
    pub web_detection: ::std::option::Option<WebDetection>,
    /// If present, product search has completed successfully.
    #[prost(message, optional, tag = "14")]
    pub product_search_results: ::std::option::Option<ProductSearchResults>,
    /// If set, represents the error message for the operation.
    /// Note that filled-in image annotations are guaranteed to be
    /// correct, even when `error` is set.
    #[prost(message, optional, tag = "9")]
    pub error: ::std::option::Option<super::super::super::rpc::Status>,
    /// If present, contextual information is needed to understand where this image
    /// comes from.
    #[prost(message, optional, tag = "21")]
    pub context: ::std::option::Option<ImageAnnotationContext>,
}
/// Response to a single file annotation request. A file may contain one or more
/// images, which individually have their own responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateFileResponse {
    /// Information about the file for which this response is generated.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::std::option::Option<InputConfig>,
    /// Individual responses to images found within the file.
    #[prost(message, repeated, tag = "2")]
    pub responses: ::std::vec::Vec<AnnotateImageResponse>,
}
/// Multiple image annotation requests are batched into a single service call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAnnotateImagesRequest {
    /// Individual image annotation requests for this batch.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::std::vec::Vec<AnnotateImageRequest>,
}
/// Response to a batch image annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAnnotateImagesResponse {
    /// Individual responses to image annotation requests within the batch.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::std::vec::Vec<AnnotateImageResponse>,
}
/// An offline file annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncAnnotateFileRequest {
    /// Required. Information about the input file.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::std::option::Option<InputConfig>,
    /// Required. Requested features.
    #[prost(message, repeated, tag = "2")]
    pub features: ::std::vec::Vec<Feature>,
    /// Additional context that may accompany the image(s) in the file.
    #[prost(message, optional, tag = "3")]
    pub image_context: ::std::option::Option<ImageContext>,
    /// Required. The desired output location and metadata (e.g. format).
    #[prost(message, optional, tag = "4")]
    pub output_config: ::std::option::Option<OutputConfig>,
}
/// The response for a single offline file annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncAnnotateFileResponse {
    /// The output location and metadata from AsyncAnnotateFileRequest.
    #[prost(message, optional, tag = "1")]
    pub output_config: ::std::option::Option<OutputConfig>,
}
/// Multiple async file annotation requests are batched into a single service
/// call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncBatchAnnotateFilesRequest {
    /// Required. Individual async file annotation requests for this batch.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::std::vec::Vec<AsyncAnnotateFileRequest>,
}
/// Response to an async batch file annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncBatchAnnotateFilesResponse {
    /// The list of file annotation responses, one for each request in
    /// AsyncBatchAnnotateFilesRequest.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::std::vec::Vec<AsyncAnnotateFileResponse>,
}
/// The desired input location and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// The Google Cloud Storage location to read the input from.
    #[prost(message, optional, tag = "1")]
    pub gcs_source: ::std::option::Option<GcsSource>,
    /// The type of the file. Currently only "application/pdf" and "image/tiff"
    /// are supported. Wildcards are not supported.
    #[prost(string, tag = "2")]
    pub mime_type: std::string::String,
}
/// The desired output location and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// The Google Cloud Storage location to write the output(s) to.
    #[prost(message, optional, tag = "1")]
    pub gcs_destination: ::std::option::Option<GcsDestination>,
    /// The max number of response protos to put into each output JSON file on
    /// Google Cloud Storage.
    /// The valid range is [1, 100]. If not specified, the default value is 20.
    ///
    /// For example, for one pdf file with 100 pages, 100 response protos will
    /// be generated. If `batch_size` = 20, then 5 json files each
    /// containing 20 response protos will be written under the prefix
    /// `gcs_destination`.`uri`.
    ///
    /// Currently, batch_size only applies to GcsDestination, with potential future
    /// support for other output configurations.
    #[prost(int32, tag = "2")]
    pub batch_size: i32,
}
/// The Google Cloud Storage location where the input will be read from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Google Cloud Storage URI for the input file. This must only be a
    /// Google Cloud Storage object. Wildcards are not currently supported.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
}
/// The Google Cloud Storage location where the output will be written to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Google Cloud Storage URI where the results will be stored. Results will
    /// be in JSON format and preceded by its corresponding input URI. This field
    /// can either represent a single file, or a prefix for multiple outputs.
    /// Prefixes must end in a `/`.
    ///
    /// Examples:
    ///
    /// *    File: gs://bucket-name/filename.json
    /// *    Prefix: gs://bucket-name/prefix/here/
    /// *    File: gs://bucket-name/prefix/here
    ///
    /// If multiple outputs, each response is still AnnotateFileResponse, each of
    /// which contains some subset of the full list of AnnotateImageResponse.
    /// Multiple outputs can happen if, for example, the output JSON is too large
    /// and overflows into multiple sharded files.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
}
/// Contains metadata for the BatchAnnotateImages operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Current state of the batch operation.
    #[prost(enumeration = "operation_metadata::State", tag = "1")]
    pub state: i32,
    /// The time when the batch request was received.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time when the operation result was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
}
pub mod operation_metadata {
    /// Batch operation states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid.
        Unspecified = 0,
        /// Request is received.
        Created = 1,
        /// Request is actively being processed.
        Running = 2,
        /// The batch processing is done.
        Done = 3,
        /// The batch processing was cancelled.
        Cancelled = 4,
    }
}
/// A bucketized representation of likelihood, which is intended to give clients
/// highly stable results across model upgrades.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Likelihood {
    /// Unknown likelihood.
    Unknown = 0,
    /// It is very unlikely that the image belongs to the specified vertical.
    VeryUnlikely = 1,
    /// It is unlikely that the image belongs to the specified vertical.
    Unlikely = 2,
    /// It is possible that the image belongs to the specified vertical.
    Possible = 3,
    /// It is likely that the image belongs to the specified vertical.
    Likely = 4,
    /// It is very likely that the image belongs to the specified vertical.
    VeryLikely = 5,
}
#[doc = r" Generated client implementations."]
pub mod image_annotator_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service that performs Google Cloud Vision API detection tasks over client"]
    #[doc = " images, such as face, landmark, logo, label, and text detection. The"]
    #[doc = " ImageAnnotator service returns detected entities from the images."]
    pub struct ImageAnnotatorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ImageAnnotatorClient<T>
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
        #[doc = " Run image detection and annotation for a batch of images."]
        pub async fn batch_annotate_images(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchAnnotateImagesRequest>,
        ) -> Result<tonic::Response<super::BatchAnnotateImagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vision.v1p3beta1.ImageAnnotator/BatchAnnotateImages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Run asynchronous image detection and annotation for a list of generic"]
        #[doc = " files, such as PDF files, which may contain multiple pages and multiple"]
        #[doc = " images per page. Progress and results can be retrieved through the"]
        #[doc = " `google.longrunning.Operations` interface."]
        #[doc = " `Operation.metadata` contains `OperationMetadata` (metadata)."]
        #[doc = " `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results)."]
        pub async fn async_batch_annotate_files(
            &mut self,
            request: impl tonic::IntoRequest<super::AsyncBatchAnnotateFilesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.vision.v1p3beta1.ImageAnnotator/AsyncBatchAnnotateFiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ImageAnnotatorClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ImageAnnotatorClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ImageAnnotatorClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod image_annotator_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ImageAnnotatorServer."]
    #[async_trait]
    pub trait ImageAnnotator: Send + Sync + 'static {
        #[doc = " Run image detection and annotation for a batch of images."]
        async fn batch_annotate_images(
            &self,
            request: tonic::Request<super::BatchAnnotateImagesRequest>,
        ) -> Result<tonic::Response<super::BatchAnnotateImagesResponse>, tonic::Status>;
        #[doc = " Run asynchronous image detection and annotation for a list of generic"]
        #[doc = " files, such as PDF files, which may contain multiple pages and multiple"]
        #[doc = " images per page. Progress and results can be retrieved through the"]
        #[doc = " `google.longrunning.Operations` interface."]
        #[doc = " `Operation.metadata` contains `OperationMetadata` (metadata)."]
        #[doc = " `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results)."]
        async fn async_batch_annotate_files(
            &self,
            request: tonic::Request<super::AsyncBatchAnnotateFilesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Service that performs Google Cloud Vision API detection tasks over client"]
    #[doc = " images, such as face, landmark, logo, label, and text detection. The"]
    #[doc = " ImageAnnotator service returns detected entities from the images."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ImageAnnotatorServer<T: ImageAnnotator> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ImageAnnotator> ImageAnnotatorServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ImageAnnotatorServer<T>
    where
        T: ImageAnnotator,
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
                "/google.cloud.vision.v1p3beta1.ImageAnnotator/BatchAnnotateImages" => {
                    #[allow(non_camel_case_types)]
                    struct BatchAnnotateImagesSvc<T: ImageAnnotator>(pub Arc<T>);
                    impl<T: ImageAnnotator>
                        tonic::server::UnaryService<super::BatchAnnotateImagesRequest>
                        for BatchAnnotateImagesSvc<T>
                    {
                        type Response = super::BatchAnnotateImagesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchAnnotateImagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_annotate_images(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchAnnotateImagesSvc(inner);
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
                "/google.cloud.vision.v1p3beta1.ImageAnnotator/AsyncBatchAnnotateFiles" => {
                    #[allow(non_camel_case_types)]
                    struct AsyncBatchAnnotateFilesSvc<T: ImageAnnotator>(pub Arc<T>);
                    impl<T: ImageAnnotator>
                        tonic::server::UnaryService<super::AsyncBatchAnnotateFilesRequest>
                        for AsyncBatchAnnotateFilesSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AsyncBatchAnnotateFilesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.async_batch_annotate_files(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AsyncBatchAnnotateFilesSvc(inner);
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
    impl<T: ImageAnnotator> Clone for ImageAnnotatorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ImageAnnotator> Clone for _Inner<T> {
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
