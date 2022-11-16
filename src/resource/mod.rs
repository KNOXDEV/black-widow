mod http_endpoint;

// the smallest atom of information that we consider
trait Resource {
    fn get_tags() -> [String];
}
