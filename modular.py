# 1
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    print(a)
    return a
# gcd(26513, 32321)


# 2 get eea inverse of a value in a prime field
def eea_mon_inv(a: int, m: int) -> int:
    # coefficient of a in gcd(a,b) b being prime field
    old_t, new_t = 1, 0
    # result of (old_t.a + (0).b) = old_r(a)
    # (new_t.a + (1).m) = new_r(m)
    old_r, new_r = a, m
    while new_r != 0:
        quotient = old_r // new_r
        old_r, new_r = new_r, old_r - (quotient * new_r)
        old_t, new_t = new_t, old_t - (quotient * new_t)
    
    if old_r > 1:
        raise ValueError("Inverse doesn't exist")
    if old_t < 0:
        old_t += m
    return old_t % m
# print(eea_mon_inv(26513, 32321))


# 3 Get gcd and co-efficient of a and b from eea
def get_a_b (p: int, q: int) -> (int, int, int):
    # co-efficient of a
    old_t, new_t = 1, 0
    # co-efficient of b
    old_s, new_s = 0, 1
    # result of the rep of co-efficient of a and b in (old_t.a + old_s.b = old_r)
    # (new_t.a + new_s.b = new_r)
    old_r, new_r = p, q
    while new_r != 0:
        quotient = old_r // new_r
        old_r, new_r = new_r, old_r - (quotient * new_r)
        old_t, new_t = new_t, old_t - (quotient * new_t)
        old_s, new_s = new_s, old_s - (quotient * new_s)
    return old_t, old_s, old_r
(a, b, r) = get_a_b(26513, 32321)
# print(f"A coefficient:{a}\nB coefficient:{b}\nResult/Gcd:{r}")


# 4 finding the mod pow of a number in a finite field
def mod_pow(base: int, pow: int, p: int) -> int:
    # if prime is 1, the result of any thing is zero because the only value in a prime field of 1 is zero
    if p == 1:
        return 0
    # Result to work up from
    result = 1
    # Base is mod by p to keep within the field
    base = base % p
    # pow kept above zero and divided by 2 each time the p%2 != 0
    while pow > 0:
        if pow % 2 == 1:
            # Result is multiplied by base and mod(p)
            result = (result * base ) % p
        pow >>= 1
        # base = base^2 % p
        base = (base * base) % p
    return result
# print("", mod_pow(3, 16, 17))

# 5 
a = 3
p = 13
# print(mod_pow(a, p-2, p))

# 6 slow and is 0(n) is as long as the size of P
def root_find(square: int, p: int) -> int:
    for i in range(p):
        if (i * i) % p == square:
            return i
    return p

# print(root_find(6, 29))

arr = [14, 6, 11]
def valid_square(arr: [int], p: int):
    # if i^2 mod(p) = arr[j]
    for i in range(p):
        for j in range(len(arr)):
            if mod_pow(i, 2, p) == arr[j]:
                print(f"value {arr[j]} is a square of {i}")
    print("No square found")

valid_square(arr, 29)
p = 10152403517453989048540857567108
        5261788758965189060164484385690801466167356667036677932998889725
        47658242173878850073873850313435
        61581972474738502735653492495738
        67251280253564698939768700489401
        960767007716413932851838937641880157263936985954881657889497583485535527613578457628399173971810541670838543309159139