class Node:
    def __init__(self, name) -> None:
        self.children = []
        self.name = name

    def add_child(self, name):
        self.children.append(Node(name))

    def breadth_first_search(self, array):
        queue = [self]

        while len(queue) > 0:
            current = queue.pop(0)
            array.append(current.name)
            for child in current.children:
                queue.append(child)
        return array


# Test
def test_breadth_first_search():
    # Create the root node and its children
    root = Node("A")
    root.add_child("B")
    root.add_child("C")
    root.children[0].add_child("D")
    root.children[0].add_child("E")
    root.children[1].add_child("F")

    # Expected order of nodes after a breadth-first search
    expected_order = ["A", "B", "C", "D", "E", "F"]

    # Perform the breadth-first search
    result = root.breadth_first_search([])

    # Assert that the result matches the expected order
    assert (
        result == expected_order
    ), f"Expected order {expected_order}, but got {result}"
