from .rig.models import Rig
from .tracking.face import LayertubeTracker


rig = Rig()
tracker = LayertubeTracker()


while tracker.reader.is_open():
    face = tracker.get_face()
    if face is not None:
        rig.render(face)
