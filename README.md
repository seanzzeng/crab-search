# trace
a lightweight, fast cli-based file search engine for windows, in rust
## features
- thousands of times faster than regular windows file search (minutes -> milliseconds)
- path reconstructions are done with no memory overhead
- smart filtering
- interactive pager, source folder of any file is launchable directly from the terminal

## installation
1. download the [latest release](https://github.com/seanzzeng/trace/releases)
2. run exe file as administrator
> [!WARNING]
> this program must be run as an administrator, as it communicates with the storage volume at a kernel level  
> not doing so will cause the program to instantly crash

> [!NOTE]
> optionally, add the file to windows PATH to run it from any privileged terminal

## usage
it will take a few seconds initially for the engine to map the C drive into memory
### standard search
```bash
Search > notes
Search > lunar client
```
### filter by extension
```bash
Search > cmd ext:exe
Search > inequalities booklet ext:pdf
```
### filter by folders
```bash
Search > system32 type:folder
Search > homework type:dir
```
when a search returns multiple files, the native pager pauses the output to prevent terminal flooding
```bash
[0] C:\Users\Name\Desktop\notes.txt
[1] C:\Users\Name\Documents\old_notes.md
[2] C:\Users\Name\AppData\secret_notes.pdf

--- Showing 3 of 3 --- [ENTER: Next Page | q: New Search | NUMBER: Open File] >
```
- press `ENTER` to scroll to the next page
- type a `Number` (e.g. `0`) to open the source folder for that file
- type `q` to clear the pager and begin a new search
### exiting
type `quit` to exit the app
```bash
Search > quit
Exiting...
```






  
