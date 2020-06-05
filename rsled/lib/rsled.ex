defmodule Rsled do
  @moduledoc """
  Documentation for `Rsled`.
  """

  use Rustler, otp_app: :rsled, crate: :rsled
  def open(_path), do: err()
  def put(_db, _key, _value), do: err()
  def get(_db, _key), do: err()
  def has_key(_db, _key), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
