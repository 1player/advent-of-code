defmodule AdventOfCode.Two do
  def wrap(input) when is_binary(input) do
    dimensions = input
    |> String.split("x")
    |> Enum.map(&String.to_integer/1)

    wrap(dimensions)
  end

  def wrap([l, w, h] = dimensions) do
    sides = [l*w, w*h, h*l]
    area = Enum.map(sides, & &1 * 2) |> Enum.sum
    slack = Enum.min(sides)

    total = area + slack

    IO.puts("#{area} sqft paper + #{slack} sqft slack = #{total} sqft")

    total
  end
end

IO.stream(:stdio, :line)
|> Stream.map(&String.trim/1)
|> Stream.map(&AdventOfCode.Two.wrap/1)
|> Enum.sum
|> IO.puts
