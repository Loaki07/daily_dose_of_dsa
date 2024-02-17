class Node:
    def __init__(self, name) -> None:
        self.children = []
        self.name = name

    def add_child(self, name):
        self.children.append(Node(name))

    # v => vertices, e => edges
    # O(v + e) time | O(v) space
    def depth_first_search(self, array):
        array.append(self.name)
        for child in self.children:
            child.depth_first_search(array)
        return array


# Test
def test_depth_first_search():
    # Create the root node and its children
    root = Node("A")
    root.add_child("B")
    root.add_child("C")
    root.children[0].add_child("D")  # Add child D to B
    root.children[0].add_child("E")  # Add child E to B
    root.children[1].add_child("F")  # Add child F to C

    # Expected order of nodes after a depth-first search
    expected_order = ["A", "B", "D", "E", "C", "F"]

    # Perform the depth-first search
    result = root.depth_first_search([])

    # Assert that the result matches the expected order
    assert (
        result == expected_order
    ), f"Expected order {expected_order}, but got {result}"
