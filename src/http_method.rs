#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

pub const GET: HttpMethod = HttpMethod::Get;
pub const HEAD: HttpMethod = HttpMethod::Head;
pub const POST: HttpMethod = HttpMethod::Post;
pub const PUT: HttpMethod = HttpMethod::Put;
pub const DELETE: HttpMethod = HttpMethod::Delete;
pub const CONNECT: HttpMethod = HttpMethod::Connect;
pub const OPTIONS: HttpMethod = HttpMethod::Options;
pub const TRACE: HttpMethod = HttpMethod::Trace;
pub const PATCH: HttpMethod = HttpMethod::Patch;
