# detect-scene-change

detect timestamp of all scene changes in video

## Usage

```bash
cargo run <video> <threshold [0-1]>
```

```bash
$ cargo run foo.mp4 0.03
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/detect_scene_change foo.mp4 0.03`
diff     time    frame
----      0.00      1/719
0.063     3.03     73/719
0.038     5.55    133/719
0.079     9.37    224/719
0.372    15.38    367/719
0.120    15.80    377/719
0.256    16.22    387/719
0.041    20.29    484/719
0.198    23.44    559/719
0.072    25.21    601/719
0.188    28.23    673/719
----     30.17    719/719
$
```
