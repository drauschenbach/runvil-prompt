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
[my run request] Prompt: parse_started
[my run request] Say hi: lua_compilation_started
[my run request] Say hi: lua_compilation_succeeded
[my run request] Prompt: parse_succeeded
[my run request] Greeter: run_started
[my run request] Say hi: section_started
[my run request] Say hi: lua_chunk_started
[my run request] Say hi: lua_chunk_succeeded
[my run request] Say hi: lua_teardown_started
[my run request] Say hi: lua_teardown_succeeded
[my run request] Say hi: section_finished
[my run request] Greeter: run_succeeded
Report: Ok(RunReport { run_id: Some(RunId(1)), outcome: Completed { final_text: "Say hello." }, output: Err(Undeclared) })
Finished
```
