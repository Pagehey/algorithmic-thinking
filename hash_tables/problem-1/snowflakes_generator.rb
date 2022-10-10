SNOWFLAKE_VALUES = (0...10_000_000).to_a

def generate_snowflakes(n)
  n.times.map do
    6.times.map do
      SNOWFLAKE_VALUES.sample
    end
  end
end

[
  10_000,
  10_000,
  25_000,
  25_000,
  50_000,
  50_000,
  75_000,
  75_000,
  100_000,
  100_000,
].each.with_index(1) do |n, i|
  snowflakes = generate_snowflakes(n)
  snowflakes_filename = "snowflakes_#{n}_#{((i + 1) % 2) + 1}.txt"

  File.open(snowflakes_filename, "wb") do |f|
    f.puts n
    snowflakes.each do |snowflake|
      f.puts snowflake.join(" ")
    end
  end
end
