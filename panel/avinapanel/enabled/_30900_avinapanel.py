# The name of the panel to be added to HORIZON_CONFIG. Required.
PANEL = "avinapanel"

# The name of the dashboard the PANEL associated with. Required.
PANEL_DASHBOARD = "lrz"

# The name of the dashboards panel group to put the PANEL in.
PANEL_GROUP = "default"

# Use this panel as the default panel of the dashboard.  Every dashboard
# requires a default panel.
DEFAULT_PANEL = "avinapanel"

# Python panel class of the PANEL to be added.
ADD_PANEL = "avinapanel.content.avinapanel.panel.AvinaPanel"

# A list of applications to be prepended to INSTALLED_APPS
ADD_INSTALLED_APPS = ["avinapanel"]

# Automatically discover static resources in installed apps
AUTO_DISCOVER_STATIC_FILES = True

# A list of js files to be included in the compressed set of files
ADD_JS_FILES = []

# A list of scss files to be included in the compressed set of files
ADD_SCSS_FILES = ["dashboard/lrz/avina/avinapanel/avinapanel.scss"]
