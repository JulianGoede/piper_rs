// check if scheduler can register pipelines
use coordinator::Coordinator;
use schema::{Pipeline, Scheduler};

struct DummyPipeline {}

impl Pipeline<()> for DummyPipeline {
    fn new() -> Self {
        DummyPipeline {}
    }

    fn run(
        &mut self,
        _args: &dyn std::any::Any,
    ) -> schema::RunResult<(), Box<dyn std::error::Error>> {
        schema::RunResult::Ok(())
    }
}

fn main() {
    let pipeline: DummyPipeline = DummyPipeline::new();
    let mut coordinator: Coordinator<DummyPipeline, ()> = Coordinator::new();
    coordinator.register(pipeline);
}
