defmodule Rsled.MixProject do
  use Mix.Project

  def project do
    [
      app: :rsled,
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: [
        rsled: [
          path: "native/rsled",
          mode: rustc_mode(Mix.env())
        ]
      ],
      version: "0.1.0",
      elixir: "~> 1.10",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp rustc_mode(:prod), do: :release
  defp rustc_mode(_), do: :debug

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.22.0-rc.0"},
      {:benchee, "~> 1.0"},
      {:uuid, "~> 1.1.8"}
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
