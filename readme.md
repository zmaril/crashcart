# Crashcart
_For when you need to save the server's life_ 


![https://upload.wikimedia.org/wikipedia/commons/6/6a/Crash_Cart.jpg](A crash cart from wikipedia)

Crashcart is an in progress suite of tools used to diagnose and treat various problems that a server might encounter during its lifetime, particularly in times of crisis. Crashcart combines an interactive notebooks built on top of rerun.io with a SQL to BPF compiler in order to let system administrators of all stripes quickly and confidently deal with a wide variety of issues that might affect a system. 

Work in progress. 

# Quickstart 

To install crashcart, run the following commands:
```bash
git clone https://github.com/zmaril/crashcart
cd crashcart
cargo install rerun-cli
cargo install --path .
crashcart
```

For now to update crashcart, run the following commands: 
```bash
cd crashcart
git pull
cargo install --path .
```

# Motivation 

I never really got AWK, and so bpftrace never really clicked with me. Likewise, using bpf directly is hard and it never became the tool I reach for first. About ten years ago, I thought about compiling SQL to BPF, but it seemed really difficult to do. I still haven't done it yet, but the tools have advanced enough now that I think it feels possible in the next few weeks with some work. 

# Architecture 

Uses rust
Clang
Compiles bpf programs in clang in container on your computer, then ssh's them over. 

Hand written lexer and parser for SQL, because I like doing that, plus writing a custom one based off sqlparser-rs seems hard. 


# Zack's todo list  
* Set up rerun 
* Figure out how to get clang to run the right way on mac. 
* Use colima and docker in order to build stuff 