defmodule Rmark do
  @moduledoc """
  Documentation for `Rmark`.
  """

  use Rustler, otp_app: :rmark, crate: :rmark

  def to_html(_md), do: :erlang.nif_error(:nif_not_loaded)
  def to_html_spawn(_md), do: :erlang.nif_error(:nif_not_loaded)
  def to_html_dirty(_md), do: :erlang.nif_error(:nif_not_loaded)

  def to_html2(md) do
    :ok = to_html_spawn(md)

    receive do
      {:ok, result} -> {:ok, result}
      {:error, error} -> {:error, error}
    after
      5000 ->
        {:error, :timeout}
    end
  end

  def to_html3(md), do: to_html_dirty(md)
end
