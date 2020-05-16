defmodule RhttpTest do
  use ExUnit.Case
  doctest Rhttp

  test "greets the world" do
    assert Rhttp.hello() == :world
  end
end
