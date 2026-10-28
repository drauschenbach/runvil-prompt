# Runvil Prompt

My first [Runvil](https://github.com/cppalliance/runvil) prompt.

## Configure the Gateway

Configure the gateway to support the Claude Opus 5.5 model via the Anthropic cloud provider. First,
get yourself an `ANTHROPIC_API_KEY`, and put it in `~/.runvil/gateway.env`:

```toml
ANTHROPIC_API_KEY=__REDACTED__
```

Next register the Claude Opus 5.5 model in `~/.runvil/gateway.toml`:

```toml
[[endpoint]]
api_key = "${ANTHROPIC_API_KEY}"
base_url = "https://api.anthropic.com/v1"
id = "anthropic"
protocol = "openai"

[[model]]
adaptive_thinking = true
context = 1000000
description = "Claude Opus 5.5"
effort_levels = [
    "low",
    "medium",
    "high",
    "xhigh",
    "max",
]
endpoints = ["anthropic"]
images = true
kind = "chat"
max_output = 128000
name = "Claude Opus 5.5"
thinking = "always"
upstream = "claude-opus-5-5"
```

## Usage

Launch the gateway:

```shell
$ cd runvil
$ cargo run -p gateway -- --print-url
http://127.0.0.1:50315/auth?key=__REDACTED__
```

Then run this program:

```shell
$ GATEWAY_URL=http://127.0.0.1:50315 GATEWAY_SECRET=__REDACTED__ cargo run
Starting
Report: Err(Model(CatalogFetchFailed(CompletionError { kind: Rejected, message: "the model backend rejected the request (status 404)", prompt_tokens: None, window: None, finish_reason: None, detail: Some("(empty body)"), source: None })))
Finished
```
