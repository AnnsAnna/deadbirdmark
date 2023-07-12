# deadbirdmark

## Development

Setup docker for db.

```bash
docker run -p 127.0.0.1:3306:3306  --name mdb -e MARIADB_ROOT_PASSWORD=Password123! -d mariadb:latest
```