*First of all I'd like to thank u for using my first ever semi-usefull Rust tool.*  
***Before u start, I would recommend building the exe first and then placing a file named*** `config.txt` ***in the same directory as it.***    
## How does it work?  
# Config file  
The config file is the most important part of this entire program. Pay attention.  
This program reads the file in a specific order and if that isn't setup correctly it ***won't work.***   
#### The Order:  
1. The path to the usb *Only available for Unix filesystems for now.*   
2. The path to the folder that you want to back everything up to.  
3. *The names of the folders (end with an* `/` *) or files that u want to back up comes after the first 2 lines, each one on a line of it's own.*  
*Please make sure that there are no empty lines in the config file.*
**An example of how a config file might look:**  
```
/media/user/MY_USB/
/MY_BACKUPS/
file1.txt
file2.txt
file3.txt
```
----------
That's about it for now. If you set up the config correctly and run it, it will back up your files. I am planning to make this automatic in the future.
