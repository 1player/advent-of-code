defmodule AdventOfCode.One do
  def step(input) do
    step(input, 0)
  end

  def step("(" <> rest, floor) do
    step(rest, floor + 1)
  end

  def step(")" <> rest, floor) do
    step(rest, floor - 1)
  end

  def step("", floor) do
    floor
  end
end

System.argv
|> Enum.map(fn command ->
  command
  |> AdventOfCode.One.step
  |> IO.puts
end)
