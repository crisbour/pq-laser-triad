import pytest
from sepia2_client_py import rpc
from sepia2_client_py import pb


def test_message_creation():
    assert pb.Empty().ByteSize() == 0
