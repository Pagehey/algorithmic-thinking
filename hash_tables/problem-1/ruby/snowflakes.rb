module Snowflakes
  SIZE = 100_000

  SnowflakeNode = Struct.new(:snowflake, :next_node)

  def self.find_pair(snowflakes_count, snowflakes)
    snowflakes_table = Array.new(SIZE)
    pair_found = false
    snowflakes.each do |snowflake|
      snowflake_code = code(snowflake)
      node = snowflakes_table[snowflake_code]
      snow = SnowflakeNode.new(snowflake, node)
      snowflakes_table[snowflake_code] = snow
    end



    if identify_identical(snowflakes_table)
      puts "Twin snowflakes found."
    else
      puts "No two snowflakes are alike."
    end
  end

  def self.code(snowflake)
    (snowflake[0]
      + snowflake[1]
      + snowflake[2]
      + snowflake[3]
      + snowflake[4]
      + snowflake[5]) % SIZE
  end

  def self.identify_identical(snowflakes_table)
    snowflakes_table.each do |node1|
      while node1
        node2 = node1.next_node
        while node2
          return true if are_identical(node1.snowflake, node2.snowflake)
          node2 = node2.next_node
        end
        node1 = node1.next_node
      end
    end
    return false
  end

  def self.are_identical(snowflake1, snowflake2)
    5.times do |i|
      return true if identical_right(snowflake1, snowflake2, i)
      return true if identical_left(snowflake1, snowflake2, i)
    end
    return false
  end

  def self.identical_right(snowflake1, snowflake2, start)
    5.times do |offset|
      return false if snowflake1[offset] != snowflake2[(start + offset) % 6]
    end
    return true
  end

  def self.identical_left(snowflake1, snowflake2, start)
    5.times do |offset|
      snow2_index = start - offset
      snow2_index += 6 if snow2_index.negative?
      return false if snowflake1[offset] != snowflake2[snow2_index]
    end
    return true
  end
end
