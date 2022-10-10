require_relative "snowflakes"

def parse_file_to_snowflake(line)
  line.split(" ").map(&:to_i)
end


Dir[[__dir__, "/../data/*.txt"].join].each_entry do |entry|
  File.open(entry) do |f|
    snowflakes_count = f.readline.to_i
    snowflakes = []
    snowflakes_count.times do
      snowflakes << parse_file_to_snowflake(f.readline)
    end
    start_time = Time.now
    Snowflakes.find_pair(snowflakes_count, snowflakes)
    duration = Time.now - start_time
    # puts "Time elapsed: #{duration}"
    # puts "------------"
  end
end
