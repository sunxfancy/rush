@echo off
rem del /F /Q 1.exe
cd %~dp0\
.\g++.exe ..\..\..\bin\%1 %3 -o ..\..\..\bin\%2 -w
cd ..\..\..\bin\