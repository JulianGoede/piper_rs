// This test looks for a run method
// that should return schema::Succcess
// with the appropriate return values
use pipeline::pipeline;
use schema::RunResult;

#[pipeline(name => "GithubTrends", retries => 3, retry_delay_secs => 120, cron => "*/5 * * * *")]
fn download_github_trends(ranking_url: String, day: String, _unused_var: u32) -> Vec<String> {
    let mut trending_repos = Vec::new();
    trending_repos.push(ranking_url);
    trending_repos.push(day);
    trending_repos
}


fn main() {
    let pipeline = GithubTrends::new();
    let args: (String, String, u32) = ("foo".to_string(), "2077-01-01".to_string(), 42);
    let actual: RunResult<Vec<String>> = pipeline.run(&args);
    assert!(actual.is_ok());
    let actual = actual.unwrap();
    let expected =  vec!["foo".to_string(), "2077-01-01".to_string()];
    assert_eq!(actual, expected);
}
