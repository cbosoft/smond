# smond: system monitor daemon

A brownie that lives in your processor taking notes on usage and statistics.

(I mean, a small executable that logs data about your data. At the moment, only battery capacity is logged.)

Some set up:
```
# ~/.config/smond/config.yaml
poll_period: 60
```

Then install with:
```
cargo install --release
```

Run on startup in .xinitrc or via your WM:
```
# ~/.config/sway/config

...
exec smond
```
