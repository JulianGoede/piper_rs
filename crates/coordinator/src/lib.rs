use schema::{Pipeline, Scheduler};
use std::any::Any;

pub struct Coordinator {
    pipelines: Vec<Box<dyn Any>>,
}

impl Scheduler for Coordinator {
    fn new() -> Self {
        Coordinator { pipelines: vec![] }
    }

    fn register<T: 'static, E: 'static>(&mut self, pipeline: impl Pipeline<T, E> + 'static) {
        self.pipelines.push(Box::new(pipeline) as Box<dyn Any>);
    }
}
