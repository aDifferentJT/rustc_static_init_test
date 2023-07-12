It seems that static initialisation code from C++ only runs if some symbol from that translation unit is used in Rust.

NOTE: This behaviour is allowed by the C++ standard.

Expected output:
```
S1()
S2()
```
or
```
S2()
S1()
```
depending on static initialisation order

Actual output:
```
S1()
```
