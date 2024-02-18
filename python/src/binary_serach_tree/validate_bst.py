from src.binary_serach_tree.bst_construction import BST

# O(n) time | O(d) space
def validate_bst(tree):
    return validate_bst_helper(
        tree, float("-inf"), float("inf")
    )


def validate_bst_helper(tree, min_value, max_value):
    if tree is None:
        return True
    if tree.value < min_value or tree.value >= max_value:
        return False
    left_is_valid = validate_bst_helper(
        tree.left, min_value, tree.value
    )
    return left_is_valid and validate_bst_helper(
        tree.right, tree.value, max_value
    )


# Test
def test_validate_bst_valid():
    bst = BST(10)
    bst.left = BST(5)
    bst.right = BST(15)
    bst.left.left = BST(2)
    bst.left.right = BST(5)
    bst.right.left = BST(13)
    bst.right.right = BST(22)
    bst.right.left.right = BST(14)

    assert (
        validate_bst(bst) == True
    ), "The tree should be a valid BST"


def test_validate_bst_invalid():
    bst = BST(10)
    bst.left = BST(5)
    bst.right = BST(15)
    bst.left.left = BST(2)
    bst.left.right = BST(5)
    bst.right.left = BST(13)
    bst.right.right = BST(22)
    bst.right.left.left = BST(
        14
    )  # This violates the BST properties

    assert (
        validate_bst(bst) == False
    ), "The tree should be invalid as a BST"


def test_validate_bst_single_node():
    bst = BST(10)

    assert (
        validate_bst(bst) == True
    ), "A single node tree should be a valid BST"


def test_validate_bst_empty():
    assert (
        validate_bst(None) == True
    ), "An empty tree should be considered a valid BST"
