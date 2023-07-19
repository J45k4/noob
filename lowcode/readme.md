

## sizes

```
du -h minimalgo
1.2M    minimalgo

du -h minimalc 
20K    minimalc

du -h minimalrust/target/release/minimalrust
476K    minimalrust/target/release/minimalrust
```

## gcc

```
gcc minimalc.c -o minimalc
objdump -d minimalc > minimalc_objdump.txt
```

## m1 asm

```
as -arch arm64 -o arm64_hello.o  arm64_hello.asm
ld -o arm64_hello arm64_hello.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64
```