use labeled::buckle::{Buckle, Component};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*************************************************
DENT OPEN
*************************************************/

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DentOpen {
    pub fd: u64,
    pub entry: Option<dent_open::Entry>
}


pub mod dent_open {
    #[derive(serde::Deserialize, serde::Serialize, Debug)]
        pub enum Entry {
        Name(String),
        Facet(super::Buckle)
    }
}

/*************************************************
DENT KIND
*************************************************/

pub enum DentKind {
    DentDirectory = 0,
    DentFile = 1,
    DentFacetedDirectory = 2,
    DentGate = 3,
    DentService = 4,
    DentBlob = 5
}

impl From<DentKind> for i32 {
    fn from(item: DentKind) -> Self {
        match item {
            DentKind::DentDirectory => 0,
            DentKind::DentFile => 1,
            DentKind::DentFacetedDirectory => 2,
            DentKind::DentGate => 3,
            DentKind::DentService => 4, 
            DentKind::DentBlob => 5
        }
    }
}

/*************************************************
DENT CREATE
*************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub struct DentCreate {
    pub label: Option<Buckle>,
    pub kind: Option<dent_create::Kind>
}

pub mod dent_create {
    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub enum Kind {
        Directory,
        File,
        FacetedDirectory,
        Gate(super::Gate),
        Service(super::Service),
        Blob(u64)
    }
}

/*************************************************
DENT UPDATE
*************************************************/
#[derive(Serialize, Deserialize, Debug)]
pub struct DentUpdate {
    pub fd: u64,
    pub kind: Option<dent_update::Kind>
}

pub mod dent_update {
    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub enum Kind {
        File(Vec<u8>),
        Gate(super::Gate),
        Service(super::Service),
        Blob(u64)
    }
}

/*************************************************
DENT LINK & DENT UNLINK
*************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub struct DentLink {
    pub dir_fd: u64,
    pub name: String,
    pub target_fd: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DentUnlink {
    pub dir_fd: u64,
    pub name: String
}

/*************************************************
DENT LIST FACETED
*************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub struct DentLsFaceted {
    pub fd: u64,
    pub clearance: Option<Buckle>
}

/*************************************************
DENT INVOKE
*************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub struct DentInvoke {
    pub fd: u64,
    pub sync: bool,
    pub payload: Vec<u8>,
    pub toblob: bool,
    pub parameters: std::collections::HashMap<String, String>
}

/*************************************************
GATES
*************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub struct Gate {
    pub kind: Option<gate::Kind>
}

pub mod gate {
    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub enum Kind {
        Direct(super::DirectGate),
        Redirect(super::RedirectGate)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectGate {
    pub privilege: Option<Component>,
    pub invoker_integrity_clearance: Option<Component>,
    pub function: Option<Function>,
    pub declassify: Option<Component>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedirectGate {
    pub privilege: Option<Component>,
    pub invoker_integrity_clearance: Option<Component>,
    pub gate: u64,
    pub declassify: Option<Component>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub privilege: Option<Component>,
    pub invoker_integrity_clearance: Option<Component>,
    pub taint: Option<Buckle>,
    pub url: String,
    pub verb: i32,
    pub headers: std::collections::HashMap<String, String>
}

/*************************************************
FUNCTION
*************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub memory: u64,
    pub app_image: u64,
    pub runtime_image: u64,
    pub kernel: u64
}

/*************************************************
HTTP VERB
*************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpVerb {
    HttpHead = 0,
    HttpGet = 1,
    HttpPost = 2,
    HttpPut = 3,
    HttpDelete = 4
}

impl Into<i32> for HttpVerb {
    fn into(self) -> i32 {
        match self {
            HttpVerb::HttpHead => 0,
            HttpVerb::HttpGet => 1,
            HttpVerb::HttpPost => 2,
            HttpVerb::HttpPut => 3,
            HttpVerb::HttpDelete => 4
        }
    }
}

/*************************************************
RESULTS
*************************************************/
#[derive(Serialize, Deserialize, Debug)]
pub struct DentResult {
    pub success: bool,
    pub fd: Option<u64>,
    pub data: Option<Vec<u8>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DentOpenResult {
    pub success: bool,
    pub fd: u64,
    pub kind: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DentListResult {
    pub success: bool,
    pub entries: HashMap<String, i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DentLsFacetedResult {
    pub success: bool,
    pub facets: Vec<Buckle>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DentLsGateResult {
    pub success: bool,
    pub gate: Option<Gate>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DentInvokeResult {
    pub success: bool,
    pub fd: Option<u64>,
    pub data: Vec<u8>,
    pub headers: std::collections::HashMap<String, Vec<u8>>
}
