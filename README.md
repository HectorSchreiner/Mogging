# Fast Logging Libary
Mogging is an aMazingly & Blazingly Fast Logging Libary written in Rust, which mogs all other logging libaries. The main Purpouse of "mogging" (as in the name of the libary) is to give the user as much customisability as possible, to how to logs are handled and displayed/outputted
# Features

**Done Diego**
- [x] Synchronous logging
- [x] Basic Logging to the console
- [x] Gangster Logging Macros
- [x] Test cases, add testing
- [x] Log Level sorting (only show logs from a certain level and up

**Upcoming Features**
- [ ] File Logging, both overwrite and append mode. optionally timestamps in filenames
- [ ] Formatting to multiple outputs
- [ ] Improve Control over the formatting, colors and more
- [ ] Add benchmarking (compare against common logging libaries)

# Benchmarking
**10000 Prints**
| Method | avg. time |
| - | - |
| Default Crossterm | 13.165259ms |
| Println! | 10.618974ms |
| Print! | 548.132µs |

```
Benchmark with: 10000 prints
_______________________________________________
Mogger Logging took             : 249.447µs
Println Logging took            : 14.162727ms
Print Logging took              : 862.997µs
Crossterm queue Logging took    : 969.456µs
```
