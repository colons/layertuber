import argparse
import logging
import os


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
    return parser.parse_args()


def main() -> None:
    from layertuber.tracking.face import FaceTracker
    from layertuber.reporter import Reporter

    args = _parse_args()

    reporter = Reporter()
    tracker = FaceTracker(capture=args.camera, show_features=args.show_features)
    for report in tracker.begin_loop():
        reporter.report(report)

        message = input()
        if message == 'calibrate':
            tracker.calibrate()


if __name__ == '__main__':
    main()
