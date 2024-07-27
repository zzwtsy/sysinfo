// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerHost {
    #[prost(string, tag = "1")]
    pub os_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub os_version: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub distribution_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub kernel_version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub cpu: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "6")]
    pub cpu_cores: u32,
    #[prost(uint64, tag = "7")]
    pub mem_total: u64,
    #[prost(uint64, tag = "8")]
    pub disk_total: u64,
    #[prost(uint64, tag = "9")]
    pub swap_total: u64,
    #[prost(string, tag = "10")]
    pub arch: ::prost::alloc::string::String,
    #[prost(uint64, tag = "11")]
    pub boot_time: u64,
    #[prost(string, tag = "12")]
    pub ipv4: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub ipv6: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub country_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ServerState {
    #[prost(float, tag = "1")]
    pub cpu_usage: f32,
    #[prost(uint64, tag = "2")]
    pub mem_used: u64,
    #[prost(uint64, tag = "3")]
    pub swap_used: u64,
    #[prost(uint64, tag = "4")]
    pub disk_used: u64,
    #[prost(uint64, tag = "5")]
    pub net_in_transfer: u64,
    #[prost(uint64, tag = "6")]
    pub net_out_transfer: u64,
    #[prost(uint64, tag = "7")]
    pub net_in_speed: u64,
    #[prost(uint64, tag = "8")]
    pub net_out_speed: u64,
    #[prost(double, tag = "9")]
    pub load1: f64,
    #[prost(double, tag = "10")]
    pub load5: f64,
    #[prost(double, tag = "11")]
    pub load15: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStateRequest {
    #[prost(message, optional, tag = "1")]
    pub server_state: ::core::option::Option<ServerState>,
    #[prost(uint64, tag = "2")]
    pub upload_time: u64,
    #[prost(string, tag = "3")]
    pub agent_version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub server_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerHostRequest {
    #[prost(message, optional, tag = "1")]
    pub server_host: ::core::option::Option<ServerHost>,
    #[prost(uint64, tag = "2")]
    pub upload_time: u64,
    #[prost(string, tag = "3")]
    pub agent_version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub server_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIpRequest {
    #[prost(string, tag = "1")]
    pub ipv4: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ipv6: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub country_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub server_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ServerResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// Generated client implementations.
pub mod server_monitor_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ServerMonitorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServerMonitorServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ServerMonitorServiceClient<T>
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
        ) -> ServerMonitorServiceClient<InterceptedService<T, F>>
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
            ServerMonitorServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn report_server_host(
            &mut self,
            request: impl tonic::IntoRequest<super::ServerHostRequest>,
        ) -> std::result::Result<tonic::Response<super::ServerResponse>, tonic::Status> {
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
                "/server_monitor.ServerMonitorService/ReportServerHost",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "server_monitor.ServerMonitorService",
                        "ReportServerHost",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn report_server_state(
            &mut self,
            request: impl tonic::IntoRequest<super::ServerStateRequest>,
        ) -> std::result::Result<tonic::Response<super::ServerResponse>, tonic::Status> {
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
                "/server_monitor.ServerMonitorService/ReportServerState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "server_monitor.ServerMonitorService",
                        "ReportServerState",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ip(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIpRequest>,
        ) -> std::result::Result<tonic::Response<super::ServerResponse>, tonic::Status> {
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
                "/server_monitor.ServerMonitorService/UpdateIP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("server_monitor.ServerMonitorService", "UpdateIP"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
