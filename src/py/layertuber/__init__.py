import argparse
import logging
import os
import sys
from queue import Queue
from threading import Thread
from typing import Optional


os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = 'true'

logging.basicConfig(
    level=os.environ.get('LOGLEVEL', 'INFO').upper(),
    style='{',
    format='{name}: {message}'
)


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', '--camera', type=int, default=0, help=(
        'The index of the camera to use. If your computer has only one webcam, you can leave this at its default 0.'
    ))
    parser.add_argument('--show-features', action='store_true', help=(
        'Show an additional window with your webcam feed and facial feature detection spots overlaid on it.'
    ))
    parser.add_argument('-b', '--background', type=str, default='#00ff00', help=(
        'The background of the viewer, in hex (by default, #00ff00; solid green).'
    ))
    parser.add_argument('-x', '--output-width', type=int, default=800)
    parser.add_argument('-y', '--output-height', type=int, default=600)
    parser.add_argument('rig_path', help=(
        'The path to the .ora file containing the art that makes up your vtuber. In the same directory as the .ora '
        'file, there must also be a similarly-named .ora.layertuber.yaml file describing how the layers are rigged.'
    ))
    return parser.parse_args()


def main() -> None:
    import pygame

    from .rig import InvalidRig, Rig
    from .tracking.face import FaceTracker, TrackerControlEvent
    from .tracking.report import TrackingReport
    from .viewer import Viewer

    args = _parse_args()

    screen = pygame.display.set_mode((args.output_width, args.output_height))

    try:
        rig = Rig(args.rig_path, (args.output_width, args.output_height))
    except InvalidRig as e:
        print(e)
        sys.exit(1)

    report_queue: Queue[Optional[TrackingReport]] = Queue()
    tracker_event_queue: Queue[TrackerControlEvent] = Queue()

    def run_tracker() -> None:
        FaceTracker(
            tracker_event_queue, report_queue, capture=args.camera, show_features=args.show_features
        ).begin_loop()

    tracker_process = Thread(target=run_tracker, daemon=True)
    tracker_process.start()

    def run_viewer() -> None:
        Viewer(rig, report_queue, tracker_event_queue, background=args.background, screen=screen).begin_loop()

    viewer_process = Thread(target=run_viewer)
    viewer_process.start()
    viewer_process.join()


if __name__ == '__main__':
    main()
