# pipe_rs

An etl pipeline framework written in rust.
This is a proof-of-conecept implementation to
bring etl tooling to rust.

## TDD-Developement 
I envision to implement features using test driven development
in a similar fashion as the great project (proc-macro-workshop)[https://github.com/dtolnay/proc-macro-workshop],
i.e. for each lib I want to first create a tests subdir, containing a sequence of tests 01-test-feature-a,
02-test-faiture-b, .. and a `progress.rs`  file which will call these tests files.

This way I can first design how I envision this project to work and then implement the features one-by-one,
