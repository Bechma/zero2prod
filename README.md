## Zero To Production

This repository is a compilation of the end result code from Luca Palmieri "Zero To Production" books' serie. 
Quoted from his anouncement:

> I have been musing for a while over the idea of writing an end-to-end guide to building a production-ready(ish) API using Rust and its async ecosystem.\
\
> The idea is to write with a mid-level Rust practioner in mind: familiar with the language, probably a couple of toy experiments under their belt, willing to get started on a serious project. I am planning to cover:
> * Databases (setup, migration management, testing) using Postgres and Diesel.
> * Domain modelling, with an emphasis on type-driven development.
> * Error handling strategies (the most controversial topic).
> * API basics.
> * Observability (logging, tracing, metrics, error diagnostics).
> * CI setup and integration/black-box testing.
> * Benchmarking.

# Table of Contents

0. [Foreword](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/)
1. [Getting Started](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/)
2. [Our driving Example](https://www.lpalmieri.com/posts/2020-06-21-zero-to-production-2-learn-by-building-an-email-newsletter/)
3. Sign up a new subscriber:
    - [Choosing a web framework(in 2020)](https://lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)
    - [How To Bootstrap A Rust Web API From Scratch](https://lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/) (end code is in branch -> [chapter/3.0](https://github.com/Bechma/zero2prod/tree/chapter/3.0))
    - [HTML forms, Databases, Integration tests](https://lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/) (end code is in branch -> [chapter/3.5](https://github.com/Bechma/zero2prod/tree/chapter/3.5))
4. [Telemetry](https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/)
5. Go Live
6. Publish A Newsletter Issue
7. Reject Invalid Subscribers
8. Survive Delivery Failures
9. Send A Confirmation Email On Sign Up
10. Metrics
11. Send Emails Asynchronously
12. Fulfilling Email Tasks
13. Benchmarking

# Credits
All credits go to Luca Palmieri:

[![Twitter URL](https://img.shields.io/twitter/url?label=%40algo_luca&style=social&url=https%3A%2F%2Ftwitter.com%2Falgo_luca)](https://twitter.com/algo_luca)
[![Github URL](https://img.shields.io/twitter/url?label=%40LukeMathWalker&logo=github&style=social&url=https%3A%2F%2Fgithub.com%2FLukeMathWalker)](https://github.com/LukeMathWalker)
[![Linkedin URL](https://img.shields.io/twitter/url?label=%40LukeMathWalker&logo=linkedin&style=social&url=https%3A%2F%2Fwww.linkedin.com%2Fin%2Fluca-palmieri%2F)](https://www.linkedin.com/in/luca-palmieri/)
