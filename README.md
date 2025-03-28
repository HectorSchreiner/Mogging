# Fast Logging Libary
Mogging is an aMazingly & Blazingly Fast Logging Libary written in Rust, which mogs all other logging libaries. The main Purpouse of "mogging" (as in the name of the libary) is to give the user as much customisability as possible, to how to logs are handled and displayed/outputted.
# Features

**Done Diego**
- [x] Synchronous logging
- [x] Basic Logging to the console
- [x] Gangster Logging Macros
- [x] Test cases, add testing
- [x] Log Level sorting (only show logs from a certain level and up)

**next**
- [ ] dynamic batch sizing, and flush timing
- [ ] Asynchronous outputting
- [ ] GPU multithreading support!

**Upcoming Features**
- [ ] File Logging, both overwrite and append mode. optionally timestamps in filenames
- [ ] Formatting to multiple outputs
- [ ] Improve Control over the formatting, colors and more
- [ ] Add benchmarking (compare against common logging libaries)
- [ ] GPU multithreading

# Benchmarking
- **10000 prints** -
| Type | avg. time |
| - | - |
| Crossterm Default | 13.165259ms |
| Println! Default | 10.618974ms |

```
Benchmark with: 10000 prints
Crossterm Logging a took : 13.165259ms
Println Logging took     : 10.618974ms
```
