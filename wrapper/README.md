# avina-ui-wrapper

Minimal Django application to simulate embedding avina-ui in Openstack Horizon.

## Development

### Requirements
You need to have `django` installed. One way to do so would be:
```bash
python3 -m venv wrapper/.env
. wrapper/.env/bin/activate
pip install django requests
```

### Run Locally
Make sure the UI runs locally, see [here](../ui/README.md) and then execute:

```bash
source admin-openrc.sh
scripts/run_ui.sh
```

This spawns the wrapper UI on `http://localhost:8888`.
