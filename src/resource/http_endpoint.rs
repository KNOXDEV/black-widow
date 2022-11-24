use crate::resource::Resource;
use url::Url;

#[derive(PartialEq, Eq, Hash)]
pub struct HttpEndpoint {
    url: Url,
}

impl Resource for HttpEndpoint {}
