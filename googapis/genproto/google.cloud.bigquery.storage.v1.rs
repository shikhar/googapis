/// Arrow schema as specified in
/// https://arrow.apache.org/docs/python/api/datatypes.html
/// and serialized to bytes using IPC:
/// https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc
///
/// See code samples on how this message can be deserialized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowSchema {
    /// IPC serialized Arrow schema.
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_schema: ::prost::alloc::vec::Vec<u8>,
}
/// Arrow RecordBatch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowRecordBatch {
    /// IPC-serialized Arrow RecordBatch.
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_record_batch: ::prost::alloc::vec::Vec<u8>,
    /// The count of rows in `serialized_record_batch`.
    #[prost(int64, tag = "2")]
    pub row_count: i64,
}
/// Contains options specific to Arrow Serialization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowSerializationOptions {
    /// The compression codec to use for Arrow buffers in serialized record
    /// batches.
    #[prost(
        enumeration = "arrow_serialization_options::CompressionCodec",
        tag = "2"
    )]
    pub buffer_compression: i32,
}
/// Nested message and enum types in `ArrowSerializationOptions`.
pub mod arrow_serialization_options {
    /// Compression codec's supported by Arrow.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompressionCodec {
        /// If unspecified no compression will be used.
        CompressionUnspecified = 0,
        /// LZ4 Frame (https://github.com/lz4/lz4/blob/dev/doc/lz4_Frame_format.md)
        Lz4Frame = 1,
        /// Zstandard compression.
        Zstd = 2,
    }
}
/// Avro schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroSchema {
    /// Json serialized schema, as described at
    /// https://avro.apache.org/docs/1.8.1/spec.html.
    #[prost(string, tag = "1")]
    pub schema: ::prost::alloc::string::String,
}
/// Avro rows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroRows {
    /// Binary serialized rows in a block.
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_binary_rows: ::prost::alloc::vec::Vec<u8>,
    /// The count of rows in the returning block.
    #[prost(int64, tag = "2")]
    pub row_count: i64,
}
/// Information about the ReadSession.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSession {
    /// Output only. Unique identifier for the session, in the form
    /// `projects/{project_id}/locations/{location}/sessions/{session_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time at which the session becomes invalid. After this time, subsequent
    /// requests to read this Session will return errors. The expire_time is
    /// automatically assigned and currently cannot be specified or updated.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Data format of the output data.
    #[prost(enumeration = "DataFormat", tag = "3")]
    pub data_format: i32,
    /// Immutable. Table that this ReadSession is reading from, in the form
    /// `projects/{project_id}/datasets/{dataset_id}/tables/{table_id}`
    #[prost(string, tag = "6")]
    pub table: ::prost::alloc::string::String,
    /// Optional. Any modifiers which are applied when reading from the specified table.
    #[prost(message, optional, tag = "7")]
    pub table_modifiers: ::core::option::Option<read_session::TableModifiers>,
    /// Optional. Read options for this session (e.g. column selection, filters).
    #[prost(message, optional, tag = "8")]
    pub read_options: ::core::option::Option<read_session::TableReadOptions>,
    /// Output only. A list of streams created with the session.
    ///
    /// At least one stream is created with the session. In the future, larger
    /// request_stream_count values *may* result in this list being unpopulated,
    /// in that case, the user will need to use a List method to get the streams
    /// instead, which is not yet available.
    #[prost(message, repeated, tag = "10")]
    pub streams: ::prost::alloc::vec::Vec<ReadStream>,
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields.
    #[prost(oneof = "read_session::Schema", tags = "4, 5")]
    pub schema: ::core::option::Option<read_session::Schema>,
}
/// Nested message and enum types in `ReadSession`.
pub mod read_session {
    /// Additional attributes when reading a table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableModifiers {
        /// The snapshot time of the table. If not set, interpreted as now.
        #[prost(message, optional, tag = "1")]
        pub snapshot_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Options dictating how we read a table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableReadOptions {
        /// Names of the fields in the table that should be read. If empty, all
        /// fields will be read. If the specified field is a nested field, all
        /// the sub-fields in the field will be selected. The output field order is
        /// unrelated to the order of fields in selected_fields.
        #[prost(string, repeated, tag = "1")]
        pub selected_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// SQL text filtering statement, similar to a WHERE clause in a query.
        /// Aggregates are not supported.
        ///
        /// Examples: "int_field > 5"
        ///           "date_field = CAST('2014-9-27' as DATE)"
        ///           "nullable_field is not NULL"
        ///           "st_equals(geo_field, st_geofromtext("POINT(2, 2)"))"
        ///           "numeric_field BETWEEN 1.0 AND 5.0"
        ///
        /// Restricted to a maximum length for 1 MB.
        #[prost(string, tag = "2")]
        pub row_restriction: ::prost::alloc::string::String,
        #[prost(
            oneof = "table_read_options::OutputFormatSerializationOptions",
            tags = "3"
        )]
        pub output_format_serialization_options:
            ::core::option::Option<table_read_options::OutputFormatSerializationOptions>,
    }
    /// Nested message and enum types in `TableReadOptions`.
    pub mod table_read_options {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OutputFormatSerializationOptions {
            /// Optional. Options specific to the Apache Arrow output format.
            #[prost(message, tag = "3")]
            ArrowSerializationOptions(super::super::ArrowSerializationOptions),
        }
    }
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// Output only. Avro schema.
        #[prost(message, tag = "4")]
        AvroSchema(super::AvroSchema),
        /// Output only. Arrow schema.
        #[prost(message, tag = "5")]
        ArrowSchema(super::ArrowSchema),
    }
}
/// Information about a single stream that gets data out of the storage system.
/// Most of the information about `ReadStream` instances is aggregated, making
/// `ReadStream` lightweight.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStream {
    /// Output only. Name of the stream, in the form
    /// `projects/{project_id}/locations/{location}/sessions/{session_id}/streams/{stream_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Data format for input or output data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataFormat {
    Unspecified = 0,
    /// Avro is a standard open source row based file format.
    /// See https://avro.apache.org/ for more details.
    Avro = 1,
    /// Arrow is a standard open source column-based message format.
    /// See https://arrow.apache.org/ for more details.
    Arrow = 2,
}
/// Request message for `CreateReadSession`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReadSessionRequest {
    /// Required. The request project that owns the session, in the form of
    /// `projects/{project_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Session to be created.
    #[prost(message, optional, tag = "2")]
    pub read_session: ::core::option::Option<ReadSession>,
    /// Max initial number of streams. If unset or zero, the server will
    /// provide a value of streams so as to produce reasonable throughput. Must be
    /// non-negative. The number of streams may be lower than the requested number,
    /// depending on the amount parallelism that is reasonable for the table. Error
    /// will be returned if the max count is greater than the current system
    /// max limit of 1,000.
    ///
    /// Streams must be read starting from offset 0.
    #[prost(int32, tag = "3")]
    pub max_stream_count: i32,
}
/// Request message for `ReadRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsRequest {
    /// Required. Stream to read rows from.
    #[prost(string, tag = "1")]
    pub read_stream: ::prost::alloc::string::String,
    /// The offset requested must be less than the last row read from Read.
    /// Requesting a larger offset is undefined. If not specified, start reading
    /// from offset zero.
    #[prost(int64, tag = "2")]
    pub offset: i64,
}
/// Information on if the current connection is being throttled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThrottleState {
    /// How much this connection is being throttled. Zero means no throttling,
    /// 100 means fully throttled.
    #[prost(int32, tag = "1")]
    pub throttle_percent: i32,
}
/// Estimated stream statistics for a given Stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStats {
    /// Represents the progress of the current stream.
    #[prost(message, optional, tag = "2")]
    pub progress: ::core::option::Option<stream_stats::Progress>,
}
/// Nested message and enum types in `StreamStats`.
pub mod stream_stats {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Progress {
        /// The fraction of rows assigned to the stream that have been processed by
        /// the server so far, not including the rows in the current response
        /// message.
        ///
        /// This value, along with `at_response_end`, can be used to interpolate
        /// the progress made as the rows in the message are being processed using
        /// the following formula: `at_response_start + (at_response_end -
        /// at_response_start) * rows_processed_from_response / rows_in_response`.
        ///
        /// Note that if a filter is provided, the `at_response_end` value of the
        /// previous response may not necessarily be equal to the
        /// `at_response_start` value of the current response.
        #[prost(double, tag = "1")]
        pub at_response_start: f64,
        /// Similar to `at_response_start`, except that this value includes the
        /// rows in the current response.
        #[prost(double, tag = "2")]
        pub at_response_end: f64,
    }
}
/// Response from calling `ReadRows` may include row data, progress and
/// throttling information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsResponse {
    /// Number of serialized rows in the rows block.
    #[prost(int64, tag = "6")]
    pub row_count: i64,
    /// Statistics for the stream.
    #[prost(message, optional, tag = "2")]
    pub stats: ::core::option::Option<StreamStats>,
    /// Throttling state. If unset, the latest response still describes
    /// the current throttling status.
    #[prost(message, optional, tag = "5")]
    pub throttle_state: ::core::option::Option<ThrottleState>,
    /// Row data is returned in format specified during session creation.
    #[prost(oneof = "read_rows_response::Rows", tags = "3, 4")]
    pub rows: ::core::option::Option<read_rows_response::Rows>,
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields. This schema is equivelant to the one returned by
    /// CreateSession. This field is only populated in the first ReadRowsResponse
    /// RPC.
    #[prost(oneof = "read_rows_response::Schema", tags = "7, 8")]
    pub schema: ::core::option::Option<read_rows_response::Schema>,
}
/// Nested message and enum types in `ReadRowsResponse`.
pub mod read_rows_response {
    /// Row data is returned in format specified during session creation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rows {
        /// Serialized row data in AVRO format.
        #[prost(message, tag = "3")]
        AvroRows(super::AvroRows),
        /// Serialized row data in Arrow RecordBatch format.
        #[prost(message, tag = "4")]
        ArrowRecordBatch(super::ArrowRecordBatch),
    }
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields. This schema is equivelant to the one returned by
    /// CreateSession. This field is only populated in the first ReadRowsResponse
    /// RPC.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// Output only. Avro schema.
        #[prost(message, tag = "7")]
        AvroSchema(super::AvroSchema),
        /// Output only. Arrow schema.
        #[prost(message, tag = "8")]
        ArrowSchema(super::ArrowSchema),
    }
}
/// Request message for `SplitReadStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitReadStreamRequest {
    /// Required. Name of the stream to split.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A value in the range (0.0, 1.0) that specifies the fractional point at
    /// which the original stream should be split. The actual split point is
    /// evaluated on pre-filtered rows, so if a filter is provided, then there is
    /// no guarantee that the division of the rows between the new child streams
    /// will be proportional to this fractional value. Additionally, because the
    /// server-side unit for assigning data is collections of rows, this fraction
    /// will always map to a data storage boundary on the server side.
    #[prost(double, tag = "2")]
    pub fraction: f64,
}
/// Response message for `SplitReadStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitReadStreamResponse {
    /// Primary stream, which contains the beginning portion of
    /// |original_stream|. An empty value indicates that the original stream can no
    /// longer be split.
    #[prost(message, optional, tag = "1")]
    pub primary_stream: ::core::option::Option<ReadStream>,
    /// Remainder stream, which contains the tail of |original_stream|. An empty
    /// value indicates that the original stream can no longer be split.
    #[prost(message, optional, tag = "2")]
    pub remainder_stream: ::core::option::Option<ReadStream>,
}
#[doc = r" Generated client implementations."]
pub mod big_query_read_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " BigQuery Read API."]
    #[doc = ""]
    #[doc = " The Read API can be used to read data from BigQuery."]
    #[derive(Debug, Clone)]
    pub struct BigQueryReadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigQueryReadClient<T>
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
        ) -> BigQueryReadClient<InterceptedService<T, F>>
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
            BigQueryReadClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a new read session. A read session divides the contents of a"]
        #[doc = " BigQuery table into one or more streams, which can then be used to read"]
        #[doc = " data from the table. The read session also specifies properties of the"]
        #[doc = " data to be read, such as a list of columns or a push-down filter describing"]
        #[doc = " the rows to be returned."]
        #[doc = ""]
        #[doc = " A particular row can be read by at most one stream. When the caller has"]
        #[doc = " reached the end of each stream in the session, then all the data in the"]
        #[doc = " table has been read."]
        #[doc = ""]
        #[doc = " Data is assigned to each stream such that roughly the same number of"]
        #[doc = " rows can be read from each stream. Because the server-side unit for"]
        #[doc = " assigning data is collections of rows, the API does not guarantee that"]
        #[doc = " each stream will return the same number or rows. Additionally, the"]
        #[doc = " limits are enforced based on the number of pre-filtered rows, so some"]
        #[doc = " filters can lead to lopsided assignments."]
        #[doc = ""]
        #[doc = " Read sessions automatically expire 24 hours after they are created and do"]
        #[doc = " not require manual clean-up by the caller."]
        pub async fn create_read_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReadSessionRequest>,
        ) -> Result<tonic::Response<super::ReadSession>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryRead/CreateReadSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reads rows from the stream in the format prescribed by the ReadSession."]
        #[doc = " Each response contains one or more table rows, up to a maximum of 100 MiB"]
        #[doc = " per response; read requests which attempt to read individual rows larger"]
        #[doc = " than 100 MiB will fail."]
        #[doc = ""]
        #[doc = " Each request also returns a set of stream statistics reflecting the current"]
        #[doc = " state of the stream."]
        pub async fn read_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRowsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadRowsResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryRead/ReadRows",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Splits a given `ReadStream` into two `ReadStream` objects. These"]
        #[doc = " `ReadStream` objects are referred to as the primary and the residual"]
        #[doc = " streams of the split. The original `ReadStream` can still be read from in"]
        #[doc = " the same manner as before. Both of the returned `ReadStream` objects can"]
        #[doc = " also be read from, and the rows returned by both child streams will be"]
        #[doc = " the same as the rows read from the original stream."]
        #[doc = ""]
        #[doc = " Moreover, the two child streams will be allocated back-to-back in the"]
        #[doc = " original `ReadStream`. Concretely, it is guaranteed that for streams"]
        #[doc = " original, primary, and residual, that original[0-j] = primary[0-j] and"]
        #[doc = " original[j-n] = residual[0-m] once the streams have been read to"]
        #[doc = " completion."]
        pub async fn split_read_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::SplitReadStreamRequest>,
        ) -> Result<tonic::Response<super::SplitReadStreamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryRead/SplitReadStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
