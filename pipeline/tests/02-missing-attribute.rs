// This test looks for an attribute-like macro with the name "pipeline" to exist.
// This test will only ensure you can annotate a function with this macro and if
// it is syntactily correct
use pipeline::pipeline;

#[pipeline(retries => 3, retry_delay_secs => 120, cron => "*/5 * * * *")]
fn download_github_trends(ranking_url: String, day: String) -> Vec<String> {
    Vec::new()
}

fn main() {}
