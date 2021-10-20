// pub mod default {}
/// to log your projects (custom)
/// 
/// # Use
/// To use the custom log, you need to use 3 steps.
/// At first you will create a new `Log`. (You can also create more than one log for different usage.)
/// 
/// Just to remind, dont forget to import the [`custom`](https://docs.rs/log_file/0.1.1/log_file/custom) mudule:
/// ```rust
/// use log_file::custom::*;
/// ```
/// Then go on with the creating the `Log`:
/// ```rust
/// // create Log
/// let mut log = Log::new(true, ':');
/// ```
/// The second step is to use it. In the following I just will show the different options:
/// ```rust
/// // with context as String
/// log.add("Titel 1",String::from("Context 1"));
/// 
/// // with context as &str
/// log.add_str("Titel 2","Context 2");
/// ```
/// If that is done, you can finaly save the log in the end. Here an example for this step:
/// ```rust
/// // save log
/// log.save("log.txt");
/// ```
/// And now you are done. Now, every time you run your project, the new log will overwrite the existing one, if the file name dosn't change.
pub mod custom {
    /// elements (entries) in the log
    pub struct LogElement {
        time_stamp  :   Option<std::time::SystemTime>,
        title       :   String,
        context     :   String
    }
    impl LogElement {
        /// creates new LogElement with context as `String`
        /// 
        /// # Example
        /// ```rust
        /// let example = LogElement::new("Test",String::from("Hello, world!", true));
        /// ```
        pub fn new(title : &str, context : String, time_stamp_on : bool) -> LogElement {
            LogElement {
                time_stamp  :   if time_stamp_on {
                    Option::Some(std::time::SystemTime::now())
                } else {
                    Option::None
                },
                title       :   String::from(title),
                context     :   context
            }
        }
        /// creates new LogElement with context as `&str`
        /// 
        /// # Example
        /// ```rust
        /// let example = LogElement::new_str("Test","Hello, world!", true);
        /// ```
        pub fn new_str(title : &str, context : &str, time_stamp_on : bool) -> LogElement {
            LogElement {
                time_stamp  :   if time_stamp_on {
                    Option::Some(std::time::SystemTime::now())
                } else {
                    Option::None
                },
                title       :   String::from(title),
                context     :   String::from(context)
            }
        }
        /// converts this struct `LogElement` to `String`
        /// 
        /// # Example
        /// ```rust
        /// let example = LogElement::new_str("Test","Hello, world!", true);
        /// 
        /// println!("{}",example.to_string(':',std::time::SystemTime::now()));
        /// ```
        /// output:
        /// ```
        /// [0:0:25:0]  Test    :   Hello, world!
        /// ```
        /// 
        /// # Panics
        /// Panics if the [`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html) is before the UNIX EPOCH.
        pub fn to_string(&self, seperator : &char, start : std::time::SystemTime) -> String {
            match self.time_stamp {
                Some(time)  =>  {
                    format!(
                        "[{}]\t{}\t{}\t{}",
                        match time.duration_since(start.clone()) {
                            Ok(n) => {
                                format!(
                                    "{}:{}:{}:{}",
                                    trim_to_1000(n.as_secs() as usize),
                                    trim_to_1000(n.as_millis() as usize),
                                    trim_to_1000(n.as_micros() as usize),
                                    trim_to_1000(n.as_nanos() as usize)
                                )
                            },
                            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                        },
                        &self.title,
                        seperator,
                        &self.context
                    )
                },
                None        =>  {
                    format!(
                        "{}\t{}\t{}",
                        &self.title,
                        seperator,
                        &self.context
                    )
                },
            }
        }
    }
    impl Clone for LogElement {
        fn clone(&self) -> LogElement {
            LogElement::new(&self.title.clone()[..], self.context.clone(), {
                match self.time_stamp {
                    Some(_)  =>  true,
                    None        =>  false
                }
            })
        }
    }
    /// The main struct for the custom log
    /// 
    /// # Example
    /// To use the custom log, you need to use 3 steps.
    /// At first you will create a new `Log`. (You can also create more than one log for different usage.)
    /// 
    /// Just to remind, dont forget to import the [`custom`](https://docs.rs/log_file/0.1.1/log_file/custom)
    /// ```rust
    /// use log_file::custom::*;
    /// ```
    /// Then go on with the creating the `Log`:
    /// ```rust
    ///     // create Log
    ///     let mut log = Log::new(true, ':');
    /// ```
    /// The second step is to use it. In the following I just will show the different options:
    /// ```rust
    /// // with context as String
    /// log.add("Titel 1",String::from("Context 1"));
    /// 
    /// // with context as &str
    /// log.add_str("Titel 2","Context 2");
    /// ```
    /// If that is done, you can finaly save the log in the end. Here an example for this step:
    /// ```rust
    /// // save log
    /// log.save("log.txt");
    /// ```
    /// And now you are done. Now, every time you run your project, the new log will overwrite the existing one, if the file name dosn't change.
    pub struct Log {
        time_stamp  :   bool,
        start_time  :   std::time::SystemTime,
        seperator   :   char,
        elements    :   Vec<LogElement>
    }
    impl Log {
        /// create a new log
        /// 
        /// # Example
        /// ```rust
        /// let example = Log::new(true, ':'));
        /// ```
        pub fn new(time_stamp_on : bool, seperate : char) -> Log {
            Log {
                time_stamp  :   time_stamp_on,
                start_time  :   std::time::SystemTime::now(),
                seperator   :   seperate,
                elements    :   Vec::new(),
            }
        }
        /// adding an element to this log with context as `String`
        /// 
        /// # Example
        /// ```rust
        /// let mut example = Log::new(true, ':'));
        /// example.add("Test",String::from("Hello, world!"));
        /// ```
        pub fn add(&mut self, titel : &str, context : String) {
            self.elements.push(LogElement::new(titel, context, self.time_stamp));
        }
        /// adding an element to this log with context as `&str`
        /// 
        /// # Example
        /// ```rust
        /// let mut example = Log::new(true, ':'));
        /// example.add_str("Test","Hello, world!");
        /// ```
        pub fn add_str(&mut self, titel : &str, context : &str) {
            self.elements.push(LogElement::new_str(titel, context, self.time_stamp));
        }
        /// saves the log in `file_name`
        /// 
        /// Creates file if it does not exist and overwrites it if it does exist.
        /// 
        /// # Note
        /// You have to give the whole file name as parameter. For example `foo.txt`. If you just give `foo` as file name, it will create a file without file type.
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        ///     example.save("foo.txt");
        /// }
        /// ```
        pub fn save(&self, file_name : &str) {
            std::fs::File::create(file_name).expect("Unable to write file");
            std::fs::write(file_name, self.to_string()).expect("Unable to write file");
        }
        /// converts this struct `Log`
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        ///     println!("{}",example.to_string());
        /// }
        /// ```
        /// output:
        /// ```batch
        /// 
        /// ```
        pub fn to_string(&self) -> String {
            // start value
            let mut value = String::new();

            // counter
            let mut count = 0_usize;

            // collect value
            for element in self.elements.iter() {
                let s : String = element.to_string(&self.seperator,self.start_time.clone());
                for c in s.chars() {
                    value.push(c);
                }
                if count < self.elements.len()-1 {
                    value.push('\n');
                }

                count += 1;
            }

            // return value
            return value;
        }
    }
    impl Clone for Log {
        fn clone(&self) -> Log {
            Log::new(self.time_stamp.clone(), self.seperator.clone())
        }
    }

    // function
    /// scale the parameter to a number lower than 1000
    /// 
    /// # Example
    /// ```rust
    /// let to_big = 12345_usize;
    /// println!("{}",trim_to_1000(to_big));
    /// ```
    /// output:
    /// ```batch
    /// 123
    /// ```
    fn trim_to_1000(n:usize) -> usize {
        let mut num = n;
        while num > 1000 {
            num /= 1000;
        }
        return num;
    }
}