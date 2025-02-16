def Anagram(s: str, t :str) -> bool:
    record_1, record_2 = {}, {}

    if len(s) != len(t):
        return False

    for i in range(len(s)):
        if s[i] not in record_1.keys():
            record_1[s[i]] = 1

        else:
            record_1[s[i]] += 1

    for j in range(len(t)):
        if t[j] not in record_1.keys():
            return False
        
        elif record_2[t[j]] > record_1[t[j]]:
            return False 

        elif t[j] not in record_2.keys():
            record_2[t[j]] = 1

        else:
            record_2[t[j]] += 1


    for k in record_2.keys():
        if record_2[k] != record_1[k]:
            return False

    return True

def MoreOtimized(s: str, t: str) -> bool:

    if len(s) != len(t):
        return False
    
    count_s = [0] * 26
    for i in range(len(s)):
        print(ord(s[i]))
        count_s[ord(s[i]) - ord('a')] += 1
        count_s[ord(t[i]) - ord('a')] -= 1

    for val in count_s:
        if val != 0:
            return False

    return True


MoreOtimized("abc","acb")
