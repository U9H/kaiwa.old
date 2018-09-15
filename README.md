# Kaiwa

Kaiwa is an opinionated, self-hosted, and simple commenting system.  It's meant for static sites & created with Rust.  It uses containers and is meant to be run on cheap servers.

## Inspiration

## Usage

You'll need `docker`.

```
docker-compose build
docker-compose run kaiwa-server
```


## API endpoints
TBD

## What will be implemented
- [ ] Multiple domain support
- [ ] Question types
- [ ] Per-page threads
- [ ] Spam mitigation

## What will not be implemented
- [x] Logins
- [x] OAuth
- [x] Images in any way
- [x] Databases other than SQLite