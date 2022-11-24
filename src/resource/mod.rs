use std::hash::Hash;

pub mod http_endpoint;

pub trait Resource: Eq + Hash {}
