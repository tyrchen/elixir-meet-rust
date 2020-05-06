defmodule Rmark do
  @moduledoc """
  Documentation for `Rmark`.
  """

  use Rustler, otp_app: :rmark, crate: :rmark

  def to_html(_md), do: :erlang.nif_error(:nif_not_loaded)
end
