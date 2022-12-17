// use rand::{thread_rng, Rng};
// use reqwest::StatusCode;
// use std::collections::BTreeMap;
// use std::future::Future;
// use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
// use std::ops::Deref;
// use std::pin::Pin;
// use std::sync::atomic::Ordering::Relaxed;
// use std::sync::atomic::{AtomicU16, AtomicUsize};
// use std::sync::RwLock;
// use std::thread::JoinHandle;
// use tokio::task::JoinHandle;
//
// static SERVER: RwLock<Option<JoinHandle<()>>> = RwLock::new(None);
// static PORT: AtomicU16 = AtomicU16::new(10000);
//
// pub const BASE_URL: &'static str = "http://localhost:8080";
//
// #[derive(Clone, Debug)]
// pub struct File {
//     pub name: String,
//     pub content_type: String,
//     pub data: Vec<u8>,
// }
//
// impl From<File> for MultipartValues {
//     fn from(f: File) -> Self {
//         (f.name, f.content_type, f.data).into()
//     }
// }
//
// pub struct ServerInfo<T> {
//     addr: SocketAddr,
//     handle: JoinHandle<T>,
// }
//
// impl<T> ServerInfo<T> {
//     pub fn addr(&self) -> &SocketAddr {
//         &self.addr
//     }
//
//     pub fn join_handle(&self) -> &JoinHandle<T> {
//         &self.handle
//     }
//
//     pub fn kill_server(&self) {
//         let _ = &self.handle.abort();
//     }
//
//     pub fn base_url(&self) -> String {
//         let base_url = format!("http://{}", self.addr.to_string());
//         println!("Using following baseurl: {}", base_url);
//         base_url
//     }
// }
//
// pub fn rnd_bytes() -> Vec<u8> {
//     let mut rng = thread_rng();
//     [(); 256].map(|_| rng.gen::<u8>()).to_vec()
// }
//
// pub fn get_test_file(name: &str, ext: &str) -> File {
//     let content_type = match ext {
//         "mp4" => "video/mp4",
//         "png" => "image/png",
//         _ => "application/octet-stream",
//     };
//     File {
//         name: format!("{}_{}.{}", name, chrono::Utc::now().timestamp_nanos(), ext),
//         content_type: content_type.to_string(),
//         data: rnd_bytes(),
//     }
// }
//
// pub fn start_test_server() -> ServerInfo<()> {
//     let port = PORT.fetch_add(1, Relaxed);
//     let addr = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));
//     let listener =
//         TcpListener::bind(addr).expect("Already in use? Please confirm the port is available");
//
//     let _addr = listener.local_addr().unwrap();
//
//     let handle = tokio::spawn(async move {
//         let _server = axum::Server::from_tcp(listener)
//             .unwrap()
//             .serve(app().into_make_service())
//             .await
//             .unwrap();
//     });
//     ServerInfo { addr, handle }
// }
//
// pub trait TestTraitValidateResponse {
//     fn validate<Body: Into<Vec<u8>>>(
//         self,
//         status_code: u16,
//         body: Body,
//     ) -> Pin<Box<dyn Future<Output = Vec<u8>> + 'static>>
//     where
//         Self: 'static;
// }
//
// impl TestTraitValidateResponse for reqwest::Response {
//     fn validate<Body: Into<Vec<u8>>>(
//         self,
//         status_code: u16,
//         body: Body,
//     ) -> Pin<Box<dyn Future<Output = Vec<u8>> + 'static>>
//     where
//         Self: 'static,
//     {
//         let status_code = StatusCode::from_u16(status_code).unwrap().clone();
//         let body = body.into().clone();
//         Box::pin(async move {
//             let resp_status_code = self.status();
//             let resp_body = self.bytes().await.unwrap().to_vec();
//             println!(
//                 "Response: \n - Status Code: {}\n - Body: {:?}",
//                 resp_status_code, resp_body
//             );
//             assert_eq!(resp_status_code, status_code);
//             assert_eq!(resp_body, body);
//             resp_body
//         })
//     }
// }
//
// pub trait MultipartValueExtension {
//     fn multipart_value(self, values: BTreeMap<String, MultipartValues>) -> Self;
// }
//
// impl MultipartValueExtension for ::reqwest::RequestBuilder {
//     fn multipart_value(self, values: BTreeMap<String, MultipartValues>) -> Self {
//         let mut form = reqwest::multipart::Form::new();
//         for (key, value) in values {
//             let part_opt = match value {
//                 MultipartValues::Number(t) => Some(reqwest::multipart::Part::text(t.to_string())),
//                 MultipartValues::Text(t) => Some(reqwest::multipart::Part::text(t.to_string())),
//                 MultipartValues::Bool(t) => Some(reqwest::multipart::Part::text(t.to_string())),
//                 MultipartValues::Data(t) => Some(
//                     reqwest::multipart::Part::bytes(t.buffer().clone())
//                         .file_name(t.filename().clone())
//                         .mime_str(&*t.content_type().clone())
//                         .unwrap(),
//                 ),
//                 _ => None,
//             };
//             if let Some(part) = part_opt {
//                 form = form.part(key, part);
//             }
//         }
//
//         self.multipart(form)
//     }
// }
