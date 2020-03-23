# reboot-pc-daemon

Windows daemon that accepts TCP requests to reboot your PC.

## Why?

I dual boot Windows to play games and sometimes forget to reboot back into Linux, this daemon allows me to easily trigger the machine to reboot to get back to Linux.

An alternate solution would be to just enable RDP in Windows then RDP into the machine and manually reboot it.

## Example usage

On windows machine:

```powershell
PS> .\reboot-pc-daemon.exe "secret-code-to-trigger-reboot"
```

On linux machine:

```bash
$ echo "secret-code-to-trigger-reboot" | nc <public ip of windows machine> 64321
```

## Installation

Save `reboot-pc-daemon.exe` to a folder on your PC, e.g "C:\\Tools\\"

Add a registry key to start the daemon on boot

```powershell
PS> reg add "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run" /V "Reboot PC Daemon" /t REG_SZ /F /D """C:\Tools\reboot-pc-daemon.exe"" ""my-secret-code"""
```

In your router settings, forward port 64321 to your machine.
