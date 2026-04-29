from django.urls import re_path
from django.shortcuts import render as django_render
import requests
from os import getenv
from http import HTTPStatus
import atexit

DEBUG = True
SECRET_KEY = "1234"
ROOT_URLCONF = __name__
TEMPLATES = [
    {
        "BACKEND": "django.template.backends.django.DjangoTemplates",
        "DIRS": ["./wrapper/templates/"],
    },
]


api_url = None
token = None


def issue_api_token(
    keystone_url, username, password, user_domain_name, project_name, project_domain_id
):
    url = keystone_url + "/auth/tokens/"
    headers = {"Content-Type": "application/json"}
    data = {
        "auth": {
            "identity": {
                "methods": ["password"],
                "password": {
                    "user": {
                        "name": username,
                        "domain": {"name": user_domain_name},
                        "password": password,
                    }
                },
            },
            "scope": {
                "project": {"name": project_name, "domain": {"id": project_domain_id}}
            },
        }
    }

    try:
        resp = requests.post(url, headers=headers, json=data)
    except:
        return None

    if resp.status_code != HTTPStatus.CREATED:
        return None

    return resp.headers["X-Subject-Token"]


def revoke_api_token(keystone_url, token):
    url = keystone_url + "/auth/tokens/"
    headers = {
        "Content-Type": "application/json",
        "X-Auth-Token": token,
        "X-Subject-Token": token,
    }

    resp = requests.delete(url, headers=headers)

    if resp.status_code != HTTPStatus.NO_CONTENT:
        return False

    return True


def setup():
    global api_url
    global token

    api_url = getenv("AVINA_API_URL")

    if not api_url:
        print("Avina API URL is not set! Set AVINA_API_URL environment variable!")
        exit(1)

    keystone_url = getenv("OS_AUTH_URL")
    username = getenv("OS_USERNAME")
    password = getenv("OS_PASSWORD")
    user_domain_name = getenv("OS_USER_DOMAIN_NAME")
    project_name = getenv("OS_PROJECT_NAME")
    project_domain_id = getenv("OS_PROJECT_DOMAIN_ID")

    if not (
        keystone_url
        and username
        and password
        and user_domain_name
        and project_name
        and project_domain_id
    ):
        print("Some OpenStack auth variable is not set! Source the openrc.sh")
        exit(1)

    token = issue_api_token(
        keystone_url,
        username,
        password,
        user_domain_name,
        project_name,
        project_domain_id,
    )


def cleanup():
    keystone_url = getenv("OS_AUTH_URL")
    revoke_api_token(keystone_url, token)


def init():
    setup()
    atexit.register(cleanup)


def home(request):
    return django_render(request, "index.html", {"api_url": api_url, "token": token})


urlpatterns = [
    re_path(r"^$", home, name="homepage"),
]


init()
