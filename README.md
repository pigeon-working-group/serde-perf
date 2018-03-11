## serde-perf
Performance comparison of various [Serde](https://github.com/serde-rs/serde) based serialization and deserialization crates.

### Tested
|Name|Version|Result|
|----|-------|------|
|[bincode](https://github.com/TyOverby/bincode)|1.0.0|4.77434ms|
|[bson](https://github.com/zonyitoo/bson-rs)|0.11.1|132.80845ms|
|[rmp-serde](https://github.com/3Hren/msgpack-rust)|0.13.7|12.59857ms|
|[ron](https://github.com/ron-rs/ron)|0.2.0|171.88187ms|
|[serde_cbor](https://github.com/pyfisch/cbor)|0.8.2|38.43308ms|
|[serde_json](https://github.com/serde-rs/json)|1.0.11|30.34049ms|
|[serde-pickle](https://github.com/birkenfeld/serde-pickle)|0.4.0|129.46340ms|

Try it yourself: `cargo install --git https://github.com/pigeon-working-group/serde-perf`

#### Testing setup
```
         _,met$$$$$gg.           philip@Naboris
      ,g$$$$$$$$$$$$$$$P.        OS: Debian testing buster
    ,g$$P""       """Y$$.".      Kernel: x86_64 Linux 4.15.0-1-amd64
   ,$$P'              `$$$.      CPU: Intel Xeon E3-1240 v3 @ 8x 3.8GHz [27.8°C]
  ',$$P       ,ggs.     `$$b:    
  `d$$'     ,$P"'   .    $$$	 rustc: 1.24.1
   $$P      d$'     ,    $$P     Build mode: release
   $$:      $$.   -    ,d$$'
   $$\;      Y$b._   _,d$P'
   Y$$.    `.`"Y$$$$P"'
   `$$b      "-.__
    `Y$$
     `Y$$.
       `$$b.
         `Y$$b.
            `"Y$b._
                `""""
```