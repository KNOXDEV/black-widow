use crate::resource::Resource;
use tokio::sync::mpsc;

struct Controller {
    resources_new: (mpsc::Sender<Resource>, mpsc::Receiver<Resource>),
    resources_todo: (mpsc::Sender<Resource>, mpsc::Receiver<Resource>),
}

impl Controller {
    fn new() -> Self {
        // create the queues we'll be using to exchange resources with the workers
        let resources_new = mpsc::channel(10);
        let resources_todo = mpsc::channel(10);

        Controller {
            resources_new,
            resources_todo,
        }
    }

    fn execute(&mut self) {}
}
