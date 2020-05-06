defmodule RmarkTest do
  use ExUnit.Case
  doctest Rmark

  test "greets the world" do
    assert Rmark.hello() == :world
  end
end
