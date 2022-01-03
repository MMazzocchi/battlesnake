# Max's Battlesnake Server

A pretty decent [Battlesnake](https://play.battlesnake.com/) server written in rust.

# Quick Start

```
$ docker-compose up
```

This should build and start the server with one snake running. You can test it with the following command:

```
$ curl -s \
  -H 'Host: example-snake.com' \
  localhost:10000
```

If successful, you should see the following:

```
{"apiversion":"1","author":"You!","color":"#000000","head":"default","tail":"default","version":"0.1.0"}
```
