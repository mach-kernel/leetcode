class StringIterator
  def initialize(compressedString)
    @tokens = compressedString.scan(/[A-Za-z][0-9]+/)
                              .map { |x| x = x.split(''); [x.shift, x.join.to_i] }
  end

  def next
    return ' ' unless has_next
    data = @tokens.first
    data[1] -= 1

    @tokens.shift if data[1] < 1 

    data[0]
  end

  def has_next
    !@tokens.empty?
  end
end