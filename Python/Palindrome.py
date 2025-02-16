import string
import re
def Palidrome_Checker(Str :str):
    print(string.punctuation)
    lower_s = re.sub(f'[{string.punctuation}]', '', Str.lower())
    lower_s = lower_s.replace(" ","")
    print(lower_s)
    print(lower_s[::-1])
    if lower_s[::-1] == lower_s:
        return True

    return False


if __name__ == '__main__':
    String = input("Enter the text you want to check: \n")

    print(Palidrome_Checker(String))
