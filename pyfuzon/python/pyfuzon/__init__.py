from .pyfuzon import *

__doc__ = pyfuzon.__doc__
if hasattr(pyfuzon, "__all__"):
    __all__ = pyfuzon.__all__

from .matcher import TermMatcher
