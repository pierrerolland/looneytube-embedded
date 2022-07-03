# LooneyTube embedded

## Install

```shell
cargo build --release
cp target/release/looneytube-embedded /usr/local/bin/
```


And configure the following env var to point to the videos directory:
`LOONEYTUBE_VIDEOS_DIR=/yourvideosdir`

## Videos directory structure

```
videos root
└── Animated series 1
    └── thumb.png
    └── episode 1.mp4
    └── episode 2.mp4
    └── thumbs
        └── episode 1.jpg
        └── episode 2.jpg
└── Animated series 2
    └── thumb.png
    └── episode 3.mp4
    └── thumbs
        └── episode 3.jpg
```

:warning: Videos must be MP4

:warning: Category thumb must be a PNG file called "thumb.png"

:warning: Episode thumb must be a JPG file stored in `thumbs` and named after the MP4 file

## Use

```shell
looneytube-embedded
```

Then browse `localhost:8000`
