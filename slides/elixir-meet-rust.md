---
marp: true
title: 'Elixir 遇见 Rust'
paginate: false
_paginate: false
theme: uncover
---

<!-- backgroundColor: #F7F8F8 -->

![bg](assets/tubi.png)
![](#fff)

##
##
##
##
##
##
##
##

### Elixir 遇见 Rust

---

## Elixir 的美妙之处

- 隔离（isolation）
- 容错和自我恢复（fault tolerant & self-healing）
- 安全并发（fearless concurrency）
- 语言表达力强（expressiveness）

---

## Elixir 的不足

- 性能（performance）
- 生态（ecosystem）
- 底层开发（low-level support）

---

## 如何解决？

- ports
- _NIFs_
- 用其他语言撰写服务（如 gRPC）

---

## NIFs：优势和不足

- 优势：
  - 运行速度最快
  - （相对）容易撰写和维护
  - 和 VM 运行在同一个进程空间，没有上下文切换开销
- 不足：
  - 不够安全（可以 crash VM 或者导致内存泄漏）

---

## Rust

- 执行效率媲美 C/C++
- 内存安全和并发安全
- 零成本抽象
- 语言表达力很强
- 强大的让人爱不释手的类型系统

---

### Rustler：为 elixir/rust 建起一座桥

- 安全：你撰写的 safe rust 不会 crash VM
- 互操作：数据在两种语言之间可以很方便地传递
  - Rust struct <-> elixir term
- 高效：数据可以按引用传递；当不再引用时自动销毁

---

<!-- _backgroundColor: #222831 -->
<!-- _color: #fff -->

## 如何使用 rusler？

以 markdown -> html compiler 为例

---

## mix.exs
<!-- _backgroundColor: #ffffed -->

```elixir
def project do
[
    ...
    compilers: [:rustler] ++ Mix.compilers,
    rustler_crates: [rmark: [
        path: "native/rmark",
        mode: rustc_mode(Mix.env)
    ]],
    ...
]

defp deps do
[
    ...
    {:rustler, "~> 0.21.0"},
    ...
]
```

---
<!-- _backgroundColor: #ffffed -->

## lib/lib.ex

```elixir
defmodule Rmark do
  @moduledoc """
  Documentation for `Rmark`.
  """

  use Rustler, otp_app: :rmark, crate: :rmark

  def to_html(_md), do: :erlang.nif_error(:nif_not_loaded)
end
```

---
<!-- _backgroundColor: #ffffed -->

## native/cargo.toml

```toml
[dependencies]
...
rustler = "0.21.0"
...
```

---
<!-- _backgroundColor: #ffffed -->

## native/src/lib.rs

```rust
use rustler::{Encoder, Env, Error, Term};
use comrak::{markdown_to_html, ComrakOptions};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.Rmark", [
        ("to_html", 1, to_html)
    ],
    None
}

fn to_html<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let md: String = args[0].decode()?;
    Ok((atoms::ok(), markdown_to_html(&md, &ComrakOptions::default())).encode(env))
}
```

---

<!-- _backgroundColor: #ffffed -->

## Benchmark

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

...

Benchmarking earmark...
Benchmarking rmark...

Name              ips        average  deviation         median         99th %
rmark          156.47        6.39 ms    ±10.59%        6.03 ms        8.77 ms
earmark         83.18       12.02 ms     ±7.39%       11.87 ms       15.16 ms

Comparison:
rmark          156.47
earmark         83.18 - 1.88x slower +5.63 ms
```

---

<!-- _backgroundColor: #222831 -->
<!-- _color: #fff -->

## Live coding

让 elixir 支持 [Roaring Bitmap](https://github.com/Nemo157/roaring-rs)

---

## 谢谢！
