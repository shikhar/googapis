/// This enum describes all the possible systems that Data Catalog integrates
/// with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntegratedSystem {
    /// Default unknown system.
    Unspecified = 0,
    /// BigQuery.
    Bigquery = 1,
    /// Cloud Pub/Sub.
    CloudPubsub = 2,
    /// Dataproc Metastore.
    DataprocMetastore = 3,
}
/// Describes the physical location of an entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSource {
    /// Service in which the data is physically stored.
    #[prost(enumeration = "data_source::Service", tag = "1")]
    pub service: i32,
    /// Full name of the resource as defined by the service, e.g.
    /// //bigquery.googleapis.com/projects/{project_id}/locations/{location}/datasets/{dataset_id}/tables/{table_id}
    #[prost(string, tag = "2")]
    pub resource: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DataSource`.
pub mod data_source {
    /// Service name where the data is stored.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Service {
        /// Default unknown service.
        Unspecified = 0,
        /// Google Cloud Storage service.
        CloudStorage = 1,
        /// BigQuery service.
        Bigquery = 2,
    }
}
/// Timestamps about this resource according to a particular system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemTimestamps {
    /// The creation time of the resource within the given system.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last-modified time of the resource within the given system.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The expiration time of the resource within the given system.
    /// Currently only apllicable to BigQuery resources.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Describes a Cloud Storage fileset entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFilesetSpec {
    /// Required. Patterns to identify a set of files in Google Cloud Storage.
    /// See [Cloud Storage
    /// documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames)
    /// for more information. Note that bucket wildcards are currently not
    /// supported.
    ///
    /// Examples of valid file_patterns:
    ///
    ///  * `gs://bucket_name/dir/*`: matches all files within `bucket_name/dir`
    ///                              directory.
    ///  * `gs://bucket_name/dir/**`: matches all files in `bucket_name/dir`
    ///                               spanning all subdirectories.
    ///  * `gs://bucket_name/file*`: matches files prefixed by `file` in
    ///                              `bucket_name`
    ///  * `gs://bucket_name/??.txt`: matches files with two characters followed by
    ///                               `.txt` in `bucket_name`
    ///  * `gs://bucket_name/[aeiou].txt`: matches files that contain a single
    ///                                    vowel character followed by `.txt` in
    ///                                    `bucket_name`
    ///  * `gs://bucket_name/[a-m].txt`: matches files that contain `a`, `b`, ...
    ///                                  or `m` followed by `.txt` in `bucket_name`
    ///  * `gs://bucket_name/a/*/b`: matches all files in `bucket_name` that match
    ///                              `a/*/b` pattern, such as `a/c/b`, `a/d/b`
    ///  * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt`
    ///
    /// You can combine wildcards to provide more powerful matches, for example:
    ///
    ///  * `gs://bucket_name/[a-m]??.j*g`
    #[prost(string, repeated, tag = "1")]
    pub file_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Sample files contained in this fileset, not all files
    /// contained in this fileset are represented here.
    #[prost(message, repeated, tag = "2")]
    pub sample_gcs_file_specs: ::prost::alloc::vec::Vec<GcsFileSpec>,
}
/// Specifications of a single file in Cloud Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFileSpec {
    /// Required. The full file path. Example: `gs://bucket_name/a/b.txt`.
    #[prost(string, tag = "1")]
    pub file_path: ::prost::alloc::string::String,
    /// Output only. Timestamps about the Cloud Storage file.
    #[prost(message, optional, tag = "2")]
    pub gcs_timestamps: ::core::option::Option<SystemTimestamps>,
    /// Output only. The size of the file, in bytes.
    #[prost(int64, tag = "4")]
    pub size_bytes: i64,
}
/// Represents a schema (e.g. BigQuery, GoogleSQL, Avro schema).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// The unified GoogleSQL-like schema of columns.
    ///
    /// The overall maximum number of columns and nested columns is 10,000.
    /// The maximum nested depth is 15 levels.
    #[prost(message, repeated, tag = "2")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
}
/// Representation of a column within a schema. Columns could be nested inside
/// other columns.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSchema {
    /// Required. Name of the column.
    ///
    /// Must be a UTF-8 string without dots (.).
    /// The maximum size is 64 bytes.
    #[prost(string, tag = "6")]
    pub column: ::prost::alloc::string::String,
    /// Required. Type of the column.
    ///
    /// Must be a UTF-8 string with the maximum size of 128 bytes.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional. Description of the column. Default value is an empty string.
    ///
    /// The description must be a UTF-8 string with the maximum size of 2000
    /// bytes.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Optional. A column's mode indicates if values in this column are required,
    /// nullable, or repeated.
    ///
    /// Only `NULLABLE`, `REQUIRED`, and `REPEATED` values are supported.
    /// Default mode is `NULLABLE`.
    #[prost(string, tag = "3")]
    pub mode: ::prost::alloc::string::String,
    /// Optional. Schema of sub-columns. A column can have zero or more sub-columns.
    #[prost(message, repeated, tag = "7")]
    pub subcolumns: ::prost::alloc::vec::Vec<ColumnSchema>,
}
/// A result that appears in the response of a search request. Each result
/// captures details of one entry that matches the search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogResult {
    /// Type of the search result. This field can be used to determine which Get
    /// method to call to fetch the full resource.
    #[prost(enumeration = "SearchResultType", tag = "1")]
    pub search_result_type: i32,
    /// Sub-type of the search result. This is a dot-delimited description of the
    /// resource's full type, and is the same as the value callers would provide in
    /// the "type" search facet.  Examples: `entry.table`, `entry.dataStream`,
    /// `tagTemplate`.
    #[prost(string, tag = "2")]
    pub search_result_subtype: ::prost::alloc::string::String,
    /// The relative resource name of the resource in URL format.
    /// Examples:
    ///
    ///  * `projects/{project_id}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}`
    ///  * `projects/{project_id}/tagTemplates/{tag_template_id}`
    #[prost(string, tag = "3")]
    pub relative_resource_name: ::prost::alloc::string::String,
    /// The full name of the cloud resource the entry belongs to. See:
    /// https://cloud.google.com/apis/design/resource_names#full_resource_name.
    /// Example:
    ///
    ///  * `//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId`
    #[prost(string, tag = "4")]
    pub linked_resource: ::prost::alloc::string::String,
    /// Last-modified timestamp of the entry from the managing system.
    #[prost(message, optional, tag = "7")]
    pub modify_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Fully Qualified Name of the resource.
    /// There are two main forms of FQNs:
    /// {system}:{project}.{dot-separated path to resource}
    ///     for non-regionalized resources
    /// {system}:{project}.{location id}.{dot-separated path to resource}
    ///     for regionalized resources
    /// Examples:
    /// * dataproc_metastore:projectId.locationId.instanceId.databaseId.tableId
    /// * bigquery:table.project_id.dataset_id.table_id
    #[prost(string, tag = "10")]
    pub fully_qualified_name: ::prost::alloc::string::String,
    /// The source system of the entry. Only applicable when `search_result_type`
    /// is ENTRY.
    #[prost(oneof = "search_catalog_result::System", tags = "8, 9")]
    pub system: ::core::option::Option<search_catalog_result::System>,
}
/// Nested message and enum types in `SearchCatalogResult`.
pub mod search_catalog_result {
    /// The source system of the entry. Only applicable when `search_result_type`
    /// is ENTRY.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum System {
        /// Output only. This field indicates the entry's source system that Data Catalog
        /// integrates with, such as BigQuery or Cloud Pub/Sub.
        #[prost(enumeration = "super::IntegratedSystem", tag = "8")]
        IntegratedSystem(i32),
        /// This field indicates the entry's source system that Data Catalog does not
        /// integrate with.
        #[prost(string, tag = "9")]
        UserSpecifiedSystem(::prost::alloc::string::String),
    }
}
/// The different types of resources that can be returned in search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchResultType {
    /// Default unknown type.
    Unspecified = 0,
    /// An [Entry][google.cloud.datacatalog.v1.Entry].
    Entry = 1,
    /// A [TagTemplate][google.cloud.datacatalog.v1.TagTemplate].
    TagTemplate = 2,
    /// An [EntryGroup][google.cloud.datacatalog.v1.EntryGroup].
    EntryGroup = 3,
}
/// Describes a BigQuery table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryTableSpec {
    /// Output only. The table source type.
    #[prost(enumeration = "TableSourceType", tag = "1")]
    pub table_source_type: i32,
    /// Output only.
    #[prost(oneof = "big_query_table_spec::TypeSpec", tags = "2, 3")]
    pub type_spec: ::core::option::Option<big_query_table_spec::TypeSpec>,
}
/// Nested message and enum types in `BigQueryTableSpec`.
pub mod big_query_table_spec {
    /// Output only.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeSpec {
        /// Table view specification. This field should only be populated if
        /// `table_source_type` is `BIGQUERY_VIEW`.
        #[prost(message, tag = "2")]
        ViewSpec(super::ViewSpec),
        /// Spec of a BigQuery table. This field should only be populated if
        /// `table_source_type` is `BIGQUERY_TABLE`.
        #[prost(message, tag = "3")]
        TableSpec(super::TableSpec),
    }
}
/// Table view specification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewSpec {
    /// Output only. The query that defines the table view.
    #[prost(string, tag = "1")]
    pub view_query: ::prost::alloc::string::String,
}
/// Normal BigQuery table spec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSpec {
    /// Output only. If the table is a dated shard, i.e., with name pattern `[prefix]YYYYMMDD`,
    /// `grouped_entry` is the Data Catalog resource name of the date sharded
    /// grouped entry, for example,
    /// `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`.
    /// Otherwise, `grouped_entry` is empty.
    #[prost(string, tag = "1")]
    pub grouped_entry: ::prost::alloc::string::String,
}
/// Spec for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`.
/// Context:
/// https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDateShardedSpec {
    /// Output only. The Data Catalog resource name of the dataset entry the current table
    /// belongs to, for example,
    /// `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`.
    #[prost(string, tag = "1")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. The table name prefix of the shards. The name of any given shard is
    /// `[table_prefix]YYYYMMDD`, for example, for shard `MyTable20180101`, the
    /// `table_prefix` is `MyTable`.
    #[prost(string, tag = "2")]
    pub table_prefix: ::prost::alloc::string::String,
    /// Output only. Total number of shards.
    #[prost(int64, tag = "3")]
    pub shard_count: i64,
}
/// Table source type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TableSourceType {
    /// Default unknown type.
    Unspecified = 0,
    /// Table view.
    BigqueryView = 2,
    /// BigQuery native table.
    BigqueryTable = 5,
    /// BigQuery materialized view.
    BigqueryMaterializedView = 7,
}
/// Tags are used to attach custom metadata to Data Catalog resources. Tags
/// conform to the specifications within their tag template.
///
/// See [Data Catalog
/// IAM](https://cloud.google.com/data-catalog/docs/concepts/iam) for information
/// on the permissions needed to create or view tags.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    /// The resource name of the tag in URL format. Example:
    ///
    /// `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id}`
    ///
    /// where `tag_id` is a system-generated identifier.
    ///
    /// Note: The tag itself might not be stored in the location specified in its
    /// name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resource name of the tag template that this tag uses. Example:
    ///
    /// `projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}`
    ///
    /// This field cannot be modified after creation.
    #[prost(string, tag = "2")]
    pub template: ::prost::alloc::string::String,
    /// Output only. The display name of the tag template.
    #[prost(string, tag = "5")]
    pub template_display_name: ::prost::alloc::string::String,
    /// Required. This maps the ID of a tag field to the value of and additional information
    /// about that field. Valid field IDs are defined by the tag's template. A tag
    /// must have at least 1 field and at most 500 fields.
    #[prost(map = "string, message", tag = "3")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, TagField>,
    /// The scope within the parent resource that this tag is attached to. If not
    /// provided, the tag is attached to the parent resource itself.
    /// Deleting the scope from the parent resource will delete all tags attached
    /// to that scope. These fields cannot be updated after creation.
    #[prost(oneof = "tag::Scope", tags = "4")]
    pub scope: ::core::option::Option<tag::Scope>,
}
/// Nested message and enum types in `Tag`.
pub mod tag {
    /// The scope within the parent resource that this tag is attached to. If not
    /// provided, the tag is attached to the parent resource itself.
    /// Deleting the scope from the parent resource will delete all tags attached
    /// to that scope. These fields cannot be updated after creation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        /// Resources like entry can have schemas associated with them. This scope
        /// allows users to attach tags to an individual column based on that schema.
        ///
        /// To attach a tag to a nested column, separate column names with a dot
        /// (`.`). Example: `column.nested_column`.
        #[prost(string, tag = "4")]
        Column(::prost::alloc::string::String),
    }
}
/// Contains the value and supporting information for a field within
/// a [Tag][google.cloud.datacatalog.v1.Tag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagField {
    /// Output only. The display name of this field.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The order of this field with respect to other fields in this tag. It can be
    /// set in [Tag][google.cloud.datacatalog.v1.TagTemplateField.order]. For
    /// example, a higher value can indicate a more important field. The value can
    /// be negative. Multiple fields can have the same order, and field orders
    /// within a tag do not have to be sequential.
    #[prost(int32, tag = "7")]
    pub order: i32,
    /// Required. The value of this field.
    #[prost(oneof = "tag_field::Kind", tags = "2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<tag_field::Kind>,
}
/// Nested message and enum types in `TagField`.
pub mod tag_field {
    /// Holds an enum value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumValue {
        /// The display name of the enum value.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
    }
    /// Required. The value of this field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Holds the value for a tag field with double type.
        #[prost(double, tag = "2")]
        DoubleValue(f64),
        /// Holds the value for a tag field with string type.
        /// The maximum length is 2000 UTF-8 characters.
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        /// Holds the value for a tag field with boolean type.
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        /// Holds the value for a tag field with timestamp type.
        #[prost(message, tag = "5")]
        TimestampValue(::prost_types::Timestamp),
        /// Holds the value for a tag field with enum type. This value must be
        /// one of the allowed values in the definition of this enum.
        #[prost(message, tag = "6")]
        EnumValue(EnumValue),
    }
}
/// A tag template defines a tag, which can have one or more typed fields.
/// The template is used to create and attach the tag to GCP resources.
/// [Tag template
/// roles](https://cloud.google.com/iam/docs/understanding-roles#data-catalog-roles)
/// provide permissions to create, edit, and use the template. See, for example,
/// the [TagTemplate
/// User](https://cloud.google.com/data-catalog/docs/how-to/template-user) role,
/// which includes permission to use the tag template to tag resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTemplate {
    /// The resource name of the tag template in URL format. Example:
    ///
    /// `projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}`
    ///
    /// Note: The tag template itself and its child resources might not be
    /// stored in the location specified in its name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Display name for this template. Defaults to an empty string.
    ///
    /// The name must contain only Unicode letters, numbers (0-9), underscores (_),
    /// dashes (-), spaces ( ), and can't start or end with spaces.
    /// The maximum length is 200 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Map of tag template field IDs to the settings for the field.
    /// This map is an exhaustive list of the allowed fields. This map must contain
    /// at least one field and at most 500 fields.
    ///
    /// The keys to this map are tag template field IDs. Field IDs can contain
    /// letters (both uppercase and lowercase), numbers (0-9) and underscores (_).
    /// Field IDs must be at least 1 character long and at most
    /// 64 characters long. Field IDs must start with a letter or underscore.
    #[prost(map = "string, message", tag = "3")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, TagTemplateField>,
}
/// The template for an individual field within a tag template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTemplateField {
    /// Output only. The resource name of the tag template field in URL format. Example:
    ///
    /// `projects/{project_id}/locations/{location}/tagTemplates/{tag_template}/fields/{field}`
    ///
    /// Note: The `TagTemplateField` itself might not be stored in the location
    /// specified in its name.
    ///
    /// The name must contain only letters (a-z, A-Z), numbers (0-9),
    /// or underscores (_), and must start with a letter or underscore.
    /// The maximum length is 64 characters.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// The display name for this field. Defaults to an empty string.
    ///
    /// The name must contain only Unicode letters, numbers (0-9), underscores (_),
    /// dashes (-), spaces ( ), and can't start or end with spaces.
    /// The maximum length is 200 characters.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The type of value this tag field can contain.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<FieldType>,
    /// Whether this is a required field. Defaults to false.
    #[prost(bool, tag = "3")]
    pub is_required: bool,
    /// The description for this field. Defaults to an empty string.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// The order of this field with respect to other fields in this tag
    /// template. For example, a higher value can indicate a more important field.
    /// The value can be negative. Multiple fields can have the same order, and
    /// field orders within a tag do not have to be sequential.
    #[prost(int32, tag = "5")]
    pub order: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldType {
    /// Required.
    #[prost(oneof = "field_type::TypeDecl", tags = "1, 2")]
    pub type_decl: ::core::option::Option<field_type::TypeDecl>,
}
/// Nested message and enum types in `FieldType`.
pub mod field_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumType {
        /// The set of allowed values for this enum.
        ///
        /// This set must not be empty and can include up to 100 allowed values.
        /// The display names of the values in this set must not be empty and must
        /// be case-insensitively unique within this set.
        ///
        /// The order of items in this set is preserved. This field can be used to
        /// create, remove and reorder enum values. To rename enum values, use the
        /// `RenameTagTemplateFieldEnumValue` method.
        #[prost(message, repeated, tag = "1")]
        pub allowed_values: ::prost::alloc::vec::Vec<enum_type::EnumValue>,
    }
    /// Nested message and enum types in `EnumType`.
    pub mod enum_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EnumValue {
            /// Required. The display name of the enum value. Must not be an empty string.
            ///
            /// The name must contain only Unicode letters, numbers (0-9), underscores
            /// (_), dashes (-), spaces ( ), and can't start or end with spaces. The
            /// maximum length is 200 characters.
            #[prost(string, tag = "1")]
            pub display_name: ::prost::alloc::string::String,
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PrimitiveType {
        /// This is the default invalid value for a type.
        Unspecified = 0,
        /// A double precision number.
        Double = 1,
        /// An UTF-8 string.
        String = 2,
        /// A boolean value.
        Bool = 3,
        /// A timestamp.
        Timestamp = 4,
    }
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeDecl {
        /// Represents primitive types - string, bool etc.
        #[prost(enumeration = "PrimitiveType", tag = "1")]
        PrimitiveType(i32),
        /// Represents an enum type.
        #[prost(message, tag = "2")]
        EnumType(EnumType),
    }
}
/// Request message for
/// [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogRequest {
    /// Required. The scope of this search request. A `scope` that has empty
    /// `include_org_ids`, `include_project_ids` AND false
    /// `include_gcp_public_datasets` is considered invalid. Data Catalog will
    /// return an error in such a case.
    #[prost(message, optional, tag = "6")]
    pub scope: ::core::option::Option<search_catalog_request::Scope>,
    /// Optional. The query string in search query syntax. An empty query string will result
    /// in all data assets (in the specified scope) that the user has access to.
    ///
    /// Query strings can be simple as "x" or more qualified as:
    ///
    /// * name:x
    /// * column:x
    /// * description:y
    ///
    /// Note: Query tokens need to have a minimum of 3 characters for substring
    /// matching to work correctly. See [Data Catalog Search
    /// Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference)
    /// for more information.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Number of results in the search page. If <=0 then defaults to 10. Max limit
    /// for page_size is 1000. Throws an invalid argument for page_size > 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token returned in an earlier
    /// [SearchCatalogResponse.next_page_token][google.cloud.datacatalog.v1.SearchCatalogResponse.next_page_token], which
    /// indicates that this is a continuation of a prior
    /// [SearchCatalogRequest][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog]
    /// call, and that the system should return the next page of data. If empty,
    /// the first page is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies the ordering of results, currently supported case-sensitive
    /// choices are:
    ///
    ///   * `relevance`, only supports descending
    ///   * `last_modified_timestamp [asc|desc]`, defaults to descending if not
    ///     specified
    ///
    /// If not specified, defaults to `relevance` descending.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SearchCatalogRequest`.
pub mod search_catalog_request {
    /// The criteria that select the subspace used for query matching.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Scope {
        /// The list of organization IDs to search within. To find your organization
        /// ID, follow instructions in
        /// https://cloud.google.com/resource-manager/docs/creating-managing-organization.
        #[prost(string, repeated, tag = "2")]
        pub include_org_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The list of project IDs to search within. To learn more about the
        /// distinction between project names/IDs/numbers, go to
        /// https://cloud.google.com/docs/overview/#projects.
        #[prost(string, repeated, tag = "3")]
        pub include_project_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// If `true`, include Google Cloud Platform (GCP) public datasets in the
        /// search results. Info on GCP public datasets is available at
        /// https://cloud.google.com/public-datasets/. By default, GCP public
        /// datasets are excluded.
        #[prost(bool, tag = "7")]
        pub include_gcp_public_datasets: bool,
        /// Optional. The list of locations to search within.
        /// 1. If empty, search will be performed in all locations;
        /// 2. If any of the locations are NOT [supported
        /// regions](https://cloud.google.com/data-catalog/docs/concepts/regions#supported_regions),
        /// error will be returned;
        /// 3. Otherwise, search only the given locations for matching results.
        /// Typical usage is to leave this field empty. When a location is
        /// unreachable as returned in the `SearchCatalogResponse.unreachable` field,
        /// users can repeat the search request with this parameter set to get
        /// additional information on the error.
        #[prost(string, repeated, tag = "16")]
        pub restricted_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Response message for
/// [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogResponse {
    /// Search results.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<SearchCatalogResult>,
    /// The token that can be used to retrieve the next page of results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable locations. Search result does not include data from those
    /// locations. Users can get additional information on the error by repeating
    /// the search request with a more restrictive parameter -- setting the value
    /// for `SearchDataCatalogRequest.scope.restricted_locations`.
    #[prost(string, repeated, tag = "6")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CreateEntryGroup][google.cloud.datacatalog.v1.DataCatalog.CreateEntryGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryGroupRequest {
    /// Required. The name of the project this entry group belongs to. Example:
    ///
    /// `projects/{project_id}/locations/{location}`
    ///
    /// Note: The entry group itself and its child resources might not be
    /// stored in the location specified in its name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the entry group to create.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and must start with a letter or underscore.
    /// The maximum size is 64 bytes when encoded in UTF-8.
    #[prost(string, tag = "3")]
    pub entry_group_id: ::prost::alloc::string::String,
    /// The entry group to create. Defaults to an empty entry group.
    #[prost(message, optional, tag = "2")]
    pub entry_group: ::core::option::Option<EntryGroup>,
}
/// Request message for
/// [UpdateEntryGroup][google.cloud.datacatalog.v1.DataCatalog.UpdateEntryGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryGroupRequest {
    /// Required. The updated entry group. "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub entry_group: ::core::option::Option<EntryGroup>,
    /// Names of fields whose values to overwrite on an entry group.
    ///
    /// If this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [GetEntryGroup][google.cloud.datacatalog.v1.DataCatalog.GetEntryGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntryGroupRequest {
    /// Required. The name of the entry group. For example,
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The fields to return. If not set or empty, all fields are returned.
    #[prost(message, optional, tag = "2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteEntryGroup][google.cloud.datacatalog.v1.DataCatalog.DeleteEntryGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryGroupRequest {
    /// Required. The name of the entry group. For example,
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If true, deletes all entries in the entry group.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [ListEntryGroups][google.cloud.datacatalog.v1.DataCatalog.ListEntryGroups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntryGroupsRequest {
    /// Required. The name of the location that contains the entry groups, which can be
    /// provided in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return. Default is 10. Max limit is 1000.
    /// Throws an invalid argument for `page_size > 1000`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Token that specifies which page is requested. If empty, the first page is
    /// returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [ListEntryGroups][google.cloud.datacatalog.v1.DataCatalog.ListEntryGroups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntryGroupsResponse {
    /// EntryGroup details.
    #[prost(message, repeated, tag = "1")]
    pub entry_groups: ::prost::alloc::vec::Vec<EntryGroup>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [CreateEntry][google.cloud.datacatalog.v1.DataCatalog.CreateEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryRequest {
    /// Required. The name of the entry group this entry belongs to. Example:
    ///
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}`
    ///
    /// Note: The entry itself and its child resources might not be stored in
    /// the location specified in its name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the entry to create.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// and underscores (_).
    /// The maximum size is 64 bytes when encoded in UTF-8.
    #[prost(string, tag = "3")]
    pub entry_id: ::prost::alloc::string::String,
    /// Required. The entry to create.
    #[prost(message, optional, tag = "2")]
    pub entry: ::core::option::Option<Entry>,
}
/// Request message for
/// [UpdateEntry][google.cloud.datacatalog.v1.DataCatalog.UpdateEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryRequest {
    /// Required. The updated entry. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub entry: ::core::option::Option<Entry>,
    /// Names of fields whose values to overwrite on an entry.
    ///
    /// If this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    ///
    /// The following fields are modifiable:
    ///
    /// * For entries with type `DATA_STREAM`:
    ///    * `schema`
    /// * For entries with type `FILESET`:
    ///    * `schema`
    ///    * `display_name`
    ///    * `description`
    ///    * `gcs_fileset_spec`
    ///    * `gcs_fileset_spec.file_patterns`
    /// * For entries with `user_specified_type`:
    ///    * `schema`
    ///    * `display_name`
    ///    * `description`
    ///    * `user_specified_type`
    ///    * `user_specified_system`
    ///    * `linked_resource`
    ///    * `source_system_timestamps`
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteEntry][google.cloud.datacatalog.v1.DataCatalog.DeleteEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryRequest {
    /// Required. The name of the entry. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [GetEntry][google.cloud.datacatalog.v1.DataCatalog.GetEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntryRequest {
    /// Required. The name of the entry. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [LookupEntry][google.cloud.datacatalog.v1.DataCatalog.LookupEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEntryRequest {
    /// Required. Represents either the Google Cloud Platform resource or SQL name
    /// for a Google Cloud Platform resource.
    #[prost(oneof = "lookup_entry_request::TargetName", tags = "1, 3, 5")]
    pub target_name: ::core::option::Option<lookup_entry_request::TargetName>,
}
/// Nested message and enum types in `LookupEntryRequest`.
pub mod lookup_entry_request {
    /// Required. Represents either the Google Cloud Platform resource or SQL name
    /// for a Google Cloud Platform resource.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetName {
        /// The full name of the Google Cloud Platform resource the Data Catalog
        /// entry represents. See:
        /// https://cloud.google.com/apis/design/resource_names#full_resource_name.
        /// Full names are case-sensitive.
        ///
        /// Examples:
        ///
        ///  * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId
        ///  * //pubsub.googleapis.com/projects/projectId/topics/topicId
        #[prost(string, tag = "1")]
        LinkedResource(::prost::alloc::string::String),
        /// The SQL name of the entry. SQL names are case-sensitive.
        ///
        /// Examples:
        ///
        ///   * `pubsub.project_id.topic_id`
        ///   * ``pubsub.project_id.`topic.id.with.dots` ``
        ///   * `bigquery.table.project_id.dataset_id.table_id`
        ///   * `bigquery.dataset.project_id.dataset_id`
        ///   * `datacatalog.entry.project_id.location_id.entry_group_id.entry_id`
        ///
        /// `*_id`s should satisfy the standard SQL rules for identifiers.
        /// https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical.
        #[prost(string, tag = "3")]
        SqlResource(::prost::alloc::string::String),
        /// Fully qualified name (FQN) of the resource.
        ///
        /// FQNs take two forms:
        ///
        /// * For non-regionalized resources:
        ///
        ///   `{SYSTEM}:{PROJECT}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}`
        ///
        /// * For regionalized resources:
        ///
        ///   `{SYSTEM}:{PROJECT}.{LOCATION_ID}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}`
        ///
        /// Example for a DPMS table:
        ///
        /// `dataproc_metastore:project_id.location_id.instance_id.database_id.table_id`
        #[prost(string, tag = "5")]
        FullyQualifiedName(::prost::alloc::string::String),
    }
}
/// Entry Metadata.
/// A Data Catalog Entry resource represents another resource in Google
/// Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic) or
/// outside of Google Cloud Platform. Clients can use the `linked_resource` field
/// in the Entry resource to refer to the original resource ID of the source
/// system.
///
/// An Entry resource contains resource details, such as its schema. An Entry can
/// also be used to attach flexible metadata, such as a
/// [Tag][google.cloud.datacatalog.v1.Tag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    /// Output only. The resource name of an entry in URL format.
    /// Example:
    ///
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}`
    ///
    /// Note: The entry itself and its child resources might not be
    /// stored in the location specified in its name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource this metadata entry refers to.
    ///
    /// For Google Cloud Platform resources, `linked_resource` is the [full name of
    /// the
    /// resource](https://cloud.google.com/apis/design/resource_names#full_resource_name).
    /// For example, the `linked_resource` for a table resource from BigQuery is:
    ///
    /// `//bigquery.googleapis.com/projects/{projectId}/datasets/{datasetId}/tables/{tableId}`
    ///
    /// Output only when entry is one of the types in the `EntryType` enum.
    ///
    /// For entries with a `user_specified_type`, this field is optional and
    /// defaults to an empty string.
    ///
    /// The resource string must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), periods (.), colons (:), slashes (/), dashes (-),
    /// and hashes (#).
    /// The maximum size is 200 bytes when encoded in UTF-8.
    #[prost(string, tag = "9")]
    pub linked_resource: ::prost::alloc::string::String,
    /// Fully qualified name (FQN) of the resource. Set automatically for entries
    /// representing resources from synced systems. Settable only during creation
    /// and read-only afterwards. Can be used for search and lookup of the entries.
    ///
    ///
    /// FQNs take two forms:
    ///
    /// * For non-regionalized resources:
    ///
    ///   `{SYSTEM}:{PROJECT}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}`
    ///
    /// * For regionalized resources:
    ///
    ///   `{SYSTEM}:{PROJECT}.{LOCATION_ID}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}`
    ///
    /// Example for a DPMS table:
    ///
    /// `dataproc_metastore:project_id.location_id.instance_id.database_id.table_id`
    #[prost(string, tag = "29")]
    pub fully_qualified_name: ::prost::alloc::string::String,
    /// Display name of an entry.
    ///
    /// The name must contain only Unicode letters, numbers (0-9), underscores (_),
    /// dashes (-), spaces ( ), and can't start or end with spaces.
    /// The maximum size is 200 bytes when encoded in UTF-8.
    /// Default value is an empty string.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Entry description that can consist of several sentences or paragraphs
    /// that describe entry contents.
    ///
    /// The description must not contain Unicode non-characters as well as C0
    /// and C1 control codes except tabs (HT), new lines (LF), carriage returns
    /// (CR), and page breaks (FF).
    /// The maximum size is 2000 bytes when encoded in UTF-8.
    /// Default value is an empty string.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Schema of the entry. An entry might not have any schema attached to it.
    #[prost(message, optional, tag = "5")]
    pub schema: ::core::option::Option<Schema>,
    /// Timestamps about the underlying resource, not about this Data Catalog
    /// entry. Output only when Entry is of type in the EntryType enum. For entries
    /// with user_specified_type, this field is optional and defaults to an empty
    /// timestamp.
    #[prost(message, optional, tag = "7")]
    pub source_system_timestamps: ::core::option::Option<SystemTimestamps>,
    /// Output only. Physical location of the entry.
    #[prost(message, optional, tag = "20")]
    pub data_source: ::core::option::Option<DataSource>,
    /// Required. Entry type.
    #[prost(oneof = "entry::EntryType", tags = "2, 16")]
    pub entry_type: ::core::option::Option<entry::EntryType>,
    /// The source system of the entry.
    #[prost(oneof = "entry::System", tags = "17, 18")]
    pub system: ::core::option::Option<entry::System>,
    /// Type specification information.
    #[prost(oneof = "entry::TypeSpec", tags = "6, 12, 15")]
    pub type_spec: ::core::option::Option<entry::TypeSpec>,
    /// Type- and system- specific information. Specifications for types contain
    /// fields common to all entries of a given type, and sub-specs with fields
    /// specific to a given source system.
    /// When extending the API with new types and systems please use this instead
    /// of legacy type_spec field.
    #[prost(oneof = "entry::Spec", tags = "24")]
    pub spec: ::core::option::Option<entry::Spec>,
}
/// Nested message and enum types in `Entry`.
pub mod entry {
    /// Required. Entry type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EntryType {
        /// The type of the entry.
        /// Only used for Entries with types in the EntryType enum.
        #[prost(enumeration = "super::EntryType", tag = "2")]
        Type(i32),
        /// Entry type if it does not fit any of the input-allowed values listed in
        /// `EntryType` enum above. When creating an entry, users should check the
        /// enum values first, if nothing matches the entry to be created, then
        /// provide a custom value, for example "my_special_type".
        /// `user_specified_type` strings must begin with a letter or underscore and
        /// can only contain letters, numbers, and underscores; are case insensitive;
        /// must be at least 1 character and at most 64 characters long.
        ///
        /// Currently, only FILESET enum value is allowed. All other entries created
        /// through Data Catalog must use `user_specified_type`.
        #[prost(string, tag = "16")]
        UserSpecifiedType(::prost::alloc::string::String),
    }
    /// The source system of the entry.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum System {
        /// Output only. This field indicates the entry's source system that Data Catalog
        /// integrates with, such as BigQuery or Pub/Sub.
        #[prost(enumeration = "super::IntegratedSystem", tag = "17")]
        IntegratedSystem(i32),
        /// This field indicates the entry's source system that Data Catalog does not
        /// integrate with. `user_specified_system` strings must begin with a letter
        /// or underscore and can only contain letters, numbers, and underscores; are
        /// case insensitive; must be at least 1 character and at most 64 characters
        /// long.
        #[prost(string, tag = "18")]
        UserSpecifiedSystem(::prost::alloc::string::String),
    }
    /// Type specification information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeSpec {
        /// Specification that applies to a Cloud Storage fileset. This is only valid
        /// on entries of type FILESET.
        #[prost(message, tag = "6")]
        GcsFilesetSpec(super::GcsFilesetSpec),
        /// Specification that applies to a BigQuery table. This is only valid on
        /// entries of type `TABLE`.
        #[prost(message, tag = "12")]
        BigqueryTableSpec(super::BigQueryTableSpec),
        /// Specification for a group of BigQuery tables with name pattern
        /// `[prefix]YYYYMMDD`. Context:
        /// https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding.
        #[prost(message, tag = "15")]
        BigqueryDateShardedSpec(super::BigQueryDateShardedSpec),
    }
    /// Type- and system- specific information. Specifications for types contain
    /// fields common to all entries of a given type, and sub-specs with fields
    /// specific to a given source system.
    /// When extending the API with new types and systems please use this instead
    /// of legacy type_spec field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Spec {
        /// Specification that applies to a table resource. Only valid
        /// for entries of `TABLE` type.
        #[prost(message, tag = "24")]
        DatabaseTableSpec(super::DatabaseTableSpec),
    }
}
/// Specification that applies to a table resource. Only valid
/// for entries of `TABLE` type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseTableSpec {
    /// Type of this table.
    #[prost(enumeration = "database_table_spec::TableType", tag = "1")]
    pub r#type: i32,
}
/// Nested message and enum types in `DatabaseTableSpec`.
pub mod database_table_spec {
    /// Type of the table.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TableType {
        /// Default unknown table type.
        Unspecified = 0,
        /// Native table.
        Native = 1,
        /// External table.
        External = 2,
    }
}
/// EntryGroup Metadata.
/// An EntryGroup resource represents a logical grouping of zero or more
/// Data Catalog [Entry][google.cloud.datacatalog.v1.Entry] resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryGroup {
    /// The resource name of the entry group in URL format. Example:
    ///
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}`
    ///
    /// Note: The entry group itself and its child resources might not be
    /// stored in the location specified in its name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A short name to identify the entry group, for example,
    /// "analytics data - jan 2011". Default value is an empty string.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Entry group description, which can consist of several sentences or
    /// paragraphs that describe entry group contents. Default value is an empty
    /// string.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Timestamps about this EntryGroup. Default value is empty timestamps.
    #[prost(message, optional, tag = "4")]
    pub data_catalog_timestamps: ::core::option::Option<SystemTimestamps>,
}
/// Request message for
/// [CreateTagTemplate][google.cloud.datacatalog.v1.DataCatalog.CreateTagTemplate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagTemplateRequest {
    /// Required. The name of the project and the template location
    /// [region](https://cloud.google.com/data-catalog/docs/concepts/regions).
    ///
    /// Example:
    ///
    /// * projects/{project_id}/locations/us-central1
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the tag template to create.
    ///
    /// The ID must contain only lowercase letters (a-z), numbers (0-9),
    /// or underscores (_), and must start with a letter or underscore.
    /// The maximum size is 64 bytes when encoded in UTF-8.
    #[prost(string, tag = "3")]
    pub tag_template_id: ::prost::alloc::string::String,
    /// Required. The tag template to create.
    #[prost(message, optional, tag = "2")]
    pub tag_template: ::core::option::Option<TagTemplate>,
}
/// Request message for
/// [GetTagTemplate][google.cloud.datacatalog.v1.DataCatalog.GetTagTemplate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagTemplateRequest {
    /// Required. The name of the tag template. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [UpdateTagTemplate][google.cloud.datacatalog.v1.DataCatalog.UpdateTagTemplate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagTemplateRequest {
    /// Required. The template to update. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub tag_template: ::core::option::Option<TagTemplate>,
    /// Names of fields whose values to overwrite on a tag template. Currently,
    /// only `display_name` can be overwritten.
    ///
    /// In general, if this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteTagTemplate][google.cloud.datacatalog.v1.DataCatalog.DeleteTagTemplate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagTemplateRequest {
    /// Required. The name of the tag template to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Currently, this field must always be set to `true`.
    /// This confirms the deletion of any possible tags using this template.
    /// `force = false` will be supported in the future.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [CreateTag][google.cloud.datacatalog.v1.DataCatalog.CreateTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagRequest {
    /// Required. The name of the resource to attach this tag to. Tags can be attached to
    /// entries. An entry can have up to 1000 attached tags. Example:
    ///
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}`
    ///
    /// Note: The tag and its child resources might not be stored in
    /// the location specified in its name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The tag to create.
    #[prost(message, optional, tag = "2")]
    pub tag: ::core::option::Option<Tag>,
}
/// Request message for
/// [UpdateTag][google.cloud.datacatalog.v1.DataCatalog.UpdateTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagRequest {
    /// Required. The updated tag. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub tag: ::core::option::Option<Tag>,
    /// Names of fields whose values to overwrite on a tag. Currently, a tag has
    /// the only modifiable field with the name `fields`.
    ///
    /// In general, if this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteTag][google.cloud.datacatalog.v1.DataCatalog.DeleteTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagRequest {
    /// Required. The name of the tag to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CreateTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.CreateTagTemplateField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagTemplateFieldRequest {
    /// Required. The name of the project and the template location
    /// [region](https://cloud.google.com/data-catalog/docs/concepts/regions).
    ///
    /// Example:
    ///
    /// * projects/{project_id}/locations/us-central1/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the tag template field to create.
    ///
    /// Note: Adding a required field to an existing template is *not* allowed.
    ///
    /// Field IDs can contain letters (both uppercase and lowercase), numbers
    /// (0-9), underscores (_) and dashes (-). Field IDs must be at least 1
    /// character long and at most 128 characters long. Field IDs must also be
    /// unique within their template.
    #[prost(string, tag = "2")]
    pub tag_template_field_id: ::prost::alloc::string::String,
    /// Required. The tag template field to create.
    #[prost(message, optional, tag = "3")]
    pub tag_template_field: ::core::option::Option<TagTemplateField>,
}
/// Request message for
/// [UpdateTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.UpdateTagTemplateField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagTemplateFieldRequest {
    /// Required. The name of the tag template field. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The template to update.
    #[prost(message, optional, tag = "2")]
    pub tag_template_field: ::core::option::Option<TagTemplateField>,
    /// Optional. Names of fields whose values to overwrite on an individual field of a tag
    /// template. The following fields are modifiable:
    ///
    ///   * `display_name`
    ///   * `type.enum_type`
    ///   * `is_required`
    ///
    /// If this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the request
    /// body, their values are emptied with one exception: when updating an enum
    /// type, the provided values are merged with the existing values. Therefore,
    /// enum values can only be added, existing enum values cannot be deleted or
    /// renamed.
    ///
    /// Additionally, updating a template field from optional to required is
    /// *not* allowed.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [RenameTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.RenameTagTemplateField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTagTemplateFieldRequest {
    /// Required. The name of the tag template. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new ID of this tag template field. For example, `my_new_field`.
    #[prost(string, tag = "2")]
    pub new_tag_template_field_id: ::prost::alloc::string::String,
}
/// Request message for
/// [RenameTagTemplateFieldEnumValue][google.cloud.datacatalog.v1.DataCatalog.RenameTagTemplateFieldEnumValue].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTagTemplateFieldEnumValueRequest {
    /// Required. The name of the enum field value. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}/enumValues/{enum_value_display_name}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new display name of the enum value. For example, `my_new_enum_value`.
    #[prost(string, tag = "2")]
    pub new_enum_value_display_name: ::prost::alloc::string::String,
}
/// Request message for
/// [DeleteTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.DeleteTagTemplateField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagTemplateFieldRequest {
    /// Required. The name of the tag template field to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Currently, this field must always be set to `true`.
    /// This confirms the deletion of this field from any tags using this field.
    /// `force = false` will be supported in the future.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [ListTags][google.cloud.datacatalog.v1.DataCatalog.ListTags].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsRequest {
    /// Required. The name of the Data Catalog resource to list the tags of. The resource
    /// could be an [Entry][google.cloud.datacatalog.v1.Entry] or an
    /// [EntryGroup][google.cloud.datacatalog.v1.EntryGroup].
    ///
    /// Examples:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of tags to return. Default is 10. Max limit is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token that specifies which page is requested. If empty, the first page is
    /// returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [ListTags][google.cloud.datacatalog.v1.DataCatalog.ListTags].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsResponse {
    /// [Tag][google.cloud.datacatalog.v1.Tag] details.
    #[prost(message, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [ListEntries][google.cloud.datacatalog.v1.DataCatalog.ListEntries].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesRequest {
    /// Required. The name of the entry group that contains the entries, which can
    /// be provided in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Default is 10. Max limit is 1000.
    /// Throws an invalid argument for `page_size > 1000`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token that specifies which page is requested. If empty, the first page is
    /// returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The fields to return for each Entry. If not set or empty, all
    /// fields are returned.
    /// For example, setting read_mask to contain only one path "name" will cause
    /// ListEntries to return a list of Entries with only "name" field.
    #[prost(message, optional, tag = "4")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for
/// [ListEntries][google.cloud.datacatalog.v1.DataCatalog.ListEntries].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesResponse {
    /// Entry details.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<Entry>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Entry resources in Data Catalog can be of different types e.g. a BigQuery
/// Table entry is of type `TABLE`. This enum describes all the possible types
/// Data Catalog contains.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntryType {
    /// Default unknown type.
    Unspecified = 0,
    /// Output only. The type of entry that has a GoogleSQL schema, including
    /// logical views.
    Table = 2,
    /// Output only. The type of models, examples include
    /// https://cloud.google.com/bigquery-ml/docs/bigqueryml-intro
    Model = 5,
    /// An entry type which is used for streaming entries. Example:
    /// Pub/Sub topic.
    DataStream = 3,
    /// An entry type which is a set of files or objects. Example:
    /// Cloud Storage fileset.
    Fileset = 4,
    /// A database.
    Database = 7,
    /// A service, for example, a Dataproc Metastore service.
    Service = 14,
}
#[doc = r" Generated client implementations."]
pub mod data_catalog_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Data Catalog API service allows clients to discover, understand, and manage"]
    #[doc = " their data."]
    #[derive(Debug, Clone)]
    pub struct DataCatalogClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataCatalogClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DataCatalogClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DataCatalogClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Searches Data Catalog for multiple resources like entries, tags that"]
        #[doc = " match a query."]
        #[doc = ""]
        #[doc = " This is a custom method"]
        #[doc = " (https://cloud.google.com/apis/design/custom_methods) and does not return"]
        #[doc = " the complete resource, only the resource identifier and high level"]
        #[doc = " fields. Clients can subsequently call `Get` methods."]
        #[doc = ""]
        #[doc = " Note that Data Catalog search queries do not guarantee full recall. Query"]
        #[doc = " results that match your query may not be returned, even in subsequent"]
        #[doc = " result pages. Also note that results returned (and not returned) can vary"]
        #[doc = " across repeated search queries."]
        #[doc = ""]
        #[doc = " See [Data Catalog Search"]
        #[doc = " Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference)"]
        #[doc = " for more information."]
        pub async fn search_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchCatalogRequest>,
        ) -> Result<tonic::Response<super::SearchCatalogResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/SearchCatalog",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an EntryGroup."]
        #[doc = ""]
        #[doc = " An entry group contains logically related entries together with Cloud"]
        #[doc = " Identity and Access Management policies that specify the users who can"]
        #[doc = " create, edit, and view entries within the entry group."]
        #[doc = ""]
        #[doc = " Data Catalog automatically creates an entry group for BigQuery entries"]
        #[doc = " (\"@bigquery\") and Pub/Sub topics (\"@pubsub\"). Users create their own entry"]
        #[doc = " group to contain Cloud Storage fileset entries or custom type entries,"]
        #[doc = " and the IAM policies associated with those entries. Entry groups, like"]
        #[doc = " entries, can be searched."]
        #[doc = ""]
        #[doc = " A maximum of 10,000 entry groups may be created per organization across all"]
        #[doc = " locations."]
        #[doc = ""]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `parent` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn create_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateEntryGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an EntryGroup."]
        pub async fn get_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/GetEntryGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an EntryGroup. The user should enable the Data Catalog API in the"]
        #[doc = " project identified by the `entry_group.name` parameter (see [Data Catalog"]
        #[doc = " Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn update_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateEntryGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an EntryGroup. Only entry groups that do not contain entries can be"]
        #[doc = " deleted. Users should enable the Data Catalog API in the project"]
        #[doc = " identified by the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn delete_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntryGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteEntryGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists entry groups."]
        pub async fn list_entry_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntryGroupsRequest>,
        ) -> Result<tonic::Response<super::ListEntryGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/ListEntryGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an entry. Only entries of types 'FILESET', 'CLUSTER', 'DATA_STREAM'"]
        #[doc = " or with a user-specified type can be created."]
        #[doc = ""]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `parent` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        #[doc = ""]
        #[doc = " A maximum of 100,000 entries may be created per entry group."]
        pub async fn create_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing entry."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `entry.name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn update_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing entry. Only entries created through"]
        #[doc = " [CreateEntry][google.cloud.datacatalog.v1.DataCatalog.CreateEntry]"]
        #[doc = " method can be deleted."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn delete_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntryRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an entry."]
        pub async fn get_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/GetEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get an entry by target resource name. This method allows clients to use"]
        #[doc = " the resource name from the source Google Cloud Platform service to get the"]
        #[doc = " Data Catalog Entry."]
        pub async fn lookup_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/LookupEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists entries."]
        pub async fn list_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntriesRequest>,
        ) -> Result<tonic::Response<super::ListEntriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/ListEntries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a tag template. The user should enable the Data Catalog API in"]
        #[doc = " the project identified by the `parent` parameter (see [Data Catalog"]
        #[doc = " Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        pub async fn create_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTagTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a tag template."]
        pub async fn get_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/GetTagTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a tag template. This method cannot be used to update the fields of"]
        #[doc = " a template. The tag template fields are represented as separate resources"]
        #[doc = " and should be updated using their own create/update/delete methods."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `tag_template.name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn update_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTagTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a tag template and all tags using the template."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn delete_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagTemplateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTagTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a field in a tag template. The user should enable the Data Catalog"]
        #[doc = " API in the project identified by the `parent` parameter (see"]
        #[doc = " [Data Catalog Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        pub async fn create_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTagTemplateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a field in a tag template. This method cannot be used to update the"]
        #[doc = " field type. Users should enable the Data Catalog API in the project"]
        #[doc = " identified by the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn update_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTagTemplateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Renames a field in a tag template. The user should enable the Data Catalog"]
        #[doc = " API in the project identified by the `name` parameter (see [Data Catalog"]
        #[doc = " Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        pub async fn rename_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/RenameTagTemplateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Renames an enum value in a tag template. The enum values have to be unique"]
        #[doc = " within one enum field."]
        pub async fn rename_tag_template_field_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameTagTemplateFieldEnumValueRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/RenameTagTemplateFieldEnumValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a field in a tag template and all uses of that field."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn delete_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTagTemplateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a tag on an [Entry][google.cloud.datacatalog.v1.Entry]."]
        #[doc = " Note: The project identified by the `parent` parameter for the"]
        #[doc = " [tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries.tags/create#path-parameters)"]
        #[doc = " and the"]
        #[doc = " [tag"]
        #[doc = " template](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.tagTemplates/create#path-parameters)"]
        #[doc = " used to create the tag must be from the same organization."]
        pub async fn create_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagRequest>,
        ) -> Result<tonic::Response<super::Tag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing tag."]
        pub async fn update_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagRequest>,
        ) -> Result<tonic::Response<super::Tag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a tag."]
        pub async fn delete_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the tags on an [Entry][google.cloud.datacatalog.v1.Entry]."]
        pub async fn list_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagsRequest>,
        ) -> Result<tonic::Response<super::ListTagsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/ListTags",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy for a resource. Replaces any existing"]
        #[doc = " policy."]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " Callers must have following Google IAM permission"]
        #[doc = "   - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag"]
        #[doc = "     templates."]
        #[doc = "   - `datacatalog.entries.setIamPolicy` to set policies on entries."]
        #[doc = "   - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups."]
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
                "/google.cloud.datacatalog.v1.DataCatalog/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a resource. A `NOT_FOUND` error"]
        #[doc = " is returned if the resource does not exist. An empty policy is returned"]
        #[doc = " if the resource exists but does not have a policy set on it."]
        #[doc = ""]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " Callers must have following Google IAM permission"]
        #[doc = "   - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag"]
        #[doc = "     templates."]
        #[doc = "   - `datacatalog.entries.getIamPolicy` to get policies on entries."]
        #[doc = "   - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups."]
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the caller's permissions on a resource."]
        #[doc = " If the resource does not exist, an empty set of permissions is returned"]
        #[doc = " (We don't return a `NOT_FOUND` error)."]
        #[doc = ""]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " A caller is not required to have Google IAM permission to make this"]
        #[doc = " request."]
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
                "/google.cloud.datacatalog.v1.DataCatalog/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A taxonomy is a collection of hierarchical policy tags that classify data
/// along a common axis. For instance a "data sensitivity" taxonomy could contain
/// the following policy tags:
///
/// ```
/// + PII
///   + Account number
///   + Age
///   + SSN
///   + Zipcode
/// + Financials
///   + Revenue
/// ```
///
/// A "data origin" taxonomy could contain the following policy tags:
///
/// ```
/// + User data
/// + Employee data
/// + Partner data
/// + Public data
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Taxonomy {
    /// Output only. Resource name of this taxonomy in format:
    /// "projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}".
    /// Note that taxonomy_id's are unique and generated by Policy Tag Manager.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. User-defined name of this taxonomy. It must: contain only unicode letters,
    /// numbers, underscores, dashes and spaces; not start or end with spaces; and
    /// be at most 200 bytes long when encoded in UTF-8.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description of this taxonomy. It must: contain only unicode characters,
    /// tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes
    /// long when encoded in UTF-8. If not set, defaults to an empty description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Number of policy tags contained in this taxonomy.
    #[prost(int32, tag = "4")]
    pub policy_tag_count: i32,
    /// Output only. Timestamps about this taxonomy. Only create_time and update_time are used.
    #[prost(message, optional, tag = "5")]
    pub taxonomy_timestamps: ::core::option::Option<SystemTimestamps>,
    /// Optional. A list of policy types that are activated for this taxonomy. If not set,
    /// defaults to an empty list.
    #[prost(
        enumeration = "taxonomy::PolicyType",
        repeated,
        packed = "false",
        tag = "6"
    )]
    pub activated_policy_types: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `Taxonomy`.
pub mod taxonomy {
    /// Defines policy types where the policy tags can be used for.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyType {
        /// Unspecified policy type.
        Unspecified = 0,
        /// Fine-grained access control policy, which enables access control on
        /// tagged sub-resources.
        FineGrainedAccessControl = 1,
    }
}
/// Denotes one policy tag in a taxonomy (e.g. ssn). Policy tags can be defined
/// in a hierarchy. For example, consider the following hierarchy:
///
/// ```
/// + Geolocation
///   + LatLong
///   + City
///   + ZipCode
/// ```
///
/// Policy tag "Geolocation" contains 3 child policy tags: "LatLong", "City", and
/// "ZipCode".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTag {
    /// Output only. Resource name of this policy tag in format:
    /// "projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policy_tag_id}".
    /// Both taxonomy_ids and policy_tag_ids are unique and generated by Policy Tag
    /// Manager.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. User-defined name of this policy tag. It must: be unique within the parent
    /// taxonomy; contain only unicode letters, numbers, underscores, dashes and
    /// spaces; not start or end with spaces; and be at most 200 bytes long when
    /// encoded in UTF-8.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of this policy tag. It must: contain only unicode characters,
    /// tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes
    /// long when encoded in UTF-8. If not set, defaults to an empty description.
    /// If not set, defaults to an empty description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Resource name of this policy tag's parent policy tag (e.g. for the
    /// "LatLong" policy tag in the example above, this field contains the
    /// resource name of the "Geolocation" policy tag). If empty, it means this
    /// policy tag is a top level policy tag (e.g. this field is empty for the
    /// "Geolocation" policy tag in the example above). If not set, defaults to an
    /// empty string.
    #[prost(string, tag = "4")]
    pub parent_policy_tag: ::prost::alloc::string::String,
    /// Output only. Resource names of child policy tags of this policy tag.
    #[prost(string, repeated, tag = "5")]
    pub child_policy_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CreateTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.CreateTaxonomy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaxonomyRequest {
    /// Required. Resource name of the project that the taxonomy will belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The taxonomy to be created.
    #[prost(message, optional, tag = "2")]
    pub taxonomy: ::core::option::Option<Taxonomy>,
}
/// Request message for
/// [DeleteTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.DeleteTaxonomy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaxonomyRequest {
    /// Required. Resource name of the taxonomy to be deleted. All policy tags in
    /// this taxonomy will also be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [UpdateTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.UpdateTaxonomy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaxonomyRequest {
    /// The taxonomy to update. Only description, display_name, and activated
    /// policy types can be updated.
    #[prost(message, optional, tag = "1")]
    pub taxonomy: ::core::option::Option<Taxonomy>,
    /// The update mask applies to the resource. For the `FieldMask` definition,
    /// see
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask
    /// If not set, defaults to all of the fields that are allowed to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [ListTaxonomies][google.cloud.datacatalog.v1.PolicyTagManager.ListTaxonomies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTaxonomiesRequest {
    /// Required. Resource name of the project to list the taxonomies of.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Must be a value between 1 and 1000.
    /// If not set, defaults to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any. If
    /// not set, defaults to an empty string.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [ListTaxonomies][google.cloud.datacatalog.v1.PolicyTagManager.ListTaxonomies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTaxonomiesResponse {
    /// Taxonomies that the project contains.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<Taxonomy>,
    /// Token used to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [GetTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.GetTaxonomy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaxonomyRequest {
    /// Required. Resource name of the requested taxonomy.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CreatePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.CreatePolicyTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyTagRequest {
    /// Required. Resource name of the taxonomy that the policy tag will belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The policy tag to be created.
    #[prost(message, optional, tag = "2")]
    pub policy_tag: ::core::option::Option<PolicyTag>,
}
/// Request message for
/// [DeletePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.DeletePolicyTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyTagRequest {
    /// Required. Resource name of the policy tag to be deleted. All of its descendant
    /// policy tags will also be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [UpdatePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.UpdatePolicyTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyTagRequest {
    /// The policy tag to update. Only the description, display_name, and
    /// parent_policy_tag fields can be updated.
    #[prost(message, optional, tag = "1")]
    pub policy_tag: ::core::option::Option<PolicyTag>,
    /// The update mask applies to the resource. Only display_name, description and
    /// parent_policy_tag can be updated and thus can be listed in the mask. If
    /// update_mask is not provided, all allowed fields (i.e. display_name,
    /// description and parent) will be updated. For more information including the
    /// `FieldMask` definition, see
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask
    /// If not set, defaults to all of the fields that are allowed to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [ListPolicyTags][google.cloud.datacatalog.v1.PolicyTagManager.ListPolicyTags].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyTagsRequest {
    /// Required. Resource name of the taxonomy to list the policy tags of.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Must be a value between 1 and 1000.
    /// If not set, defaults to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any. If
    /// not set, defaults to an empty string.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [ListPolicyTags][google.cloud.datacatalog.v1.PolicyTagManager.ListPolicyTags].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyTagsResponse {
    /// The policy tags that are in the requested taxonomy.
    #[prost(message, repeated, tag = "1")]
    pub policy_tags: ::prost::alloc::vec::Vec<PolicyTag>,
    /// Token used to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [GetPolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.GetPolicyTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyTagRequest {
    /// Required. Resource name of the requested policy tag.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod policy_tag_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Policy Tag Manager API service allows clients to manage their policy tags and"]
    #[doc = " taxonomies."]
    #[doc = ""]
    #[doc = " Policy tags are used to tag BigQuery columns and apply additional access"]
    #[doc = " control policies. A taxonomy is a hierarchical grouping of policy tags that"]
    #[doc = " classify data along a common axis."]
    #[derive(Debug, Clone)]
    pub struct PolicyTagManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PolicyTagManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PolicyTagManagerClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PolicyTagManagerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Creates a taxonomy in a specified project. The taxonomy is initially empty,"]
        #[doc = " i.e., does not contain policy tags."]
        pub async fn create_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaxonomyRequest>,
        ) -> Result<tonic::Response<super::Taxonomy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/CreateTaxonomy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a taxonomy. This method will also delete all policy tags in this"]
        #[doc = " taxonomy, their associated policies, and the policy tags references from"]
        #[doc = " BigQuery columns."]
        pub async fn delete_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTaxonomyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/DeleteTaxonomy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a taxonomy. This method can update the taxonomy's display name,"]
        #[doc = " description, and activated policy types."]
        pub async fn update_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaxonomyRequest>,
        ) -> Result<tonic::Response<super::Taxonomy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/UpdateTaxonomy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all taxonomies in a project in a particular location that the caller"]
        #[doc = " has permission to view."]
        pub async fn list_taxonomies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTaxonomiesRequest>,
        ) -> Result<tonic::Response<super::ListTaxonomiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/ListTaxonomies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a taxonomy."]
        pub async fn get_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaxonomyRequest>,
        ) -> Result<tonic::Response<super::Taxonomy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/GetTaxonomy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a policy tag in a taxonomy."]
        pub async fn create_policy_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePolicyTagRequest>,
        ) -> Result<tonic::Response<super::PolicyTag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/CreatePolicyTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a policy tag. This method also deletes:"]
        #[doc = ""]
        #[doc = " * all of its descendant policy tags, if any"]
        #[doc = " * the policies associated with the policy tag and its descendants"]
        #[doc = " * references from BigQuery table schema of the policy tag and its"]
        #[doc = "   descendants."]
        pub async fn delete_policy_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyTagRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/DeletePolicyTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a policy tag. This method can update the policy tag's display"]
        #[doc = " name, description, and parent policy tag."]
        pub async fn update_policy_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePolicyTagRequest>,
        ) -> Result<tonic::Response<super::PolicyTag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/UpdatePolicyTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all policy tags in a taxonomy."]
        pub async fn list_policy_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPolicyTagsRequest>,
        ) -> Result<tonic::Response<super::ListPolicyTagsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/ListPolicyTags",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a policy tag."]
        pub async fn get_policy_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyTagRequest>,
        ) -> Result<tonic::Response<super::PolicyTag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManager/GetPolicyTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the IAM policy for a policy tag or a taxonomy."]
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the IAM policy for a policy tag or a taxonomy."]
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the permissions that a caller has on a specified policy tag or"]
        #[doc = " taxonomy."]
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Message representing a taxonomy, including its policy tags in hierarchy, as a
/// nested proto. Used for taxonomy replacement, import, and export.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedTaxonomy {
    /// Required. Display name of the taxonomy. At most 200 bytes when encoded in UTF-8.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the serialized taxonomy. At most 2000 bytes when encoded in
    /// UTF-8. If not set, defaults to an empty description.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Top level policy tags associated with the taxonomy, if any.
    #[prost(message, repeated, tag = "3")]
    pub policy_tags: ::prost::alloc::vec::Vec<SerializedPolicyTag>,
    /// A list of policy types that are activated per taxonomy.
    #[prost(enumeration = "taxonomy::PolicyType", repeated, tag = "4")]
    pub activated_policy_types: ::prost::alloc::vec::Vec<i32>,
}
/// Message representing one policy tag, including all its descendant policy
/// tags, as a nested proto.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedPolicyTag {
    /// Resource name of the policy tag.
    ///
    /// This field will be ignored when calling ImportTaxonomies.
    #[prost(string, tag = "1")]
    pub policy_tag: ::prost::alloc::string::String,
    /// Required. Display name of the policy tag. At most 200 bytes when encoded in UTF-8.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the serialized policy tag. The length of the description is
    /// limited to 2000 bytes when encoded in UTF-8. If not set, defaults to an
    /// empty description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Children of the policy tag, if any.
    #[prost(message, repeated, tag = "4")]
    pub child_policy_tags: ::prost::alloc::vec::Vec<SerializedPolicyTag>,
}
/// Request message for
/// [ImportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ImportTaxonomies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTaxonomiesRequest {
    /// Required. Resource name of project that the imported taxonomies will belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Source taxonomies to be imported.
    #[prost(oneof = "import_taxonomies_request::Source", tags = "2, 3")]
    pub source: ::core::option::Option<import_taxonomies_request::Source>,
}
/// Nested message and enum types in `ImportTaxonomiesRequest`.
pub mod import_taxonomies_request {
    /// Source taxonomies to be imported.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Inline source used for taxonomies import.
        #[prost(message, tag = "2")]
        InlineSource(super::InlineSource),
        /// Cross-regional source taxonomy to be imported.
        #[prost(message, tag = "3")]
        CrossRegionalSource(super::CrossRegionalSource),
    }
}
/// Inline source containing taxonomies to import.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineSource {
    /// Required. Taxonomies to be imported.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<SerializedTaxonomy>,
}
/// Cross-regional source used to import an existing taxonomy into a different
/// region.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossRegionalSource {
    /// Required. The resource name of the source taxonomy to be imported.
    #[prost(string, tag = "1")]
    pub taxonomy: ::prost::alloc::string::String,
}
/// Response message for
/// [ImportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ImportTaxonomies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTaxonomiesResponse {
    /// Taxonomies that were imported.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<Taxonomy>,
}
/// Request message for
/// [ExportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ExportTaxonomies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTaxonomiesRequest {
    /// Required. Resource name of the project that the exported taxonomies belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Resource names of the taxonomies to be exported.
    #[prost(string, repeated, tag = "2")]
    pub taxonomies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Taxonomies export destination.
    #[prost(oneof = "export_taxonomies_request::Destination", tags = "3")]
    pub destination: ::core::option::Option<export_taxonomies_request::Destination>,
}
/// Nested message and enum types in `ExportTaxonomiesRequest`.
pub mod export_taxonomies_request {
    /// Required. Taxonomies export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Export taxonomies as serialized taxonomies, which contain all the policy
        /// tags as nested protos.
        #[prost(bool, tag = "3")]
        SerializedTaxonomies(bool),
    }
}
/// Response message for
/// [ExportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ExportTaxonomies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTaxonomiesResponse {
    /// List of taxonomies and policy tags as nested protos.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<SerializedTaxonomy>,
}
#[doc = r" Generated client implementations."]
pub mod policy_tag_manager_serialization_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Policy Tag Manager serialization API service allows clients to manipulate"]
    #[doc = " their policy tags and taxonomies in serialized format, where taxonomy is a"]
    #[doc = " hierarchical group of policy tags."]
    #[derive(Debug, Clone)]
    pub struct PolicyTagManagerSerializationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PolicyTagManagerSerializationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PolicyTagManagerSerializationClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PolicyTagManagerSerializationClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Creates new taxonomies (including their policy tags) by importing from"]
        #[doc = " inlined source or cross-regional source. New taxonomies will be created in"]
        #[doc = " a given parent project."]
        #[doc = ""]
        #[doc = " If using the cross-regional source, a new taxonomy is created by copying"]
        #[doc = " from a source in another region."]
        #[doc = ""]
        #[doc = " If using the inlined source, this method provides a way to bulk create"]
        #[doc = " taxonomies and policy tags using nested proto structure."]
        pub async fn import_taxonomies(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportTaxonomiesRequest>,
        ) -> Result<tonic::Response<super::ImportTaxonomiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManagerSerialization/ImportTaxonomies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports taxonomies as the requested type and returns the taxonomies"]
        #[doc = " including their policy tags. The requested taxonomies must belong to one"]
        #[doc = " project."]
        #[doc = ""]
        #[doc = " SerializedTaxonomy protos with nested policy tags that are generated by"]
        #[doc = " this method can be used as input for future ImportTaxonomies calls."]
        pub async fn export_taxonomies(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportTaxonomiesRequest>,
        ) -> Result<tonic::Response<super::ExportTaxonomiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.PolicyTagManagerSerialization/ExportTaxonomies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
