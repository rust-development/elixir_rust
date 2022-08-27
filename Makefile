MAKEFLAGS += --silent

init:
	mix deps.get

launch:
	iex -S mix
