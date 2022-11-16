mod crawl;

trait Processor {
    type Resource;

    fn push_resource(&mut self, resource: Self::Resource);
}
