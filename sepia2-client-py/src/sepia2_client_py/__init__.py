# __init__.py

# Make api_pb2 available in the package namespace
import sys
import os

# Add the directory containing this package to sys.path
# so the relative imports in the generated files can work
sys.path.insert(0, os.path.dirname(__file__))

# Now import the modules
from . import api_pb2 as pb
from . import api_pb2_grpc as rpc

__all__ = ['pb', 'rpc']
