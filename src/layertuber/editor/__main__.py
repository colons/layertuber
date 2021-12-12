import argparse

from .window import main


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument('rig_path')
    return parser.parse_args()


if __name__ == '__main__':
    args = _parse_args()
    main(rig_path=args.rig_path)
