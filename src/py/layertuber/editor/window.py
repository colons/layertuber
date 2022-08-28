import gi; gi.require_version('Gtk', '3.0')  # noqa
from gi.repository import Gtk

from ..rig.rig import Rig


class EditorWindow(Gtk.Window):
    def __init__(self, rig: Rig) -> None:
        self.rig = Rig
        super().__init__(title='layertuber editor')
        store = Gtk.TreeStore(str)
        tree = Gtk.TreeView(model=store)

        for layer in rig.layers:
            store.append(None, [layer.name])

        column = Gtk.TreeViewColumn('layer', Gtk.CellRendererText(), text=0)
        tree.append_column(column)
        self.add(tree)


def main(rig_path: str) -> None:
    window = EditorWindow(Rig(rig_path, (800, 600)))
    window.show_all()
    window.connect('destroy', Gtk.main_quit)
    Gtk.main()
