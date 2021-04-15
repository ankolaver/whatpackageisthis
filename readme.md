# whatpackageisthis?

Bckgrd: Hopefully the tool will allow one to understand the packages which are found on or about to update on their linux - machine through the package manager

- read in (using standard input stdin) the different packages provides description from the package repositories 
- Calculates the last updated dates
- Fetches (most recent) reported bugs from GitHub and displays them

### Debug Log

#### [Learning Concurrency vs Parallelism](https://tokio.rs/tokio/tutorial/spawning)

Tried async features to handle more tasks at once. However, the program still ran at the same speed as before. 

Suspicious, I attempted executing multiple of such commands in the terminal emulator.

As it turns out, trying to run multiple commands simultaneously leads to a PID process error waiting. I.e, the program only allows one instance of each item to be run, and prevent multiple instances of same program.

Eventually, I managed to find a replacement command using `$ rpm -q <reponame>`

#### Borrowed Value does not look long enough

![Screenshot from 2021-04-15 20-22-57](https://user-images.githubusercontent.com/47784720/114887733-4999b000-9e3b-11eb-8363-d7d6ae47febd.png)

Unlike normal functions, tokio::spawn creates a runtime by itself. 
After some digging, it becomes clear that we cannot use a `&str` type for calling, as that would create double references. 

![Screenshot from 2021-04-15 20-31-25](https://user-images.githubusercontent.com/47784720/114888692-10157480-9e3c-11eb-8353-48056701ce7a.png)

Instead we change the function input from `&str` to a `String` type. This helps to resolve this issue.

 https://doc.rust-lang.org/rust-by-example/scope/lifetime.html

### Refactoring code
Faced difficulty trying to import module from one file to another.

Crates need to be registered prior in the `main.rs` using 
```rust
mod <module1>
``` 
before using the use crate syntax to utilise another module in a sub file (a file that is not `main.rs`).
```rust
use crate::mod2;
```
https://stackoverflow.com/questions/46829539/how-to-include-files-from-same-directory-in-a-module-using-cargo-rust


### usage 

For RHEL-linuxes / Fedora, use
```
dnf check-update | cargo run
```


Extracting Issue names from github 
```
data-hovercard-type="issue"
```
no. of comments for each one
```
class="Link--muted" aria-label=
```