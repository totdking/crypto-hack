# 1
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    print(a)
    return a
gcd(66528, 52920)

