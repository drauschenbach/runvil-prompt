# Runvil Prompt

My first [Runvil](https://github.com/cppalliance/runvil) prompt.

## Usage

First launch the gateway:

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
