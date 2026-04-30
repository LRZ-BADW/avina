from django.conf.urls import url

from avinapanel.content.avinapanel import views

urlpatterns = [
    url(r"^$", views.IndexView.as_view(), name="index"),
]
