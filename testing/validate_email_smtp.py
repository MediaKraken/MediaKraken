import os
import shlex
import smtplib
import subprocess
from dotenv import load_dotenv
    
def com_net_send_email(user, pwd, recipient, subject, body, smtp_server='smtp.gmail.com',
                       smtp_port=587):
    """
    Send email smtp server
    """
    TO = recipient if type(recipient) is list else [recipient]
    message = """From: %s\nTo: %s\nSubject: %s\n\n%s
    """ % (user, ", ".join(TO), subject, body)
    if smtp_port == 465:
        server = smtplib.SMTP_SSL(smtp_server, smtp_port)
        server.login(user, pwd)
        server.sendmail(user, TO, message)
        server.quit()
    else:
        try:
            server = smtplib.SMTP(smtp_server, smtp_port)
            server.ehlo()
            server.starttls()
            server.login(user, pwd)
            server.sendmail(user, TO, message)
            server.close()
            return True
        except smtplib.SMTPNotSupportedError:
            server = smtplib.SMTP(smtp_server, smtp_port)
            server.ehlo()
            server.login(user, pwd)
            server.sendmail(user, TO, message)
            server.close()
            return True
        except:
            return False

# load .env stats
load_dotenv()
    
com_net_send_email(os.environ['MAILUSER'],
                                os.environ['MAILPASS'],
                                os.environ['MAILUSER'],
                                "subject",
                                "body",
                                smtp_server=os.environ['MAILSERVER'],
                                smtp_port=os.environ['MAILPORT'])