// check if scheduler can register pipelines
use coordinator::Coordinator;
use schema::{Pipeline, Scheduler};

struct StringPipeline {value: String}

impl Pipeline<String> for StringPipeline {
    fn new() -> Self {
        StringPipeline {value: "Hello World".to_string()}
    }

    fn run(
        &mut self,
        _args: &dyn std::any::Any,
    ) -> schema::RunResult<String, Box<dyn std::error::Error>> {
        schema::RunResult::Ok(self.value.clone())
    }
}

struct IntegerPipeline {x: u32}

impl Pipeline<u32> for IntegerPipeline {
    fn new() -> Self {
        IntegerPipeline {x: 42}
    }

    fn run(
        &mut self,
        _args: &dyn std::any::Any,
    ) -> schema::RunResult<u32, Box<dyn std::error::Error>> {
        schema::RunResult::Ok(self.x)
    }
}

fn main() {
    let string_pipeline: StringPipeline = StringPipeline::new();
    let integer_pipeline: IntegerPipeline = IntegerPipeline::new();
    let mut coordinator = Coordinator::new();
    coordinator.register(string_pipeline);
    coordinator.register(integer_pipeline);
}
