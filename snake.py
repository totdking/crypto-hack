#!/usr/bin/env python3

import sys
# import this

if sys.version_info.major == 2:
    print("You are running Python 2, which is no longer supported. Please update to Python 3.")

# 1 from xor ascii to string
ords = [81, 64, 75, 66, 70, 93, 73, 72, 1, 92, 109, 2, 84, 109, 66, 75, 70, 90, 2, 92, 79]
# print("".join(
#             chr(o ^ 0x32) 
#             for o in ords
#             )
#     )


# 2 from ascii to string
ord2 = [99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125]
# print("Here is your flag:")
# print("".join(
#             chr(o) 
#             for o in ord2
#             )
#     )


# 3: from hex to string
hex_value = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d"
# print("",bytes.fromhex(hex).decode("utf-8"))


# 4: from base64 to string
import base64
base_64 = b'72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf'
# print(base64.b64encode(base_64).decode("utf-8"))


# 5: hex (decode)-> bytes (encode)-> base64
from base64 import b64encode
hex_val = '72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf'
# print("", b64encode(bytes.fromhex(hex)).decode('utf-8'))


# 6: convert integer to message
from Crypto.Util.number import long_to_bytes
integer = 11515195063862318899931685488813747395775516287289682636499965282714637259206269
int_bytes = long_to_bytes(integer)
# print("", int_bytes.decode("utf-8"))


# 7 XOR
from pwn import xor
string = "label"
string_byte = string.encode('utf-8')
integer = 13
# print(xor(string_byte, integer).decode("utf-8"))


# 8 decoding
key1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313"
key1_bytes = bytes.fromhex(key1)

key2_xor_key1 = '37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e'
key2_xor_key1_bytes = bytes.fromhex(key2_xor_key1)

key2_xor_key3 = 'c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1'
key2_xor_key3_bytes = bytes.fromhex(key2_xor_key3)

flag_xor_132 = '04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf'
flag_xor_132_bytes = bytes.fromhex(flag_xor_132)

# 1^2^3^flag = b
# flag = 1^2^3^b
flag_bytes = xor(key1_bytes, flag_xor_132_bytes, key2_xor_key3_bytes)
flag = "".join([hex(b) for b in flag_bytes])
# 0x630x720x790x700x740x6f0x7b0x780x300x720x5f0x690x350x5f0x610x730x730x300x630x310x610x740x310x760x330x7d
# print(flag)
# print(flag_bytes.decode("utf-8"))


# 9 Some dumb bitch ass puzzle
some = "73 62 69 60 64 7f 6b 20 68 21 20 4f 21 25 4f 7d 69 4f 76 24 66 20 65 62 21 27 23 4f 72 69 27 75 6d "
some_byte = bytes.fromhex(some)

def loop_around (sing: bytes):
    for i in range(128):  # Loop through all ASCII values (0-127)
        result = bytes([b ^ i for b in sing])
        try:
            decoded = result.decode('utf-8')
            if decoded.startswith("crypto"):
                print(f"Key {i}: {decoded}")
        except UnicodeDecodeError:
            pass  # Skip non-decodable results

# loop_around(some_byte)
"""16 is the number"""
# xor_res = xor(some_byte, 16)
# Fucking amateur


# 10 Another bitch ass xor puzzle
puzz = "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104"
puzz_byte = bytes.fromhex(puzz)
def find_key(some: bytes):
    for i in range(128):
        result = bytes([b ^ i for b in some])
        try: 
            decode = result.decode("utf-8")
            # if decode.startswith("crypto"):
            print(f"Key {i}: {decode}")
        except UnicodeDecodeError:
            pass
# find_key(puzz_byte)