use crate::processor::Processor;
use crate::resource::http_endpoint::HttpEndpoint;

struct CrawlProcessor {}

impl Processor for CrawlProcessor {
    type Item = HttpEndpoint;

    fn should_process(&self, resource: &Self::Item) -> bool {
        todo!()
    }

    fn process(&self, resource: &mut Self::Item) {
        todo!()
    }
}
