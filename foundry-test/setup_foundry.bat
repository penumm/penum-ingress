@echo off
echo Foundry installation on Windows requires WSL (Windows Subsystem for Linux) or Git BASH.
echo Native Windows PowerShell is NOT supported for Foundry installation.

echo To install Foundry, please follow these steps:

echo 1. Install WSL (Windows Subsystem for Linux):
echo    - Open PowerShell as Administrator
echo    - Run: wsl --install
echo    - Restart your computer after installation
echo.

echo 2. After WSL is set up:
echo    - Launch Ubuntu from the Start menu
echo    - Run these commands in the Ubuntu terminal:
echo      curl -L https://foundry.paradigm.xyz ^| bash
echo      foundryup
echo.

echo 3. Alternatively, you can install Git BASH:
echo    - Download from: https://gitforwindows.org/
echo    - Run Foundry installer in Git BASH terminal:
echo      curl -L https://foundry.paradigm.xyz ^| bash
echo      foundryup
echo.

echo 4. Once installed, run tests with:
echo      forge test

echo For more information, visit: https://getfoundry.sh/getting-started/installation