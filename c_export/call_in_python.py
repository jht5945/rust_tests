import ctypes

stringtools = ctypes.CDLL("target/debug/libstringtools.dylib")
print(stringtools.count_substrings(b"banana", b"na"))

