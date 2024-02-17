from typing import List, Set


# Method 2
# O(n * m) time | O(m) space
def common_characters(strings: List[str]) -> List[str]:
    smallest_string = get_smallest_string(strings)
    potential_common_characters = set(smallest_string)

    for string in strings:
        remove_non_existant_characters(
            string, potential_common_characters
        )
    return list(potential_common_characters)


def get_smallest_string(strings: List[str]) -> str:
    smallest_string = strings[0]
    for string in strings:
        if len(string) < len(smallest_string):
            smallest_string = string

    return smallest_string


def remove_non_existant_characters(
    string: str, potential_common_characters
):
    unique_string_characters = set(string)

    for character in list(potential_common_characters):
        if character not in unique_string_characters:
            potential_common_characters.remove(character)


# Method 1: using all the unique strings
# from the list of strings
# O(n * m) time | O(c) space
def common_characters_1(strings: List[str]) -> List[str]:
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
