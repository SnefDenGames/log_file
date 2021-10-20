<p align="center"><img src ="https://github.com/SnefDenGames/log_file/blob/master/logo.png?raw=true" /></p>

<p align="center">
    A log.txt writer
</p>

---

[![log_file at crates.io](https://img.shields.io/crates/v/docx-rs.svg)](https://crates.io/crates/log_file)

## Table of contents

1.	[Instalation](#instalation)
2.	[Example](#example)
3.	[Features](#features)
4.	[Requirements](#requirements)

## Instalation

### update Cargo.toml file
Add [`log_file`](https://crates.io/crates/log_file) to your dependencies:
```
[dependencies]
log_file = "0.1.1"
```

### import it
Import as much as you like from the log_file lib:
```rust
use log_file;
```

## Example

### Log types
* custom:
	* [structure example](#structure-example-(custom))
	* [second example](#second-example-(custom))

### structure example (custom)
Here an example for [`log_file::custom`](https://docs.rs/log_file/0.1.1/log_file/custom/index.html):
```rust
// import everything from the `custom` module
use log_file::custom::*;

fn main() {
	// create the log
	let mut log = Log::new(false, ':');
	
	// using it
	log.add_str("Programmer","SnefDen");
	log.add_str("crate name","log_file");
	log.add_str("crate version","0.1.1");
	
	// save log in log.txt
	log.save("log.txt");
}
```
The `log.txt` file then should contain the following text:
```
Programmer	:	SnefDen
crate name	:	log_file
crate version	:	0.1.1
```
If the time stamp is on (`let mut log = Log::new(true, ':');`), the result looks like this for example:
```
[0:0:0:100]	Programmer	:	SnefDen
[0:0:0:250]	crate name	:	log_file
[0:0:0:250]	crate version	:	0.1.1
```
So the structure is the following:
```
time(optional)	title	separator	content
```
The time is written in the following format and started if the log is created:
```
s:ms:µs:ns

s	=	seconds
ms	=	milliseconds (0.001s)
µs	=	mikroseconds (0.001ms)
ns	=	nanoseconds  (0.001µs)
```

### second example (custom)
For this example we use the pythagorean_theorem method. Here is the implementation without log:
```rust
fn pythagorean_theorem(log : &mut log, a : f64, b : f64) -> f64 {
	let a_sq = a*a;
    let b_sq = b*b;
    let c = (a_sq + b_sq).sqrt();
    
    return c;
}
```
This time we don't create a new log. Instead we change our header, so that we can use an existing one (and don't forget to make it mutable).
At last we update the log, based on the steps in the `pythagorean_theorem()` method. Now the method should look like this:

```rust
fn pythagorean_theorem(log : &mut Log, a : f64, b : f64) -> f64 {
log.add_str("pythagorean_theorem - step 1","a*a");
	let a_sq = a*a;
log.add_str("pythagorean_theorem - step 2","b*b");
    let b_sq = b*b;
log.add_str("pythagorean_theorem - step 3","(a_sq + b_sq) * (a_sq + b_sq)");
    let c = (a_sq + b_sq).sqrt();
    
    return c;
}
```
If we use this function in `main()`, it looks like this:
```rust
use log_file::custom::*;

fn main() {
	// create log
	let mut log = Log::new(false, ':'));
	
	// call pythagorean_theorem() of 2 and 3
	println!("{}",pythagorean_theorem(&mut log, 2, 3));
	
	// save log in `log.txt`
	log.save("log.txt");
}

fn pythagorean_theorem(log : &mut Log, a : f64, b : f64) -> f64 {
	// snipped //
}
```
The `log.txt` file now contains the following text:
```
pythagorean_theorem - step 1	:	a*a
pythagorean_theorem - step 2	:	b*b
pythagorean_theorem - step 3	:	(a_sq + b_sq) * (a_sq + b_sq)
```

## Features
* [x] [custom](https://docs.rs/log_file/0.1.1/log_file/custom/)
	* [x] [Log](https://docs.rs/log_file/0.1.1/log_file/custom/struct.Log.html)
	
	  * [x] [new()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.Log.html#method.new)
	  * [x] [new_str()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.Log.html#method.new_str)
	  * [x] [add()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.Log.html#method.add)
	  * [x] [add_str()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.Log.html#method.add_str)
	  * [x] [save()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.Log.html#method.save)
	  * [x] [to_string()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.Log.html#method.to_string)
	  * [x] [clone()](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)
	
	* [x] [LogElement](https://docs.rs/log_file/0.1.1/log_file/custom/struct.LogElement.html)
	
	  * [x] [new()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.LogElement.html#method.new)
	  * [x] [new_str()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.LogElement.html#method.new_str)
	  * [x] [to_string()](https://docs.rs/log_file/0.1.1/log_file/custom/struct.LogElement.html#method.to_string)
	  * [x] [clone()](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)
	
	* [x] [trim_to_1000()](https://docs.rs/log_file/0.1.1/log_file/custom/fn.trim_to_1000.html)
	
 ## Requirements
 None