# misc

Experiment with an immediate-gui (imgui-rs) scuttlebot chat in rust.

Featuring super-naive api calls to `sbot` using `std::process:Command`. Also lots of shitty code.

Sometimes the scuttlebot will not respond as expected, and the app will hang. Just try turning it off and on again:)


## run

`cargo run`

`cargo run --release` (most of the performance bottlenecks are not on the rust side so not much of a speedup there)

## altnet config

TODO

Idea is to get most configuration work done by the program itself.
For now you need to provide necessary config file if you want to use an altnet.
Here's a working template, providing custom `shs` and `sign` keys, 
specifying ports (these are different from the default ssb config so the sbot instances won't "collide"),
ssb_appname and some other stuff.

```
{
  "caps": {
      "shs": "0uPCJtTF36Eq3rez/dPx4uiE6fVYpTSJ19kyIjcvG/c=",
      "sign": "mLGd5u2NEgHXVdVCpr4bQO4D1lnxLIQy22c21bBke6w="
  },
  "connections": {
    "incoming": {
      "net": [{ "port": 8888, "scope": "public", "transform": "shs" }]
    },
    "outgoing": {
      "net": [{ "transform": "shs" }]
    }
  },
  "port": 8888,
  "ws": {
    "port": 8988
  },
  "ssb_appname": "misc-altnet",
  "logging": {
    "level": "info"
  },
  "verbose": true
}
```