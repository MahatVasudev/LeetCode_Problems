def Max_Profit(data :list[int]) -> int:
    n :int = len(data)
    left = 0
    right = 1
    sell = 0
    if n < 2:
        return 0
    for i in range(n):
        buy = data[left]
        profit = data[right] - buy
        print(profit)
        if profit < 0:
            left = right

        else:
            sell = max(sell, profit)
        right = i + 1
        if right > n:
            return sell

    return sell


if  __name__ == '__main__':
    stock = [2,1,2,1,0,1,2]
    print(Max_Profit(stock))
