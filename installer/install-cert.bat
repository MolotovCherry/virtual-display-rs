@echo off

certutil -addstore -f root "DriverCertificate.cer" >NUL 2>NUL
certutil -addstore -f TrustedPublisher "DriverCertificate.cer" >NUL 2>NUL
