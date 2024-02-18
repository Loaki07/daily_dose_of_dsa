# This is an input class. Do not edit.
class BinaryTree:
    def __init__(self, value, left=None, right=None):
        self.value = value
        self.left = left
        self.right = right


# Average: O(n) time | O(h) space
# Worst: O(n) time | O(n) space
def binary_tree_diameter(tree):
    return get_tree_info(tree).diameter


def get_tree_info(tree):
    if tree is None:
        return TreeInfo(0, 0)

    left_tree_info = get_tree_info(tree.left)
    right_tree_info = get_tree_info(tree.right)

    longest_path_through_root = (
        left_tree_info.height + right_tree_info.height
    )
    max_diameter_so_far = max(
        left_tree_info.diameter, right_tree_info.diameter
    )
    current_diameter = max(
        longest_path_through_root, max_diameter_so_far
    )
    current_height = 1 + max(
        left_tree_info.height, right_tree_info.height
    )

    return TreeInfo(current_diameter, current_height)


class TreeInfo:
    def __init__(self, diameter, height) -> None:
        self.diameter = diameter
        self.height = height


# Test
def test_binary_tree_diameter_balanced_tree():
    tree = BinaryTree(1)
    tree.left = BinaryTree(2)
    tree.right = BinaryTree(3)
    tree.left.left = BinaryTree(4)
    tree.left.right = BinaryTree(5)
    tree.right.left = BinaryTree(6)
    tree.right.right = BinaryTree(7)

    assert (
        binary_tree_diameter(tree) == 4
    ), "Test failed for a balanced tree"


def test_binary_tree_diameter_unbalanced_tree():
    tree = BinaryTree(1)
    tree.left = BinaryTree(2)
    tree.left.left = BinaryTree(3)
    tree.left.left.left = BinaryTree(4)

    assert (
        binary_tree_diameter(tree) == 3
    ), "Test failed for an unbalanced tree"


def test_binary_tree_diameter_single_branch():
    tree = BinaryTree(1)
    tree.right = BinaryTree(2)
    tree.right.right = BinaryTree(3)

    assert (
        binary_tree_diameter(tree) == 2
    ), "Test failed for a single branch tree"


def test_binary_tree_diameter_full_tree():
    tree = BinaryTree(1)
    tree.left = BinaryTree(2)
    tree.right = BinaryTree(3)
    tree.left.left = BinaryTree(4)
    tree.left.right = BinaryTree(5)
    tree.left.left.left = BinaryTree(6)
    tree.right.right = BinaryTree(7)
    tree.right.right.right = BinaryTree(8)

    assert (
        binary_tree_diameter(tree) == 6
    ), "Test failed for a full tree"


def test_binary_tree_diameter_empty_tree():
    tree = None

    assert (
        binary_tree_diameter(tree) == 0
    ), "Test failed for an empty tree"
