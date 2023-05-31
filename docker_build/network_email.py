import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from email.header import Header

# For this to work with google smtp you must allow access from less secure apps
# on google's account site.  Otherwise one will not be able to log in.
def com_net_send_email(user, pwd, recipient, subject, body, smtp_server='smtp.gmail.com',
                       smtp_port=587):

    msg = MIMEMultipart('alternative')
    msg.set_charset('utf8')
    msg['FROM'] = user
    msg['Subject'] = subject
    msg['To'] = recipient
    _attach = MIMEText(body.replace("\n", "<BR>").encode('utf-8'), 'html', 'UTF-8')        
    msg.attach(_attach)

    server = smtplib.SMTP(smtp_server, smtp_port)
    server.ehlo()
    server.login(user, pwd)
    server.sendmail(user, recipient, msg.as_string())
    server.close()
    return True
