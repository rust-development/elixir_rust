# README about elixir rust NIF

1. create elixir project
```sh
mix new ex_rust
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
mix rustler.new --module ExRust --name rustlib_add --otp-app ex_rust

```

5. Update the elixir module

```elixir
defmodule ExRust do
  use Rustler,
    otp_app: :ex_rust,
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
ExRust.add 1, 2
```
