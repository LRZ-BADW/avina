from django.views import generic
from django.utils.translation import ugettext_lazy as _

from avinapanel import api


class IndexView(generic.TemplateView):
    template_name = "lrz/avinapanel/index.html"
    page_title = _("Avina")

    def get_context_data(self, **kwargs):
        context = super(IndexView, self).get_context_data(**kwargs)
        context["page_title"] = self.page_title

        context["ui_url"] = api.api.get_ui_url(self.request)
        context["api_url"] = api.api.get_api_url(self.request)
        context["token"] = api.api.get_token(self.request)

        return context
