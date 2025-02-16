def TwoSum(nums: list[int], target: int) -> list[int]:
    hashmap = {}

    for i in range(len(nums)):
        subst = target - nums[i]
        if nums[i] not in hashmap.keys():
            hashmap[subst] = i 
        else:
            return hashmap[nums[i]], i 


    return [-1, -1]

Listing = [3,1,2,4]
print("The list is ", Listing)    
target = int(input("Write your target: "))
print(TwoSum(Listing, target= target))
