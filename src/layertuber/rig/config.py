from __future__ import annotations

import logging
from typing import Dict, Optional

from pydantic import BaseModel


logger = logging.getLogger('config')


class ThresholdConfig(BaseModel):
    option: str  # it'd be nice to verify these are present in TrackingReport; enum?
    greater_than: float


class LayerConfig(BaseModel):
    visible: bool = True
    visible_when: Optional[ThresholdConfig]
    invisible_when: Optional[ThresholdConfig]


class RigConfig(BaseModel):
    layers: Dict[str, LayerConfig]
