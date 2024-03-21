# https://www.codewars.com/kata/523f5d21c841566fde000009/python
# https://github.com/AlphabetsAlphabets/leet/issues/7

def array_diff(original, to_remove):
    indices = []
    next = 0
    count = 0

    for element in to_remove:
        repeats = original.count(element)
        while repeats != 0:
            index = original.index(element)
            original.pop(index)
            repeats -= 1

    return original

print(array_diff([-10, 5, -9, 14, 6, 9, -5, -12, 17, 19, 9, -3, 1, 15, -4, 10, -20, 11], [19, -20, 1, 17, 7, -15, 15, 13, 12, -5, 12, -4]))
