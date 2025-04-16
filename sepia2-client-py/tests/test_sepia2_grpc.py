import pytest
from sepia2_client import rpc
from sepia2_client import pb


def test_message_creation():
    assert pb.Empty().ByteSize() == 0
