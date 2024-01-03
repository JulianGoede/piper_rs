use schema::{Pipeline, Scheduler};
// use std::any::Any;
use std::marker::PhantomData;

pub struct Coordinator<P, S>
where
    P: Pipeline<S>,
{
    phantom: PhantomData<S>,
    pipelines: Vec<P>,
}

impl<P, S> Scheduler<P, S> for Coordinator<P, S>
where
    P: Pipeline<S>,
{
    fn new() -> Self {
        Coordinator { phantom: PhantomData, pipelines: vec![] }
    }

    fn register(&mut self, pipeline: P) where P: Pipeline<S> {
        self.pipelines.push(pipeline);
    }
    // fn new() -> Self {
    //     Coordinator { pipelines: vec![] }
    // }
    //
    // fn register(&mut self, pipeline: P)
    // where
    //     P: Pipeline<&'static dyn std::any::Any>,
    // {
    //     self.pipelines.push(pipeline);
    // }
}
