use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryEvent {
    pub id: String,
    pub project: String,
    #[serde(rename = "project_name")]
    pub project_name: String,
    #[serde(rename = "project_slug")]
    pub project_slug: String,
    pub logger: Value,
    pub level: String,
    pub culprit: String,
    pub message: String,
    pub url: String,
    #[serde(rename = "triggering_rules")]
    pub triggering_rules: Vec<Value>,
    pub event: Event,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "event_id")]
    pub event_id: String,
    pub level: String,
    pub version: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub logentry: Logentry,
    pub logger: String,
    pub modules: Modules,
    pub platform: String,
    pub timestamp: f64,
    pub received: f64,
    pub environment: String,
    pub user: User,
    pub request: Request,
    pub contexts: Contexts,
    pub stacktrace: Stacktrace,
    pub tags: Vec<Vec<String>>,
    pub extra: Extra3,
    pub metadata: Metadata,
    pub fingerprint: Vec<String>,
    pub hashes: Vec<String>,
    pub culprit: String,
    pub title: String,
    pub location: Value,
    #[serde(rename = "_ref")]
    pub ref_field: i64,
    #[serde(rename = "_ref_version")]
    pub ref_version: i64,
    #[serde(rename = "_metrics")]
    pub metrics: Metrics,
    #[serde(rename = "nodestore_insert")]
    pub nodestore_insert: f64,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logentry {
    pub formatted: String,
    pub message: Value,
    pub params: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modules {
    #[serde(rename = "my.package")]
    pub my_package: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    pub username: String,
    pub name: String,
    pub geo: Geo,
    #[serde(rename = "sentry_user")]
    pub sentry_user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geo {
    #[serde(rename = "country_code")]
    pub country_code: String,
    pub city: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub url: String,
    pub method: String,
    pub data: Data,
    #[serde(rename = "query_string")]
    pub query_string: Vec<Vec<String>>,
    pub cookies: Vec<Vec<String>>,
    pub headers: Vec<Vec<String>>,
    pub env: Env,
    #[serde(rename = "inferred_content_type")]
    pub inferred_content_type: String,
    #[serde(rename = "api_target")]
    pub api_target: Value,
    pub fragment: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub hello: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    #[serde(rename = "ENV")]
    pub env: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contexts {
    pub browser: Browser,
    #[serde(rename = "client_os")]
    pub client_os: ClientOs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Browser {
    pub name: String,
    pub version: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientOs {
    pub name: String,
    pub version: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stacktrace {
    pub frames: Vec<Frame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub function: String,
    pub module: String,
    pub filename: String,
    #[serde(rename = "abs_path")]
    pub abs_path: String,
    pub lineno: i64,
    #[serde(rename = "pre_context")]
    pub pre_context: Vec<String>,
    #[serde(rename = "context_line")]
    pub context_line: String,
    #[serde(rename = "post_context")]
    pub post_context: Option<Vec<String>>,
    #[serde(rename = "in_app")]
    pub in_app: bool,
    pub vars: Vars,
    pub colno: Value,
    pub data: Value,
    pub errors: Value,
    #[serde(rename = "raw_function")]
    pub raw_function: Value,
    #[serde(rename = "image_addr")]
    pub image_addr: Value,
    #[serde(rename = "instruction_addr")]
    pub instruction_addr: Value,
    #[serde(rename = "addr_mode")]
    pub addr_mode: Value,
    pub package: Value,
    pub platform: Value,
    #[serde(rename = "source_link")]
    pub source_link: Value,
    pub symbol: Value,
    #[serde(rename = "symbol_addr")]
    pub symbol_addr: Value,
    pub trust: Value,
    pub snapshot: Value,
    pub lock: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vars {
    #[serde(rename = "'culprit'")]
    pub culprit: Value,
    #[serde(rename = "'data'")]
    pub data: Option<Data2>,
    #[serde(rename = "'date'")]
    pub date: Option<String>,
    #[serde(rename = "'event_id'")]
    pub event_id: Option<String>,
    #[serde(rename = "'event_type'")]
    pub event_type: Option<String>,
    #[serde(rename = "'extra'")]
    pub extra: Option<Extra>,
    #[serde(rename = "'frames'")]
    pub frames: Option<String>,
    #[serde(rename = "'handler'")]
    pub handler: Option<String>,
    #[serde(rename = "'k'")]
    pub k: Option<String>,
    #[serde(rename = "'kwargs'")]
    pub kwargs: Option<Kwargs>,
    #[serde(rename = "'public_key'")]
    pub public_key: Value,
    #[serde(rename = "'result'")]
    pub result: Option<Result>,
    #[serde(rename = "'self'")]
    pub self_field: Option<String>,
    #[serde(rename = "'stack'")]
    pub stack: Option<bool>,
    #[serde(rename = "'tags'")]
    pub tags: Value,
    #[serde(rename = "'time_spent'")]
    pub time_spent: Value,
    #[serde(rename = "'v'")]
    pub v: Option<V>,
    #[serde(rename = "'message'")]
    pub message: Option<String>,
    #[serde(rename = "'client'")]
    pub client: Option<String>,
    #[serde(rename = "'options'")]
    pub options: Option<Options>,
    #[serde(rename = "'args'")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "'dsn'")]
    pub dsn: Option<String>,
    #[serde(rename = "'opts'")]
    pub opts: Option<String>,
    #[serde(rename = "'parser'")]
    pub parser: Option<String>,
    #[serde(rename = "'root'")]
    pub root: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data2 {
    #[serde(rename = "'message'")]
    pub message: String,
    #[serde(rename = "'sentry.interfaces.Message'")]
    pub sentry_interfaces_message: SentryInterfacesMessage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryInterfacesMessage {
    #[serde(rename = "'message'")]
    pub message: String,
    #[serde(rename = "'params'")]
    pub params: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra {
    #[serde(rename = "'go_deeper'")]
    pub go_deeper: Vec<Vec<String>>,
    #[serde(rename = "'loadavg'")]
    pub loadavg: Vec<f64>,
    #[serde(rename = "'user'")]
    pub user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kwargs {
    #[serde(rename = "'level'")]
    pub level: i64,
    #[serde(rename = "'message'")]
    pub message: Option<String>,
    #[serde(rename = "'data'")]
    pub data: Value,
    #[serde(rename = "'extra'")]
    pub extra: Option<Extra2>,
    #[serde(rename = "'stack'")]
    pub stack: Option<bool>,
    #[serde(rename = "'tags'")]
    pub tags: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra2 {
    #[serde(rename = "'go_deeper'")]
    pub go_deeper: Vec<String>,
    #[serde(rename = "'loadavg'")]
    pub loadavg: Vec<f64>,
    #[serde(rename = "'user'")]
    pub user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    #[serde(rename = "'message'")]
    pub message: String,
    #[serde(rename = "'sentry.interfaces.Message'")]
    pub sentry_interfaces_message: SentryInterfacesMessage2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryInterfacesMessage2 {
    #[serde(rename = "'message'")]
    pub message: String,
    #[serde(rename = "'params'")]
    pub params: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V {
    #[serde(rename = "'message'")]
    pub message: String,
    #[serde(rename = "'params'")]
    pub params: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    #[serde(rename = "'data'")]
    pub data: Value,
    #[serde(rename = "'tags'")]
    pub tags: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra3 {
    pub empty_list: Vec<Value>,
    pub empty_map: EmptyMap,
    pub length: i64,
    pub results: Vec<i64>,
    pub session: Session,
    pub unauthorized: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyMap {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub foo: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub title: String,
    #[serde(rename = "in_app_frame_mix")]
    pub in_app_frame_mix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    #[serde(rename = "bytes.stored.event")]
    pub bytes_stored_event: i64,
}
