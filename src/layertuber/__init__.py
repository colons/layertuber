import logging
import os
import sys

from .vendor.OpenSeeFace import remedian


logging.basicConfig(
    level=os.environ.get('LOGLEVEL', 'INFO').upper(),
    style='{',
    format='{name}: {message}'
)

sys.path.append(os.path.dirname(remedian.__file__))
