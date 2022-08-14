# README about elixir rust NIF

1. create elixir project
```sh
mix new elixir_rust
```

2. Add deps:
```elixir

defp deps do
  [
    {:rustler, "~> 0.25.0"}
  ]
end
```
3. Update the deps

```sh
mix dpes.get
```

4. Create rust lib

mix rustler.new --module <MODULE> --name <NAME> --otp-app <OTP_APP>
```sh
mix rustler.new --module ElixirRust --name rustlib_add --otp-app elixir_rust

```

5. Update the elixir module

```elixir
defmodule ElixirRust do
  use Rustler,
    otp_app: :elixir_rust,
    crate: :rustlib_add

  def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
end

```

6. Start the application
```sh
iex -S mix
```

7. Then, start the application call the function
```elixir
ElixirRust.add 1, 2
```
