# Errors
1 in []
1 not in []
2 in list()
2 not in list()
_ in ()
_ not in ()
'x' in tuple()
'y' not in tuple()
'a' in set()
'a' not in set()
'b' in {}
'b' not in {}
1 in dict()
2 not in dict()
"a" in ""
b'c' in b""
"b" in f""
b"a" in bytearray()
b"a" in bytes()
1 in frozenset()

# OK
1 in [2]
1 in [1, 2, 3]
_ in ('a')
_ not in ('a')
'a' in set('a', 'b')
'a' not in set('b', 'c')
'b' in {1: 2}
'b' not in {3: 4}
"a" in "x"
b'c' in b"x"
"b" in f"x"
b"a" in bytearray([2])
b"a" in bytes("a", "utf-8")
1 in frozenset("c")
