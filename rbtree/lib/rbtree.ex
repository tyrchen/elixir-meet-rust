defmodule Rbtree do
  @moduledoc """
  Documentation for `Rbtree`.
  """

  use Rustler, otp_app: :rbtree, crate: :rbtree
  def new, do: err()
  def put(_btree, _key, _value), do: err()
  def get(_btree, _key), do: err()
  def has_key(_btree, _key), do: err()
  def delete(_btree, _key), do: err()
  def to_list(_btree), do: err()
  def crash_me_please(_btree), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
