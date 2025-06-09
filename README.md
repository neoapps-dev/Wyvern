<div align="center">
<h1>Wyvern</h1>
<h3>A simple, blazing-fast Wayland compositor built with Rust.</h3>
</div>

## Dependencies

You will need to install the following dependencies (note, that those package
names may vary depending on your OS and GNU/Linux distro):

- `libwayland`
- `libxkbcommon`

#### These are needed for the "Udev/DRM backend" (default backend, recommended)

- `libudev`
- `libinput`
- `libgbm`
- [`libseat`](https://git.sr.ht/~kennylevinsen/seatd)

If you want to enable X11 support (to run X11 applications within Wyvern),
then you will need to install the following packages as well:
    - `xwayland`

## Build and run

You can run it with cargo after having cloned this repository:

```
cd Wyvern
cargo run -- --{backend} # or, without the --{backend} to use Udev
```

The currently available backends are:

- `--x11`: start Wyvern as an X11 client. This allows you to run the compositor inside an X11 session or any compositor supporting XWayland. Should be preferred over the winit backend where possible.
- `--winit`: start Wyvern as a [Winit](https://github.com/tomaka/winit) application. This allows you to run it
  inside of an other X11 or Wayland session.
- `--tty-udev` (default): start Wyvern in a tty with udev support. This is the "traditional" launch of a Wayland
  compositor. Note that this requires you to start Wyvern as root if your system does not have logind
  available.

### Supported Environment Variables

| Variable                      | Example         | Backends  |
|-------------------------------|-----------------|-----------|
| WYVERN_DRM_DEVICE              | /dev/dri/card0  | tty-udev  |
| WYVERN_DISABLE_10BIT           | any             | tty-udev  |
| WYVERN_DISABLE_DIRECT_SCANOUT  | any             | tty-udev  |
| WYVERN_NO_VULKAN               | 1,true,yes,y    | x11       |
| SMITHAY_USE_LEGACY             | 1,true,yes,y    | tty-udev  |
| SMITHAY_VK_VERSION             | 1.3             |           |
