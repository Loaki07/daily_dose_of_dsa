from typing import List


# Method 1: using all the unique strings
# from the list of strings
# O(n * m) time | O(c) space
def common_characters(strings: List[str]) -> List[str]:
    character_counts = {}
    for string in strings:
        unique_string_characters = set(string)
        for character in unique_string_characters:
            if character not in character_counts:
                character_counts[character] = 0
            character_counts[character] += 1

    final_characters = []
    for character, count in character_counts.items():
        if count == len(strings):
            final_characters.append(character)

    return final_characters


# Test
def test_ex1():
    result = common_characters(["abc", "bcd", "cbad"])
    assert result == ["b", "c"] or result == ["c", "b"]
