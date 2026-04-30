from django.utils.translation import ugettext_lazy as _
import horizon


class AvinaPanel(horizon.Panel):
    name = _("Avina")
    slug = "avinapanel"
