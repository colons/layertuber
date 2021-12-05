from sys import argv

from .rig import Rig
from .tracking.face import FaceTracker
from .viewer import Viewer


Viewer(
    Rig(argv[-1], (800, 600)),
    FaceTracker(),
).begin_loop()
