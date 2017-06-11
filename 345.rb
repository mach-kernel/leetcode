def reverse_vowels(s)
  new_word = []
  vowels = 'aeiou'

  original_word = s.split('')
  word_vowels = []

  original_word.map! do |c| 
    if vowels.include?(c.downcase) 
      word_vowels << c
      nil
    else
      c
    end
  end 

  original_word.map! { |c| c.nil? ? word_vowels.pop : c }

  original_word.join('')
end

puts reverse_vowels('leetcode')