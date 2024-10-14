import copy

original_list = [[1, 2, 3], [4, 5, 6]]
shallow_copied_list = copy.copy(original_list)
print("before",original_list,shallow_copied_list)  # [[99, 2, 3], [4, 5, 6]]


deep_copied_list = copy.deepcopy(original_list)

deep_copied_list[0][0] = 99  # Modifying inner list

print(original_list)  # [[1, 2, 3], [4, 5, 6]]
print(deep_copied_list)  # [[99, 2, 3], [4, 5, 6]]