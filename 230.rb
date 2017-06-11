# @param {TreeNode} root
# @param {Integer} k
# @return {Integer}

# Fails on [1, null, 2]

def kth_smallest(root, k)
  results = []
  inorder_traverse(root, results)
  arr[k]
end

def inorder_traverse(root, arr)
  inorder_traverse(root.left, arr) unless root.left.nil?
  arr << root.val
  inorder_traverse(root.right, arr) unless root.right.nil?
end