//pub mod default {}
//pub mod custom {}
pub mod default_code {
    // enums
    /// contains types of the log context
    pub enum LogContext {
        Variable(String, String),
        FunctionCall(String, String),
        IfStatement(String),
        ElseIfStatement(String),
        ElseStatement,
        ReturnStatement(String),
        Loops(),
        //Loop(),
        //WhileLoop(),
        //ForLoop(),
    }

    // structs
    /// The main struct for the default_code log
    /// 
    /// # Example
    /// To use the default_code log, you need to use 3 steps.
    /// At first you will create a new `Log`. (You can also create more than one log for different usage.)
    /// 
    /// Just to remind, dont forget to import the [`default_log`](http://https://docs.rs/log_file/0.1.0/log_file/default_code)
    /// ```rust
    /// use log_file::default_code::*;
    /// ```
    /// Then go on with the creating the `Log`. Therefore you need to know, if you want the time attribute ore not. Here an example with:
    /// ```batch
    /// "0:0:2:2"	main()	:	num = 2
    /// "0:0:6:6"	main()	:	call faculty(2)
    /// "0:0:7:7"	faculty()	:	if 2 == 1
    /// "0:0:8:8"	faculty()	:	else
    /// "0:0:9:9"	faculty()	:	return 2 * faculty(2-1)
    /// "0:0:18:18"	faculty()	:	if 1 == 1
    /// "0:0:19:19"	faculty()	:	return 1
    /// "0:0:21:21"	main()	:	fac = 2
    /// "0:0:131:131"	main()	:	call println!(()! = (), 2, 2)
    /// ```
    /// and now the same without the time stamps:
    /// ```batch
    /// main()	:	num = 2
    /// main()	:	call faculty(2)
    /// faculty()	:	if 2 == 1
    /// faculty()	:	else
    /// faculty()	:	return 2 * faculty(2-1)
    /// faculty()	:	if 1 == 1
    /// faculty()	:	return 1
    /// main()	:	fac = 2
    /// main()	:	call println!(()! = (), 2, 2)
    /// ```
    /// The time stamp will be the first parameter.
    /// 
    /// The second parameter is the seperator. In the last examples you can see, that the : seperated the running function from the action in the function. But you can customize it if you want.
    /// How ever, the next step is to create a `Log` with your preferences. Here an example:
    /// ```rust
    /// fn main() {
    ///     // create Log
    ///     let mut log = Log::new(false, String::from(":"));
    /// }
    /// ```
    /// The second step is to use it. In the following I just will show the different options:
    /// ```rust
    ///     // variables
    ///     log.add_variable("main", "x", "9");
    /// 
    ///     // calling a function
    ///     log.add_function_call("main", "println!", "Hello, World!");
    /// 
    ///     // return something
    ///     log.add_return_statement("main", "Ok(())");
    /// ```
    /// Like you can see, there are some of them. But if that is done, you can finaly save the log in the end. Here an example for this step:
    /// ```rust
    /// fn main() {
    ///     // create Log
    ///     let mut log = Log::new(false, String::from(":"));
    /// 
    ///     // use log
    /// 
    ///     // save log
    ///     log.save("log.txt");
    /// }
    /// ```
    /// And now you are done. Now, every time you run your project, the new log will overwrite the existing one, if the file name dosn't change.
    pub struct Log {
        start_time  :   std::time::SystemTime,
        time_stamp  :   bool,
        seperator   :   String,
        elements    :   Vec<LogElement>
    }
    /// An entry for the [`Log`](http://https://docs.rs/log_file/0.1.0/log_file/default_code/struct.Log.html)
    /// 
    /// # Examples
    /// At first we should create a `LogElement`
    /// ```rust
    /// use log_file::default_code::LogElement;
    /// 
    /// fn main() {
    ///     // representing:    x = 9   in main()
    ///     let example = LogElement::new("main","x","9","V");
    /// 
    ///     // representing:    x.to_string()   in main()
    ///     let example = LogElement::new("main","x.to_string","","FC");
    /// 
    ///     // representing:    rng.gen_range(0, 10)    in main()
    ///     let example = LogElement::new("main","rng.gen_range","0, 10","FC");
    /// }
    /// ```
    /// Now we can call the `to_string()` method, to parse the `LogElement` to a `String`.
    /// ```rust
    /// fn main() {
    ///     -- snippet --
    ///     
    ///     // with time    (s:ms:µs:ns)
    ///     println!("{}",example.to_string(String::from(":"), SystemTime::now(), true));
    ///     
    ///     // without time
    ///     println!("\n{}",example.to_string(String::from(":"), SystemTime::now(), false));
    /// }
    /// ```
    pub struct LogElement {
        time_stamp      :   std::time::SystemTime,
        function_name   :   String,
        context         :   LogContext,
    }

    // impls
    impl Log {
        /// create a new log
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        /// }
        /// ```
        pub fn new(time_stamp_on : bool, seperate : String) -> Log {
            Log {
                start_time  :   std::time::SystemTime::now(),
                time_stamp  :   time_stamp_on,
                seperator   :   seperate,
                elements    :   Vec::new(),
            }
        }
        /// adding an variable change to this log
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        ///     example.add_variable("main", "x", "9");
        /// }
        /// ```
        pub fn add_variable(&mut self, function_name : &str, name : &str, value : &str) {
            let element = LogElement::new(function_name, name, value,"V");

            self.elements.push(element);
        }
        /// adding an function call to this log
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        ///     example.add_function_call("main", "println!", "\"Hello, World!\"");
        /// }
        /// ```
        pub fn add_function_call(&mut self, function_name : &str, name : &str, parameter : &str) {
            let element = LogElement::new(function_name, name, parameter,"FC");
            
            self.elements.push(element);
        }
        /// adding an return statement to this log
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        ///     example.add_return_statement("main", "Ok(())");
        /// }
        /// ```
        pub fn add_return_statement(&mut self, function_name : &str, value : &str) {
            let element = LogElement::new(function_name, "", value, "R");

            self.elements.push(element);
        }
        /// adding an if statement to this log
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        ///     example.add_if_statement("main", "3==2");
        /// }
        /// ```
        pub fn add_if_statement(&mut self, function_name : &str, value : &str) {
            let element = LogElement::new(function_name, "", value,"IF");

            self.elements.push(element);
        }
        /// adding an else if statement to this log
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        ///     example.add_else_if_statement("main", "'A'=='a'");
        /// }
        /// ```
        pub fn add_else_if_statement(&mut self, function_name : &str, value : &str) {
            let element = LogElement::new(function_name, "", value,"ELIF");

            self.elements.push(element);
        }
        /// adding an else statement to this log
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::Log;
        /// 
        /// fn main() {
        ///     let example = Log::new(true, String::from(":"));
        ///     example.add_else_if_statement("main");
        /// }
        /// ```
        pub fn add_else_statement(&mut self, function_name : &str) {
            let element = LogElement::new(function_name, "", "", "ELSE");

            self.elements.push(element);
        }
        /// saves the log in `file_name` (creates file if it doesn't exist)
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
        /// 
        /// # Note
        /// You have to give the whole file name as parameter. For example `foo.txt`. If you just give `foo` as file name, it will create a file without file type.
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
                let s : String = element.to_string(self.seperator.to_string(), &self.start_time, self.time_stamp.clone());
                for c in s.chars() {
                    value.push(c);
                }
                if count < self.elements.len() {
                    value.push('\n');
                }

                count += 1;
            }

            // return value
            return value;
        }
    }
    impl LogElement {
        /// creates new LogElement
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::LogElement;
        /// 
        /// fn main() {
        ///     let example = LogElement::new("main","x","9","V");
        /// }
        /// ```
        /// 
        /// # Panics
        /// Panics if if typ is nothing of them:
        /// * V : Variable
        /// * FC : FunctionCall
        pub fn new(function_name : &str, name : &str, zusatz : &str, typ : &str) -> LogElement {
            LogElement {
                time_stamp      :   std::time::SystemTime::now(),
                function_name   :   function_name.to_string(),
                context         :   match typ {
                    "V"     =>  LogContext::Variable(name.to_string(), zusatz.to_string()),
                    "FC"    =>  LogContext::FunctionCall(name.to_string(), zusatz.to_string()),
                    "IF"    =>  LogContext::IfStatement(zusatz.to_string()),
                    "ELIF"  =>  LogContext::ElseIfStatement(zusatz.to_string()),
                    "ELSE"    =>  LogContext::ElseStatement,
                    "R"     =>  LogContext::ReturnStatement(zusatz.to_string()),
                    _ => {
                        panic!{"Unreachable Option"};
                    }
                }
            }
        }
        /// converts this struct `LogElement`
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::LogElement;
        /// use std::time::SystemTime;
        /// 
        /// fn main() {
        ///     let example = LogElement::new("main","x","9","V");
        /// 
        ///     // with time    (s:ms:µs:ns)
        ///     println!("{}",example.to_string(String::from(":"), SystemTime::now(), true));
        ///     
        ///     // without time
        ///     println!("\n{}",example.to_string(String::from(":"), SystemTime::now(), false));
        /// }
        /// ```
        /// output:
        /// ```batch
        /// 0:0:0:0    main()   :   x = 9
        /// 
        /// main()  :   x = 9
        /// ```
        /// 
        /// # Panics
        /// Panics if the [`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html) is before the UNIX EPOCH.
        pub fn to_string(&self,seperator : String, time : &std::time::SystemTime, time_stamp : bool) -> String {
            if time_stamp {
                format!(
                    "{:?}\t{}()\t{}\t{}",
                    match self.time_stamp.duration_since(time.clone()) {
                        Ok(n) => {
                            format!(
                                "{}:{}:{}:{}",
                                self.trim_to_1000(n.as_secs() as usize),
                                self.trim_to_1000(n.as_millis() as usize),
                                self.trim_to_1000(n.as_micros() as usize),
                                self.trim_to_1000(n.as_nanos() as usize)
                            )
                        },
                        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                    },
                    self.function_name, seperator,
                    self.context.to_string()
                )
            } else {
                format!(
                    "{}()\t{}\t{}",
                    self.function_name, seperator,
                    self.context.to_string()
                )
            }
        }
        /// scale the parameter to a number lower than 1000
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::LogElement::*;
        /// 
        /// fn main() {
        ///     let to_big = 12345_usize;
        ///     println!("{}",trim_to_1000(to_big));
        /// }
        /// ```
        /// output:
        /// ```batch
        /// 123
        /// ```
        fn trim_to_1000(&self, n:usize) -> usize {
            let mut num = n;
            while num > 1000 {
                num /= 1000;
            }
            return num;
        }
    }
    impl LogContext {
        /// converts this enum `LogContext`
        /// 
        /// # Example
        /// ```rust
        /// use log_file::default_code::LogContext;
        /// 
        /// fn main {
        ///     let example = LogContext::FunctionCall("println!","\"Hello World!\"");
        ///     println!("{}",example.to_string());
        /// }
        /// ```
        /// output:
        /// ```batch
        /// println!("Hello World!")
        /// ```
        /// 
        /// # Panics
        /// This method panics if the LogContext is invalid.
        pub fn to_string(&self) -> String {
            match self {
                LogContext::Variable(name, value)           =>  format!("{} = {}", name, value),
                LogContext::FunctionCall(name, parameter)   =>  format!("call {}({})", name, parameter),
                LogContext::IfStatement(value)              =>  format!("if {}",value),
                LogContext::ElseIfStatement(value)          =>  format!("else if {}",value),
                LogContext::ReturnStatement(value)          =>  format!("return {}",value),
                LogContext::ElseStatement                   =>  format!("else"),
                LogContext::Loops()                         =>  format!("Loops()"),
                _                                           =>  panic!("Invalid LogContext")
            }
        }
    }

    #[test]
    fn name() {
        let mut log = Log::new(true,String::from(":"));
        let num = 2;
        let s = format!("{}",&num);
        log.add_variable("main","num", &s[..]);
        let s = format!("{}",&num);
        log.add_function_call("main", "faculty", &s[..]);
        let fac = faculty(&mut log, num as usize);
        let s = format!("{}",&fac);
        log.add_variable("main","fac", &s[..]);
        println!("{}! = {}", num, fac);
        let s = format!("{}, {}, {}","()! = ()",&num,&fac);
        log.add_function_call("main", "println!", &s[..]);
        log.save("log.txt");
    }

    fn faculty(log : &mut Log, num : usize) -> usize {
        let s = format!("{} == 1",&num);
        log.add_if_statement("faculty", &s[..]);
        if num == 1 {
            log.add_return_statement("faculty", "1");
            return 1
        } else {
            log.add_else_statement("faculty");
            let s = format!("{x} * faculty({x}-1)",x=&num,);
            log.add_return_statement("faculty", &s[..]);
            return num * faculty(log, num-1)
        }
    }
}