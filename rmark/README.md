# Rmark

Markdown parser using [rustler](https://github.com/rusterlium/rustler) and [comrak](https://github.com/kivikakk/comrak). Compared with [earmark](https://github.com/pragdave/earmark), it is 20x-30x faster. This is a demo project to show how to use rustler to boost elixir/erlang projects.

## Benchmark result

Don't forget to benchmark again

```bash
➜ mix run benchmark/markdown.exs
Compiling NIF crate :rmark (native/rmark)...
    Finished release [optimized] target(s) in 0.03s
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
Estimated total run time: 28 s

Benchmarking earmark...
Benchmarking rmark...
Benchmarking rmark_dirty...
Benchmarking rmark_spawn...

Name                  ips        average  deviation         median         99th %
rmark              2.39 K      419.01 μs    ±11.72%         407 μs      662.14 μs
rmark_dirty        2.10 K      476.58 μs    ±13.78%         464 μs         787 μs
rmark_spawn        1.95 K      514.06 μs    ±14.77%         496 μs      883.74 μs
earmark          0.0806 K    12408.99 μs     ±7.81%       12262 μs    15110.40 μs

Comparison:
rmark              2.39 K
rmark_dirty        2.10 K - 1.14x slower +57.57 μs
rmark_spawn        1.95 K - 1.23x slower +95.05 μs
earmark          0.0806 K - 29.61x slower +11989.98 μs
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
