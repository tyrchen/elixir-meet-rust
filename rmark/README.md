# Rmark

Markdown parser using [rustler](https://github.com/rusterlium/rustler) and [comrak](https://github.com/kivikakk/comrak). Compared with [earmark](https://github.com/pragdave/earmark), it is 2x faster. This is a demo project to show how to use rustler to boost elixir/erlang projects.

## Benchmark result

```bash
mix run benchmark/markdown.exs
Compiling NIF crate :rmark (native/rmark)...
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
Operating System: macOS
CPU Information: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz
Number of Available Cores: 16
Available memory: 64 GB
Elixir 1.10.2
Erlang 22.3.2

Benchmark suite executing with the following configuration:
warmup: 2 s
time: 5 s
memory time: 0 ns
parallel: 1
inputs: none specified
Estimated total run time: 14 s

Benchmarking earmark...
Benchmarking rmark...

Name              ips        average  deviation         median         99th %
rmark          156.47        6.39 ms    ±10.59%        6.03 ms        8.77 ms
earmark         83.18       12.02 ms     ±7.39%       11.87 ms       15.16 ms

Comparison:
rmark          156.47
earmark         83.18 - 1.88x slower +5.63 ms
```

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `rmark` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:rmark, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/rmark](https://hexdocs.pm/rmark).
