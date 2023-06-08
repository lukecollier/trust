# Next Milestone
- [ ] Refactor
- [ ] CI/CD Packaged on github packages
- [ ] Brew release

# Example 
Key points
1. Cascades, non declarative
2. Encapsulates
3. Works similar to HTML (without CSS complexity)
4. Depth of nesting decides type, going session/window/pane
5. Global commands can exist

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes" ?>
<!-- global commands work! Cascading to all sessions below -->
echo | pwd
<!-- every session pane will now echo the pwd -->
<work>
  cd ~/projects/important-project <!-- commands cascade to all children -->
  <edit>
    $EDITOR 
  </edit> <!-- will open editor with after cding into ~/projects/important-project -->
  <term>
    $BUILD_TOOL build
  </term>
</work> <!-- creates a session with two windows for editing and a terminal -->
```

So what happens with the above? Well it evaluates from top to bottom, the commands are replicated in every sub tree of the format. This opens the door to be able to share environment variables with certain windows.

## Commands
trust \[-a\] (start all sessions)
trust <session> (start and connect to session if available)
trust -a <session> (start all sessions and connect to session if available)
trust —-list or trust -l (shows all sessions in a list)

## Flags
- -a --all starts all session
- -c --config selects the config file other then default (~/.config/trust/setup.xml)
- -f --first (synonymous with force) will always find a way to connect to a session, can be a useful alias if only one session is in use

# Roadmap
1. CLI Session Management (with benchmarks)
2. Put on homebrew 
3. Tab completion

# Features RFC
- Plugins
- Save current state
- Apply config state
- Diff current state with config state then apply

- Allow height, and width with different units and auto resizing
```xml 
<window_one>
    <pane_one width="100px">
     //blah
    </pane_one>
    <pane_two width="auto", height="auto"
     //blah
    </pane_two>
</window_one>
``` 
with the above we can default height and width to auto, which means _my parent layout should decide my size_.
This means parents have logical layouts they can choose, will look into if HTML has any terms to ~steal~ borrow.
This is a pretty hard problem, and CCS solves it best so will look into how that all works.
Key would be to _keep it logical and keep it simple!_ with the above example we get to pane\_one and place it first requiring it has a 100px width, the height is auto so we make it take up the current remaining space.
With the next pane we do the same only now we're operating in the reduce screen size of `screen-width - 100px` so we can use the `wh` units from CSS to do this well 


- nice errors if the desired set-up is not possible
```xml 
<window_one>
    <pane_one width="100px", height="100px">
     //blah
    </pane_one>
    <pane_two>
     //blah
    </pane_two>
</window_one>

--> 

[ERROR] Not enough space for <pane_two> to be placed!

hint: You can add another pane and the formatter will be able to organise a _best guess_
```

- fun one is we already have a serialized format for the data, could allow teams to share common dev set ups + commands... if we fail we can report back the failures 

- syntax sugar for `cd` using attributes
```xml 
<root pwd="~/path/to/thing"></root>
```
- syntax sugar for `export ENV=some_thing` using attributes
```xml 
<root ENV_VAR="Hello">echo $ENV_VAR</root>
```

# Justifications
## XML
XML has fallen out of favor, but I think carries strong qualities for this kind of thing. I personally find it easier to read for UI and the encapsulating nature lends itself well to hierarchies.
## Rust
Good way to learn rust
### quick\_xml
Incredibly fast xml parsing with a nice event based output

## Changes
- panes can no longer be nested
 
