def Duplicate_Integer(nums: list[int]) -> bool:
    for i in range(len(nums)):
        print(nums[i+1:])
        if nums[i] in nums[i+1:]:
            return True
    print("\n\nMoving to next Array...\n\n")
    return False

test_array_1, test_result_1 = [1,2,3,3], True

test_array_2, test_result_2 = [1,2,3,4], False

for array, result_actual in [(test_array_1, test_result_1), (test_array_2, test_result_2)]:
    result = Duplicate_Integer(array)
    if result == result_actual:
        print("The Test Passed ... ")
    else:
        print("The Test Failed ...")
