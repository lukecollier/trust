# Status
[Active Development]

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

# Interface
The interface aims to be minimal with useful flags available for most commands
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
3. Fast low profile tab completion (must be in the ns of speed cost)
4. Tmux module changed to external lib
5. Tmux module published to crate.io

# Features RFC
- Allow for specifying pwd in attributes 
```xml 
<root pwd="~/path/to/thing"></root>
```
- env variables set as attributes seems cool too
```xml 
<root ENV_VAR="Hello">echo $ENV_VAR</root>
```
- Compiling / serializing the xml into a binary format for quicker access, this might not be too useful though given the speed at which the fast\_xml crate parses. The idea here would allow you too install multiple config files to a compiled conf


# Justifications
## XML
XML has fallen out of favour, but I think carries strong qualities for this kind of thing. I personally find it easier to read for UI and the encapsulating nature lends itself well to hierarchies.
## Rust
Good way to learn rust
### quick\_xml
Incredibly fast xml parsing with a nice event based output


## Changes
- panes can no longer be nested
 
