import gi; gi.require_version('Gtk', '3.0')  # noqa
from gi.repository import Gtk


def main() -> None:
    window = Gtk.Window(title='layertuber editor')
    window.show()
    window.connect('destroy', Gtk.main_quit)
    Gtk.main()
