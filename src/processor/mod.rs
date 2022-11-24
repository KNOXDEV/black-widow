use crate::resource::Resource;

mod crawl;

trait Processor {
    type Item: Resource;

    fn should_process(&self, resource: &Self::Item) -> bool;
    fn process(&self, resource: &mut Self::Item);
}
