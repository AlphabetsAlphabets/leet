# https://www.codewars.com/kata/513e08acc600c94f01000001/train/python
# Uses math to convert base10 which is our usual system to base16 (hexadecimal)
# I learned it from https://www.youtube.com/watch?v=QJW6qnfhC70&t=700s

def rgb(r, g, b):
    denary_to_hex = {10: "A", 11: "B", 12: "C", 13: "D", 14: "E", 15: "F"}
    def to_hex(value):
        # Values below the minimum accepted value will be set to the minimum.
        # The same applies to max. If above max set to max allowed value.
        if value < 0:
            return '00'
        elif value > 255:
            return 'FF'

        hex = []
        remainder = value % 16
        value = value // 16

        if remainder in denary_to_hex:
            hex.append(denary_to_hex[remainder])
        else:
            hex.append(str(remainder))

        # You stop doing when the value is 0. Because dividing by 0
        # only gives 0. So, this stops the infinite loop.
        while value >= 1:
            remainder = value % 16
            value = value // 16

            if remainder in denary_to_hex:
                hex.append(denary_to_hex[remainder])
            else:
                hex.append(str(remainder))

        hex.reverse()

        if len(hex) == 1:
            hex = ['0', hex[0]]

        return "".join(hex)

    r = to_hex(r)
    g = to_hex(g)
    b = to_hex(b)

    return f"{r}{g}{b}"
