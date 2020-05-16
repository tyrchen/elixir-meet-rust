defmodule Rhttp do
  @moduledoc """
  Documentation for `Rhttp`.
  """

  use Rustler, otp_app: :rhttp, crate: :rhttp

  def get(_url), do: :erlang.nif_error(:nif_not_loaded)
end
