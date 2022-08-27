defmodule ElixirRust do
  @moduledoc """
  Documentation for `ElixirRust`.
  """

  @doc """
  Rust integration

  ## Examples

      iex> ElixirRust.add(1, 2)
      3

  """
  use Rustler,
    otp_app: :elixir_rust,
    crate: :rustlib_add,
    crate: :rustlib_start

  def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
  def start(), do: :erlang.nif_error(:nif_not_loaded)

end
