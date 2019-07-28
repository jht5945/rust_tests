
> https://docs.rs/jni/0.13.0/jni/
> 
> https://crates.io/crates/jni

Run:
```bash
cd samplejni; cargo build
cd -; javac HelloWorld.java
LD_LIBRARY_PATH=samplejni/target/debug/ java HelloWorld
```


