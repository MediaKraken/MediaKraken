/*
@blueprint_user_internet_vimeo.route('/user_internet/vimeo')
@common_global.jinja_template.template('bss_user/internet/bss_user_internet_vimeo.html')
@common_global.auth.login_required
async def url_bp_user_internet_vimeo(request):
    """
    Display vimeo page
    """
    return {}


@blueprint_user_internet_vimeo.route(
    '/user_internet/internet/bss_user_internet_vimeo_detail/<guid>')
@common_global.auth.login_required
async def url_bp_user_internet_vimeo_detail(request, guid):
    """
 */