# skctx

Simple kubernetes context manager

## Why?

- I wanted to be able to set a context per shell but without launching a nested shell
- I was looking for a simple project to study Rust

## how it works

- Contexts should be defined one per file in a predefined configs paths (default: ~/.kube/configs)
- `list`command outputs the available single-context config files
- `env CONTEXT_NAME`outputs env configuration instructions to set the context using KUBECONFIG envvar.
- To set a context, just use the following command:

  ```bash
  $ eval $(skctx env CONTEXT_NAME)
  ```
