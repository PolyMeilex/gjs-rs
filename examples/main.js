import Gio from 'gi://Gio';

// Gio.Settings.get_string("org.gnome.desktop.wm.preferences.theme");
            
let settings = new Gio.Settings({ schema_id: "org.gnome.desktop.wm.preferences" });

console.log(settings.get_string("theme"));

// export default {};
