import unittest
import struct
from unittest.mock import patch, MagicMock

import sys
import os
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../scripts')))

from generate_test_pcaps import _build_packet, main

class TestGenerateTestPcaps(unittest.TestCase):

    def test_build_packet_returns_bytes(self):
        pkt = _build_packet(1)
        self.assertIsInstance(pkt, bytes)
        self.assertGreater(len(pkt), 0)

    @patch('generate_test_pcaps.Path')
    @patch('generate_test_pcaps.os.environ')
    @patch('generate_test_pcaps.time.time')
    @patch('generate_test_pcaps._build_packet')
    def test_main_writes_file(self, mock_build, mock_time, mock_env, mock_path):
        mock_env.get.return_value = None
        mock_out = MagicMock()
        mock_path.return_value = mock_out
        mock_out.resolve.return_value = mock_out
        mock_out.parents.__getitem__.return_value = mock_out.parent
        mock_out.parent.mkdir.return_value = None

        mock_open = MagicMock()
        mock_file = MagicMock()
        mock_open.return_value.__enter__.return_value = mock_file

        with patch('builtins.open', mock_open):
            mock_build.return_value = b'abc'
            mock_time.return_value = 0
            main()
            mock_open.assert_called_once()
            mock_file.write.assert_any_call(struct.pack('<IHHIIII', 0xA1B2C3D4, 2, 4, 0, 0, 65535, 101))
            self.assertEqual(mock_file.write.call_count, 1 + (3 * 2))

if __name__ == '__main__':
    unittest.main()
