## Media Warehouse

It's a mix of a media player (VLC) with a media gallery.

## Requirements

In order for this to work in your server, you will need `vlc` and `playerctl`, which can be
installed with the following commands:

```sh
    sudo apt install vlc playerctl
    playerctld daemon
```

You will also need a `movies.json` file at the root of the project. Its content should be
similar to

```json
[
    {
        "id": 1,
        "name": "The Martian",
        "description": "A very good movie",
        "duration": "3 hours"
    },
    {
        "id": 2,
        "name": "The Good, The Bad and The Ugly",
        "description": "A very good movie with guns",
        "duration": "3 hours"
    }
]
```

Furthermore, movie files (`mp4`, `mkv`, etc...) should be placed in
`src/movies/`. Each file should be named accordingly to the id of the movie specified in
`movies.json`.

You can see an example of a correctly fulfilled `src/movies/` directory bellow. Here, 1 is the
`mp4` file that represents "The Martian" and 2 represents "The Good, The Bad and The Ugly".
```
src/movies/
    1
    2
```

