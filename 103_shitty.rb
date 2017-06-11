def zigzag_level_order(root)
	arr = loader(root)

	arr.map.with_index do |a, i|
		next a unless i % 2 == 0
		a.reverse
	end
end

def loader(root, level = 0, arr = [])
	return if root.nil?
	arr << [] if arr[level].nil?

	arr[level] << root.val

	zigzag_level_order(root.left, level + 1, arr)
	zigzag_level_order(root.right, level + 1, arr)

	return arr
end