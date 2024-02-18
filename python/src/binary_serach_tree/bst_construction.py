class BST:
    def __init__(self, value) -> None:
        self.value = value
        self.left = None
        self.right = None

    # iterative approach
    # average: O(log(n)) time | O(1) space
    # worst: O(n) time | O(1) space
    def insert(self, value):
        current_node = self

        while True:
            if value < current_node.value:
                if current_node.left is None:
                    current_node.left = BST(value)
                    break
                else:
                    current_node = current_node.left
            else:
                if current_node.right is None:
                    current_node.right = BST(value)
                    break
                else:
                    current_node = current_node.right

        return self

    # iterative approach
    # average: O(log(n)) time | O(1) space
    # worst: O(n) time | O(1) space
    def contains(self, value):
        current_node = self

        while current_node is not None:
            if value < current_node.value:
                current_node = current_node.left
            elif value > current_node.value:
                current_node = current_node.right
            else:
                return True

        return False

    # iterative approach
    # average: O(log(n)) time | O(1) space
    # worst: O(n) time | O(1) space
    def remove(self, value, parent_node=None):
        current_node = self

        while current_node is not None:
            if value < current_node.value:
                parent_node = current_node
                current_node = current_node.left
            elif value > current_node.value:
                parent_node = current_node
                current_node = current_node.right
            else:
                if (
                    current_node.left is not None
                    and current_node.right is not None
                ):
                    current_node.value = (
                        current_node.right.get_min_value()
                    )
                    # current_node.value = smallest value of right subtree
                    current_node.right.remove(
                        current_node.value, current_node
                    )
                # root node case
                elif parent_node is None:
                    if current_node.left is not None:
                        left_child = current_node.left
                        current_node.value = (
                            left_child.value
                        )
                        current_node.right = (
                            left_child.right
                        )
                        current_node.left = left_child.left
                    elif current_node.right is not None:
                        right_child = current_node.right
                        current_node.value = (
                            right_child.value
                        )
                        current_node.left = right_child.left
                        current_node.right = (
                            right_child.right
                        )
                    else:
                        # this is a single-node tree; do nothing
                        pass
                elif parent_node.left == current_node:
                    parent_node.left = (
                        current_node.left
                        if current_node.left is not None
                        else current_node.right
                    )
                elif parent_node.right == current_node:
                    parent_node.right = (
                        current_node.left
                        if current_node.left is not None
                        else current_node.right
                    )
                break

        return self

    def get_min_value(self):
        current_node = self

        while current_node.left is not None:
            current_node = current_node.left

        return current_node.value


# Test
def test_insert():
    bst = BST(10)
    bst.insert(5).insert(15).insert(5).insert(2).insert(
        14
    ).insert(22)

    assert bst.left.value == 5
    assert bst.right.value == 15
    assert bst.left.left.value == 2
    assert bst.left.right.value == 5  # Duplicate value
    assert bst.right.left.value == 14
    assert bst.right.right.value == 22


def test_contains():
    bst = BST(10)
    bst.insert(5).insert(15).insert(5).insert(2).insert(
        14
    ).insert(22)

    assert bst.contains(10) is True
    assert bst.contains(14) is True
    assert bst.contains(22) is True
    assert bst.contains(5) is True
    assert bst.contains(99) is False


def test_remove():
    bst = BST(10)
    bst.insert(5).insert(15).insert(5).insert(2).insert(
        14
    ).insert(22)

    # Remove leaf nodes
    bst.remove(2)
    bst.remove(14)
    assert bst.contains(2) is False
    assert bst.contains(14) is False

    # Remove nodes with one child
    bst.remove(22)
    assert bst.contains(22) is False

    # Remove node with two children
    bst.remove(15)
    assert bst.contains(15) is False
    assert (
        bst.contains(5) is True
    )  # Ensure other nodes are still present

    # Edge cases, such as removing root or nodes that don't exist, can be tested as well
    bst.remove(10)  # Removing root
    assert bst.contains(10) is False or (
        bst.value != 10
    )  # Root's value changes or no longer exists


def test_get_min_value():
    bst = BST(10)
    bst.insert(5).insert(15).insert(1).insert(2).insert(
        14
    ).insert(22)

    assert bst.get_min_value() == 1
    assert bst.right.get_min_value() == 14


def test_bst_sequence():
    # Initialize the BST with the root value
    bst = BST(10)

    # Insert 5
    bst.insert(5)
    assert bst.contains(
        5
    ), "BST should contain 5 after insertion"

    # Attempt to remove 10 (root)
    bst.remove(10)
    if bst.value is not None:  # If the tree is not empty
        assert not bst.contains(
            10
        ), "BST should not contain 10 after removal"
    else:
        assert (
            bst.value != 10
        ), "BST root value should not be 10 after removal"

    # Check if BST contains 15
    contains_15 = bst.contains(15)
    assert (
        not contains_15
    ), "BST should not contain 15 before insertion"
