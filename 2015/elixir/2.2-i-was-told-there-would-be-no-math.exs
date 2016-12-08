defmodule AdventOfCode.Two do
  def wrap(input) when is_binary(input) do
    dimensions = input
    |> String.split("x")
    |> Enum.map(&String.to_integer/1)

    wrap(dimensions)
  end

  def wrap(dimensions) do
    twoSmallestDimensions = Enum.sort(dimensions) |> Enum.take(2)
    ribbonToWrap = twoSmallestDimensions
    |> Enum.map(& &1 * 2)
    |> Enum.sum

    volume = Enum.reduce(dimensions, &*/2)
    ribbonForBow = volume

    total = ribbonToWrap + ribbonForBow

    IO.puts("#{ribbonToWrap} ft to wrap + #{ribbonForBow} ft for the bow = #{total} ft")
    total
  end
end

IO.stream(:stdio, :line)
|> Stream.map(&String.trim/1)
|> Stream.map(&AdventOfCode.Two.wrap/1)
|> Enum.sum
|> IO.puts
