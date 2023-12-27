// This test looks a 
use pipeline::pipeline;

#[pipeline(name => "GithubTrends", retries => 3, retry_delay_secs => 120, cron => "*/5 * * * *")]
fn download_github_trends(ranking_url: String, day: String) -> Vec<String> {
    Vec::new()
}


fn main() {
    let pipeline = GithubTrends::new();
    assert_eq!(pipeline.retries, 3);
    assert_eq!(pipeline.retry_delay_secs, 120);
    assert_eq!(pipeline.cron, "*/5 * * * *".to_string());
}
