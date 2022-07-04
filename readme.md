# REHD doorskip patcher
based on [the runtime patcher by FluffyQuack](https://github.com/FluffyQuack/rehd-nodoors)
but done once on the executable itself. this has the advantage of not needing the patcher when the
game is running and making it work on linux (my main incentive for doing this). 

i also tried to make them more future proof by not hardcoding the offsets and instead looking for the respective 2-3 
instructions to be replaced, if that worked we'll see with the next update i suppose

# usage
```
cargo run -- path/to/game/bhd.exe
```
thats it

# Credits
- FluffyQuack for the original [patches](https://github.com/FluffyQuack/rehd-nodoors)
