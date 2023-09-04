@echo off

set CERTIFICATE="%~dp0DriverCertificate.cer"

call "%ProgramFiles%\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat
certmgr /add DriverCertificate.cer /s /r localMachine root
pause
