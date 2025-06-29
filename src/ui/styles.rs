use gtk::{gdk, CssProvider};

pub fn load_css() {
    let provider = CssProvider::new();
    let css = "
* { 
  font-family: 'JetBrainsMono Nerd Font Propo', 'Font Awesome 6 Free',
  FontAwesome, Roboto, Helvetica, Arial, sans-serif;
  font-size: 14px;
}

window {
  border-radius: 0px 0px 8px 8px;
}


window#bar.hidden {
  opacity: 0.2;
}

button{
  outline: none;
  border: none;
}

#launcher {
  min-width: 12px;
  border-radius: 12px;
  transition: 200ms;
  margin-right: 4px;
}
#launcher:hover {
  color: inherit;
  border-radius: 8px;
}
#launcher:active {
  border: 0px;
}

.modules {
  padding: 0 12px;
  margin: 0 2px;
  border-radius: 8px;
  transition: all 0.3s ease;
}

.modules:hover {
  background: rgba(137, 180, 250, 0.2);
}

#window {
  margin: 0 4px;
  border-radius: 10px;
  padding: 0 6px;
}
#workspaces {
  border-radius: 8px 0px 8px 0px;
}

#workspaces #ws-button {
  border-radius: 8px 0px 8px 0px;
  min-width: 12px;
  transition: all 200ms ease;
  border-radius: 12px;
}

#workspaces #ws-button:hover {
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

#workspaces #ws-button.active {
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  border: 2px solid;
}

/* #workspaces button.urgent {
} */


#clock,
#battery,
#cpu,
#memory,
#volume,
#window-title,
#network{
border-radius: 12px;
transition: all 0.3s ease;
padding: 0px;
margin: 0px 0px 0px 12px;
}

#clock {
  font-weight: bold;
  padding: 8px;
}

#volume {
  font-weight: bold;
  padding: 6px;
}

#battery {
  padding: 6px;
}

/* #battery.charging,
#battery.plugged {
} */

@keyframes blink {
  to {
    background-color: @error;
    color: @on_error;
  }
}

#battery.critical:not(.charging) {
  animation-name: blink;
  animation-duration: 0.5s;
  animation-timing-function: linear;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}

#cpu {
  padding: 6px;
  color: white;
}

/* #cpu #cpu-label {
} 
#cpu.warning {
}

#cpu.critical {
} */

#memory {
  padding: 6px;
  transition: all 200ms;
}
/* #memory.medium {
  color: yellow;
}
#memory.high {
  color: red;
} */

#backlight {
  background-color: @surface_container_high;
  color: @on_surface;
}

#network {
  padding: 6px;
}

/* #network.disconnected {
} */

.system-info {
  font-size: 12px;
}

#cpu-bar,
#memory-bar {
  min-width: 60px;
  min-height: 6px;
}

#cpu-bar trough {
  background: rgba(88, 91, 112, 0.5);
  border-radius: 3px;
}

#cpu-bar progress {
  background: linear-gradient(to right, #a6e3a1, #f9e2af, #f38ba8);
  border-radius: 3px;
}

#memory-bar trough {
  background: rgba(88, 91, 112, 0.5);
  border-radius: 3px;
}

#memory-bar progress {
  background: linear-gradient(to right, #89b4fa, #cba6f7);
  border-radius: 3px;
}

.volume {
  padding: 6px;
  color: #cba6f7;
}


#workspace-button{
  all: unset;
  background-color: green;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  
  to {
    opacity: 1;
  }
}

#clock,
#battery,
#cpu,
#memory,
#network,
#volume,
#panel {
  animation: fadeIn 300ms ease-in-out;
}

    ";
    provider.load_from_string(&css);
    
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}