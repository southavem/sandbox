def display_value
  puts yield
end

display_value do
  4423
end

display_value do
  next 42
end

display_value do
  break 42
end



def with_current_time
  yield Time.now
end

with_current_time do |now|
  puts now.year
end