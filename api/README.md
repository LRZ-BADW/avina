# avina-api
API server application for the Openstack-based LRZ Compute Cloud,
[https://cc.lrz.de](https://cc.lrz.de), first and foremost the budgeting system.

## Development

### Requirements
To work with the API locally you need Docker as well as the MariaDB client.
The `init.sh` script is then used to setup and migrate a database for you.
```bash
scripts/init.sh
```
This is required prior to both `cargo test` and running the API locally.

### Running the API server locally
```bash
# optional: insert database dump
scripts/enter_db.sh
source lrz_budgeting.sql
quit

source admin-openrc.sh
scripts/run_api.sh
```

### Calling the local API server
```bash
source admin-openrc.sh
cargo run --bin avina -- -u http://localhost:8000/api -r http://localhost:8000/api user me
```
