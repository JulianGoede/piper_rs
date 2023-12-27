// This test should verify the pipeline macro will fail
// if any of the attributes name, retries, retry_delay_secs, cron
// wasn't provided
use pipeline::pipeline;

#[pipeline(retries => 3, retry_delay_secs => 120, cron => "*/5 * * * *")]
fn download_github_trends(ranking_url: String, day: String) -> Vec<String> {
    Vec::new()
}

fn main() {}
