class BST:
    def __init__(self, value) -> None:
        self.value = value
        self.left = None
        self.right = None


def find_closest_value_in_bst(
    tree: BST, target: int
) -> int:
    # return find_closest_value_in_bst_recursive_helper(
    #     tree, target, float("inf")
    # )
    return find_closest_value_in_bst_iterative_helper(
        tree, target, float("inf")
    )


# iterative solution
# Average: O(log(n)) time | O(1) space
# worst: O(n) time | O(1) space
def find_closest_value_in_bst_iterative_helper(
    tree: BST, target: int, closest
):
    current_node = tree
    while current_node is not None:
        if abs(target - closest) > abs(
            target - current_node.value
        ):
            closest = current_node.value
        if target < current_node.value:
            current_node = current_node.left
        elif target > current_node.value:
            current_node = current_node.right
        else:
            break
    return closest


# recursive solution
# Average: O(log(n)) time | O(log(n)) space
# worst: O(n) time | O(n) space
def find_closest_value_in_bst_recursive_helper(
    tree: BST, target: int, closest
):
    if tree is None:
        return closest

    if abs(target - closest) > abs(target - tree.value):
        closest = tree.value
    if target < tree.value:
        return find_closest_value_in_bst_recursive_helper(
            tree.left, target, closest
        )
    elif target > tree.value:
        return find_closest_value_in_bst_recursive_helper(
            tree.right, target, closest
        )
    else:
        return closest


# Test
def build_bst():
    # Create a BST and manually insert nodes to use in the test
    root = BST(10)
    root.left = BST(5)
    root.right = BST(15)
    root.left.left = BST(2)
    root.left.right = BST(5)
    root.right.left = BST(13)
    root.right.right = BST(22)
    root.left.left.left = BST(1)
    root.right.left.right = BST(14)
    return root


def test_find_closest_value():
    bst = build_bst()

    # Test 1: Target value is closer to the root
    assert (
        find_closest_value_in_bst(bst, 12) == 13
    ), "Test 1 Failed"

    # Test 2: Target value has an exact match in the BST
    assert (
        find_closest_value_in_bst(bst, 14) == 14
    ), "Test 2 Failed"

    # Test 3: Target value is between two nodes
    assert (
        find_closest_value_in_bst(bst, 18) == 15
    ), "Test 3 Failed"

    # Test 4: Target value is outside the range of the BST values
    assert (
        find_closest_value_in_bst(bst, 23) == 22
    ), "Test 4 Failed"

    # Test 5: Target value is less than any node in the BST
    assert (
        find_closest_value_in_bst(bst, 0) == 1
    ), "Test 5 Failed"
