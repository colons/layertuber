import argparse

from .rig import Rig
from .tracking.face import FaceTracker
from .viewer import Viewer


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument('-c', '--camera', type=int, default=0)
    parser.add_argument('-w', '--width', type=int, default=800)
    parser.add_argument('-h', '--height', type=int, default=600)
    parser.add_argument('rig_path')
    return parser.parse_args()


if __name__ == '__main__':
    args = _parse_args()
    Viewer(
        Rig(args.rig_path, (args.width, args.height)),
        FaceTracker(capture=args.camera),
    ).begin_loop()
