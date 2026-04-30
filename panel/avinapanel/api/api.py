PROD_UI_URL = "https://cc.lrz.de:1339"
TEST_UI_URL = "https://tcc.cloud.mwn.de:1339"

PROD_API_URL = "https://cc.lrz.de:1338/api"
TEST_API_URL = "https://tcc.cloud.mwn.de:1338/api"


def get_ui_url(request):
    if request.build_absolute_uri().startswith("https://tcc.cloud.mwn.de"):
        return TEST_UI_URL
    else:
        return PROD_UI_URL


def get_api_url(request):
    if request.build_absolute_uri().startswith("https://tcc.cloud.mwn.de"):
        return TEST_API_URL
    else:
        return PROD_API_URL


def get_token(request):
    return request.user.token.id
