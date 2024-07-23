# https://www.codewars.com/kata/586ee462d0982081bf001f07

# Stock of all products is dict
# merch is what customer wanna buy is string
# n how many of merch to buy is integer
def fillable(stock, merch, n):
    if merch not in stock:
        return False

    if n > stock[merch]:
        return False

    return True
