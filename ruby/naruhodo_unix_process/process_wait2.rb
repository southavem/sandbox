5.times do
  fork do
    if rand(5).even?
      exit 111
    else
      exit 112
    end
  end
end


5.times do
  pid, status = Process.wait2

  if status.exitstatus == 111
    puts "#{pid} encounterd an even number!"
  else
    puts "#{pid} encounterd an odd  number!"
  end
end
