# Example 
Key points
1. Cascades, non declarative
2. Encapsulates
3. Works similar to HTML (without CSS complexity)
4. Depth of nesting decides type, going session/window/pane
5. Global commands can exist

<?xml version="1.0" encoding="UTF-8" standalone="yes" ?>
<!-- global commands work! Cascading to all sessions below -->
echo (pwd)
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
