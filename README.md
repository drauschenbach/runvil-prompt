# Runvil Prompt

My first [Runvil](https://github.com/cppalliance/runvil) prompt.

## Configure the Gateway

Configure the gateway to listen on a stable port in `~/.runvil/gateway.toml`:

```toml
[server]
bind = "127.0.0.1:5000"
```

Next, configure the gateway to support the Claude Opus 5.5 model via the Anthropic cloud provider. First,
get yourself an `ANTHROPIC_API_KEY`, and put it in `~/.runvil/gateway.env`:

```toml
ANTHROPIC_API_KEY=__REDACTED__
```

Next register the Anthropic cloud provider endpoint in `~/.runvil/gateway.toml`:

```toml
[[endpoint]]
id = "anthropic"
protocol = "openai"
base_url = "https://api.anthropic.com/v1"
api_key = "${ANTHROPIC_API_KEY}"
```

Next register the Claude Opus 5.5 model in `~/.runvil/gateway.toml`:

```toml
[[model]]
name = "Claude Opus 5.5"
description = "Claude Opus 5.5"
upstream = "claude-opus-5-5"
endpoints = ["anthropic"]
context = 1000000
effort_levels = [
    "low",
    "medium",
    "high",
    "xhigh",
    "max",
]
images = true
kind = "chat"
max_output = 128000
thinking = "always"
adaptive_thinking = true
```

## Usage

Launch the gateway:

```shell
$ cd runvil
$ cargo run -p gateway -- --print-url
http://127.0.0.1:5000/auth?key=__REDACTED__
```

Then run this program:

```shell
$ GATEWAY_URL=http://127.0.0.1:5000 GATEWAY_SECRET=__REDACTED__ cargo run
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
