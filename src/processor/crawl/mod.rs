use crate::processor::Processor;

struct CrawlProcessor {}

impl Processor for CrawlProcessor {
    type Resource = ();

    fn push_resource(&mut self, resource: Self::Resource) {
        todo!()
    }
}
