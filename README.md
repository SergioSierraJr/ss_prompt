# ss_prompt
A prompt in rust written by me.  Copied it from a fireship video. Should be cross platform but no guarantees. Uses termion and homedir.
# what youll need
working installations of cargo and git. Also a shell(obviously).
# how to use
First, clone the repository and compile the program for release using cargo
```sh
git clone https://github.com/SergioSierraJr/ss_prompt.git
cd ss_prompt/
cargo build --release
```
Now all you have to do is execute the executable in your shells prompt function. For bash you would add the following code to your .bashrc after replacing `/path/to/ss_prompt` with the actual path to the `ss_prompt` executable.
```sh
PROMPT_COMMAND='PS1="$(/path/to/ss_prompt)"'
```
Keep in mind this example works for bash but is unlikely to work for other shells. Please refer to your shell of choice's documentation.
# credtis
Credits to the termion, homedir, and rust team for making this project possible.
