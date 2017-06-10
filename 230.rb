require 'pry'

# Best test case
# [41,37,44,24,39,42,48,1,35,38,40,null,43,46,49,0,2,30,36,null,null,null,null,null,null,45,47,null,null,null,null,null,4,29,32,null,null,null,null,null,null,3,9,26,null,31,34,null,null,7,11,25,27,null,null,33,null,6,8,10,16,null,null,null,28,null,null,5,null,null,null,null,null,15,19,null,null,null,null,12,null,18,20,null,13,17,null,null,22,null,14,null,null,21,23]
# 25

# Definition for a binary tree node.
class TreeNode
  attr_accessor :val, :left, :right, :parent

  def initialize(val)
    @val = val
    @left, @right, @parent = nil, nil, nil
  end
end

# @param {TreeNode} root
# @param {Integer} k
# @return {Integer}

# Fails on [1, null, 2]

def kth_smallest(root, k)
  min_heap = min_heapify(root)

  retval = nil
  for x in 1..k
    break if min_heap.nil?
    retval = min_heap.val
    min_heap = pop_smallest_from_min_heap(min_heap)
  end

  retval
end

def min_heapify(root)
  return if root.nil?

  # Set parent annotations for children
  [root.left, root.right].compact.each { |x| x.parent = root }

  if (root.left) && (root.left.val <= root.val)
    new_root_val = root.left.val
    root.left.val = root.val
    root.val = new_root_val

    min_heapify(root.parent) unless root.parent.nil?
  end

  if (root.right) && (root.right.val <= root.val)
    new_root_val = root.right.val
    root.right.val = root.val
    root.val = new_root_val

    min_heapify(root.parent) unless root.parent.nil?
  end

  if (root.left && root.right) && (root.left.val > root.right.val)
    new_left_val = root.right.val
    root.right.val = root.left.val
    root.left.val = new_left_val
  end

  min_heapify(root.left)
  min_heapify(root.right)

  root
end

def pop_smallest_from_min_heap(root)
  children = [root.left, root.right]
  new_root = root.left || root.right
  children.delete(new_root)
  children.compact!

  if children.empty?
    new_root
  else
    insert_into_min_heap(new_root, children.first)
  end
end

def insert_into_min_heap(root, newval)
  return root if root.nil? || newval.nil?
  dummy = root

  while !dummy.nil? do
    if dummy.left.nil?
      dummy.left = newval
      break
    elsif dummy.right.nil?
      dummy.right = newval
      break
    else
      dummy = dummy.left || dummy.right
    end
  end

  min_heapify(root)
end

test_tree = TreeNode.new(3)
test_tree.left = TreeNode.new(2)
test_tree.right = TreeNode.new(4)
test_tree.right.left = TreeNode.new(1)

puts kth_smallest(test_tree, 1)