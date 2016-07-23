defmodule AdventOfCode.One do
  def step(input) do
    step(input, 0, 1)
  end

  def step("(" <> rest, floor, pos) do
    step(rest, floor + 1, pos + 1)
  end

  def step(")" <> _rest, 0, pos) do
    pos
  end

  def step(")" <> rest, floor, pos) do
    step(rest, floor - 1, pos + 1)
  end

  def step("", _floor, _pos) do
    "Never entered the basement."
  end
end

System.argv
|> Enum.map(fn command ->
  command
  |> AdventOfCode.One.step
  |> IO.puts
end)
