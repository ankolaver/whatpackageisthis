# what package is this?

> Bckgrd: Hopefully the tool will allow one to understand the packages which are found on or about to update on their linux - machine through the package manager

Id3a:
- read in (using standard input stdin) the different packages provides description from the package repositories 
- Calculates the last updated dates (using the [chrono crate](https://crates.io/crates/chrono))
- Fetches (most recent) reported bugs from GitHub and displays them
- Showcases the timelag between when packages are released upstream and when they are actually packaged and built for the package manager

![Screenshot from 2021-04-16 13-44-53](https://user-images.githubusercontent.com/47784720/114977282-016da280-9eba-11eb-8fbc-fd7e607627f9.png)


### Debug Log

#### [Learning Concurrency vs Parallelism](https://tokio.rs/tokio/tutorial/spawning)

Tried [async features](https://tokio.rs/) to handle more tasks at once. However, the program still ran at the same speed as before. 

Suspicious, I attempted executing multiple of such commands in the terminal emulator.

As it turns out, trying to run multiple commands simultaneously leads to a PID process error waiting. I.e, the program only allows one instance of each item to be run, and prevent multiple instances of same program.

Eventually, I managed to find a replacement command using `$ rpm -q <reponame>`

#### Borrowed Value does not live long enough

![Screenshot from 2021-04-15 20-22-57](https://user-images.githubusercontent.com/47784720/114887733-4999b000-9e3b-11eb-8363-d7d6ae47febd.png)

`tokio::spawn` creates a runtime by itself. 
`&variable_name[..]` converts an `String` type to an `&str` type by slicing it with the `[..]` syntax

After some digging, it becomes clear that we cannot use a `&str` type for calling, as that would create references from the function `scrape_web` to the `&str` variable `github_link` (as seen below), but the lifetime of the variable has already ended once the function is called (and spawned off via `tokio::spawn`). 

![Screenshot from 2021-04-15 20-31-25](https://user-images.githubusercontent.com/47784720/114888692-10157480-9e3c-11eb-8353-48056701ce7a.png)

Instead we change the function input from `&str` to a `String` type. This helps to resolve the issue.

###### To Reference: https://doc.rust-lang.org/rust-by-example/scope/lifetime.html

#### Issues faced with scraping the github website
![Screenshot from 2021-04-16 13-52-51](https://user-images.githubusercontent.com/47784720/114978007-329aa280-9ebb-11eb-8621-b20d83f42ffb.png)

The main idea was to correlate the version of the repository with the one found on github and extract the date that it was released from the repository. However, it seemed slightly difficult to extract that from the HTML code as the time were in two separate classes.

My first instinct was to simply bruteforce to find the version number; exit the `<pre class>` and traverse down to the `<ul class` and slowly enter the child classes but it seemed way too difficult to accomplish. Instead, I found it more efficient to simply look out for the HTML Class Names
```html 
<!-- for version numbers -->
<pre class="text-small color-text-secondary"></pre>
```
and 
```html
<!-- to extract time when repo was published -->
<relative-time class="no-wrap"></relative-time>
```

and traverse them using Rust's `izip!` macro from the `itertools` crate (similar to how `zip()` works in Python) to enumerate through two items at once. We assume here that the time of publishing almost always comes after the version number. 

:arrow_right: Correction: not all websites with `/tag` address in github look like that, and hence causes this functionality to be buggy. It may be better to use Github's API, but there's a limit for that :man_shrugging:

#### Refactoring code
Faced difficulty trying to import module from one file to another.

Crates need to be registered prior in the `main.rs` using 
```rust
mod <module1>;
``` 
before using the use crate syntax to utilise another module in a sub file (a file that is not `main.rs`).
```rust
use crate::mod2;
```
https://stackoverflow.com/questions/46829539/how-to-include-files-from-same-directory-in-a-module-using-cargo-rust


### Improvements
- enabling support for repositories which are not just from github; eg. gitlab, bugz websites

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