# ss-csv
* ss-csv is simple simd csv parser. This library use simd instruction in rust stable.
  * only x64 simd
* ss-csv use https://github.com/yiunsr/bufchr library for use simd search.
* 


## example 

```
let haystack = b"a1,b11\na2,b22\n";
let mut csv_parser = CoreBuilder::new().from_buffer(haystack);
```

## performance of ss-csv
* https://gist.github.com/yiunsr/c0b0768d9e3938461214ec073f053b44

