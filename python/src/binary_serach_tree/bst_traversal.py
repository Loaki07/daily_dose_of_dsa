from src.binary_serach_tree.bst_construction import BST


# O(n) time | O(n) space
def inorder_traverse(tree, array):
    if tree is not None:
        inorder_traverse(tree.left, array)
        array.append(tree.value)
        inorder_traverse(tree.right, array)
    return array


# O(n) time | O(n) space
def pre_order_traverse(tree, array):
    if tree is not None:
        array.append(tree.value)
        pre_order_traverse(tree.left, array)
        pre_order_traverse(tree.right, array)
    return array


# O(n) time | O(n) space
def post_order_traverse(tree, array):
    if tree is not None:
        post_order_traverse(tree.left, array)
        post_order_traverse(tree.right, array)
        array.append(tree.value)
    return array


# Test
def create_test_bst():
    bst = BST(10)
    bst.left = BST(5)
    bst.right = BST(15)
    bst.left.left = BST(2)
    bst.left.right = BST(
        5
    )  # This could be a different value for more variance
    bst.right.left = BST(13)
    bst.right.right = BST(22)
    bst.right.left.right = BST(14)
    return bst


def test_inorder_traverse():
    bst = create_test_bst()
    expected_order = [2, 5, 5, 10, 13, 14, 15, 22]
    assert inorder_traverse(bst, []) == expected_order


def test_pre_order_traverse():
    bst = create_test_bst()
    expected_order = [10, 5, 2, 5, 15, 13, 14, 22]
    assert pre_order_traverse(bst, []) == expected_order


def test_post_order_traverse():
    bst = create_test_bst()
    expected_order = [2, 5, 5, 14, 13, 22, 15, 10]
    assert post_order_traverse(bst, []) == expected_order
