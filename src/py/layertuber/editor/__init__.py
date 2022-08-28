import argparse

from .window import main as gtk_main


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument('rig_path')
    return parser.parse_args()


def main() -> None:
    args = _parse_args()
    gtk_main(rig_path=args.rig_path)


if __name__ == '__main__':
    main()
